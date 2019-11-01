// A very basic PDF parser.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;
use std::rc::Rc;
use std::convert::TryInto;
use std::collections::{VecDeque, BTreeSet};
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser, Location, LocatedVal};
use parsley_rust::pdf_lib::pdf_file::{HeaderP, StartXrefP, XrefSectP, XrefEntT, TrailerP};
use parsley_rust::pdf_lib::pdf_obj::{PDFObjT, PDFObjP, PDFObjContext};

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
            println!("couldn't open {}: {}", display, why.description());
            process::exit(1)
        },
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("couldn't read {}: {}", display, why.description());
            process::exit(1)
        },
        Ok(_)    => ()
    }

    let mut pb = ParseBuffer::new(Vec::from(s.as_bytes()));

    // Handle leading garbage.
    let pdf_hdr_ofs =
        match pb.scan("%PDF-".as_bytes()) {
            Ok(nbytes) => {
                if nbytes != 0 {
                    println!("Found {} bytes of leading garbage, dropping from buffer.",
                             nbytes);
                    pb.drop_upto()
                };
                nbytes
            },
            Err(e) => {
                println!("Cannot find header: {}", e.val());
                process::exit(1)
            }
        };

    let file_offset = |o: usize| { o + pdf_hdr_ofs };

    let buflen = pb.remaining();
    let mut p = HeaderP;
    let hdr = p.parse(&mut pb);
    if let Err(_) = hdr {
        println!("Unable to parse header from {}: {:?}", display, hdr);
        process::exit(1)
    }
    // Todo: some sanity check of header.

    // From end of buffer, scan backwards for %EOF.
    pb.set_cursor(buflen);
    let eof = pb.backward_scan("%%EOF".as_bytes());
    if let Err(_) = eof {
        println!("Could not find %%EOF in {}: {:?}", display, eof);
        process::exit(1)
    }
    let eof_ofs = buflen - eof.unwrap();
    println!("Found %%EOF at offset {}.", file_offset(eof_ofs));

    // Scan backward for startxref.
    let sxref = pb.backward_scan("startxref".as_bytes());
    if let Err(_) = sxref {
        println!("Could not find startxref in {}: {:?}", display, sxref);
        process::exit(1)
    }
    let sxref_ofs = buflen - sxref.unwrap();
    println!("Found startxref at offset {}.", file_offset(sxref_ofs));
    let mut p = StartXrefP;
    let sxref = p.parse(&mut pb);
    if let Err(_) = sxref {
        println!("Could not parse startxref in {} at pos {}: {:?}",
                 display, file_offset(pb.get_cursor()), sxref);
        process::exit(1)
    }
    let sxref = sxref.unwrap();
    println!(" startxref span: {}..{}.",
             file_offset(sxref.loc_start()), file_offset(sxref.loc_end()));
    let sxref = sxref.unwrap();
    println!("startxref points to offset {} for xref",
             file_offset(sxref.offset().try_into().unwrap()));

    // Parse xref at that offset.
    pb.set_cursor(sxref.offset().try_into().unwrap());
    let mut p = XrefSectP;
    let xref = p.parse(&mut pb);
    if let Err(_) = xref {
        println!("Could not parse xref in {} at pos {}: {:?}",
                 display, file_offset(pb.get_cursor()), xref);
        process::exit(1)
    }
    let xref = xref.unwrap().unwrap();
    let mut offsets : Vec<usize> = Vec::new();
    for ls in xref.sects().iter() {
        let s = ls.val();
        println!("Found {} objects starting at {}:", s.count(), s.start());
        for o in s.ents() {
            match o.val() {
                XrefEntT::Inuse(x) => {
                    println!("   inuse object at {}.", x.info());
                    offsets.push(x.info().try_into().unwrap())
                },
                XrefEntT::Free(x)  => {
                    println!("   free object (next is {}).", x.info())
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
            println!("Found trailer {} bytes from end of xref table.", nbytes),
        Err(e)     => {
            println!("Cannot find trailer: {}", e.val());
            process::exit(1)
        }
    }
    let mut p = TrailerP::new(&mut ctxt);
    let trlr  = p.parse(&mut pb);
    if let Err(e) = trlr {
        println!("Cannot parse trailer: {}", e.val());
        process::exit(1)
    }
    let trlr = trlr.unwrap().unwrap();
    let root_ref = match trlr.dict().get("Root".as_bytes()) {
        Some(rt) => rt,
        None     => {
            println!("No root reference found!");
            process::exit(1)
        }
    };

    // Now get the outermost objects at each offset in the xref table.
    let mut ctxt = PDFObjContext::new();
    let mut objs = Vec::new();
    for o in offsets.iter() {
        let mut p = PDFObjP::new(&mut ctxt);
        pb.set_cursor((*o).try_into().unwrap());
        let lobj = p.parse(&mut pb);
        if let Err(_) = lobj {
            println!("Cannot parse object at offset {} in {}: {:?}",
                     file_offset(*o), display, lobj);
            process::exit(1)
        }
        let obj = lobj.unwrap().unwrap();
        if let PDFObjT::Indirect(_) = obj {
            objs.push(obj)
        } else {
            println!("found non-indirect object at offset {}!",
                     file_offset(*o))
        }
    }

    let root_obj : &Rc<LocatedVal<PDFObjT>> =
        if let PDFObjT::Reference(r) = root_ref.val() {
            match ctxt.lookup_obj(r.id()) {
                Some(obj) => obj,
                None      => {
                    println!("Root object not found from reference!");
                    process::exit(1)
                }
            }
        } else {
            // Is there any case where this is not the case?  Should
            // this constraint be part of the safe subset specification?
            println!("Root object is not a reference!");
            process::exit(1);
        };

    // Perform a breadth-first traversal of the root object, logging
    // each object type and location as we go.

    println!("Beginning breadth-first traversal of root object:");

    let log_obj = |t: &str, loc: &dyn Location, depth: u32| {
           println!(" depth:{} type:{} start:{} end:{}  ",
                    depth, t, file_offset(loc.loc_start()), file_offset(loc.loc_end()))
    };

    let mut obj_queue = VecDeque::new();
    obj_queue.push_back((Rc::clone(root_obj), 0));  // depth 0
    let mut processed = BTreeSet::new();
    processed.insert(Rc::clone(root_obj));
    while obj_queue.len() > 0 {
        let o = obj_queue.pop_front();
        if o.is_none() { break };
        let (o, depth) = o.unwrap();

        match o.val() {
            PDFObjT::Array(a) => {
                log_obj("array", o.as_ref() as (&dyn Location), depth);
                for elem in a.objs() {
                    if !processed.contains(elem) {
                        obj_queue.push_back((Rc::clone(elem), depth + 1));
                        processed.insert(Rc::clone(elem));
                    }
                }
            },
            PDFObjT::Dict(d)  => {
                log_obj("dict", o.as_ref() as (&dyn Location), depth);
                for (_, v) in d.map().iter() {
                    if !processed.contains(v) {
                        obj_queue.push_back((Rc::clone(v), depth + 1));
                        processed.insert(Rc::clone(v));
                    }
                }
            },
            PDFObjT::Stream(s) => {
                log_obj("dict", o.as_ref() as (&dyn Location), depth);
                for (_, v) in s.dict().val().map().iter() {
                    // TODO: print key names
                    if !processed.contains(v) {
                        obj_queue.push_back((Rc::clone(v), depth + 1));
                        processed.insert(Rc::clone(v));
                    }
                }
            },
            PDFObjT::Indirect(i) => {
                log_obj("indirect", o.as_ref() as (&dyn Location), depth);
                if !processed.contains(i.obj()) {
                    obj_queue.push_back((Rc::clone(i.obj()), depth + 1));
                    processed.insert(Rc::clone(i.obj()));
                }
            },
            PDFObjT::Reference(r) => {
                log_obj("ref", o.as_ref() as (&dyn Location), depth);
                match ctxt.lookup_obj(r.id()) {
                    Some(obj) => {
                        if !processed.contains(obj) {
                            obj_queue.push_back((Rc::clone(obj), depth + 1));
                            processed.insert(Rc::clone(obj));
                        }
                    },
                    None      => {
                        println!(" ref ({},{}) does not point to a defined object!",
                                 r.num(), r.gen())
                    }
                }
            },
            PDFObjT::Boolean(_) => {
                log_obj("boolean", o.as_ref() as (&dyn Location), depth)
            },
            PDFObjT::String(_) => {
                log_obj("string", o.as_ref() as (&dyn Location), depth)
            },
            PDFObjT::Name(_) => {
                log_obj("name", o.as_ref() as (&dyn Location), depth)
            },
            PDFObjT::Null(_) => {
                log_obj("null", o.as_ref() as (&dyn Location), depth)
            },
            PDFObjT::Comment(_) => {
                log_obj("comment", o.as_ref() as (&dyn Location), depth)
            },
            PDFObjT::Integer(_) => {
                log_obj("number<integer>", o.as_ref() as (&dyn Location), depth)
            },
            PDFObjT::Real(_) => {
                log_obj("number<real>", o.as_ref() as (&dyn Location), depth)
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
        (Some(s), 2) => parse_file(&s),
        (_, _)       => print_usage(1)
    }
}
