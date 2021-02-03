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

/// A very basic JPEG parser.
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate log_panics;
extern crate serde;
extern crate serde_json;

use std::collections::{BTreeSet, VecDeque};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::rc::Rc;

use env_logger::Builder;
use log::{debug, error, log, Level, LevelFilter};

use clap::{App, Arg};

//use serde::{Deserialize, Serialize};
use serde_json::Value;

use parsley_rust::pcore::parsebuffer::{
    LocatedVal, Location, ParseBuffer, ParseBufferT, ParsleyParser,
};
use parsley_rust::pcore::transforms::{BufferTransformT, RestrictView};

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


fn parse_file(test_file: &str) {
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
    println!("{:?}", pb);
}

fn main() {
    // parsing command line arguments:
    let matches = App::new("Parsley JPEG Parser")
        // TODO: use Cargo Metadata here?  See ../../cargo.toml
        // .version("0.1.0")
        // .author("Prashant Anantharaman <prashant.barca@gmail.com>")
        .about("=> parses given JPEG file")
        .arg(
            Arg::with_name("jpeg_file")
                .value_name("JPEG_FILE")
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
        .get_matches();

    // set logging level based on -v:
    let log_filter = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };
    // set up log format with file name (if > TRACE):
    let filename = Path::new(matches.value_of("jpeg_file").unwrap())
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

    parse_file(matches.value_of("jpeg_file").unwrap())
}
