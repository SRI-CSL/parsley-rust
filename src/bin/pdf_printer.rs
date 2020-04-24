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
use parsley_rust::pdf_lib::pdf_file::{HeaderP, StartXrefP, TrailerP, XrefSectP, XrefSectT};
use parsley_rust::pdf_lib::pdf_obj::{IndirectP, PDFObjContext, PDFObjT};
use parsley_rust::pdf_lib::pdf_streams::{XrefEntStatus, XrefStreamP};

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

fn info_from_xref_table(
    fi: &FileInfo, ctxt: &mut PDFObjContext, xref: &LocatedVal<XrefSectT>,
    pb: &mut dyn ParseBufferT,
) -> (Vec<ObjInfo>, Rc<LocatedVal<PDFObjT>>) {
    let (xref_loc_start, xref_loc_end) = (xref.start(), xref.end());
    let xref = xref.val();
    if let Some(err) = xref.is_valid() {
        panic!("Invalid xref table found: {}", err)
    }
    let mut id_offsets: Vec<ObjInfo> = Vec::new();
    for ls in xref.sects().iter() {
        let s = ls.val();
        ta3_log!(
            Level::Info,
            fi.file_offset(xref_loc_start),
            "Found {} objects in xref section starting at object {}:",
            s.count(),
            s.start()
        );
        for (idx, o) in s.ents().iter().enumerate() {
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
                        "   inuse object at {}.",
                        *file_ofs
                    );
                    id_offsets.push(ObjInfo {
                        id:  s.start() + idx,
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
    }

    // Get trailer following the xref table, which should give us the
    // id of the Root object.
    match pb.scan(b"trailer") {
        Ok(nbytes) => ta3_log!(
            Level::Info,
            fi.file_offset(xref_loc_end + nbytes),
            "Found trailer {} bytes from end of xref table.",
            nbytes
        ),
        Err(e) => panic!("Cannot find trailer: {}", e.val()),
    }
    let mut p = TrailerP::new(ctxt);
    let trlr = p.parse(pb);
    if let Err(e) = trlr {
        panic!("Cannot parse trailer: {}", e.val());
    }
    let trlr = trlr.unwrap().unwrap();
    // TODO: this constraint should be enforced in the library.
    let root_ref = match trlr.dict().get(b"Root") {
        Some(rt) => rt,
        None => {
            panic!("No root reference found!");
        },
    };

    (id_offsets, Rc::clone(root_ref))
}

fn parse_objects(
    fi: &FileInfo, ctxt: &mut PDFObjContext, obj_infos: &[ObjInfo], pb: &mut dyn ParseBufferT,
) {
    // Now get the outermost objects at each offset in the xref table.
    // These have to be indirect/labelled objects.
    let mut objs = Vec::new();
    for ObjInfo { id, gen, ofs } in obj_infos.iter() {
        let mut p = IndirectP::new(ctxt);
        let ofs = (*ofs).try_into().unwrap();
        ta3_log!(
            Level::Info,
            fi.file_offset(ofs),
            "parsing object ({},{}) at file-offset {} (pdf-offset {})",
            id,
            gen,
            fi.file_offset(ofs),
            ofs
        );
        pb.set_cursor(ofs);
        let lobj = p.parse(pb);
        if let Err(e) = lobj {
            panic!(
                "Cannot parse object at file-offset {} (pdf-offset {}) in {}: {}",
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
            panic!(
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
        panic!("Cannot get current dir!");
    }
    let mut path = path.unwrap();
    path.push(test_file);
    let display = path.as_path().display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path.as_path()) {
        Err(why) => {
            panic!("Couldn't open {}: {}", display, why.to_string());
        },
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut v = Vec::new();
    match file.read_to_end(&mut v) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, why.to_string());
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
            panic!("Cannot find header: {}", e.val());
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
        panic!("Unable to parse header from {}: {}", fi.display(), e.val());
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
        panic!("Could not find startxref in {}: {}", fi.display(), e.val());
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
        panic!(
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
    let mut p = XrefSectP;
    let xref = p.parse(&mut pb);
    if let Err(ref e) = xref {
        ta3_log!(
            Level::Info,
            fi.file_offset(sxref_offset),
            "Could not parse xref in {} at file-offset {} (pdf-offset {}): {}",
            fi.display(),
            fi.file_offset(e.start()),
            e.start(),
            e.val()
        );
        // Check if we have an xref stream
        let mut sp = IndirectP::new(&mut ctxt);
        let xref_obj = sp.parse(&mut pb);
        if let Err(e) = xref_obj {
            panic!("Could not parse object for xref stream in {} at file-offset {} (pdf-offset {}): {}",
                   fi.display(), fi.file_offset(e.start()), e.start(), e.val());
        };
        let xref_obj = xref_obj.unwrap();
        if let PDFObjT::Stream(ref s) = xref_obj.val().obj().val() {
            let mut xp = XrefStreamP::new(s);
            let xref_stm = xp.parse(&mut pb);
            if let Err(e) = xref_stm {
                panic!(
                    "Could not parse xref stream in {} at file-offset {} (pdf-offset {}): {}",
                    fi.display(),
                    fi.file_offset(e.start()),
                    e.start(),
                    e.val()
                );
            }
        } else {
            panic!(
                "Could not find valid xref information in {} at file-offset {} (pdf-offset {})",
                fi.display(),
                fi.file_offset(sxref_offset),
                sxref_offset
            );
        }
    }

    let xref = xref.unwrap();
    let (id_offsets, root_ref) = info_from_xref_table(&fi, &mut ctxt, &xref, &mut pb);

    // Parse the objects using their xref entries, and put them into the context.
    parse_objects(&fi, &mut ctxt, &id_offsets, &mut pb);

    let root_obj: &Rc<LocatedVal<PDFObjT>> = if let PDFObjT::Reference(r) = root_ref.val() {
        // TODO: this constraint should be enforced in the library.
        match ctxt.lookup_obj(r.id()) {
            Some(obj) => obj,
            None => {
                panic!("Root object not found from reference!");
            },
        }
    } else {
        // Is there any case where this is not the case?  Should
        // this constraint be part of the safe subset specification?
        panic!("Root object is not a reference!");
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
