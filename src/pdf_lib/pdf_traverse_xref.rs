// Copyright (c) 2019-2020 SRI International.
// All rights reserved.
//
//    This file is part of the Parsley parser.
//
//    Parsley is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    Parsley is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// A very basic PDF parser.
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate log_panics;

use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::rc::Rc;

use log::{log, Level};

use crate::pcore::parsebuffer::{
    ErrorKind, LocatedVal, Location, ParseBuffer, ParseBufferT, ParsleyParser,
};
use crate::pcore::transforms::{BufferTransformT, RestrictView};
use crate::pdf_lib::pdf_file::{HeaderP, StartXrefP, TrailerP, XrefSectP};
use crate::pdf_lib::pdf_obj::{IndirectP, PDFObjContext, PDFObjT};
use crate::pdf_lib::pdf_streams::{ObjStreamP, XrefEntStatus, XrefEntT, XrefStreamP};

/* from: https://osr.jpl.nasa.gov/wiki/pages/viewpage.action?spaceKey=SD&title=TA2+PDF+Safe+Parser+Evaluation

CRITICAL	This error level must be used when the TA2 parser is going to terminate parsing based on
            unexpected input.
            => panic!
ERROR       This error level must be used when the TA2 parser has found invalid data to parse, but
            intends to continue parsing. ERROR or CRITICAL must be used to flag any "unsafe parsing
            events"
            => error!
WARNING     This error level can be used when the TA2 parser has found unexpected data to parse.
            This error level can be used to flag safe, but unexpected parsing events.
            => warn!
INFO    	This error level must be used to instrument components being parsed by the PDF parser.
            Each component should have some INFO parser output.
            => info!
DEBUG   	Any messages that the TA2 parser needs to output for debug information should use this
            error level.
            => debug!

Note: Rust level trace! is not included.  Those messages will print without the TA3 preamble.
*/
// use this macro to log messages with position argument:
macro_rules! ta3_log {
    ($lvl:expr, $pos:expr, $($arg:tt)+) => ({
        log!($lvl, "at {:>10} - {}", $pos, format_args!($($arg)+))
    })
}

macro_rules! exit_log {
    ($pos:expr, $($arg:tt)+) => ({
        log!(Level::Error, "at {:>10} - {}", $pos, format_args!($($arg)+));
        process::exit(1)
    })
}

pub struct FileInfo {
    path:        std::path::PathBuf,
    pdf_hdr_ofs: usize,
}

impl FileInfo {
    pub fn file_offset(&self, o: usize) -> usize { self.pdf_hdr_ofs + o }
    pub fn path(&self) -> &std::path::Path { &self.path }
}

enum ObjInfo {
    InFile { id: usize, gen: usize, ofs: usize },
    Stream { id: usize, gen: usize },
}

type RootObjRef = Rc<LocatedVal<PDFObjT>>;
type XRefSectInfo = (Vec<LocatedVal<XrefEntT>>, Option<RootObjRef>, Option<usize>);

