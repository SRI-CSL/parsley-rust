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

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate log_panics;

use std::collections::{BTreeSet, VecDeque};
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::path::Path;
use std::process;
use std::rc::Rc;

use env_logger::Builder;
use log::{Level, LevelFilter};

use parsley_rust::pcore::parsebuffer::{
    LocatedVal, Location, ParseBuffer, ParseBufferT, ParsleyParser,
};
use parsley_rust::pcore::transforms::{BufferTransformT, RestrictView};
use parsley_rust::pdf_lib::pdf_file::{
    HeaderP, StartXrefP, TrailerP, TrailerT, XrefSectP, XrefSectT,
};
use parsley_rust::pdf_lib::pdf_obj::{IndirectP, PDFObjContext, PDFObjT};
use parsley_rust::pdf_lib::pdf_streams::{XrefEntStatus, XrefEntT, XrefStreamP, XrefStreamT};

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

struct FileInfo<'a> {
    display:     std::path::Display<'a>,
    pdf_hdr_ofs: usize,
}

impl FileInfo<'_> {
    fn file_offset(&self, o: usize) -> usize { self.pdf_hdr_ofs + o }
    fn display(&self) -> &std::path::Display { &self.display }
}

struct ObjInfo {
    id:  usize,
    gen: usize,
    ofs: usize,
}

// This assumes that the parse cursor is set at the startxref location.
// The xref tables are arranged with the newest first and oldest last.
fn parse_xref_with_trailer(
    fi: &FileInfo, ctxt: &mut PDFObjContext, pb: &mut dyn ParseBufferT,
) -> Option<(Vec<LocatedVal<XrefSectT>>, TrailerT)> {
    // Collect all xref tables, following the /Prev entries.  This is
    // not quite the right thing to do for linearized PDF from an
    // application point-of-view, since they should be loaded lazily,
    // but that's a difficult use case to support currently.
    let mut xrefs = Vec::new();
    let mut cursorset = BTreeSet::new(); // to prevent infinite loops
    let mut trlr = None;
    loop {
        let mut p = XrefSectP;
        let xref = p.parse(pb);
        if let Err(ref e) = xref {
            ta3_log!(
                Level::Info,
                fi.file_offset(pb.get_cursor()),
                "Could not parse xref table in {} at file-offset {} (pdf-offset {}): {}",
                fi.display(),
                fi.file_offset(e.start()),
                e.start(),
                e.val()
            );
            return None
        }
        xrefs.push(xref.unwrap());
        // Get trailer following the xref table.
        match pb.scan(b"trailer") {
            Ok(_) => ta3_log!(
                Level::Info,
                fi.file_offset(pb.get_cursor()),
                "Found trailer."
            ),
            Err(e) => {
                ta3_log!(
                    Level::Info,
                    fi.file_offset(e.start()),
                    "Cannot find trailer: {}",
                    e.val()
                );
                return None
            },
        }
        let mut p = TrailerP::new(ctxt);
        let tloc = pb.get_cursor();
        let t = p.parse(pb);
        if let Err(e) = t {
            exit_log!(fi.file_offset(tloc), "Cannot parse trailer: {}", e.val());
        }
        let t = t.unwrap();

        // update trailer-derived info.
        let prev = t.val().dict().get_usize(b"Prev");
        match trlr {
            None => trlr = Some(t),
            Some(_) => {
                // Newer trailers make older ones redundant.
                // TODO: should we check any constraints here?
            },
        }
        if let Some(start) = prev {
            // Go to the next xref table, after ensuring we are not in
            // an infinite loop.
            // TODO: ensure that there is an %%EOF marker following
            // the trailer.
            if cursorset.insert(start) {
                // This is a new location.
                pb.set_cursor(start);
                continue
            }
            exit_log!(
                fi.file_offset(tloc),
                "Infinite /Prev loop detected in xref tables."
            )
        }
        // There is no Prev, so this was the last xref table.
        break
    }
    return Some((xrefs, trlr.unwrap().unwrap()))
}

