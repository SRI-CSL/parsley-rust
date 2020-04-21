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

use std::io::Write;
use flate2::write::DeflateDecoder;

use super::super::pcore::parsebuffer::{
    ParseBufferT, ParseBuffer, ErrorKind, Location, locate_value
};
use super::super::pcore::transforms::{
    BufferTransformT, TransformResult
};
use super::super::pdf_lib::pdf_obj::{DictT};

pub struct FlateDecode;

impl FlateDecode {
    pub fn new(_: &Option<&DictT>) -> Self { Self }
}

impl BufferTransformT for FlateDecode {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {
        let mut decoder = DeflateDecoder::new(Vec::new());
        match decoder.write_all(buf.buf()) {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode error: {}", e));
                let loc = buf.get_location();
                return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(()) => {}
        }
        match decoder.finish() {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode error: {}", e));
                let loc = buf.get_location();
                return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(decoded) => {
                Ok(ParseBuffer::new(decoded))
            }
        }
    }
}