// Parse a single xref stream.  It assumes that the parse cursor is
// positioned at the stream object location, either via a startxref, a
// /Prev, or a XRefStm entry in a trailer.
fn parse_xref_stream(
    fi: &FileInfo, ctxt: &mut PDFObjContext, pb: &mut dyn ParseBufferT,
) -> Option<XRefSectInfo> {
    let mut xrefs = Vec::new();
    let mut root = None;
    let mut prev = None;

    let mut sp = IndirectP::new(ctxt);
    let xref_obj_loc = pb.get_cursor();
    let xref_obj = sp.parse(pb);
    if let Err(e) = xref_obj {
        ta3_log!(
            Level::Info,
            fi.file_offset(pb.get_cursor()),
            "Could not parse object for xref stream in {} at file-offset {} (pdf-offset {}): {}",
            fi.path.display(),
            fi.file_offset(e.start()),
            e.start(),
            e.val()
        );
        return None
    };
    let xref_obj = xref_obj.unwrap();
    if let PDFObjT::Stream(ref s) = xref_obj.val().obj().val() {
        ta3_log!(
            Level::Info,
            fi.file_offset(0),
            "parsing xref stream ({},{})",
            xref_obj.val().num(),
            xref_obj.val().gen()
        );

        let content = s.stream().val();
        let mut xref_buf = ParseBuffer::new_view(pb, content.start(), content.size());
        let mut xp = XrefStreamP::new(ctxt.is_encrypted(), s);
        let xref_stm = xp.parse(&mut xref_buf);
        if let Err(e) = xref_stm {
            ta3_log!(
                Level::Info,
                fi.file_offset(pb.get_cursor()),
                "Cannot parse xref stream in {} at file-offset {} (pdf-offset {}): {}",
                fi.path().display(),
                fi.file_offset(e.start()),
                e.start(),
                e.val()
            );
            return None
        }
        let xref_stm = xref_stm.unwrap();
        ta3_log!(
            Level::Info,
            fi.file_offset(pb.get_cursor()),
            "Found xref stream with {} entries at {}.",
            xref_stm.val().ents().len(),
            xref_obj_loc
        );
        // Convert the XrefStreamT into XrefEntTs so that they
        // can be merged with any XrefSectT.
        for e in xref_stm.val().ents() {
            xrefs.push(*e)
        }
        // Extract trailer-like entries from the stream dict.
        root = match xref_stm.val().dict().get(b"Root") {
            Some(rt) => Some(Rc::clone(rt)),
            None => None,
        };
        prev = xref_stm.val().dict().get_usize(b"Prev");
    }
    Some((xrefs, root, prev))
}

// Parses a single xref section.  This section could be (a) an xref
// table with trailer, (b) an xref stream, or (c) a hybrid-reference,
// with an xref table and a trailer with a pointer to an xref stream.
// It assumes that the parse cursor is set appropriately, either via
// handling a startxref or a /Prev.
fn parse_xref_section(
    fi: &FileInfo, ctxt: &mut PDFObjContext, pb: &mut dyn ParseBufferT,
) -> Option<XRefSectInfo> {
    // save the cursor
    let start = pb.get_cursor();

    // First try to parse a conventional xref table.
    let mut p = XrefSectP;
    let xrsect = p.parse(pb);
    if let Err(e) = xrsect {
        ta3_log!(
            Level::Info,
            fi.file_offset(start),
            "Error parsing xref table, checking for xref stream at {}: {}",
            start,
            e.val()
        );
        // No xref section; check for xref stream.
        pb.set_cursor_unsafe(start);
        return parse_xref_stream(fi, ctxt, pb)
    }
    let xrs = xrsect.unwrap();
    let mut xrefs = Vec::new();
    // Convert the XrefSectT into XrefEntTs so that they can be merged
    // with any XRefStms.
    for e in xrs.val().ents() {
        xrefs.push(e);
    }
    // Get trailer following the xref table.
    match pb.scan(b"trailer") {
        Ok(_) => {
            let c = pb.get_cursor();
            ta3_log!(Level::Info, fi.file_offset(c), "Found trailer at {}.", c)
        },
        Err(e) => {
            ta3_log!(
                Level::Info,
                fi.file_offset(pb.get_cursor()),
                "No trailer found: {}",
                e.val()
            );
            return Some((xrefs, None, None))
        },
    }
    let mut p = TrailerP::new(ctxt);
    let tloc = pb.get_cursor();
    let t = p.parse(pb);
    if let Err(e) = t {
        ta3_log!(
            Level::Error,
            fi.file_offset(tloc),
            "Cannot parse trailer: {}",
            e.val()
        );
        return Some((xrefs, None, None))
    }
    let t = t.unwrap();

    // extract trailer-derived info: /Root and /Prev
    let prev = t.val().dict().get_usize(b"Prev");
    let root = t.val().dict().get(b"Root");
    let root = match root {
        Some(rt) => Some(Rc::clone(rt)),
        None => None,
    };
    // check for encryption
    if t.val().dict().get(b"Encrypt").is_some() {
        ctxt.set_encrypted();
    }

    // Section 7.5.8.4: check for XRefStm in hybrid-reference
    // file. TODO: This should be conditioned on a flag for
    // versions < PDF-1.5.
    let xrefstm = t.val().dict().get_usize(b"XRefStm");
    let xrefstm_loc = t.start();
    if let Some(xrstart) = xrefstm {
        ta3_log!(
            Level::Info,
            fi.file_offset(xrefstm_loc),
            "Found hybrid specifier for XRefStm located at {}",
            xrstart,
        );
        match pb.set_cursor(xrstart) {
            Ok(()) => {
                let xref_stm = parse_xref_stream(fi, ctxt, pb);
                if let Some((xrents, _root, _prev)) = xref_stm {
                    // Ignore the /Root and /Prev specifiers coming from
                    // the XRefStm.  (This seems to be implicit in the
                    // spec.)
                    for e in xrents {
                        xrefs.push(e);
                    }
                } else {
                    exit_log!(
                        fi.file_offset(xrefstm_loc),
                        "/XRefStm points to an invalid or encrypted XRefStm at {}",
                        xrstart
                    );
                }
            },
            Err(_) => exit_log!(
                fi.file_offset(xrefstm_loc),
                "/XRefStm specifies out-of-bounds offset {}",
                xrstart
            ),
        }
    }
    Some((xrefs, root, prev))
}

