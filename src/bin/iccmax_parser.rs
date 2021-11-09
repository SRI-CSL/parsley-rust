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

extern crate clap;
extern crate env_logger;
extern crate log;
extern crate log_panics;
extern crate serde;
extern crate serde_json;

use std::io::Write;
use std::path::Path;

use env_logger::Builder;
use log::{Level, LevelFilter};

use clap::{App, Arg};

use parsley_rust::iccmax_lib::execution_tree::ExecutionTree;
use parsley_rust::iccmax_lib::iccmax_prim::{
    CalcFunctionP, CalculatorElementP, GeneralElementP, HeaderP, MPetElementP, MPetOptions,
    TaggedElementP,
};
use parsley_rust::pcore::parsebuffer::{ErrorKind, LocatedVal, ParseBuffer, ParsleyParser};

use parsley_rust::pcore::prim_binary::{Endian, UInt32P};
use parsley_rust::pcore::prim_combinators::{Alt, Alternate};

use std::fs::File;
use std::io::Read;

type IccError = String;
type IccResult<T> = std::result::Result<T, IccError>;

fn read_file(file_name: &str) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    return data
}

fn parse_iccmax(data: Vec<u8>) -> IccResult<IccError> {
    let mut pb = ParseBuffer::new(data);
    // Header consumes 128 bytes
    let mut parser = HeaderP;
    parser.parse(&mut pb).ok();
    // Consume the next 4 bytes to get the count: number of Tags
    let mut tag_count_parser = UInt32P::new(Endian::Big);
    let tag_count_p = tag_count_parser.parse(&mut pb).unwrap();
    let tag_count = tag_count_p.val();
    //println!("Tag {:?}", *tag_count);
    let mut counter = 0;
    let mut tagged_element_parser = TaggedElementP;
    while counter < *tag_count {
        // Tag signature
        // Offset to beginning of tag element
        // Size of tag data element
        let tag_e = tagged_element_parser.parse(&mut pb).unwrap();
        let tag = tag_e.val();
        // With the tag now we need to seek to those locations and check
        // if it is a calculator element
        let mut window = ParseBuffer::new_view(&mut pb, tag.offset() as usize, tag.size() as usize);
        let mut mpet_parser = MPetElementP;
        let mpet_e = mpet_parser.parse(&mut window);
        match mpet_e {
            Ok(v) => {
                let output_channels_outer = v.val().clone().output_channels();
                let input_channels_outer = v.val().clone().input_channels();
                let v1 = v.val().clone();
                let v2 = v.val().clone();
                // Check if it is an MPET element
                if v1.signature() {
                    //println!("{:?}", v2);
                    // Extract list of positionNumbers
                    let position_list = v2.proc_table();

                    // Uncomment to debug
                    //println!("Outer values--- output-{:?} input-{:?} elements-{:?}",
                    //output_channels_outer, input_channels_outer, position_list.len());
                    let mut counter = 0;
                    let mut previous_output_channel = 0;
                    //println!("{:?}", position_list.len());
                    for position in &position_list {
                        // Create a view, run the Calculator parser
                        // or the GeneralElement parser
                        let mut calc_buffer = ParseBuffer::new_view(
                            &mut window,
                            position.position() as usize,
                            position.size() as usize,
                        );
                        let mut calc_parser = CalculatorElementP;
                        let mut general_parser = GeneralElementP;
                        let mut parser = Alternate::new(&mut calc_parser, &mut general_parser);
                        match parser.parse(&mut calc_buffer) {
                            Ok(v) => {
                                // Uncomment to debug
                                //println!("{:?}", v);

                                /*
                                 * 1. Extract subelements and parse them
                                 * 2. Parse the main function and parse it
                                 */
                                //----------------------
                                let v_cloned = v.clone();
                                match v_cloned.unwrap() {
                                    Alt::Left(t) => {
                                        let main_function_position =
                                            t.clone().unwrap().main_function_position();
                                        let main_function_size =
                                            t.clone().unwrap().main_function_size();
                                        let mut main_buf = ParseBuffer::new_view(
                                            &mut calc_buffer,
                                            main_function_position as usize,
                                            main_function_size as usize,
                                        );
                                        //ParseBuffer::buf_to_string(&mut calc_buffer);
                                        //ParseBuffer::buf_to_string(&mut main_buf);

                                        let positions_list = t.unwrap().subelement_positions();
                                        let mut pos_array: Vec<MPetOptions> = vec![];
                                        for position in positions_list {
                                            let mut subelement_buf = ParseBuffer::new_view(
                                                &mut calc_buffer,
                                                position.position() as usize,
                                                position.size() as usize,
                                            );
                                            let result = parser.parse(&mut subelement_buf);
                                            match result.unwrap().unwrap() {
                                                Alt::Left(v) => {
                                                    let s =
                                                        MPetOptions::new(Some(v.unwrap()), None);
                                                    pos_array.push(s);
                                                },
                                                Alt::Right(v) => {
                                                    let s =
                                                        MPetOptions::new(None, Some(v.unwrap()));
                                                    pos_array.push(s);
                                                },
                                            }
                                        }
                                        //println!("{:?}", pos_array);
                                        let pos_array_clone = pos_array.clone();

                                        let mut func_parser = CalcFunctionP::new(pos_array);
                                        let func_result = func_parser.parse(&mut main_buf);
                                        let mut exec = ExecutionTree::new(
                                            0,
                                            0,
                                            None,
                                            func_result.unwrap().unwrap().instructions(),
                                            false,
                                            pos_array_clone,
                                        );
                                        let ret = exec.execute()?;
                                        //println!("{:?}", ret);
                                        //if let Err(s) = &ret {
                                        //let err = ErrorKind::GuardError(s.
                                        // clone());
                                        // let err = LocatedVal::new(err, start,
                                        // buf.get_cursor());
                                        // println!("{:?}", err);
                                        //return Err(err)
                                        //}
                                    },
                                    Alt::Right(_) => {},
                                }

                                //----------------------

                                // We need to make copies because Alt struct does not implement the
                                // Copy trait
                                let parsed_object1 = v.unwrap();
                                let parsed_object2 = parsed_object1.clone();
                                let parsed_object3 = parsed_object2.clone();

                                // Logic to check the chaining of input/output channels
                                // If we are the first value,
                                // MPET element input_channels value must be equal to the first
                                // elements input_channels value
                                if counter == 0 {
                                    match parsed_object1 {
                                        Alt::Left(t) => {
                                            let unwrapped_t_1 = t.unwrap();
                                            let unwrapped_t_2 = unwrapped_t_1.clone();
                                            assert_eq!(
                                                input_channels_outer,
                                                unwrapped_t_1.input_channels()
                                            );
                                            previous_output_channel =
                                                unwrapped_t_2.output_channels();
                                        },
                                        Alt::Right(t) => {
                                            let unwrapped_t_1 = t.unwrap();
                                            let unwrapped_t_2 = unwrapped_t_1.clone();
                                            assert_eq!(
                                                input_channels_outer,
                                                unwrapped_t_1.input_channels()
                                            );
                                            previous_output_channel =
                                                unwrapped_t_2.output_channels();
                                        },
                                    }
                                }

                                // If we are at the last value
                                if counter == position_list.len() - 1 {
                                    match parsed_object2 {
                                        Alt::Left(t) => {
                                            let unwrapped_t_1 = t.unwrap();
                                            let unwrapped_t_2 = unwrapped_t_1.clone();
                                            assert_eq!(
                                                output_channels_outer,
                                                unwrapped_t_1.output_channels()
                                            );
                                            // If this element is not also the first
                                            if counter != 0 {
                                                assert_eq!(
                                                    previous_output_channel,
                                                    unwrapped_t_2.input_channels()
                                                );
                                            }
                                        },
                                        Alt::Right(t) => {
                                            let unwrapped_t_1 = t.unwrap();
                                            let unwrapped_t_2 = unwrapped_t_1.clone();
                                            assert_eq!(
                                                output_channels_outer,
                                                unwrapped_t_1.output_channels()
                                            );
                                            // If this element is not also the first
                                            if counter != 0 {
                                                assert_eq!(
                                                    previous_output_channel,
                                                    unwrapped_t_2.input_channels()
                                                );
                                            }
                                        },
                                    }
                                }

                                // If we are neither first nor last
                                if counter != position_list.len() - 1 && counter != 0 {
                                    match parsed_object3 {
                                        Alt::Left(t) => {
                                            let unwrapped_t_1 = t.unwrap();
                                            let unwrapped_t_2 = unwrapped_t_1.clone();
                                            assert_eq!(
                                                previous_output_channel,
                                                unwrapped_t_1.input_channels()
                                            );
                                            previous_output_channel =
                                                unwrapped_t_2.output_channels();
                                        },
                                        Alt::Right(t) => {
                                            let unwrapped_t_1 = t.unwrap();
                                            let unwrapped_t_2 = unwrapped_t_1.clone();
                                            assert_eq!(
                                                previous_output_channel,
                                                unwrapped_t_1.input_channels()
                                            );
                                            previous_output_channel =
                                                unwrapped_t_2.output_channels();
                                        },
                                    }
                                }
                            },
                            Err(_e) => {
                                println!("{:?}", _e);
                            },
                        }

                        counter = counter + 1;
                    }

                    // There are eight types of  operation  encodings  (Push
                    // floating  point  constant,  channel  vector,  sub-element
                    // invocation,  stack operation, matrix, sequence
                    // functional, function vector, and conditional).

                    //ParseBuffer::buf_to_string(&mut new_buf);
                }
            },
            Err(_) => {},
        }
        counter = counter + 1;
    }
    Ok("".to_string())
}

fn main() {
    // parsing command line arguments:
    let matches = App::new("Parsley ICC Parser")
        // TODO: use Cargo Metadata here?  See ../../cargo.toml
        // .version("0.1.0")
        // .author("Prashanth Mundkur <prashanth.mundkur@gmail.com>")
        .about("=> parses given ICC file")
        .arg(
            Arg::with_name("icc_file")
                .value_name("ICC_FILE")
                .help("the ICC file to parse")
                .required(true)
                .index(1),
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
    let filename = Path::new(matches.value_of("icc_file").unwrap())
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

    println!("{:?}", matches.value_of("icc_file").unwrap());
    let buffer = read_file(matches.value_of("icc_file").unwrap());
    let r = parse_iccmax(buffer);
    println!("{:?}", r);
}
