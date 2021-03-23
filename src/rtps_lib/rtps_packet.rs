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

use super::rtps_prim::{Header, HeaderP, SubMessage, SubMessageP};
use crate::pcore::parsebuffer::{ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser};

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    hdr:  Header,
    msgs: Vec<SubMessage>,
}
impl Packet {
    pub fn new(hdr: Header, msgs: Vec<SubMessage>) -> Self { Self { hdr, msgs } }
    pub fn hdr(&self) -> &Header { &self.hdr }
    pub fn msgs(&self) -> &[SubMessage] { &self.msgs }
}

pub struct PacketP;
impl ParsleyParser for PacketP {
    type T = LocatedVal<Packet>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut hp = HeaderP;
        let start = buf.get_cursor();

        let hdr = hp.parse(buf)?;
        let mut msgs = Vec::new();
        let mut err: Option<(ErrorKind, usize, usize)> = None;
        loop {
            if buf.remaining() == 0 { break };

            let mut smp = SubMessageP;
            match smp.parse(buf) {
                Ok(sm) => msgs.push(sm.val().clone()),
                Err(e) => {
                    err = Some((e.val().clone(), e.start(), e.end()));
                    break
                },
            }
        }
        if let Some((err, s, e)) = err {
            return Err(LocatedVal::new(err, s, e))
        }
        let p = Packet::new(*hdr.val(), msgs);
        Ok(LocatedVal::new(p, start, buf.get_cursor()))
    }
}