// This assumes that the parse cursor is set at the startxref
// location.  It traverses a chain of xref tables (conventional or
// hybrid) or xref streams to return the xref entries and a root.
fn get_xref_info(
    fi: &FileInfo, ctxt: &mut PDFObjContext, pb: &mut dyn ParseBufferT,
) -> (Vec<LocatedVal<XrefEntT>>, RootObjRef) {
    // Collect all xref tables or streams, following the /Prev chain.
    let mut xrefs = Vec::new();
    let mut root = None;

    let mut cursorset = BTreeSet::new(); // to prevent infinite loops
    let mut idset = BTreeSet::new(); // to keep newest entries

    let mut next = pb.get_cursor();
    loop {
        if !cursorset.insert(next) {
            exit_log!(
                fi.file_offset(next),
                "Xref cycle detected at offset {}!",
                next,
            );
        }
        if !pb.check_cursor(next) {
            exit_log!(
                fi.file_offset(next),
                "Xref offset {} is out of bounds!",
                next,
            );
        }
        pb.set_cursor_unsafe(next);
        // Check for a conventional or hybrid xref section.
        let mut xinfo = parse_xref_section(fi, ctxt, pb);
        if xinfo.is_none() {
            xinfo = parse_xref_stream(fi, ctxt, pb);
        }
        if xinfo.is_none() {
            exit_log!(
                fi.file_offset(next),
                "No xref found at specified offset {}!",
                next,
            );
        }
        let (ents, rt, prev) = xinfo.unwrap();
        // update root
        if root.is_none() {
            if rt.is_some() {
                root = rt;
            } else {
                exit_log!(
                    fi.file_offset(next),
                    "No Root specified in xref at {}!",
                    next,
                );
            }
        }
        // add entries
        for e in ents {
            let id = (e.val().obj(), e.val().gen());
            if idset.insert(id) {
                // This is the newest version of the object.
                xrefs.push(e)
            }
        }
        // goto prev
        match prev {
            None => break,
            Some(p) => next = p, // continue
        }
    }
    if root.is_none() {
        exit_log!(fi.file_offset(next), "No root object found!!",);
    }
    (xrefs, root.unwrap())
}

// Get the in-use object locations from the xref entries.
fn info_from_xref_entries(fi: &FileInfo, xref_ents: &[LocatedVal<XrefEntT>]) -> Vec<ObjInfo> {
    let mut id_offsets: Vec<ObjInfo> = Vec::new();
    for o in xref_ents {
        let ent = o.val();
        match ent.status() {
            XrefEntStatus::Free { next } => ta3_log!(
                Level::Info,
                fi.file_offset(o.loc_start()),
                "   free object (next is {}).",
                *next
            ),
            XrefEntStatus::InUse { file_ofs } => {
                ta3_log!(
                    Level::Info,
                    fi.file_offset(o.loc_start()),
                    "   inuse object ({},{}) at {}{}.",
                    ent.obj(),
                    ent.gen(),
                    *file_ofs,
                    if *file_ofs == 0 {
                        " (possibly invalid entry)"
                    } else {
                        ""
                    }
                );
                id_offsets.push(ObjInfo::InFile {
                    id:  ent.obj(),
                    gen: ent.gen(),
                    ofs: *file_ofs,
                })
            },
            XrefEntStatus::InStream {
                stream_obj,
                obj_index,
            } => {
                ta3_log!(
                    Level::Info,
                    fi.file_offset(o.loc_start()),
                    "   instream object ({},{}) at index {} in stream {}.",
                    ent.obj(),
                    ent.gen(),
                    obj_index,
                    stream_obj
                );
                id_offsets.push(ObjInfo::Stream {
                    id:  *stream_obj,
                    gen: 0, // object streams have an implicit generation of 0
                })
            },
        }
    }
    id_offsets
}

