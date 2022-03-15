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

extern crate adobe_cmap_parser;
/// A very basic PDF parser.
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate log_panics;
extern crate serde;
extern crate serde_json;
use encoding::all::{MAC_ROMAN, WINDOWS_1252};
use encoding::{DecoderTrap, Encoding};
use utf16string::WStr;

#[cfg(feature = "kuduafl")]
extern crate afl;

use std::collections::HashMap;
use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::io::Write;
use std::panic;
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
use parsley_rust::pdf_lib::pdf_obj::{DictKey, ObjectId, PDFObjContext, PDFObjT};
use parsley_rust::pdf_lib::pdf_page_dom::Resources;
use parsley_rust::pdf_lib::pdf_page_dom::{to_page_dom, FeaturePresence, FontEncoding, PageKid};
use parsley_rust::pdf_lib::pdf_streams::decode_stream;
use parsley_rust::pdf_lib::pdf_traverse_xref::{parse_file, FileInfo};
use parsley_rust::pdf_lib::pdf_type_check::{check_type, TypeCheckContext};
use parsley_rust::pdf_lib::reducer::{reduce, serializer};

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

fn mac_expert_encoding() -> HashMap<u32, String> {
    let mut hash = HashMap::new();
    let map_char = [247, 230];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(190, var_1);
    let map_char = [247, 225];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(135, var_1);
    let map_char = [247, 226];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(137, var_1);
    let map_char = [247, 180];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(39, var_1);
    let map_char = [247, 228];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(138, var_1);
    let map_char = [247, 224];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(136, var_1);
    let map_char = [247, 229];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(140, var_1);
    let map_char = [247, 97];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(97, var_1);
    let map_char = [247, 227];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(139, var_1);
    let map_char = [246, 244];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(243, var_1);
    let map_char = [247, 98];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(98, var_1);
    let map_char = [246, 245];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(174, var_1);
    let map_char = [247, 231];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(141, var_1);
    let map_char = [247, 184];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(201, var_1);
    let map_char = [246, 246];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(94, var_1);
    let map_char = [247, 99];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(99, var_1);
    let map_char = [247, 168];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(172, var_1);
    let map_char = [246, 247];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(250, var_1);
    let map_char = [247, 100];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(100, var_1);
    let map_char = [247, 233];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(142, var_1);
    let map_char = [247, 234];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(144, var_1);
    let map_char = [247, 235];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(145, var_1);
    let map_char = [247, 232];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(143, var_1);
    let map_char = [247, 101];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(101, var_1);
    let map_char = [247, 240];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(68, var_1);
    let map_char = [247, 102];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(102, var_1);
    let map_char = [247, 96];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(96, var_1);
    let map_char = [247, 103];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(103, var_1);
    let map_char = [247, 104];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(104, var_1);
    let map_char = [246, 248];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(34, var_1);
    let map_char = [247, 237];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(146, var_1);
    let map_char = [247, 238];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(148, var_1);
    let map_char = [247, 239];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(149, var_1);
    let map_char = [247, 236];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(147, var_1);
    let map_char = [247, 105];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(105, var_1);
    let map_char = [247, 106];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(106, var_1);
    let map_char = [247, 107];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(107, var_1);
    let map_char = [246, 249];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(194, var_1);
    let map_char = [247, 108];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(108, var_1);
    let map_char = [247, 175];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(244, var_1);
    let map_char = [247, 109];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(109, var_1);
    let map_char = [247, 110];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(110, var_1);
    let map_char = [247, 241];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(150, var_1);
    let map_char = [246, 250];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(207, var_1);
    let map_char = [247, 243];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(151, var_1);
    let map_char = [247, 244];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(153, var_1);
    let map_char = [247, 246];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(154, var_1);
    let map_char = [246, 251];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(242, var_1);
    let map_char = [247, 242];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(152, var_1);
    let map_char = [247, 248];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(191, var_1);
    let map_char = [247, 111];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(111, var_1);
    let map_char = [247, 245];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(155, var_1);
    let map_char = [247, 112];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(112, var_1);
    let map_char = [247, 113];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(113, var_1);
    let map_char = [246, 252];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(251, var_1);
    let map_char = [247, 114];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(114, var_1);
    let map_char = [246, 253];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(167, var_1);
    let map_char = [247, 115];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(115, var_1);
    let map_char = [247, 254];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(185, var_1);
    let map_char = [246, 254];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(126, var_1);
    let map_char = [247, 116];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(116, var_1);
    let map_char = [247, 250];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(156, var_1);
    let map_char = [247, 251];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(158, var_1);
    let map_char = [247, 252];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(159, var_1);
    let map_char = [247, 249];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(157, var_1);
    let map_char = [247, 117];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(117, var_1);
    let map_char = [247, 118];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(118, var_1);
    let map_char = [247, 119];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(119, var_1);
    let map_char = [247, 120];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(120, var_1);
    let map_char = [247, 253];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(180, var_1);
    let map_char = [247, 255];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(216, var_1);
    let map_char = [247, 121];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(121, var_1);
    let map_char = [246, 255];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(189, var_1);
    let map_char = [247, 122];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(122, var_1);
    let map_char = [247, 38];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(38, var_1);
    let map_char = [246, 233];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(129, var_1);
    let map_char = [246, 234];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(245, var_1);
    let map_char = [246, 223];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(169, var_1);
    let map_char = [247, 162];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(35, var_1);
    let map_char = [246, 224];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(130, var_1);
    let map_char = [0, 58];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(58, var_1);
    let map_char = [32, 161];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(123, var_1);
    let map_char = [0, 44];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(44, var_1);
    let map_char = [246, 225];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(178, var_1);
    let map_char = [246, 226];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(248, var_1);
    let map_char = [246, 227];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(182, var_1);
    let map_char = [247, 36];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(36, var_1);
    let map_char = [246, 228];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(37, var_1);
    let map_char = [246, 235];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(235, var_1);
    let map_char = [32, 136];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(165, var_1);
    let map_char = [247, 56];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(56, var_1);
    let map_char = [32, 120];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(161, var_1);
    let map_char = [246, 236];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(228, var_1);
    let map_char = [247, 161];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(214, var_1);
    let map_char = [247, 33];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(33, var_1);
    let map_char = [251, 0];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(86, var_1);
    let map_char = [251, 3];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(89, var_1);
    let map_char = [251, 4];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(90, var_1);
    let map_char = [251, 1];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(87, var_1);
    let map_char = [32, 18];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(208, var_1);
    let map_char = [33, 93];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(76, var_1);
    let map_char = [32, 133];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(176, var_1);
    let map_char = [247, 53];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(53, var_1);
    let map_char = [32, 117];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(222, var_1);
    let map_char = [251, 2];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(88, var_1);
    let map_char = [32, 132];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(162, var_1);
    let map_char = [247, 52];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(52, var_1);
    let map_char = [32, 116];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(221, var_1);
    let map_char = [32, 68];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(47, var_1);
    let map_char = [0, 45];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(45, var_1);
    let map_char = [246, 229];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(95, var_1);
    let map_char = [246, 230];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(209, var_1);
    let map_char = [246, 237];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(233, var_1);
    let map_char = [246, 238];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(241, var_1);
    let map_char = [246, 239];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(247, var_1);
    let map_char = [32, 137];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(187, var_1);
    let map_char = [247, 57];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(57, var_1);
    let map_char = [32, 121];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(225, var_1);
    let map_char = [32, 127];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(246, var_1);
    let map_char = [32, 36];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(43, var_1);
    let map_char = [33, 91];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(74, var_1);
    let map_char = [246, 220];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(124, var_1);
    let map_char = [0, 189];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(72, var_1);
    let map_char = [32, 129];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(193, var_1);
    let map_char = [247, 49];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(49, var_1);
    let map_char = [0, 188];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(71, var_1);
    let map_char = [0, 185];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(218, var_1);
    let map_char = [33, 83];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(78, var_1);
    let map_char = [246, 240];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(175, var_1);
    let map_char = [32, 141];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(91, var_1);
    let map_char = [32, 125];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(40, var_1);
    let map_char = [32, 142];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(93, var_1);
    let map_char = [32, 126];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(41, var_1);
    let map_char = [0, 46];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(46, var_1);
    let map_char = [246, 231];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(179, var_1);
    let map_char = [246, 232];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(249, var_1);
    let map_char = [247, 191];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(192, var_1);
    let map_char = [247, 63];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(63, var_1);
    let map_char = [246, 241];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(229, var_1);
    let map_char = [246, 221];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(125, var_1);
    let map_char = [0, 59];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(59, var_1);
    let map_char = [33, 94];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(77, var_1);
    let map_char = [32, 135];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(166, var_1);
    let map_char = [247, 55];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(55, var_1);
    let map_char = [32, 119];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(224, var_1);
    let map_char = [32, 134];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(164, var_1);
    let map_char = [247, 54];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(54, var_1);
    let map_char = [32, 118];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(223, var_1);
    let map_char = [0, 32];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(32, var_1);
    let map_char = [246, 242];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(234, var_1);
    let map_char = [33, 92];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(75, var_1);
    let map_char = [32, 131];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(163, var_1);
    let map_char = [247, 51];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(51, var_1);
    let map_char = [0, 190];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(73, var_1);
    let map_char = [246, 222];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(61, var_1);
    let map_char = [0, 179];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(220, var_1);
    let map_char = [246, 243];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(230, var_1);
    let map_char = [32, 37];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(42, var_1);
    let map_char = [32, 130];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(170, var_1);
    let map_char = [247, 50];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(50, var_1);
    let map_char = [0, 178];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(219, var_1);
    let map_char = [33, 84];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(79, var_1);
    let map_char = [32, 128];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(188, var_1);
    let map_char = [247, 48];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(48, var_1);
    let map_char = [32, 112];

    let s_chars: Vec<char> = WStr::from_utf16be(&map_char)
        .unwrap_or(WStr::from_utf16be(&[]).unwrap())
        .chars()
        .collect();
    let var_1: String = s_chars[0 ..].into_iter().collect();
    hash.insert(226, var_1);
    hash
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
    ctxt: &mut PDFObjContext, pid: &ObjectId, r: &Resources, buf: &mut ParseBuffer,
    dump: &mut Option<fs::File>,
) -> ParseResult<()> {
    let mut te = TextExtractor::new(ctxt, pid);
    let tokens = te.parse(buf)?;
    // condense multiple spaces into a single one.
    let mut spaced = false;
    let tokens_all_vec = tokens.val();
    for to in tokens_all_vec {
        let font_name = to.font_name();
        let tokens_all = to.string();
        for t in tokens_all.iter() {
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
                    /* There are three ways to extract unicode mapping:
                     * 1. ToUnicode cmap
                     * 2. If the simple font uses a predefined glyph name,
                     * we can lookup the name on Adobe Flyph List for New fonts
                     * and extract the corresponding unicode value.
                     * 3. If it is a composite font that uses predefined cmap,
                     * then map the character code to a CID according to cmap.
                     * -- obtain registry and ordering from CIDSystemInfo dictionary
                     * -- obtain the cmap from the name constructed
                     * -- map CIDs to unicode, not character codes
                     */
                    // If ToUnicode field present, then we traverse the
                    // cmap object to find the UTF16BE mappings of chars

                    let fonts_dict = r.fonts();
                    let font_dict_key = DictKey::new(font_name.val().to_vec());
                    let mut otherencoding_flag: Option<&FontEncoding> = None;
                    let cmap_obj = match fonts_dict.get(&font_dict_key) {
                        Some(a) => {
                            let unicode_dict = a.to_unicode();
                            let ret = match unicode_dict {
                                Some(uni) => ctxt.lookup_obj(*uni),
                                None => {
                                    match a.encoding().as_ref() {
                                        Some(a_encoding) => {
                                            if &FontEncoding::MacRoman == a_encoding
                                                || &FontEncoding::MacExpert == a_encoding
                                                || &FontEncoding::WinAnsi == a_encoding
                                            {
                                                otherencoding_flag = Some(a_encoding);
                                            }
                                        },
                                        None => {},
                                    }
                                    None
                                },
                            };
                            ret
                        },
                        None => None,
                    };
                    let mut cmap_flag = false;
                    // Stores the entire UTF16BE string from cmap mapping
                    // Only read if cmap_flag is true
                    let mut var: String = "".to_string();
                    match cmap_obj {
                        Some(cmap) => {
                            if let PDFObjT::Stream(s1) = cmap.val() {
                                let decoded_cmap_buf = decode_stream(s1);

                                // If we are unable to decode the cmap stream, we treat the buffer
                                // as if the cmap stream was absent.
                                match decoded_cmap_buf {
                                    Ok(v) => {
                                        let cmap_buf = v.stream().val().content();
                                        // Rely on this library to give us the mappings
                                        // It gives a blank hashmap if the parsing failed
                                        // The library can panic
                                        // Catch the panic and check the output
                                        let prev_hook = panic::take_hook();
                                        panic::set_hook(Box::new(|_info| {}));
                                        let mapping1 = panic::catch_unwind(|| {
                                            match adobe_cmap_parser::get_unicode_map(&cmap_buf) {
                                                Ok(r) => r,
                                                Err(_) => HashMap::new(),
                                            }
                                        });

                                        let mapping = match mapping1 {
                                            Ok(r) => r,
                                            Err(_) => HashMap::new(),
                                        };

                                        panic::set_hook(prev_hook);
                                        // If mapping is present cmap_flag is set
                                        // Not all cmap streams contain the mappings
                                        // If library panicked also we set the mapping to {}
                                        if mapping.keys().len() > 0 {
                                            cmap_flag = true;
                                            // Mappings are of the form {48: [00, 40]}
                                            // [00 40] is the UTF16BE encoding for char 48
                                            for ch in s {
                                                match mapping.get(&(*ch as u32)) {
                                                    Some(map_char) => {
                                                        let s_chars: Vec<char> =
                                                            WStr::from_utf16be(map_char)
                                                                .unwrap_or(
                                                                    WStr::from_utf16be(&[])
                                                                        .unwrap(),
                                                                )
                                                                .chars()
                                                                .collect();
                                                        let var_1: String =
                                                            s_chars[0 ..].into_iter().collect();
                                                        var = format!("{}{}", var, var_1);
                                                        // unconventional way of
                                                        // appending to string
                                                    },
                                                    None => {
                                                        // No mapping for a certain character.
                                                        // Convert decimal to ascii
                                                        var = format!("{}{}", var, *ch as char);
                                                    },
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        // Just throw a warning if we can't decode the cmap stream
                                        ta3_log!(
                                            Level::Warn,
                                            0,
                                            " error decoding cmap stream {:?}",
                                            e
                                        );
                                    },
                                }
                            }
                        },
                        None => {},
                    };
                    if cmap_flag == false {
                        // CMAP file was absent
                        // First check encoding: if set, try deciphering
                        // If not, then just use utf8
                        let mut already_printed = false;
                        // This flag is set if decoding successful
                        // We leave it as false to try UTF8
                        if let Some(encoding) = otherencoding_flag {
                            match encoding {
                                FontEncoding::MacRoman => {
                                    match MAC_ROMAN.decode(s, DecoderTrap::Replace) {
                                        Ok(res) => {
                                            match dump {
                                                None => println!("{}", res),
                                                Some(f) => {
                                                    let _ = write!(f, "{}", res);
                                                },
                                            };
                                            already_printed = true;
                                        },
                                        Err(e) => {
                                            ta3_log!(
                                                Level::Warn,
                                                0,
                                                " error decoding WINDOWS 1252 stream: {:?}",
                                                e
                                            );
                                        },
                                    };
                                },
                                FontEncoding::MacExpert => {
                                    // This is a special encoding, we wrote
                                    // a function for that
                                    let mac_expert = mac_expert_encoding();
                                    for ch in s {
                                        match mac_expert.get(&(*ch as u32)) {
                                            Some(map_char) => {
                                                var = format!("{}{}", var, map_char);
                                                // unconventional way of
                                                // appending to string
                                            },
                                            None => {
                                                // No mapping for a certain character.
                                                // Convert decimal to ascii
                                                var = format!("{}{}", var, *ch as char);
                                            },
                                        }
                                    }
                                    match dump {
                                        None => println!("{}", var),
                                        Some(f) => {
                                            let _ = write!(f, "{}", var);
                                        },
                                    };
                                    already_printed = true;
                                },
                                FontEncoding::WinAnsi => {
                                    match WINDOWS_1252.decode(s, DecoderTrap::Replace) {
                                        Ok(res) => {
                                            match dump {
                                                None => println!("{}", res),
                                                Some(f) => {
                                                    let _ = write!(f, "{}", res);
                                                },
                                            };
                                            already_printed = true;
                                        },
                                        Err(e) => {
                                            ta3_log!(
                                                Level::Warn,
                                                0,
                                                " error decoding WINDOWS 1252 stream: {:?}",
                                                e
                                            );
                                        },
                                    };
                                },
                                _ => {},
                            }
                            if already_printed == false {
                                // Unsuccessful at decoding
                                match std::str::from_utf8(s) {
                                    Ok(v) => match dump {
                                        None => println!("{}", v),
                                        Some(f) => {
                                            let _ = write!(f, "{}", v);
                                        },
                                    },
                                    Err(_) => (), // println!("not UTF8"),
                                };
                            }
                        } else {
                            match std::str::from_utf8(s) {
                                Ok(v) => match dump {
                                    None => println!("{}", v),
                                    Some(f) => {
                                        let _ = write!(f, "{}", v);
                                    },
                                },
                                Err(_) => (), // println!("not UTF8"),
                            };
                        }
                    } else {
                        // cmap was present, so read var instead of s directly
                        match dump {
                            None => println!("{}", var),
                            Some(f) => {
                                let _ = write!(f, "{}", var);
                            },
                        }
                    }
                    spaced = false
                },
            }
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

fn chk_info(fi: &FileInfo, ctxt: &mut PDFObjContext, info_id: ObjectId) -> Option<String> {
    match ctxt.lookup_obj(info_id) {
        Some(obj) => {
            let producer: Option<String> = match obj.val() {
                PDFObjT::Dict(d) => d
                    .get("Producer".as_bytes())
                    .map_or(None, |a| match a.val() {
                        PDFObjT::String(string) => {
                            str::from_utf8(string).map_or(None, |prod| Some(prod.to_string()))
                        },
                        _ => None,
                    }),
                _ => None,
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
            producer
        },
        None => {
            ta3_log!(Level::Warn, 0, "Info object {:?} not found!", info_id);
            None
        },
    }
}

fn type_check_file(
    fi: &FileInfo, ctxt: &mut PDFObjContext, root_id: ObjectId, info_id: Option<ObjectId>,
) {
    let producer = info_id.map_or(None, |id| chk_info(fi, ctxt, id));

    let root_obj: &Rc<LocatedVal<PDFObjT>> = match ctxt.lookup_obj(root_id) {
        Some(obj) => obj,
        None => exit_log!(0, "Root object {:?} not found!", root_id),
    };

    let mut tctx = TypeCheckContext::new();
    let typ = catalog_type(&mut tctx);
    if let Some(err) = check_type(&ctxt, &tctx, Rc::clone(&root_obj), typ) {
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
            process::exit(0);
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
    text_dump_file: &mut Option<fs::File>, cleaned_file: &mut Option<fs::File>, reducer_flag: bool,
) {
    dump_file(fi, ctxt, root_id);
    let (mut object_ids, mut objects) = ctxt.defns();
    if reducer_flag {
        /*
         * Add an additional pass to fix minor malformations
         * We are not invoking the serializer just yet, but it exists
         */
        let root_obj: &Rc<LocatedVal<PDFObjT>> = match ctxt.lookup_obj(root_id) {
            Some(obj) => obj,
            None => exit_log!(0, "Root object {:?} not found!", root_id),
        };
        let (tmp_object_ids, tmp_objects) = reduce(root_obj, object_ids, objects, root_id);
        for id in 0 .. tmp_object_ids.len() {
            ctxt.insert(tmp_object_ids[id], tmp_objects[id].clone());
        }
        object_ids = tmp_object_ids;
        objects = tmp_objects;
    }
    type_check_file(fi, ctxt, root_id, info_id);
    file_extract_text(ctxt, root_id, text_dump_file);
    match cleaned_file {
        Some(f) => {
            serializer(object_ids, objects, root_id, f, info_id);
        },
        None => {},
    }
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
            Arg::with_name("strict type checking")
                .long("strict")
                .short("s")
                .value_name("STRICT")
                .takes_value(false)
                .help("apply a strict, version-specific type checker on the PDF file"),
        )
        .arg(
            Arg::with_name("apply fixups")
                .short("f")
                .long("fix")
                .value_name("FIX")
                .takes_value(false)
                .help("apply fixup transformations to remove common type malformations"),
        )
        .arg(
            Arg::with_name("output_cleanup")
                .short("r")
                .long("rewrite")
                .value_name("OUTPUT_PDF_FILE")
                .takes_value(true)
                .help("produce a canonical PDF file from the Parsley intermediate representation"),
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
    let mut output_cleanup_file = if matches.is_present("output_cleanup") {
        let fname = matches.value_of("output_cleanup").unwrap();
        match fs::File::create(fname) {
            Ok(f) => Some(f),
            Err(e) => exit_log!(0, "Could not create output file at {}: {}", fname, e),
        }
    } else {
        None
    };
    let reducer_flag = if matches.is_present("apply fixups") {
        true
    } else {
        false
    };
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
    process_file(
        &fi,
        &mut ctxt,
        root_id,
        info_id,
        &mut output_text_file,
        &mut output_cleanup_file,
        reducer_flag,
    );
}

#[cfg(feature = "kuduafl")]
fn main() {
    let path = std::env::current_dir();
    let path = path.unwrap();
    afl::fuzz!(|data: &[u8]| {
        let (fi, mut ctxt, root_id, info_id) = parse_data(&path, data);
        process_file(&fi, &mut ctxt, root_id, info_id, &mut None, &mut None);
    });
}
