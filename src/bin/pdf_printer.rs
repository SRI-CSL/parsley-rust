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

use env_logger::Builder;
use log::{Level, LevelFilter};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::panic;
use std::path::Path;
use std::process;
use std::rc::Rc;
use std::convert::TryInto;
use std::collections::{VecDeque, BTreeSet};
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser, Location, LocatedVal};
use parsley_rust::pdf_lib::pdf_file::{HeaderP, StartXrefP, XrefSectP, XrefEntT, TrailerP};
use parsley_rust::pdf_lib::pdf_obj::{PDFObjT, PDFObjP, PDFObjContext};

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
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            panic!("Couldn't open {}: {}", display, why.description());
        }
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut v = Vec::new();
    match file.read_to_end(&mut v) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, why.description());
        }
        Ok(_) => ()
    };

    let mut pb = ParseBuffer::new(v);

    // Handle leading garbage.
    let pdf_hdr_ofs =
        match pb.scan("%PDF-".as_bytes()) {
            Ok(nbytes) => {
                if nbytes != 0 {
                    ta3_log!(Level::Info, nbytes,
                             "Found {} bytes of leading garbage, dropping from buffer.",
                             nbytes);
                    pb.drop_upto()
                };
                nbytes
            }
            Err(e) => {
                panic!("Cannot find header: {}", e.val());
            }
        };

    let file_offset = |o: usize| { o + pdf_hdr_ofs };

    let buflen = pb.remaining();
    let mut p = HeaderP;
    let hdr = p.parse(&mut pb);
    if let Err(e) = hdr {
        panic!("Unable to parse header from {}: {}", display, e.val());
    }
    // Todo: some sanity check of header.

    // From end of buffer, scan backwards for %EOF, if present.
    pb.set_cursor(buflen);
    let eof = pb.backward_scan("%%EOF".as_bytes());
    if let Err(e) = eof {
        ta3_log!(Level::Info, file_offset(0), "No %%EOF in {}: {}", display, e.val());
    } else {
        let eof_ofs = buflen - eof.unwrap();
        ta3_log!(Level::Info, file_offset(eof_ofs), "Found %%EOF at file-offset {} (pdf-offset {}).",
                 file_offset(eof_ofs), eof_ofs);
    }

    // Scan backward for startxref.
    let sxref = pb.backward_scan("startxref".as_bytes());
    if let Err(e) = sxref {
        panic!("Could not find startxref in {}: {}", display, e.val());
    }
    let sxref_ofs = buflen - sxref.unwrap();
    ta3_log!(Level::Info, file_offset(sxref_ofs), "Found startxref at file-offset {} (pdf-offset {}).",
             file_offset(sxref_ofs), sxref_ofs);
    let mut p = StartXrefP;
    let sxref = p.parse(&mut pb);
    if let Err(e) = sxref {
        panic!("Could not parse startxref in {} at file-offset {} (pdf-offset {}): {}",
               display, file_offset(e.start()), e.start(), e.val());
    }
    let sxref = sxref.unwrap();
    let sxref_loc_start = sxref.loc_start();
    ta3_log!(Level::Info, file_offset(sxref_loc_start), " startxref span (in file-offsets): {}..{}.",
             file_offset(sxref.loc_start()), file_offset(sxref.loc_end()));
    let sxref = sxref.unwrap();
    let sxref_offset : usize = sxref.offset().try_into().unwrap();
    ta3_log!(Level::Info, file_offset(sxref_loc_start),
             "startxref points to file-offset {} (pdf-offset {}) for xref",
             file_offset(sxref_offset), sxref_offset);

    // Parse xref at that offset.
    pb.set_cursor(sxref.offset().try_into().unwrap());
    let mut p = XrefSectP;
    let xref = p.parse(&mut pb);
    if let Err(e) = xref {
        panic!("Could not parse xref in {} at file-offset {} (pdf-offset {}): {}",
               display, file_offset(e.start()), e.start(), e.val());
    }

    let xref_loc = xref.unwrap();
    let (xref_loc_start, xref_loc_end) = (xref_loc.start(), xref_loc.end());
    let xref = xref_loc.unwrap();
    let mut offsets : Vec<usize> = Vec::new();
    for ls in xref.sects().iter() {
        let s = ls.val();
        ta3_log!(Level::Info, file_offset(xref_loc_start),
                 "Found {} objects in xref section starting at object {}:",
                 s.count(), s.start());
        for o in s.ents() {
            match o.val() {
                XrefEntT::Inuse(x) => {
                    ta3_log!(Level::Info, file_offset(o.loc_start()), "   inuse object at {}.", x.info());
                    offsets.push(x.info().try_into().unwrap())
                }
                XrefEntT::Free(x) => {
                    ta3_log!(Level::Info, file_offset(o.loc_start()), "   free object (next is {}).", x.info())
                }
            }
        }
    }

    // Create the pdf object context.
    let mut ctxt = PDFObjContext::new();

    // Get trailer following the xref table, which should give us the
    // id of the Root object.
    match pb.scan("trailer".as_bytes()) {
        Ok(nbytes) =>
            ta3_log!(Level::Info, file_offset(xref_loc_end + nbytes),
                     "Found trailer {} bytes from end of xref table.",
                     nbytes),
        Err(e)     =>
            panic!("Cannot find trailer: {}", e.val())
    }
    let mut p = TrailerP::new(&mut ctxt);
    let trlr = p.parse(&mut pb);
    if let Err(e) = trlr {
        panic!("Cannot parse trailer: {}", e.val());
    }
    let trlr = trlr.unwrap().unwrap();
    let root_ref = match trlr.dict().get("Root".as_bytes()) {
        Some(rt) => rt,
        None => {
            panic!("No root reference found!");
        }
    };

    // Now get the outermost objects at each offset in the xref table.
    let mut ctxt = PDFObjContext::new();
    let mut objs = Vec::new();
    for o in offsets.iter() {
        let mut p = PDFObjP::new(&mut ctxt);
        pb.set_cursor((*o).try_into().unwrap());
        let lobj = p.parse(&mut pb);
        if let Err(e) = lobj {
            panic!("Cannot parse object at file-offset {} (pdf-offset {}) in {}: {}",
                   file_offset(e.start()), e.start(), display, e.val());
        }
        let obj = lobj.unwrap().unwrap();
        if let PDFObjT::Indirect(_) = obj {
            objs.push(obj)
        } else {
            ta3_log!(Level::Info, file_offset(*o),
                     "found non-indirect object at file-offset {} (pdf-offset {})!",
                     file_offset(*o), *o)
        }
    }

    let root_obj: &Rc<LocatedVal<PDFObjT>> =
        if let PDFObjT::Reference(r) = root_ref.val() {
            match ctxt.lookup_obj(r.id()) {
                Some(obj) => obj,
                None => {
                    panic!("Root object not found from reference!");
                }
            }
        } else {
            // Is there any case where this is not the case?  Should
            // this constraint be part of the safe subset specification?
            panic!("Root object is not a reference!");
        };

    // Perform a breadth-first traversal of the root object, logging
    // each object type and location as we go.

    debug!("Beginning breadth-first traversal of root object:");

    let log_obj = |t: &str, loc: &dyn Location, depth: u32| {
        ta3_log!(Level::Info, file_offset(loc.loc_start()),
                 "depth:{} type:{} start-file-offset:{} end-file-offset:{}  ",
                 depth, t, file_offset(loc.loc_start()), file_offset(loc.loc_end()))
    };

    let mut obj_queue = VecDeque::new();
    obj_queue.push_back((Rc::clone(root_obj), 0));  // depth 0
    let mut processed = BTreeSet::new();
    processed.insert(Rc::clone(root_obj));
    while obj_queue.len() > 0 {
        let o = obj_queue.pop_front();
        if o.is_none() { break; };
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
            }
            PDFObjT::Dict(d) => {
                log_obj("dict", o.as_ref() as &dyn Location, depth);
                for (_, v) in d.map().iter() {
                    if !processed.contains(v) {
                        obj_queue.push_back((Rc::clone(v), depth + 1));
                        processed.insert(Rc::clone(v));
                    }
                }
            }
            PDFObjT::Stream(s) => {
                log_obj("stream", o.as_ref() as (&dyn Location), depth);
                for (_, v) in s.dict().val().map().iter() {
                    // TODO: print key names
                    if !processed.contains(v) {
                        obj_queue.push_back((Rc::clone(v), depth + 1));
                        processed.insert(Rc::clone(v));
                    }
                }
            }
            PDFObjT::Indirect(i) => {
                log_obj("indirect", o.as_ref() as &dyn Location, depth);
                if !processed.contains(i.obj()) {
                    obj_queue.push_back((Rc::clone(i.obj()), depth + 1));
                    processed.insert(Rc::clone(i.obj()));
                }
            }
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
                    None      => {
                        ta3_log!(Level::Warn, file_offset(o.start()),
                                 " ref ({},{}) does not point to a defined object!",
                                 r.num(), r.gen())
                    }
                }
            }
            PDFObjT::Boolean(_) => {
                log_obj("boolean", o.as_ref() as &dyn Location, depth)
            }
            PDFObjT::String(_) => {
                log_obj("string", o.as_ref() as &dyn Location, depth)
            }
            PDFObjT::Name(_) => {
                log_obj("name", o.as_ref() as &dyn Location, depth)
            }
            PDFObjT::Null(_) => {
                log_obj("null", o.as_ref() as &dyn Location, depth)
            }
            PDFObjT::Comment(_) => {
                log_obj("comment", o.as_ref() as &dyn Location, depth)
            }
            PDFObjT::Integer(_) => {
                log_obj("number<integer>", o.as_ref() as &dyn Location, depth)
            }
            PDFObjT::Real(_) => {
                log_obj("number<real>", o.as_ref() as &dyn Location, depth)
            }
        }
    }
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
            let filename = Path::new(&s).file_name().unwrap().to_str().unwrap().to_string();
            Builder::new()
                .format(move |buf, record| {
                    if record.level() == Level::Trace {
                        writeln!(buf,
                                 "{} - {}",
                                 record.level(),
                                 record.args()
                        )
                    } else {
                        if format!("{}", record.args()).contains("panicked") {
                            // hacking a panic! log message (usually at level Error)
                            writeln!(buf,
                                     "CRITICAL - {} at NaN - {}",
                                     filename,
                                     record.args()
                            )
                        } else {
                            writeln!(buf,
                                     "{:8} - {} {}",
                                     record.level(),
                                     filename,
                                     record.args()
                            )
                        }
                    }
                })
                .filter(None, LevelFilter::Trace)
                .init();
            log_panics::init();  // cause panic! to log errors instead of simply printing them

            parse_file(&s)
        }
        (_, _) => print_usage(1)
    }
}
