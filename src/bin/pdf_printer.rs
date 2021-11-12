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
extern crate serde;
extern crate serde_json;

#[cfg(feature = "kuduafl")]
extern crate afl;

use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;
use std::rc::Rc;
use std::str;

use env_logger::Builder;
use log::{debug, error, log, Level, LevelFilter};

use clap::{App, Arg};
use serde_json::Value;

use parsley_rust::pcore::parsebuffer::{
    ErrorKind, LocatedVal, Location, ParseBuffer, ParseResult, ParsleyParser, StreamBufferT,
};
use parsley_rust::pdf_lib::catalog::{catalog_type, info_type};
use parsley_rust::pdf_lib::pdf_content_streams::{TextExtractor, TextToken};
use parsley_rust::pdf_lib::pdf_obj::{ObjectId, PDFObjContext, PDFObjT};
use parsley_rust::pdf_lib::pdf_page_dom::Resources;
use parsley_rust::pdf_lib::pdf_page_dom::{to_page_dom, FeaturePresence, PageKid};
use parsley_rust::pdf_lib::pdf_streams::decode_stream;
use parsley_rust::pdf_lib::pdf_traverse_xref::{parse_file, FileInfo};
use parsley_rust::pdf_lib::pdf_type_check::{check_type, TypeCheckContext};

#[cfg(feature = "kuduafl")]
use parsley_rust::pdf_lib::pdf_traverse_xref::parse_data;

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
        log!($lvl, "at {:>10} ({:#x}) - {}", $pos, $pos, format_args!($($arg)+))
    })
}

macro_rules! exit_log {
    ($pos:expr, $($arg:tt)+) => ({
        log!(Level::Error, "at {:>10} ({:#x}) - {}", $pos, $pos, format_args!($($arg)+));
        process::exit(1)
    })
}

