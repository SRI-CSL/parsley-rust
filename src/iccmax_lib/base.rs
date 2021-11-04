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

use crate::pcore::parsebuffer::{LocatedVal, ParseBufferT, ParseResult, ParsleyParser};
use crate::pcore::prim_binary::{BinaryMatcher, Endian, UInt16P, UInt32P, UInt8P};
use crate::pcore::prim_combinators::Star;

pub type IccError = String;
pub type IccResult<T> = std::result::Result<T, IccError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PositionNumber {
    position: u32,
    size:     u32,
}
impl PositionNumber {
    pub fn new(position: u32, size: u32) -> Self { Self { position, size } }
    pub fn position(self) -> u32 { self.position }
    pub fn size(self) -> u32 { self.size }
}
pub struct PositionNumberP;
impl ParsleyParser for PositionNumberP {
    type T = LocatedVal<PositionNumber>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        let mut int32 = UInt32P::new(Endian::Big);

        let position = int32.parse(buf)?;

        let size = int32.parse(buf)?;

        let g = PositionNumber::new(*position.val(), *size.val());
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MPetElement {
    signature:          bool,
    input_channels:     u16,
    output_channels:    u16,
    number_of_elements: u32,
    pos_table:          Vec<PositionNumber>,
    data:               Vec<u8>,
}
impl MPetElement {
    pub fn new(
        signature: bool, input_channels: u16, output_channels: u16, number_of_elements: u32,
        pos_table: Vec<PositionNumber>, data: Vec<u8>,
    ) -> Self {
        Self {
            signature,
            input_channels,
            output_channels,
            number_of_elements,
            pos_table,
            data,
        }
    }
    pub fn signature(self) -> bool { self.signature }
    pub fn data(self) -> Vec<u8> { self.data }
    pub fn number_of_elements(self) -> u32 { self.number_of_elements }
    pub fn pos_table(self) -> Vec<PositionNumber> { self.pos_table }
    pub fn input_channels(self) -> u16 { self.input_channels }
    pub fn output_channels(self) -> u16 { self.output_channels }
}
pub struct MPetElementP;
impl ParsleyParser for MPetElementP {
    type T = LocatedVal<MPetElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut g1 = BinaryMatcher::new(b"mpet");

        let start = buf.get_cursor();
        let signature = g1.parse(buf)?;
        let signature = *signature.val();

        let mut uint32_parser = UInt32P::new(Endian::Big);
        let reserved = uint32_parser.parse(buf)?;

        // This field must be 0
        assert_eq!(reserved.unwrap(), 0);

        let mut g2 = UInt16P::new(Endian::Big);
        let input_channels = g2.parse(buf)?;
        let input_channels = *input_channels.val();

        let output_channels = g2.parse(buf)?;
        let output_channels = *output_channels.val();

        let number_of_elements = uint32_parser.parse(buf)?;
        let number_of_elements = *number_of_elements.val();

        let mut pos_table: Vec<PositionNumber> = Vec::new();
        for _ in 0 .. number_of_elements {
            let mut parser = PositionNumberP;
            let pos = parser.parse(buf)?;
            pos_table.push(*pos.val());
        }

        let mut int8 = UInt8P;
        let mut star_int = Star::new(&mut int8);

        let data = star_int.parse(buf)?;
        let mut data_list = Vec::<u8>::new();
        for datum in data.val() {
            data_list.push(*datum.val());
        }

        let g = MPetElement::new(
            signature,
            input_channels,
            output_channels,
            number_of_elements,
            pos_table,
            data_list,
        );
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TaggedElement {
    signature: u32,
    offset:    u32,
    size:      u32,
}
impl TaggedElement {
    pub fn new(signature: u32, offset: u32, size: u32) -> Self {
        Self {
            signature,
            offset,
            size,
        }
    }
    pub fn signature(self) -> u32 { self.signature }
    pub fn size(self) -> u32 { self.size }
    pub fn offset(self) -> u32 { self.offset }
}
pub struct TaggedElementP;
impl ParsleyParser for TaggedElementP {
    type T = LocatedVal<TaggedElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut gp = UInt32P::new(Endian::Big);

        let start = buf.get_cursor();
        let signature = gp.parse(buf)?;
        let offset = gp.parse(buf)?;
        let size = gp.parse(buf)?;

        let g = TaggedElement::new(*signature.val(), *offset.val(), *size.val());
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TagTable {
    entries: Vec<TaggedElement>,
}
impl TagTable {
    pub fn new(entries: Vec<TaggedElement>) -> Self { Self { entries } }
}

pub struct TagTableP;
impl ParsleyParser for TagTableP {
    type T = LocatedVal<TagTable>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        // Consume the next 4 bytes to get the count: number of Tags
        let mut count_p = UInt32P::new(Endian::Big);
        let count = count_p.parse(buf)?;
        let count = count.val();
        // We could create a view here to restrict the rest of the
        // table, which may be the idiomatic Parsley thing to do.
        let mut entries = Vec::new();
        for _ in 0 .. *count {
            let mut tep = TaggedElementP;
            let te = tep.parse(buf)?;
            entries.push(*te.val());
        }

        let tt = TagTable::new(entries);
        Ok(LocatedVal::new(tt, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Header;

pub struct HeaderP;
impl ParsleyParser for HeaderP {
    type T = LocatedVal<Header>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut parser = UInt8P;
        let start = buf.get_cursor();
        let mut counter = 0;
        while counter < 128 {
            let _res = parser.parse(buf)?;
            counter = counter + 1;
        }
        Ok(LocatedVal::new(Header, start, buf.get_cursor()))
    }
}

#[cfg(test)]
mod test_iccmax_base {}
