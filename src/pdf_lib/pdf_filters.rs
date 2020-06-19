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
            .unwrap_or(-1);

        let colors = self.options
            .and_then(|x| { x.get(b"Colors") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(-1);

        let bitspercolumn = self.options
            .and_then(|x| { x.get(b"BitsPerComponent") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(-1);

        let columns = self.options
            .and_then(|x| { x.get(b"Columns") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(-1);

        let earlyexchange = self.options
            .and_then(|x| { x.get(b"EarlyExchange") })
            .and_then(|x| { match x.val() {
                             PDFObjT::Integer(x) => Some(x.int_val()),
                             _ => None
                        }})
            .unwrap_or(-1);

        // debug info
        println!("Read options as:");
        println!("Predictor: {:?}", predictor);
        println!("Colors: {:?}", colors);
        println!("Bpc: {:?}", bitspercolumn);
        println!("Early exhange: {:?}", earlyexchange);
        println!("All options {:?}", self.options);

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
                // n bytes consumed
            },
        }

        match decoder.finish() {
            Err(e) => {
                let err = ErrorKind::TransformError(format!("flatedecode finish error: {}", e));
                let loc = buf.get_location();
                return Err(locate_value(err, loc.loc_start(), loc.loc_end()))
            },
            Ok(decoded) => {
                println!("Flate decoding finished!");
                println!("Predictor value: {:?}", predictor);

                let mut row_data = std::vec::Vec::<u8>::new();
                let mut pOutBuffer = std::vec::Vec::<u8>::new();

                if predictor > 1 {
                    if predictor == 2 {
                        println!("Tiff encoding");

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

                        println!("here!! {:?}", decoded);


                        // 0-255  -255 255 ; 0-255=-255;
                        for i in 0..rows {
                            // get a row
                            for k in row_length*i .. row_length*(i+1) {
                                let v = decoded[k as usize];
                                row_data.push(v);
                            }
                            // Predicts the same as the sample to the left.
                            // Interleaved by colors.
                            for j in colors.. row_length {
                                row_data[j as usize] += row_data[(j-colors) as usize];
                            }

                            // add to output
                            for e in &row_data { pOutBuffer.push(*e); }
                        }

                        return Ok(ParseBuffer::new(pOutBuffer));

                    } else if predictor >= 10 && predictor <= 15 {
                        // PNG
                        let row_length = columns * colors + 1;
                        let rows = (decoded.len() as i64) /row_length;

                        if (decoded.len() as i64) % row_length != 0 {
                            println!("invalid row length");
                        }

                        if row_length > (decoded.len() as i64) {
                            println!("row length cannot be greater than data len");
                        }

                        let mut prevRow = std::vec::Vec::<u8>::new();

                        for _ in 0..row_length {
                            prevRow.push(0);
                        }

                        // TODO: fixme
                        let bytes_per_pixel = 8;
                        for i in 0 .. rows {
                            for j in row_length*i .. row_length*(i+1) {
                                row_data.push(decoded[j as usize]);
                            }

                            let fb = row_data[0];

                            match fb {
                                0 => {
                                    // pfNone no prediction (raw).
                                    println!("No prediction!")
                                }
                                1 => {
                                    // pfSub predicts same as left sample.
                                    for k in 1 + bytes_per_pixel..row_length {
                                        row_data[k as usize] = row_data[(k - bytes_per_pixel) as usize]
                                    }
                                }
                                2 => {
                                    // pfUp  Predicts same as sample above.
                                    for j in 1 .. row_length {
                                        row_data[j as usize] += prevRow[j as usize];
                                    }
                                }
                                3 => {
                                    // pfAvg  Predict based on left and above.
                                    for j in 1..bytes_per_pixel+1 {
                                        row_data[j as usize] += prevRow[j as usize]/2;
                                    }

                                    for j in bytes_per_pixel+1..row_length {
                                        row_data[j as usize] = (row_data[(j-bytes_per_pixel) as usize] + prevRow[j as usize])/2;
                                    }
                                }
                                4 => {
                                    // pfPaeth  Paeth algorithm prediction.
                                    let mut a = 0;
                                    let mut c = 0;
                                    for j in 1 .. row_length {
                                        let b = prevRow[j as usize];
                                        if j >= bytes_per_pixel + 1 {
                                            a = row_data[(j - bytes_per_pixel) as usize];
                                            c = prevRow[(j - bytes_per_pixel) as usize];
                                        }
                                        row_data[j as usize] = paeth(a, b, c);
                                    }
                                }
                                _ => { println!("fb > 4!");}
                            }

                            // update prev row
                            for j in 0 .. row_length {
                                prevRow[j as usize] = row_data[j as usize];
                            }

                            // put data in output buffer
                            for i in &row_data {
                                pOutBuffer.push(*i);
                            }
                        }
                    }
                } else {
                    // predictor <= 1
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