// Perform a breadth-first traversal of the root object, logging
// each object type and location as we go.
fn dump_root(fi: &FileInfo, ctxt: &PDFObjContext, root_obj: &Rc<LocatedVal<PDFObjT>>) {
    if false {
        debug!("Beginning breadth-first traversal of root object:");
    }

    let log_obj = |t: &str, loc: &dyn Location, depth: u32| {
        if false {
            ta3_log!(
                Level::Info,
                fi.file_offset(loc.loc_start()),
                "depth:{} type:{} start-file-offset:{} end-file-offset:{}  ",
                depth,
                t,
                fi.file_offset(loc.loc_start()),
                fi.file_offset(loc.loc_end())
            )
        }
    };

    let mut obj_queue = VecDeque::new();
    obj_queue.push_back((Rc::clone(root_obj), 0)); // depth 0
    let mut processed = BTreeSet::new();
    processed.insert(Rc::clone(root_obj));
    while !obj_queue.is_empty() {
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
                if !ctxt.is_encrypted() {
                    match decode_stream(s) {
                        Ok(_) => (),
                        Err(e) => ta3_log!(
                            Level::Warn,
                            fi.file_offset(o.start()),
                            " error decoding stream: {:?}",
                            e
                        ),
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

fn extract_text(
    ctxt: &mut PDFObjContext, pid: &ObjectId, _r: &Resources, buf: &mut ParseBuffer,
    dump: &mut Option<fs::File>,
) -> ParseResult<()> {
    let mut te = TextExtractor::new(ctxt, pid);
    let tokens = te.parse(buf)?;
    // condense multiple spaces into a single one.
    let mut spaced = false;
    for t in tokens.val().iter() {
        match t {
            TextToken::Space => {
                if !spaced {
                    match dump {
                        None => print!(" "),
                        Some(f) => {
                            let _ = write!(f, " ");
                        },
                    };
                    spaced = true;
                }
            },
            TextToken::RawText(s) => {
                // TODO: font conversion.  For now, just try blind
                // unicode conversion.
                match std::str::from_utf8(s) {
                    Ok(v) => match dump {
                        None => println!("{}", v),
                        Some(f) => {
                            let _ = write!(f, "{}", v);
                        },
                    },
                    Err(_) => (), // println!("not UTF8"),
                };
                spaced = false
            },
        }
    }
    return Ok(())
}

fn dump_file(fi: &FileInfo, ctxt: &mut PDFObjContext, root_id: ObjectId) {
    // TODO: this constraint should be enforced in the library.
    let root_obj: &Rc<LocatedVal<PDFObjT>> = match ctxt.lookup_obj(root_id) {
        Some(obj) => obj,
        None => exit_log!(0, "Root object {:?} not found!", root_id),
    };

    // Run this to get warnings on stream decoding.
    dump_root(fi, ctxt, root_obj);
}

fn type_check_file(
    fi: &FileInfo, ctxt: &mut PDFObjContext, root_id: ObjectId, info_id: Option<ObjectId>,
) {
    let root_obj: &Rc<LocatedVal<PDFObjT>> = match ctxt.lookup_obj(root_id) {
        Some(obj) => obj,
        None => exit_log!(0, "Root object {:?} not found!", root_id),
    };
    let mut producer: Option<String> = None;
    match info_id {
        Some(i_id) => {
            match ctxt.lookup_obj(i_id) {
                Some(obj) => {
                    let o1 = Rc::clone(obj);
                    match o1.val() {
                        PDFObjT::Dict(d) => {
                            match d.get("Producer".as_bytes()) {
                                Some(a) => {
                                    match a.val() {
                                        PDFObjT::String(string) => {
                                            match str::from_utf8(string) {
                                                Ok(prod) => {
                                                    producer = Some(prod.to_string());
                                                },
                                                _ => {},
                                            };
                                        },
                                        _ => {},
                                    };
                                },
                                _ => {},
                            };
                        },
                        _ => {},
                    };
                    let mut tctx = TypeCheckContext::new();
                    let typ = info_type(&mut tctx);
                    if let Some(err) = check_type(&ctxt, &tctx, Rc::clone(obj), typ) {
                        ta3_log!(
                            Level::Warn,
                            fi.file_offset(err.loc_start()),
                            "Info Type Check Error: {:?}, Producer: {:?}",
                            err.val(),
                            producer,
                        );
                    }
                },
                None => ta3_log!(Level::Warn, 0, "Info object {:?} not found!", info_id),
            };
        },
        None => {},
    };

    let mut tctx = TypeCheckContext::new();
    let typ = catalog_type(&mut tctx);
    if let Some(err) = check_type(&ctxt, &tctx, Rc::clone(root_obj), typ) {
        exit_log!(
            fi.file_offset(err.loc_start()),
            "Type Check Error: {:?}, Producer: {:?}",
            err.val(),
            producer,
        );
    }
}

fn file_extract_text(
    ctxt: &mut PDFObjContext, root_id: ObjectId, text_dump_file: &mut Option<fs::File>,
) {
    let root_obj: &Rc<LocatedVal<PDFObjT>> = match ctxt.lookup_obj(root_id) {
        Some(obj) => obj,
        None => {
            exit_log!(0, "Root object {:?} not found!", root_id)
        },
    };

    let page_dom = match to_page_dom(&ctxt, &root_obj) {
        Ok((_cat, dom)) => {
            println!("Page DOM built with {} page nodes.", dom.pages().len());
            dom
        },
        Err(e) => {
            ta3_log!(Level::Warn, e.loc_start(), "Page DOM error: {:?}", e.val());
            process::exit(1);
        },
    };
    // We will consider a file as having text if it has a non-symbolic
    // font that is embedded.
    'page_loop: for (pid, p) in page_dom.pages().iter() {
        match p {
            PageKid::Leaf(l) => {
                println!(
                    " page {:?} with {} content streams",
                    pid,
                    l.contents().len()
                );
                for (_, fd) in l.resources().fonts().iter() {
                    if fd.is_embedded() == FeaturePresence::False {
                        exit_log!(0, "page {:?} has a non-embedded font", pid)
                    }
                }
                // If there are multiple content streams, they need to
                // be concatenated into a single buffer to work with
                // the content stream parser.
                let mut buf = ParseBuffer::new(Vec::new());
                '_content_loop: for c in l.contents() {
                    match c.val() {
                        PDFObjT::Stream(s) => match decode_stream(s) {
                            Ok(cs) => {
                                buf.append(b" ");
                                buf.append(cs.content());
                            },
                            Err(e) => {
                                ta3_log!(
                                    Level::Warn,
                                    0,
                                    " collecting error when decoding stream in page {:?}: {:?}",
                                    pid,
                                    e
                                );
                                // go to the next page
                                continue 'page_loop
                            },
                        },
                        _ => {
                            ta3_log!(
                                Level::Error,
                                0,
                                " unexpected object found as content stream!"
                            );
                            // go to the next page
                            continue 'page_loop
                        },
                    }
                }
                match extract_text(ctxt, pid, l.resources(), &mut buf, text_dump_file) {
                    Ok(_) => (),
                    Err(e) => match e.val() {
                        ErrorKind::GuardError(s) => {
                            if s.contains("EndOfBuffer") {
                                ta3_log!(
                                    Level::Warn,
                                    0,
                                    " error parsing content in page {:?}: {:?}",
                                    pid,
                                    e
                                );
                            }
                        },
                        _ => {
                            exit_log!(0, " error parsing content in page {:?}: {:?}", pid, e);
                        },
                    },
                }
            },
            PageKid::Node(_n) => {
                // println!(" tree node {:?} with {} kids", pid, n.kids().len())
            },
        }
    }
}

fn process_file(
    fi: &FileInfo, ctxt: &mut PDFObjContext, root_id: ObjectId, info_id: Option<ObjectId>,
    text_dump_file: &mut Option<fs::File>,
) {
    dump_file(fi, ctxt, root_id);
    type_check_file(fi, ctxt, root_id, info_id);
    file_extract_text(ctxt, root_id, text_dump_file);
}

#[cfg(not(feature = "kuduafl"))]
fn main() {
    // parsing command line arguments:
    let matches = App::new("Parsley PDF Parser")
        // TODO: use Cargo Metadata here?  See ../../cargo.toml
        // .version("0.1.0")
        // .author("Prashanth Mundkur <prashanth.mundkur@gmail.com>")
        .about("=> parses given PDF file")
        .arg(
            Arg::with_name("pdf_file")
                .value_name("PDF_FILE")
                .help("the PDF file to parse")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output_json")
                .short("o")
                .long("output")
                .value_name("JSON_FILE")
                .takes_value(true)
                .help("output file where to write JSON for TA1 to"),
        )
        .arg(
            Arg::with_name("input_json")
                .short("i")
                .long("input")
                .value_name("JSON_FILE")
                .takes_value(true)
                .help("input file with TA1 JSON content to guide the parsing"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity that increases logging level (default: INFO)"),
        )
        .arg(
            Arg::with_name("output_text_extract")
                .short("t")
                .long("output_text_extract")
                .value_name("TEXT_FILE")
                .takes_value(true)
                .help("output file where to store extracted text"),
        )
        .get_matches();

    // set logging level based on -v:
    let log_filter = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };
    // set up log format with file name (if > TRACE):
    let filename = Path::new(matches.value_of("pdf_file").unwrap())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    Builder::new()
        .format(move |buf, record| {
            if record.level() == Level::Trace {
                writeln!(buf, "{}", record.args())
            } else if format!("{}", record.args()).contains("panicked") {
                // hacking a panic! log message (usually at level Error)
                writeln!(buf, "CRITICAL - {} at NaN - {}", filename, record.args())
            } else {
                writeln!(buf, "{:8} - {} {}", record.level(), filename, record.args())
            }
        })
        .filter(None, log_filter)
        .init();
    log_panics::init(); // cause panic! to log errors instead of simply printing them

    if matches.is_present("output_json") {
        debug!(
            "writing JSON output to:\t{}",
            matches.value_of("output_json").unwrap()
        );
        // TODO: actually write something into this file...
    }
    if matches.is_present("input_json") {
        // read file to string and parse as JSON, then pass it to `parse_file` as
        // appropriate...
        let filename = matches.value_of("input_json").unwrap();
        let path = Path::new(filename);

        // see: https://dev.to/0xbf/day15-load-and-dump-json-100dayofrust-3l1c
        let json_str = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());

        if json_str.is_empty() {
            error!("Could not open input JSON file at:\t{}", filename);
        } else {
            let json_input: Value = serde_json::from_str(&json_str).unwrap();
            debug!("parsed input JSON: {}", json_input); // TODO: use in
                                                         // parse_file()?
        }
    }
    let mut output_text_file = if matches.is_present("output_text_extract") {
        let fname = matches.value_of("output_text_extract").unwrap();
        match fs::File::create(fname) {
            Ok(f) => Some(f),
            Err(e) => exit_log!(0, "Could not create output file at {}: {}", fname, e),
        }
    } else {
        None
    };

    let test_file = matches.value_of("pdf_file").unwrap();
    let (fi, mut ctxt, root_id, info_id) = parse_file(test_file);
    process_file(&fi, &mut ctxt, root_id, info_id, &mut output_text_file);
}

#[cfg(feature = "kuduafl")]
fn main() {
    let path = std::env::current_dir();
    let path = path.unwrap();
    afl::fuzz!(|data: &[u8]| {
        let (fi, mut ctxt, root_id) = parse_data(&path, data);
        process_file(&fi, &mut ctxt, root_id, &mut None);
    });
}
