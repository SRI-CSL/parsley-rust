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

use crate::pcore::parsebuffer::{ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser};
use crate::pcore::prim_binary::{ByteVecP, Endian, UInt16P, UInt8P};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GuidPrefix {
    id: [u8; 12],
}
impl GuidPrefix {
    pub fn new(id: [u8; 12]) -> Self { Self { id } }
    pub fn id(&self) -> &[u8; 12] { &self.id }
}

pub struct GuidPrefixP;
impl ParsleyParser for GuidPrefixP {
    type T = LocatedVal<GuidPrefix>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut gp = ByteVecP::new(12);

        let start = buf.get_cursor();
        let g = gp.parse(buf)?;

        let guid_prefix: [u8; 12] = match TryFrom::try_from(g.val().as_slice()) {
            Ok(v) => v,
            Err(_) => {
                buf.set_cursor(start);
                return Err(LocatedVal::new(ErrorKind::BoundsError, start, start))
            },
        };
        let gp = GuidPrefix::new(guid_prefix);
        Ok(LocatedVal::new(gp, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VendorId {
    id: u16,
}
impl VendorId {
    pub fn new(id: u16) -> Self { Self { id } }
}

pub struct VendorIdP;
impl ParsleyParser for VendorIdP {
    type T = LocatedVal<VendorId>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = UInt16P::new(Endian::Little);
        let v = p.parse(buf)?;
        let vid = VendorId::new(*v.val());
        Ok(LocatedVal::new(vid, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ProtocolVersion {
    id: u16,
}
impl ProtocolVersion {
    pub fn new(id: u16) -> Self { Self { id } }
}

pub struct ProtocolVersionP;
impl ParsleyParser for ProtocolVersionP {
    type T = LocatedVal<ProtocolVersion>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = UInt16P::new(Endian::Little);
        let v = p.parse(buf)?;
        let vid = ProtocolVersion::new(*v.val());
        Ok(LocatedVal::new(vid, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Header {
    version:     ProtocolVersion,
    vendorid:    VendorId,
    guid_prefix: GuidPrefix,
}
impl Header {
    pub fn new(version: ProtocolVersion, vendorid: VendorId, guid_prefix: GuidPrefix) -> Self {
        Self {
            version,
            vendorid,
            guid_prefix,
        }
    }
}

pub struct HeaderP;
impl ParsleyParser for HeaderP {
    type T = LocatedVal<Header>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut pvp = ProtocolVersionP;
        let mut vip = VendorIdP;
        let mut gpp = GuidPrefixP;

        let start = buf.get_cursor();
        match buf.exact(b"RTPS") {
            Ok(_) => (),
            Err(e) => {
                let err = ErrorKind::GuardError("invalid magic".to_string());
                return Err(e.place(err))
            },
        }
        let pv = match pvp.parse(buf) {
            Ok(pv) => pv,
            Err(e) => {
                buf.set_cursor(start);
                return Err(e)
            },
        };
        let vi = match vip.parse(buf) {
            Ok(vi) => vi,
            Err(e) => {
                buf.set_cursor(start);
                return Err(e)
            },
        };
        let gp = match gpp.parse(buf) {
            Ok(gp) => gp,
            Err(e) => {
                buf.set_cursor(start);
                return Err(e)
            },
        };
        let h = Header::new(*pv.val(), *vi.val(), *gp.val());
        Ok(LocatedVal::new(h, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SubMessageHeader {
    sub_msg_id: u8,
    flags:      u8,
    length:     u16,
}
impl SubMessageHeader {
    pub fn new(sub_msg_id: u8, flags: u8, length: u16) -> Self {
        Self {
            sub_msg_id,
            flags,
            length,
        }
    }
    pub fn id(&self) -> u8 { self.sub_msg_id }
    pub fn length(&self) -> u16 { self.length }
}

pub struct SubMessageHeaderP;
impl ParsleyParser for SubMessageHeaderP {
    type T = LocatedVal<SubMessageHeader>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut uip = UInt8P;
        let mut usp = UInt16P::new(Endian::Little);

        let start = buf.get_cursor();
        let id = uip.parse(buf)?;
        let flags = match uip.parse(buf) {
            Ok(f) => f,
            Err(e) => {
                buf.set_cursor(start);
                return Err(e)
            },
        };
        let len = match usp.parse(buf) {
            Ok(l) => l,
            Err(e) => {
                buf.set_cursor(start);
                return Err(e)
            },
        };

        let sh = SubMessageHeader::new(*id.val(), *flags.val(), *len.val());
        Ok(LocatedVal::new(sh, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubMessage {
    header:  SubMessageHeader,
    payload: Vec<u8>,
}

impl SubMessage {
    pub fn new(header: SubMessageHeader, payload: Vec<u8>) -> Self { Self { header, payload } }
}

pub struct SubMessageP;
impl ParsleyParser for SubMessageP {
    type T = LocatedVal<SubMessage>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut smhp = SubMessageHeaderP;
        let start = buf.get_cursor();

        let hdr = smhp.parse(buf)?;
        let hdr = hdr.unwrap();

        let mut bvp = ByteVecP::new(hdr.length().into());
        let pld = bvp.parse(buf)?;
        let pld = pld.unwrap();

        Ok(LocatedVal::new(
            SubMessage::new(hdr, pld),
            start,
            buf.get_cursor(),
        ))
    }
}