// Parse the objects at their specified locations, which updates the
// context with their identities.  Also validate that the identity of
// the parsed object matches the identity expected from the xref
// information.
fn parse_objects(
    fi: &FileInfo, ctxt: &mut PDFObjContext, obj_infos: &[ObjInfo], pb: &mut dyn ParseBufferT,
) {
    // Get the outermost objects at each offset in the xref table.
    // These have to be indirect/labelled objects.  Collect any
    // references to object streams since they will need to be parsed
    // subsequently.
    let mut obj_streams = BTreeSet::new();
    // Some stream objects use references for their lengths, so
    // collect them for a second pass.
    let mut second_pass = Vec::new();
    for obj in obj_infos.iter() {
        match obj {
            ObjInfo::Stream { id, gen } => {
                let _ = obj_streams.insert((*id, *gen));
            },
            ObjInfo::InFile { id, gen, ofs } => {
                // If we've already parsed this object (e.g. it is one of the
                // xref-streams), skip it.  This can happen because the
                // xref-streams are objects themselves, and their catalog of
                // objects can include an entry for themselves.
                if ctxt.lookup_obj((*id, *gen)).is_some() {
                    ta3_log!(
                        Level::Info,
                        fi.file_offset(*ofs),
                        "skipping already parsed object ({},{})",
                        id,
                        gen
                    );
                    continue
                }

                let mut p = IndirectP::new(ctxt);
                let ofs = *ofs;
                ta3_log!(
                    Level::Info,
                    fi.file_offset(ofs),
                    "parsing object ({},{}) at {}file-offset {} (pdf-offset {})",
                    id,
                    gen,
                    if ofs == 0 { "(possibly invalid) " } else { "" },
                    fi.file_offset(ofs),
                    ofs
                );
                if !pb.check_cursor(ofs) {
                    exit_log!(
                        fi.file_offset(ofs),
                        "object offset {} is out of bounds!",
                        ofs,
                    );
                }
                pb.set_cursor_unsafe(ofs);
                let lobj = match p.parse(pb) {
                    Ok(o) => o,
                    Err(e) => {
                        if let ErrorKind::InsufficientContext = e.val() {
                            second_pass.push((id, gen, ofs));
                            continue
                        } else {
                            exit_log!(
                            fi.file_offset(e.start()),
                            "Cannot parse object ({},{}) at file-offset {} (pdf-offset {}) in {}: {}",
                            id,
                            gen,
                            fi.file_offset(e.start()),
                            e.start(),
                            fi.path().display(),
                            e.val()
                        )
                        }
                    },
                };
                let io = lobj.unwrap(); // unwrap LocatedVal.

                // Validate that the object is what we expect.
                // TODO: this constraint should be enforced in the library.
                if (io.num(), io.gen()) != (*id, *gen) {
                    exit_log!(
                        fi.file_offset(ofs),
                        "unexpected object ({},{}) found: expected ({},{}) from xref entry",
                        io.num(),
                        io.gen(),
                        id,
                        gen
                    )
                }
            },
        }
    }

    // Do the second pass over objects that needed it.
    for (id, gen, ofs) in second_pass {
        // If we've already parsed this object, skip it.
        if ctxt.lookup_obj((*id, *gen)).is_some() {
            ta3_log!(
                Level::Info,
                fi.file_offset(ofs),
                "skipping already parsed object ({},{})",
                id,
                gen
            );
            continue
        }
        let mut p = IndirectP::new(ctxt);
        let ofs = ofs;
        ta3_log!(
            Level::Info,
            fi.file_offset(ofs),
            "second parse of object ({},{}) at {}file-offset {} (pdf-offset {})",
            id,
            gen,
            if ofs == 0 { "(possibly invalid) " } else { "" },
            fi.file_offset(ofs),
            ofs
        );
        if !pb.check_cursor(ofs) {
            exit_log!(
                fi.file_offset(ofs),
                "object offset {} is out of bounds!",
                ofs,
            );
        }
        pb.set_cursor_unsafe(ofs);
        let lobj = match p.parse(pb) {
            Ok(o) => o,
            Err(e) => exit_log!(
                fi.file_offset(e.start()),
                "Cannot parse object ({},{}) at file-offset {} (pdf-offset {}) in {}: {}",
                id,
                gen,
                fi.file_offset(e.start()),
                e.start(),
                fi.path().display(),
                e.val()
            ),
        };
        let io = lobj.unwrap(); // unwrap LocatedVal.
                                // Validate that the object is what we expect.
                                // TODO: this constraint should be enforced in the library.
        if (io.num(), io.gen()) != (*id, *gen) {
            exit_log!(
                fi.file_offset(ofs),
                "unexpected object ({},{}) found: expected ({},{}) from xref entry",
                io.num(),
                io.gen(),
                id,
                gen
            )
        }
    }

    // Now do the pass over the object streams, collecting only
    // those that are actually defined.  In this pass, we only use
    // immutable borrows on ctxt.
    let mut defined_obj_streams = BTreeSet::new();
    for id in obj_streams.iter() {
        match ctxt.lookup_obj(*id) {
            None => {
                ta3_log!(
                    Level::Warn,
                    fi.file_offset(0),
                    "stream object ({},{}) not found",
                    id.0,
                    id.1
                );
            },
            Some(obj) => {
                if let PDFObjT::Stream(_) = obj.val() {
                    let _ = defined_obj_streams.insert((id, Rc::clone(obj)));
                } else {
                    ta3_log!(
                        Level::Warn,
                        fi.file_offset(0),
                        "object ({},{}) is not a stream",
                        id.0,
                        id.1
                    );
                }
            },
        }
    }

    // Now parse the object streams.
    for (id, obj) in defined_obj_streams.iter() {
        ta3_log!(
            Level::Info,
            fi.file_offset(0),
            "parsing object stream ({},{})",
            id.0,
            id.1
        );

        if let PDFObjT::Stream(ref s) = obj.val() {
            let content = s.stream().val();
            let mut obj_buf = ParseBuffer::new_view(pb, content.start(), content.size());
            let mut op = ObjStreamP::new(ctxt, s);
            let obj_stm = op.parse(&mut obj_buf);
            if let Err(e) = obj_stm {
                ta3_log!(
                    Level::Error,
                    fi.file_offset(pb.get_cursor()),
                    "Cannot parse object stream in {} at file-offset {} (pdf-offset {}): {}",
                    fi.path().display(),
                    fi.file_offset(e.start()),
                    e.start(),
                    e.val()
                );
            // TODO: we could stop parsing here, but since this is
            // a nested parse, we opt to continue for now.
            } else {
                let obj_stm = obj_stm.unwrap();
                for o in obj_stm.val().objs() {
                    ta3_log!(
                        Level::Info,
                        fi.file_offset(content.start()),
                        "Parsed object ({},{}) of {} from stream ({},{}).",
                        o.val().num(),
                        o.val().gen(),
                        obj_stm.val().objs().len(),
                        id.0,
                        id.1
                    );
                }
            }
        }
    }
}

