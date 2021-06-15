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

use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParseBufferT, ParsleyParser};
use parsley_rust::rtps_lib::rtps_packet::PacketP;
use std::env;
use std::io::Read;

fn parse(f: &str, data: &[u8]) {
    println!("Parsing packet of {} bytes in {} ..", data.len(), f);
    let v = Vec::from(data);
    let mut pb = ParseBuffer::new(v);

    let mut count = 0;
    loop {
        if pb.remaining() == 0 {
            break
        }
        let mut pp = PacketP;
        let p = match pp.parse(&mut pb) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parse error at {}: {:?}", e.start(), e.val());
                break
            },
        };
        for (i, m) in p.val().msgs().iter().enumerate() {
            println!("Packet {}, sub-msg {}: {:?}", count, i, m.kind())
        }
        println!(
            "Received RTPS packet with {} sub messages.",
            p.val().msgs().len()
        );
        count += 1
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <pcap-conv-packet.dat>*", args[0]);
        eprintln!("  specify the list of per-packet files in an RTPS conversation.");
        std::process::exit(0)
    }
    for f in &args[1 ..] {
        let mut file = match std::fs::File::open(f) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening {}: {}", &args[1], e);
                std::process::exit(1)
            },
        };
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        parse(f, &data)
    }
}
