// Copyright (c) 2019-2021 SRI International.
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

use std::env;
use std::io::Read;
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser};
use parsley_rust::rtps_lib::rtps_packet::PacketP;

fn parse(data: &[u8]) {
    println!("Parsing conversation of {} bytes ..\n", data.len());
    let v = Vec::from(data);
    let mut pb = ParseBuffer::new(v);

    let mut count = 0;
    loop {
        let mut pp = PacketP;
        let p = match pp.parse(&mut pb) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parse error at {}: {:?}", e.start(), e.val());
                break
            },
        };
        println!(
            "Received RTPS packet with {} sub messages.\n",
            p.val().msgs().len()
        );
        count += 1
    }
    println!("Parsed {} packets in conversation.", count)
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pcap-conv.dat>\n", args[0]);
        std::process::exit(0)
    }
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    parse(&data)
}
