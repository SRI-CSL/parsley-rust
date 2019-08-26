// A test application for the PDF parsing primitives.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::convert::TryInto;
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser};
use parsley_rust::pdf_lib::pdf_file::{HeaderP, StartXrefP, XrefSectP, XrefEntT};
use parsley_rust::pdf_lib::pdf_obj::{PDFObjT, PDFObjP};

#[test]
fn parse_file() {
     //Print current path
    let path = env::current_dir();
    if let Err(_) = path {
        println!("Cannot get current dir!");
        assert!(false);
    }
    let mut path = path.unwrap();
    path.push("tests/test_files/minimal.pdf");
    let display = path.as_path().display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path.as_path()) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_)    => ()
    }

    //let mut pb_new = ParseBuffer::new(Vec::from(s.as_bytes()));
    let mut pb = parsley_rust::parse_file("tests/test_files/minimal.pdf");
    assert_eq!(pb.get_cursor(), 0);
    let buflen = pb.remaining();

    let mut p = HeaderP;
    let hdr = p.parse(&mut pb);
    if let Err(_) = hdr {
        panic!("Unable to parse header from {}: {:?}", display, hdr)
    }
    // Todo: some sanity check of header.

    // From end of buffer, scan backwards for %EOF.
    pb.set_cursor(buflen);
    let eof = pb.backward_scan("%%EOF".as_bytes());
    if let Err(_) = eof {
        panic!("Could not find %%EOF in {}: {:?}", display, eof);
    }
    let eof_ofs = buflen - eof.unwrap();
    println!("Found %%EOF at offset {}.", eof_ofs);

    // Scan backward for startxref.
    let sxref = pb.backward_scan("startxref".as_bytes());
    if let Err(_) = sxref {
        panic!("Could not find startxref in {}: {:?}", display, sxref);
    }
    let sxref_ofs = buflen - sxref.unwrap();
    println!("Found startxref at offset {}.", sxref_ofs);
    let mut p = StartXrefP;
    let sxref = p.parse(&mut pb);
    if let Err(_) = sxref {
        panic!("Could not parse startxref in {} at pos {}: {:?}",
               display, pb.get_cursor(), sxref);
    }
    let sxref = sxref.unwrap();
    println!("startxref points to offset {} for xref", sxref.offset());

    // Parse xref at that offset.
    pb.set_cursor(sxref.offset().try_into().unwrap());
    let mut p = XrefSectP;
    let xref = p.parse(&mut pb);
    if let Err(_) = xref {
        panic!("Could not parse xref in {} at pos {}: {:?}",
               display, pb.get_cursor(), xref);
    }
    let xref = xref.unwrap();
    let mut offsets = Vec::new();
    for s in xref.sects().iter() {
        println!("Found {} objects starting at {}:", s.count(), s.start());
        for o in s.ents() {
            match o {
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

    // Now get the objects at each offset.
    let mut objs = Vec::new();
    for o in offsets.iter() {
        let mut p = PDFObjP;
        pb.set_cursor((*o).try_into().unwrap());
        let obj = p.parse(&mut pb);
        if let Err(_) = obj {
            panic!("Cannot parse object at offset {} in {}: {:?}",
                   o, display, obj)
        }
        let obj = obj.unwrap();
        if let PDFObjT::Indirect(_) = obj {
            objs.push(obj)
        } else {
            println!("found non-indirect object at offset {}!", o)
        }
    }
}
