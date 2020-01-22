#[macro_use]
extern crate log;
extern crate env_logger;
extern crate log_panics;
extern crate etherparse;
use etherparse::SlicedPacket;

use std::io::prelude::*;
use std::net::TcpStream;
use env_logger::Builder;
use pcap_parser::*;
use pcap_parser::traits::PcapReaderIterator;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use std::panic;
use std::path::Path;
use std::process;
use std::rc::Rc;
use std::convert::TryInto;
use std::collections::{VecDeque, BTreeSet};
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser, Location, LocatedVal};
use parsley_rust::pcore::prim_binary::*;
use parsley_rust::pcore::prim_combinators::{Sequence};
use byteorder::{ByteOrder, BigEndian};

fn parse_dns(test_file: &str) {
    let path = env::current_dir();
    if let Err(_) = path {
        panic!("Cannot get current directory");
    }
    let mut path = path.unwrap();
    path.push(test_file);
    let display = path.as_path().display();
}

fn print_usage(code: i32) {
    println!("Usage:\n\t{} <dns-capture-file>", env::args().nth(0).unwrap());
    process::exit(code)
}

fn main() -> std::io::Result<()>{
    match (env::args().nth(1), env::args().len()) {
        (Some(s), 2) => {
            // set up log format with file name (if > TRACE):
            //let filename = Path::new(&s).file_name().unwrap().to_str().unwrap().to_string();
            let filename = File::open(&s).unwrap();
            let mut num_blocks = 0;
            let mut reader = LegacyPcapReader::new(65536, filename).expect("LegacyPcapReader");
            loop {
                match reader.next() {
                    Ok((offset, block)) => {
                        //println!("got new block");
                        num_blocks += 1;
                        match block {
                            PcapBlockOwned::LegacyHeader(_hdr) => {
                                // save hdr.network (linktype)
                                //println!("{}", _hdr.network);
                            },
                            PcapBlockOwned::Legacy(_b) => {
                                // use linktype to parse b.data()
                                //println!("{:?}", _b.data);
                                match SlicedPacket::from_ethernet(_b.data) {
                                    Err(value) => println!("Err {:?}", value),
                                    Ok(value) => {
                                        use etherparse::LinkSlice::*;
                                        use etherparse::InternetSlice::*;
                                        use etherparse::TransportSlice::*;
                                        use etherparse::VlanSlice::*;
                                        use bit_vec::BitVec;
                                        use bit_set::BitSet;
                                        // value.payload here contains the DNS payload
                                        // TODO: The pcap I am using contains only DNS packets, I need to filter ot packets with no payload? Or other payloads?
                                        //println!("{:?}", value.payload);
                                        let mut pb1 = ParseBuffer::new(value.payload.to_vec());
                                        let mut pb2 = ParseBuffer::new(value.payload.to_vec());
                                        let mut s3 = IntObj32::new();
                                        let mut s2 = IntObj32::new();
                                        let mut s1 = BitObj8::new();
                                        let mut s = Sequence::new(&mut s1, &mut s2);
                                        let mut s_new = Sequence::new(&mut s, &mut s3);
                                        println!("{:?}", s_new.parse(&mut pb1));
                                        println!("{:?}", s1.parse(&mut pb2));
                                        println!("{:?}", s2.parse(&mut pb2));
                                        // insert all primes less than 10

                                    }
                                }
                            },
                            PcapBlockOwned::NG(_) => unreachable!(),
                        }
                        reader.consume(offset);
                    },
                    Err(PcapError::Eof) => break,
                    Err(PcapError::Incomplete) => {
                        reader.refill().unwrap();
                    },
                    Err(e) => panic!("error while reading: {:?}", e),
                }
            }
            println!("num_blocks: {}", num_blocks);
            parse_dns(&s)
        }
        (_,_) => print_usage(1)
    }
        let mut character = TokenParser::new("\x01\x01", 2);
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("\x01\x02".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = vec![0];
        let mut r = character.parse(&mut pb);
        println!("---{:?}---", r);
    Ok(())
}
//let mut stream = TcpStream::connect("127.0.0.1:8000")?;
//{
//let mut string = "abcd";
//stream.write(b"abcd")?;
//let mut buffer = String::new();
//stream.read_to_string(&mut buffer)?;
//let mut pb = ParseBuffer::new(Vec::from(s.as_bytes()));
//println!("Printing response: {}", buffer);
//}
//Ok(())
