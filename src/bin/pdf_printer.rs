// A very basic PDF parser.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;
use std::convert::TryInto;
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser, Location};
use parsley_rust::pdf_lib::pdf_file::{HeaderP, StartXrefP, XrefSectP, XrefEntT};
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
    match pb.scan("%PDF-".as_bytes()) {
        Ok(nbytes) => {
            println!("Found {} bytes of leading garbage, dropping ...", nbytes);
            pb.drop_upto()
        },
        Err(e) => {
            println!("Cannot find header: {}", e);
            process::exit(1)
        }
    }

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
    println!("Found %%EOF at offset {}.", eof_ofs);

    // Scan backward for startxref.
    let sxref = pb.backward_scan("startxref".as_bytes());
    if let Err(_) = sxref {
        println!("Could not find startxref in {}: {:?}", display, sxref);
        process::exit(1)
    }
    let sxref_ofs = buflen - sxref.unwrap();
    println!("Found startxref at offset {}.", sxref_ofs);
    let mut p = StartXrefP;
    let sxref = p.parse(&mut pb);
    if let Err(_) = sxref {
        println!("Could not parse startxref in {} at pos {}: {:?}",
                 display, pb.get_cursor(), sxref);
        process::exit(1)
    }
    let sxref = sxref.unwrap();
    println!(" startxref span: {}..{}.", sxref.loc_start(), sxref.loc_end());
    let sxref = sxref.unwrap();
    println!("startxref points to offset {} for xref", sxref.offset());

    // Parse xref at that offset.
    pb.set_cursor(sxref.offset().try_into().unwrap());
    let mut p = XrefSectP;
    let xref = p.parse(&mut pb);
    if let Err(_) = xref {
        println!("Could not parse xref in {} at pos {}: {:?}",
                 display, pb.get_cursor(), xref);
        process::exit(1)
    }
    let xref = xref.unwrap().unwrap();
    let mut offsets = Vec::new();
    for ls in xref.sects().iter() {
        let s = ls.val();
        println!("Found {} objects starting at {}:", s.count(), s.start());
        for o in s.ents() {
            match o.val() {
                XrefEntT::Inuse(x) => {
                    println!("   inuse object at {}.", x.info());
                    offsets.push(x.info())
                },
                XrefEntT::Free(x)  => {
                    println!("   free object (next is {}).", x.info())
                }
            }
        }
    }

    // Now create a context and get the objects at each offset.
    let mut ctxt = PDFObjContext::new();
    let mut objs = Vec::new();
    for o in offsets.iter() {
        let mut p = PDFObjP::new(&mut ctxt);
        pb.set_cursor((*o).try_into().unwrap());
        let lobj = p.parse(&mut pb);
        if let Err(_) = lobj {
            println!("Cannot parse object at offset {} in {}: {:?}",
                     o, display, lobj);
            process::exit(1)
        }
        let lobj = lobj.unwrap();
        let obj = lobj.unwrap();
        if let PDFObjT::Indirect(_) = obj {
            objs.push(obj)
        } else {
            println!("found non-indirect object at offset {}!", o)
        }
    }
}

fn print_usage(code: i32) {
    println!("{}: <pdf-file>", env::args().nth(0).unwrap());
    process::exit(code)
}

fn main() {
    // TODO: add useful cli options
    match (env::args().nth(1), env::args().len()) {
        (Some(s), 2) => parse_file(&s),
        (_, _)       => print_usage(1)
    }
}