// This assumes that the parse cursor is set at the startxref location.
fn parse_xref_stream(
    fi: &FileInfo, ctxt: &mut PDFObjContext, pb: &mut dyn ParseBufferT,
) -> Option<Vec<LocatedVal<XrefStreamT>>> {
    let mut xrefs = Vec::new();
    let mut cursorset = BTreeSet::new(); // to prevent infinite loops
    loop {
        let mut sp = IndirectP::new(ctxt);
        let xref_obj = sp.parse(pb);
        if let Err(e) = xref_obj {
            ta3_log!(
                Level::Info,
                fi.file_offset(pb.get_cursor()),
                "Could not parse object for xref stream in {} at file-offset {} (pdf-offset {}): {}",
                fi.display(),
                fi.file_offset(e.start()),
                e.start(),
                e.val()
            );
            return None
        };
        let xref_obj = xref_obj.unwrap();
        if let PDFObjT::Stream(ref s) = xref_obj.val().obj().val() {
            let mut xp = XrefStreamP::new(s);
            let xref_stm = xp.parse(pb);
            if let Err(e) = xref_stm {
                ta3_log!(
                    Level::Info,
                    fi.file_offset(pb.get_cursor()),
                    "Object is not a xref stream in {} at file-offset {} (pdf-offset {}): {}",
                    fi.display(),
                    fi.file_offset(e.start()),
                    e.start(),
                    e.val()
                );
                return None
            } else {
                ta3_log!(
                    Level::Info,
                    fi.file_offset(pb.get_cursor()),
                    "Found xref stream.",
                );
                let xref_stm = xref_stm.unwrap();
                let prev = xref_stm.val().dict().get_usize(b"Prev");
                xrefs.push(xref_stm);
                if let Some(start) = prev {
                    // Go to the next xref table, after ensuring we
                    // are not in an infinite loop.
                    if cursorset.insert(start) {
                        pb.set_cursor(start);
                        continue
                    }
                    exit_log!(
                        fi.file_offset(start),
                        "Infinite /Prev loop detected in xref tables."
                    )
                }
                // There is no Prev, so this was the last xref table.
                break
            }
        }
        ta3_log!(
            Level::Info,
            fi.file_offset(pb.get_cursor()),
            "Could not find xref stream in {} at file-offset {} (pdf-offset {})",
            fi.display(),
            fi.file_offset(pb.get_cursor()),
            pb.get_cursor()
        );
        return None
    }
    return Some(xrefs)
}

// This assumes that the parse cursor is set at the startxref
// location.  This tries getting it from the xref table and trailer,
// failing which it tries getting it from a xref stream.
fn get_xref_info(
    fi: &FileInfo, ctxt: &mut PDFObjContext, pb: &mut dyn ParseBufferT,
) -> (Vec<LocatedVal<XrefEntT>>, Rc<LocatedVal<PDFObjT>>) {
    let cursor = pb.get_cursor();
    let info = parse_xref_with_trailer(fi, ctxt, pb);
    if info.is_some() {
        let (xrsects, trailer) = info.unwrap();
        // Collect the entries from possibly multiple xref tables,
        // keeping the newest ones.
        let mut xrents = Vec::new();
        let mut idset = BTreeSet::new();
        for ls in xrsects {
            let xrsect = ls.val();
            for e in xrsect.ents() {
                let id = (e.val().obj(), e.val().gen());
                if idset.insert(id) {
                    // This is the newest version of the object.
                    xrents.push(e)
                }
            }
        }
        let root_ref = match trailer.dict().get(b"Root") {
            Some(rt) => Rc::clone(rt),
            None => {
                // FIXME: Make this a constraint on the Trailer parser.
                exit_log!(
                    fi.file_offset(pb.get_cursor()),
                    "No root reference found in trailer!"
                );
            },
        };
        return (xrents, root_ref)
    }

    pb.set_cursor(cursor);
    let xrefs = parse_xref_stream(fi, ctxt, pb);
    if xrefs.is_none() {
        exit_log!(
            fi.file_offset(cursor),
            "No valid xref information found at startxref."
        )
    }
    let xrstrms = xrefs.unwrap();
    let mut root = None;
    let mut xrents = Vec::new();
    let mut idset = BTreeSet::new();
    for xrstrm in xrstrms {
        let xrstrm = xrstrm.val();
        for e in xrstrm.ents() {
            let id = (e.val().obj(), e.val().gen());
            if idset.insert(id) {
                // This is the newest version of the object.
                xrents.push(*e)
            }
        }
        if root.is_none() {
            match xrstrm.dict().get(b"Root") {
                Some(rt) => root = Some(Rc::clone(rt)),
                None => {},
            }
        }
    }
    if root.is_none() {
        exit_log!(
            fi.file_offset(pb.get_cursor()),
            "No root reference found in xref stream dictionary!"
        );
    };
    let root = root.unwrap();
    return (xrents, root)
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
                id_offsets.push(ObjInfo {
                    id:  ent.obj(),
                    gen: ent.gen(),
                    ofs: (*file_ofs).try_into().unwrap(),
                })
            },
            XrefEntStatus::InStream { .. } => {
                // TODO:
                assert!(false)
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
    // These have to be indirect/labelled objects.
    let mut objs = Vec::new();
    for ObjInfo { id, gen, ofs } in obj_infos.iter() {
        let mut p = IndirectP::new(ctxt);
        let ofs = (*ofs).try_into().unwrap();
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
        pb.set_cursor(ofs);
        let lobj = p.parse(pb);
        if let Err(e) = lobj {
            exit_log!(
                fi.file_offset(e.start()),
                "Cannot parse object ({},{}) at file-offset {} (pdf-offset {}) in {}: {}",
                id,
                gen,
                fi.file_offset(e.start()),
                e.start(),
                fi.display(),
                e.val()
            );
        }
        let io = lobj.unwrap().unwrap(); // unwrap Result, LocatedVal.
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
        objs.push(io)
    }
}