pub fn parse_file(test_file: &str) -> (FileInfo, PDFObjContext, (usize, usize)) {
    // Print current path
    let path = env::current_dir();
    if path.is_err() {
        exit_log!(0, "Cannot get current dir!");
    }
    let mut path = path.unwrap();
    path.push(test_file);
    let display = path.as_path().display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path.as_path()) {
        Err(why) => {
            exit_log!(0, "Couldn't open {}: {}", display, why.to_string());
        },
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut v = Vec::new();
    if let Err(why) = file.read_to_end(&mut v) {
        exit_log!(0, "Couldn't read {}: {}", display, why.to_string());
    };

    let mut pb = ParseBuffer::new(v);

    // Handle leading garbage.
    let pdf_hdr_ofs = match pb.scan(b"%PDF-") {
        Ok(nbytes) => {
            if nbytes != 0 {
                ta3_log!(
                    Level::Info,
                    nbytes,
                    "Found {} bytes of leading garbage, dropping from buffer.",
                    nbytes
                );
                let size = pb.remaining();
                // Restrict the view to the pdf content.
                let mut view = RestrictView::new(nbytes, size);
                pb = view.transform(&pb).unwrap();
            };
            nbytes
        },
        Err(e) => {
            exit_log!(0, "Cannot find PDF magic: {}", e.val());
        },
    };
    let fi = FileInfo {
        pdf_hdr_ofs,
        path: path.to_path_buf(),
    };

    let buflen = pb.remaining();
    let mut p = HeaderP;
    let hdr = p.parse(&mut pb);
    if let Err(e) = hdr {
        exit_log!(
            fi.file_offset(0),
            "Unable to parse header from {}: {}",
            fi.path().display(),
            e.val()
        );
    }
    // Todo: some sanity check of header.

    // From end of buffer, scan backwards for %EOF, if present.
    pb.set_cursor_unsafe(buflen);
    let eof = pb.backward_scan(b"%%EOF");
    if let Err(e) = eof {
        ta3_log!(
            Level::Info,
            fi.file_offset(0),
            "No %%EOF in {}: {}",
            fi.path().display(),
            e.val()
        );
    } else {
        let eof_ofs = buflen - eof.unwrap();
        ta3_log!(
            Level::Info,
            fi.file_offset(eof_ofs),
            "Found %%EOF at file-offset {} (pdf-offset {}).",
            fi.file_offset(eof_ofs),
            eof_ofs
        );
    }

    // Scan backward for startxref.
    let sxref = pb.backward_scan(b"startxref");
    if let Err(e) = sxref {
        exit_log!(
            0,
            "Could not find startxref in {}: {}",
            fi.path().display(),
            e.val()
        );
    }
    let sxref_ofs = buflen - sxref.unwrap();
    ta3_log!(
        Level::Info,
        fi.file_offset(sxref_ofs),
        "Found startxref at file-offset {} (pdf-offset {}).",
        fi.file_offset(sxref_ofs),
        sxref_ofs
    );
    let mut p = StartXrefP;
    let sxref = p.parse(&mut pb);
    if let Err(e) = sxref {
        exit_log!(
            fi.file_offset(sxref_ofs),
            "Could not parse startxref in {} at file-offset {} (pdf-offset {}): {}",
            fi.path().display(),
            fi.file_offset(e.start()),
            e.start(),
            e.val()
        );
    }
    let sxref = sxref.unwrap();
    let sxref_loc_start = sxref.loc_start();
    ta3_log!(
        Level::Info,
        fi.file_offset(sxref_loc_start),
        " startxref span (in file-offsets): {}..{}.",
        fi.file_offset(sxref.loc_start()),
        fi.file_offset(sxref.loc_end())
    );
    let sxref = sxref.unwrap();
    let sxref_offset = sxref.offset();
    ta3_log!(
        Level::Info,
        fi.file_offset(sxref_loc_start),
        "startxref points to file-offset {} (pdf-offset {}) for xref",
        fi.file_offset(sxref_offset),
        sxref_offset
    );

    // Create the pdf object context.
    // TODO: control max-depth via command-line option.
    let mut ctxt = PDFObjContext::new(50);

    // Parse xref table at that offset.
    if !pb.check_cursor(sxref_offset) {
        exit_log!(
            fi.file_offset(sxref_loc_start),
            "startxref specifies out-of-bounds offset {}",
            sxref_offset
        );
    }
    pb.set_cursor_unsafe(sxref_offset);
    let (xref_ents, root_ref) = get_xref_info(&fi, &mut ctxt, &mut pb);
    ta3_log!(
        Level::Info,
        fi.file_offset(pb.get_cursor()),
        "Found {} objects in xref table.",
        xref_ents.len()
    );

    let id_offsets = info_from_xref_entries(&fi, &xref_ents);

    // Parse the objects using their xref entries, and put them into the context.
    parse_objects(&fi, &mut ctxt, &id_offsets, &mut pb);

    let root_id: (usize, usize) = if let PDFObjT::Reference(r) = root_ref.val() {
        r.id()
    } else {
        // Is there any case where this is not the case?  Should
        // this constraint be part of the safe subset specification?
        exit_log!(
            fi.file_offset(root_ref.loc_start()),
            "Root object is not a reference!"
        );
    };

    (fi, ctxt, root_id)
}