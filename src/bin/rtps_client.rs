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

use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser};
use parsley_rust::rtps_lib::rtps_packet::PacketP;
use std::net::UdpSocket;

fn get_sock(host: &str, port: &str) -> UdpSocket {
    let addr = format!("{}:{}", host, port);
    match UdpSocket::bind(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error binding to {}: {}\n", addr, e);
            std::process::exit(1)
        },
    }
}

fn run(sock: UdpSocket) {
    let mut buf = [0; 67 * 1024]; // max UDP payload
    let (amt, from) = match sock.recv_from(&mut buf) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Socket error: {}\n", e);
            std::process::exit(1)
        },
    };
    println!("received {} bytes from {}.\n", amt, from);
    parse(&buf[.. amt])
}

fn parse(data: &[u8]) {
    let v = Vec::from(data);
    let mut pb = ParseBuffer::new(v);

    let mut pp = PacketP;
    let p = match pp.parse(&mut pb) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error at {}: {:?}", e.start(), e.val());
            return
        },
    };
    println!(
        "Received RTPS packet with {} sub messages.\n",
        p.val().msgs().len()
    );
}

pub fn main() {
    let sock = get_sock("127.0.0.1", "1234");
    run(sock)
}