// Perform a breadth-first traversal of the root object, logging
// each object type and location as we go.
fn dump_root(fi: &FileInfo, ctxt: &PDFObjContext, root_obj: &Rc<LocatedVal<PDFObjT>>) {
    debug!("Beginning breadth-first traversal of root object:");

    let log_obj = |t: &str, loc: &dyn Location, depth: u32| {
        ta3_log!(
            Level::Info,
            fi.file_offset(loc.loc_start()),
            "depth:{} type:{} start-file-offset:{} end-file-offset:{}  ",
            depth,
            t,
            fi.file_offset(loc.loc_start()),
            fi.file_offset(loc.loc_end())
        )
    };

    let mut obj_queue = VecDeque::new();
    obj_queue.push_back((Rc::clone(root_obj), 0)); // depth 0
    let mut processed = BTreeSet::new();
    processed.insert(Rc::clone(root_obj));
    while obj_queue.len() > 0 {
        let o = obj_queue.pop_front();
        if o.is_none() {
            break
        };
        let (o, depth) = o.unwrap();

        match o.val() {
            PDFObjT::Array(a) => {
                log_obj("array", o.as_ref() as &dyn Location, depth);
                for elem in a.objs() {
                    if !processed.contains(elem) {
                        obj_queue.push_back((Rc::clone(elem), depth + 1));
                        processed.insert(Rc::clone(elem));
                    }
                }
            },
            PDFObjT::Dict(d) => {
                log_obj("dict", o.as_ref() as &dyn Location, depth);
                for (_, v) in d.map().iter() {
                    if !processed.contains(v) {
                        obj_queue.push_back((Rc::clone(v), depth + 1));
                        processed.insert(Rc::clone(v));
                    }
                }
            },
            PDFObjT::Stream(s) => {
                log_obj("stream", o.as_ref() as &dyn Location, depth);
                for (_, v) in s.dict().val().map().iter() {
                    // TODO: print key names
                    if !processed.contains(v) {
                        obj_queue.push_back((Rc::clone(v), depth + 1));
                        processed.insert(Rc::clone(v));
                    }
                }
            },
            PDFObjT::Reference(r) => {
                let loc = o.as_ref() as &dyn Location;
                log_obj("ref", loc, depth);
                match ctxt.lookup_obj(r.id()) {
                    Some(obj) => {
                        if !processed.contains(obj) {
                            obj_queue.push_back((Rc::clone(obj), depth + 1));
                            processed.insert(Rc::clone(obj));
                        }
                    },
                    None => ta3_log!(
                        Level::Warn,
                        fi.file_offset(o.start()),
                        " ref ({},{}) does not point to a defined object!",
                        r.num(),
                        r.gen()
                    ),
                }
            },
            PDFObjT::Boolean(_) => log_obj("boolean", o.as_ref() as &dyn Location, depth),
            PDFObjT::String(_) => log_obj("string", o.as_ref() as &dyn Location, depth),
            PDFObjT::Name(_) => log_obj("name", o.as_ref() as &dyn Location, depth),
            PDFObjT::Null(_) => log_obj("null", o.as_ref() as &dyn Location, depth),
            PDFObjT::Comment(_) => log_obj("comment", o.as_ref() as &dyn Location, depth),
            PDFObjT::Integer(_) => log_obj("number<integer>", o.as_ref() as &dyn Location, depth),
            PDFObjT::Real(_) => log_obj("number<real>", o.as_ref() as &dyn Location, depth),
        }
    }
}

