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

use flate2::write::ZlibDecoder;
use lzw::{Decoder, DecoderEarlyChange, LsbReader};
use std::io::Write;

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, Location, ParseBuffer, ParseBufferT,
};
use super::super::pcore::transforms::{BufferTransformT, TransformResult};
use super::super::pdf_lib::pdf_obj::DictT;
use super::super::pdf_lib::pdf_obj::PDFObjT;

pub struct FlateDecode<'a> {
    options: &'a Option<&'a DictT>,
}

impl FlateDecode<'_> {
    pub fn new<'a>(options: &'a Option<&'a DictT>) -> FlateDecode { FlateDecode { options } }
}

impl BufferTransformT for FlateDecode<'_> {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {
        // Extract values from options if available, else use defaults.
        let predictor = self
            .options
            .and_then(|x| x.get(b"Predictor"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);
        let colors = self
            .options
            .and_then(|x| x.get(b"Colors"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);
        let bitspercolumn = self
            .options
            .and_then(|x| x.get(b"BitsPerComponent"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(8);
        let columns = self
            .options
            .and_then(|x| x.get(b"Columns"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);
        let _earlyexchange = self
            .options
            .and_then(|x| x.get(b"EarlyExchange"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);

        let mut decoder = ZlibDecoder::new(Vec::new());

        // PDF streams can have bytes trailing the filter content, so
        // write_all() could cause spurious errors due to the trailing
        // bytes not being consumed by the decoder.  Since write() has
        // an internal consuming loop, we could rely on it to consume
        // all relevant bytes in a single call.

        if let Err(e) = decoder.write(buf.buf()) {
            let err = ErrorKind::TransformError(format!("flatedecode write error: {}", e));
            let loc = buf.get_location();
            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        };
        // otherwise, all bytes were consumed.

        match decoder.finish() {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode finish error: {}", e));
                let loc = buf.get_location();
                Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(decoded) => flate_lzw_filter(
                decoded,
                &buf.get_location(),
                predictor as usize,
                colors as usize,
                columns as usize,
                bitspercolumn as usize,
            ),
        }
    }
}

// the paeth prediction algorithm
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a + b - c;
    let pa = if p > a { p - a } else { a - p };
    let pb = if p > b { p - b } else { b - p };
    let pc = if p > c { p - c } else { c - p };

    // algorithm
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

pub struct LZWDecode<'a> {
    options: &'a Option<&'a DictT>,
}

impl LZWDecode<'_> {
    pub fn new<'a>(options: &'a Option<&'a DictT>) -> LZWDecode { LZWDecode { options } }
}

impl BufferTransformT for LZWDecode<'_> {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {
        // Extract values from options if available, else use defaults.
        let predictor = self
            .options
            .and_then(|x| x.get(b"Predictor"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);
        let colors = self
            .options
            .and_then(|x| x.get(b"Colors"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);
        let bitspercolumn = self
            .options
            .and_then(|x| x.get(b"BitsPerComponent"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(8);
        let columns = self
            .options
            .and_then(|x| x.get(b"Columns"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);
        let earlyexchange = self
            .options
            .and_then(|x| x.get(b"EarlyExchange"))
            .and_then(|x| match x.val() {
                PDFObjT::Integer(x) => Some(x.int_val()),
                _ => None,
            })
            .unwrap_or(1);

        let decoded = decode_bytes_lzw(buf, earlyexchange);

        flate_lzw_filter(
            decoded,
            &buf.get_location(),
            predictor as usize,
            colors as usize,
            columns as usize,
            bitspercolumn as usize,
        )
    }
}

fn decode_bytes_lzw(buf: &dyn ParseBufferT, earlyexchange: i64) -> Vec<u8> {
    let mut out = Vec::<u8>::new();
    let reader = LsbReader::new();
    let size: u8 = 8;

    if earlyexchange == 1 {
        let mut decoder = DecoderEarlyChange::new(reader, size);
        let mut read = 0;
        let input = buf.buf();
        loop {
            if read >= input.len() {
                break
            }
            let (len, bytes) = decoder.decode_bytes(&input[read ..]).unwrap();
            read += len;
            out.extend(bytes.iter().copied());
        }
    } else {
        let mut decoder = Decoder::new(reader, size);
        let mut read = 0;
        let input = buf.buf();
        loop {
            if read >= input.len() {
                break
            }
            let (len, bytes) = decoder.decode_bytes(&input[read ..]).unwrap();
            read += len;
            out.extend(bytes.iter().copied());
        }
    }
    out
}

fn flate_lzw_filter(
    decoded: Vec<u8>, loc: &dyn Location, predictor: usize, colors: usize, columns: usize,
    bitspercolumn: usize,
) -> TransformResult {
    let mut row_data = Vec::<u8>::new();
    let mut out_buffer = Vec::<u8>::new();

    if predictor == 1 {
        Ok(ParseBuffer::new(decoded))
    } else if predictor == 2 {
        // TIFF encoding
        let row_length = columns * colors;
        if row_length < 1 {
            // No data.
            return Ok(ParseBuffer::new([].to_vec()))
        }

        let rows = decoded.len() / row_length;
        if decoded.len() % row_length != 0 {
            let err = ErrorKind::TransformError(format!(
                "PNG filter: decoded size {} does not match multiple of expected row size {}",
                decoded.len(),
                row_length
            ));
            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        }
        for i in 0 .. rows {
            row_data.clear();

            // get a row
            for d in decoded
                .iter()
                .take(row_length * (i + 1))
                .skip(row_length * i)
            {
                row_data.push(*d);
            }
            // Predicts based on the sample to the left,
            // interleaved by colors.
            for j in colors .. row_length {
                row_data[j] += row_data[j - colors];
            }
            // add to output
            for &e in &row_data {
                out_buffer.push(e);
            }
        }
        Ok(ParseBuffer::new(out_buffer))
    } else if predictor >= 10 && predictor <= 15 {
        // PNG
        let row_length = columns * colors + 1;
        let rows = decoded.len() / row_length;
        let bytes_per_pixel = bitspercolumn / 8;

        if row_length > decoded.len() {
            let err = ErrorKind::TransformError(
                "PNG filter: decoded size too small for specified columns".to_string(),
            );
            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        }
        if decoded.len() % row_length != 0 {
            let err = ErrorKind::TransformError(format!(
                "PNG filter: decoded size {} does not match multiple of expected row size {}",
                decoded.len(),
                row_length
            ));
            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        }

        let mut prev_row = vec![0; row_length];
        for r in 0 .. rows {
            row_data.clear();
            for d in decoded
                .iter()
                .take(row_length * (r + 1))
                .skip(row_length * r)
            {
                row_data.push(*d)
            }
            match predictor {
                10 => {
                    // PNG None
                    if row_data[0] != 0 {
                        let err = ErrorKind::TransformError(format!(
                            "PNG filter: row filter {} is not None for None predictor",
                            row_data[0]
                        ));
                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                    }
                },
                11 => {
                    // PNG Sub
                    if row_data[0] != 1 {
                        let err = ErrorKind::TransformError(format!(
                            "PNG filter: row filter {} is not Sub for Sub predictor",
                            row_data[0]
                        ));
                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                    }
                    for k in 1 + bytes_per_pixel .. row_length {
                        row_data[k] = row_data[k].wrapping_add(row_data[k - bytes_per_pixel])
                    }
                },
                12 => {
                    // PNG Up
                    if row_data[0] != 2 {
                        let err = ErrorKind::TransformError(format!(
                            "PNG filter: row filter {} is not Up for Up predictor",
                            row_data[0]
                        ));
                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                    }
                    for j in 1 .. row_length {
                        row_data[j] = row_data[j].wrapping_add(prev_row[j]);
                    }
                },
                13 => {
                    // PNG Avg
                    if row_data[0] != 3 {
                        let err = ErrorKind::TransformError(format!(
                            "PNG filter: row filter {} is not Avg for Avg predictor",
                            row_data[0]
                        ));
                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                    }
                    for j in 1 .. 1 + bytes_per_pixel {
                        row_data[j] = row_data[j].wrapping_add(prev_row[j] / 2);
                    }
                    for j in bytes_per_pixel .. row_length {
                        let incr = (row_data[j - bytes_per_pixel] + prev_row[j]) / 2;
                        row_data[j] = row_data[j].wrapping_add(incr)
                    }
                },
                14 => {
                    if row_data[0] != 4 {
                        let err = ErrorKind::TransformError(format!(
                            "PNG filter: row filter {} is not Paeth for Paeth predictor",
                            row_data[0]
                        ));
                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                    }
                    // Paeth algorithm prediction.
                    let mut a = 0;
                    let mut c = 0;
                    for j in 1 .. row_length {
                        let b = prev_row[j];
                        if j > bytes_per_pixel {
                            a = row_data[j - bytes_per_pixel];
                            c = prev_row[j - bytes_per_pixel];
                        }
                        row_data[j] = paeth(a, b, c);
                    }
                },
                _ => {
                    let err = ErrorKind::TransformError(format!(
                        "PNG filter: unknown predictor {}",
                        predictor
                    ));
                    return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                },
            }
            // update prev row
            prev_row[.. row_length].clone_from_slice(&row_data[.. row_length]);
            // put data in output buffer
            for d in row_data.iter().take(row_length).skip(1) {
                out_buffer.push(*d);
            }
        }
        Ok(ParseBuffer::new(out_buffer))
    } else {
        let err = ErrorKind::TransformError(format!("PNG filter: unknown predictor {}", predictor));
        Err(locate_value(err, loc.loc_start(), loc.loc_end()))
    }
}
