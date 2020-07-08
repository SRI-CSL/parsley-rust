// Copyright (c) 2020 SRI International.
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

// use this macro to log messages with position argument:

use log::{Level};

macro_rules! ta3_log {
    ($lvl:expr, $pos:expr, $($arg:tt)+) => ({
        log!($lvl, "at {:>10} - {}", $pos, format_args!($($arg)+))
    })
}

use flate2::write::ZlibDecoder;
use std::io::Write;

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, Location, ParseBuffer, ParseBufferT,
};
use super::super::pcore::transforms::{BufferTransformT, TransformResult};
use super::super::pdf_lib::pdf_obj::DictT;

pub struct FlateDecode<'a> {
    options: &'a Option<&'a DictT>,
}

impl FlateDecode<'_> {
    pub fn new<'a>(options: &'a Option<&'a DictT>) -> FlateDecode { FlateDecode { options } }
}

impl BufferTransformT for FlateDecode<'_> {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {
        let comp_size = buf.buf().len();
        let mut decoder = ZlibDecoder::new(Vec::new());
        // PDF streams can have bytes trailing the filter content, so
        // write_all() could cause spurious errors due to the trailing
        // bytes not being consumed by the decoder.  Since write() has
        // an internal consuming loop, we could rely on it to consume
        // all relevant bytes in a single call.
        match decoder.write(buf.buf()) {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode write error: {}", e));
                let loc = buf.get_location();
                return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(_n) => {
                // 'n' bytes consumed
            },
        }
        let res = match decoder.finish() {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode finish error: {}", e));
                let loc = buf.get_location();
                return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(decoded) => {
                let decomp_size = decoded.len();
                ta3_log!(
                    Level::Info,
                    0,
                    " DECOMPRESSION: compressed_size={}  decompress_size={}",
                    comp_size,
                    decomp_size,
                );
                println!("comp_size={} decomp_size={}", comp_size, decomp_size);
                Ok(ParseBuffer::new(decoded))
            },
        };
        if self.options.is_some() {
            let err = ErrorKind::TransformError(format!(
                "flatedecode error: filter options not yet supported"
            ));
            let loc = buf.get_location();
            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        }
        res
    }
}