fn parse_file(test_file: &str) {
    // Print current path
    let path = env::current_dir();
    if let Err(_) = path {
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
    match file.read_to_end(&mut v) {
        Err(why) => {
            exit_log!(0, "Couldn't read {}: {}", display, why.to_string());
        },
        Ok(_) => (),
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
        display,
    };

    let buflen = pb.remaining();
    let mut p = HeaderP;
    let hdr = p.parse(&mut pb);
    if let Err(e) = hdr {
        exit_log!(
            fi.file_offset(0),
            "Unable to parse header from {}: {}",
            fi.display(),
            e.val()
        );
    }
    // Todo: some sanity check of header.

    // From end of buffer, scan backwards for %EOF, if present.
    pb.set_cursor(buflen);
    let eof = pb.backward_scan(b"%%EOF");
    if let Err(e) = eof {
        ta3_log!(
            Level::Info,
            fi.file_offset(0),
            "No %%EOF in {}: {}",
            fi.display(),
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
            fi.display(),
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
            fi.display(),
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
    let sxref_offset: usize = sxref.offset().try_into().unwrap();
    ta3_log!(
        Level::Info,
        fi.file_offset(sxref_loc_start),
        "startxref points to file-offset {} (pdf-offset {}) for xref",
        fi.file_offset(sxref_offset),
        sxref_offset
    );

    // Create the pdf object context.
    let mut ctxt = PDFObjContext::new();

    // Parse xref table at that offset.
    pb.set_cursor(sxref.offset().try_into().unwrap());

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

    let root_obj: &Rc<LocatedVal<PDFObjT>> = if let PDFObjT::Reference(r) = root_ref.val() {
        // TODO: this constraint should be enforced in the library.
        match ctxt.lookup_obj(r.id()) {
            Some(obj) => obj,
            None => {
                exit_log!(0, "Root object {:?} not found!", r.id());
            },
        }
    } else {
        // Is there any case where this is not the case?  Should
        // this constraint be part of the safe subset specification?
        exit_log!(
            fi.file_offset(root_ref.loc_start()),
            "Root object is not a reference!"
        );
    };

    dump_root(&fi, &ctxt, &root_obj);
}

fn print_usage(code: i32) {
    println!("Usage:\n\t{} <pdf-file>", env::args().nth(0).unwrap());
    process::exit(code)
}

fn main() {
    // TODO: add useful cli options
    match (env::args().nth(1), env::args().len()) {
        (Some(s), 2) => {
            // set up log format with file name (if > TRACE):
            let filename = Path::new(&s)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            Builder::new()
                .format(move |buf, record| {
                    if record.level() == Level::Trace {
                        writeln!(buf, "{} - {}", record.level(), record.args())
                    } else {
                        if format!("{}", record.args()).contains("panicked") {
                            // hacking a panic! log message (usually at level Error)
                            writeln!(buf, "CRITICAL - {} at NaN - {}", filename, record.args())
                        } else {
                            writeln!(buf, "{:8} - {} {}", record.level(), filename, record.args())
                        }
                    }
                })
                .filter(None, LevelFilter::Trace)
                .init();
            log_panics::init(); // cause panic! to log errors instead of simply printing them

            parse_file(&s)
        },
        (_, _) => print_usage(1),
    }
}
