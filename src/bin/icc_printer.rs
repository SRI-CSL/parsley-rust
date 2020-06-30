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

// Let us use all the combinators
use parsley_rust::pcore::prim_binary::*;
use parsley_rust::pcore::prim_combinators::{Sequence};

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
    profile_header(&mut pb);
    tag_table(&mut pb)
}

fn tag_table (pb: &mut ParseBuffer)
{
}

fn profile_header (pb: &mut ParseBuffer)

{
    let mut size = IntObj64::new();
    println!("{:?}", size.parse(pb));

    let mut preferred_cmm_type = IntObj64::new(); // there is an enum here need to check
    println!("{:?}", preferred_cmm_type.parse(pb));

    // version
    let mut major = TokenParser::new("\x04", 1);
    println!("{:?}", major.parse(pb));

    let mut bug_fix_level = IntObj4::new();
    println!("{:?}", bug_fix_level.parse(pb));

    let mut minor = IntObj4::new();
    println!("{:?}", minor.parse(pb));

    let mut reserved = TokenParser::new("\x00\x00", 2); // whats b4?;
    println!("{:?}", reserved.parse(pb));

    // version ends


    let mut device_class = IntObj64::new();
    println!("{:?}", device_class.parse(pb));

    let mut color_space = IntObj64::new(); // enum present here as well
    println!("{:?}", color_space.parse(pb));

    let mut pcs = BinaryBuffer::new(4);
    println!("{:?}", pcs.parse(pb));

    // TODO creation_data_time

    let mut file_signature = TokenParser::new("\x61\x63\x73\x70", 4);
    println!("{:?}", file_signature.parse(pb));

    let mut primary_platform = IntObj64::new();
    println!("{:?}", primary_platform.parse(pb));

    // TODO device_manufacturer

    // begin profile_flags

    let mut embedded_profile = IntObj1::new() ;
    println!("{:?}", primary_platform.parse(pb));

    let mut profile_can_be_used_independently_of_embedded_colour_data = IntObj1::new();
    println!("{:?}", profile_can_be_used_independently_of_embedded_colour_data.parse(pb));

    let mut other_flags = IntObj30::new();
    println!("{:?}", other_flags.parse(pb));

    
    // end profile flags

    let mut device_model = BinaryBuffer::new(4);
    println!("{:?}", pcs.parse(pb));

    // TODO device_attributes

    let mut rendering_intents = BinaryBuffer::new(4);
    println!("{:?}", pcs.parse(pb));

    // TODO nciexyz_values_of_illuminant_of_pcs
    
    // TODO creator

    let mut identifier = BinaryBuffer::new(16);
    println!("{:?}", pcs.parse(pb));

    let mut reserved_data = BinaryBuffer::new(16);
    println!("{:?}", pcs.parse(pb));
}

fn print_usage(code: i32) {
    println!("Usage:\n\t{} <iccv4-file>", env::args().nth(0).unwrap());
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
