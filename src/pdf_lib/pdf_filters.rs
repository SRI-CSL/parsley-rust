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
        // Extract values from options if available
        let predictor = self.options
            .and_then(|x| { x.get(b"Predictor") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);
        let colors = self.options
            .and_then(|x| { x.get(b"Colors") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);
        let bitspercolumn = self.options
            .and_then(|x| { x.get(b"BitsPerComponent") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(8);
        let columns = self.options
            .and_then(|x| { x.get(b"Columns") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);
        let _earlyexchange = self.options
            .and_then(|x| { x.get(b"EarlyExchange") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);

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
            Ok(_) => {
                // all bytes consumed
            },
        }

        match decoder.finish() {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode finish error: {}", e));
                let loc = buf.get_location();
                return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(decoded) => {
                let mut row_data = std::vec::Vec::<u8>::new();
                let mut out_buffer = std::vec::Vec::<u8>::new();

                if predictor > 1 {
                    if predictor == 2 {
                        // TIFF encoding
                        let row_length = columns * colors;
                        if row_length < 1 {
                            // No data.
                            return Ok(ParseBuffer::new([].to_vec()));
                        }

                        let rows = (decoded.len() as i64) / row_length;
                        if (decoded.len() as i64)%row_length != 0 {
                            println!("ERROR: TIFF encoding: Invalid row length.");
                        }

                        if row_length%colors != 0 {
                            println!("ERROR: TIFF encoding: Invalid row length.");
                        }

                        if row_length > (decoded.len() as i64) {
                            println!("ERROR: Row length cannot be longer than data length.");
                        }

                        for i in 0..rows {
                            row_data.clear();

                            // get a row
                            for k in row_length*i .. row_length*(i+1) {
                                row_data.push(decoded[k as usize]);
                            }
                            // Predicts the same as the sample to the left.
                            // Interleaved by colors.
                            for j in colors.. row_length {
                                row_data[j as usize] += row_data[(j-colors) as usize];
                            }
                            // add to output
                            for &e in &row_data { out_buffer.push(e); }
                        }

                        return Ok(ParseBuffer::new(out_buffer));

                    } else if predictor >= 10 && predictor <= 15 {
                        // PNG
                        let row_length = (columns * colors + 1) as usize;
                        let rows = (decoded.len() as usize)/row_length;
                        let bytes_per_pixel = (bitspercolumn / 8) as usize;

                        if row_length > (decoded.len() as usize) {
                            let err = ErrorKind::TransformError(format!("PNG filter: decoded size too small for specified columns"));
                            let loc = buf.get_location();
                            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                        }

                        if (decoded.len() as usize) % row_length != 0 {
                            let err = ErrorKind::TransformError(format!("PNG filter: decoded size does not match multiple of specified columns"));
                            let loc = buf.get_location();
                            return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                        }

                        let mut prev_row = std::vec::Vec::<u8>::new();
                        for _ in 0 .. row_length {
                            prev_row.push(0);
                        }
                        for r in 0 .. rows {
                            row_data.clear();
                            for j in row_length*r .. row_length*(r+1) {
                                row_data.push(decoded[j]);
                            }

                            match predictor {
                                10 => {
                                    // PNG None
                                    if row_data[0] != 0 {
                                        let err = ErrorKind::TransformError(format!("PNG filter: a row filter is not None for None predictor"));
                                        let loc = buf.get_location();
                                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                                    }
                                }
                                11 => {
                                    // PNG Sub
                                    if row_data[0] != 1 {
                                        let err = ErrorKind::TransformError(format!("PNG filter: a row filter is not Sub for Sub predictor"));
                                        let loc = buf.get_location();
                                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                                    }
                                    for k in 1 + bytes_per_pixel .. row_length {
                                        row_data[k] = row_data[k].wrapping_add(row_data[k - bytes_per_pixel])
                                    }
                                }
                                12 => {
                                    // PNG Up
                                    if row_data[0] != 2 {
                                        let err = ErrorKind::TransformError(format!("PNG filter: a row filter is not Up for Up predictor"));
                                        let loc = buf.get_location();
                                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                                    }
                                    for j in 1 .. row_length {
                                        row_data[j] = row_data[j].wrapping_add(prev_row[j]);
                                    }
                                }
                                13 => {
                                    // PNG Avg
                                    if row_data[0] != 3 {
                                        let err = ErrorKind::TransformError(format!("PNG filter: a row filter is not Avg for Avg predictor"));
                                        let loc = buf.get_location();
                                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                                    }
                                    for j in 1 .. 1 + bytes_per_pixel {
                                        row_data[j] = row_data[j].wrapping_add(prev_row[j]/2);
                                    }
                                    for j in bytes_per_pixel .. row_length {
                                        let incr = (row_data[j - bytes_per_pixel] + prev_row[j]) / 2;
                                        row_data[j] = row_data[j].wrapping_add(incr)
                                    }
                                }
                                14 => {
                                    if row_data[0] != 4 {
                                        let err = ErrorKind::TransformError(format!("PNG filter: a row filter is not Paeth for Paeth predictor"));
                                        let loc = buf.get_location();
                                        return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                                    }
                                    // Paeth algorithm prediction.
                                    let mut a = 0;
                                    let mut c = 0;
                                    for j in 1 .. row_length {
                                        let b = prev_row[j];
                                        if j >= bytes_per_pixel + 1 {
                                            a = row_data[j - bytes_per_pixel];
                                            c = prev_row[j - bytes_per_pixel];
                                        }
                                        row_data[j] = paeth(a, b, c);
                                    }
                                }
                                _ => {
                                    let err = ErrorKind::TransformError(format!("PNG filter: unknown predictor {}", predictor));
                                    let loc = buf.get_location();
                                    return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                                }
                            }

                            // update prev row
                            for j in 0 .. row_length {
                                prev_row[j] = row_data[j];
                            }

                            // put data in output buffer
                            for j in 1 .. row_length {
                                out_buffer.push(row_data[j]);
                            }
                        }
                        return Ok(ParseBuffer::new(out_buffer));
                    }
                } else {
                    let err = ErrorKind::TransformError(format!("PNG filter: unknown predictor {}", predictor));
                    let loc = buf.get_location();
                    return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
                }
                return Ok(ParseBuffer::new(decoded));
            }
        }
    }
}

// the paeth prediction algorithm
fn paeth(a:u8, b:u8, c:u8) -> u8 {
    let p = a + b - c;
    let pa = if p > a {p - a} else { a - p};
    let pb = if p > b {p - b} else { b - p};
    let pc = if p > c {p - c} else { p - c};

    // algorithm
    if pa <= pb && pa <= pc {
        return a;
    } else if pb <= pc {
        return b;
    } else {
        return c;
    }
}

pub struct LZWDecode<'a> {
    options: &'a Option<&'a DictT>,
}

impl LZWDecode<'_> {
    pub fn new<'a>(options: &'a Option<&'a DictT>) -> FlateDecode { FlateDecode { options } }
}

impl BufferTransformT for LZWDecode<'_> {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {

        // Extract values from options if available
        let predictor = self.options
            .and_then(|x| { x.get(b"Predictor") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);

        let colors = self.options
            .and_then(|x| { x.get(b"Colors") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);

        let bitspercolumn = self.options
            .and_then(|x| { x.get(b"BitsPerComponent") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(8);

        let columns = self.options
            .and_then(|x| { x.get(b"Columns") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);

        let earlyexchange = self.options
            .and_then(|x| { x.get(b"EarlyExchange") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(1);

        // debug info
        println!("LZW read options as:");
        println!("Predictor: {:?}", predictor);
        println!("Colors: {:?}", colors);
        println!("Bpc: {:?}", bitspercolumn);
        println!("Early exhange: {:?}", earlyexchange);
        println!("All options {:?}", self.options);

        let mut out = std::vec::Vec::<u8>::new();

        let decoded = decode_bytes_lzw(buf, earlyexchange);

        // apply transformations
        if predictor > 1 {
            if predictor == 2 {
                println!("TIFF encoding");
                let row_length : i64 = colors * columns;

                if row_length < 1 {
                    println!("ERROR: Row length < 1...");
                    return Ok(ParseBuffer::new([].to_vec()));
                }

                let rows = (decoded.len() as i64)/row_length;

		if (decoded.len() as i64) % row_length != 0 {
                    println!("TIFF : invalid row length");
                    return Ok(ParseBuffer::new([].to_vec()));
		}

                if row_length % colors != 0 {
                    println!("Invalid row length for colors.");
                    return Ok(ParseBuffer::new([].to_vec()));
                }

                if row_length > (decoded.len()) as i64 {
                    println!("ERROR: row len cannot be > data len!");

                }

                let mut row_data = vec![];

                for i in 0..rows {
                    row_data.clear();
                    row_data = decoded[((row_length * i) as usize) .. ((row_length * (i+1)) as usize)].to_vec();
                    for j in colors .. row_length {
                        row_data[j as usize] = (((row_data[j as usize] as i64) + (row_data[(j-colors) as usize] as i64)) % 256) as u8;
                    }
                    out.extend(row_data.iter().copied());
                }

                return Ok(ParseBuffer::new(out.to_vec()));
            } else if predictor >= 10 && predictor <= 15 {
                println!("PNG encoding...");
                let row_length = columns * colors + 1;

                if row_length < 1 {
                    println!("ERROR: Row length < 1...");
                    return Ok(ParseBuffer::new([].to_vec()));
                }

                let rows = (decoded.len() as i64)/row_length;

		if (decoded.len() as i64) % row_length != 0 {
                    println!("PNG : invalid row length");
                    return Ok(ParseBuffer::new([].to_vec()));
		}

                if row_length > (decoded.len() as i64) {
                    println!("ERROR: row len cannot be > data len!");
                }

                // output buffer
                let mut out = std::vec::Vec::<u8>::new();

                // init prev row vec
                let mut prev_row = std::vec::Vec::<u8>::new();
                for _ in 0 .. row_length {
                    prev_row.push(0);
                }

                let mut row_data = vec![];

                for i in 0 .. rows {
                    row_data.clear();
                    let start_index = (row_length * i) as usize;
                    let end_index = (row_length * (i + 1)) as usize;

                    row_data = decoded[start_index .. end_index].to_vec();

                    let fb = row_data[0];
                    match fb {
                        0 => println!("no predition made"),
                        1 => {
                            // sub: same as left
                            for k in 2usize .. (row_length as usize) {
                                row_data[k] = (((row_data[k] + row_data[k-1]) as i64) % 256) as u8;
                            }
                        },
                        2 => {
                            for k in 1usize .. (row_length as usize) {
                                row_data[k] = (((row_data[k]+prev_row[k]) as i64) % 256) as u8;
                            }
                        },
                        _ => {
                            println!("Error: invalid filter byte!");
                        }
                    }

                    for k in 0usize .. (row_length as usize) {
                        prev_row[k] = row_data[k];
                    }

                    out.extend(row_data[1 ..].to_vec().iter().copied());
                }

                // return
                return Ok(ParseBuffer::new(out));
            } else {
                println!("ERROR: unsupported predictor!");
                return Ok(ParseBuffer::new([].to_vec()));
            }
        }

        // pred is not > 1
        return Ok(ParseBuffer::new([].to_vec()));
    }
}

fn decode_bytes_lzw (buf : &dyn ParseBufferT, earlyexchange: i64) -> Vec<u8> {
    let mut out = std::vec::Vec::<u8>::new();
    let reader = LsbReader::new();
    let size:u8 = 8;

    if earlyexchange == 1 {
        let mut decoder = DecoderEarlyChange::new(reader, size);
        let mut read = 0;
        let input = buf.buf();
        loop {
            if read >= input.len() {
                break
            }
            let (len, bytes) = decoder.decode_bytes(&input[read..]).unwrap();
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
            let (len, bytes) = decoder.decode_bytes(&input[read..]).unwrap();
            read += len;
            out.extend(bytes.iter().copied());
        }
    }
    return out;
}
