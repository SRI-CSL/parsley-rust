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

extern crate byteorder; // 1.3.1
use byteorder::{ByteOrder, BigEndian};
/// Primitives for handling binary data.
use super::parsebuffer::{ParsleyParser, ParseBuffer, ParseResult, LocatedVal};
use std::slice;
use bit_vec::BitVec;
use bit_set::BitSet;

pub struct BinaryScanner {
    tag: Vec<u8>
}

impl BinaryScanner {
    pub fn new(tag: &[u8]) -> BinaryScanner {
        let mut t = Vec::new();
        t.extend_from_slice(tag);
        BinaryScanner { tag: t }
    }
}

impl ParsleyParser for BinaryScanner {
    type T = LocatedVal<usize>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let val = buf.scan(&self.tag)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(val, start, end))
    }
}

pub struct BinaryMatcher {
    tag: Vec<u8>
}

impl BinaryMatcher {
    pub fn new(tag: &[u8]) -> BinaryMatcher {
        let mut t = Vec::new();
        t.extend_from_slice(tag);
        BinaryMatcher { tag: t }
    }
}

impl ParsleyParser for BinaryMatcher {
    type T = LocatedVal<bool>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let val = buf.exact(&self.tag)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(val, start, end))
    }
}

/* Ideally, we would have a binary buffer that keeps a reference
   with the lifetime of the parser.  The naive approach runs into
   error E0207.

   A workaround could be to use PhantomData such as something like below.
   But this doesn't quite work, and variously crashes rustc!

   For now, just do the inefficient thing and copy the data, until
   we properly grok lifetimes and traits.

   pub struct BinaryBuffer {
   len: usize,
   phantom: PhantomData<&'a [u8]>
   }

   impl<'a> BinaryBuffer<'a> {
   pub fn new(len: usize) -> BinaryBuffer<'a> {
   BinaryBuffer { len, phantom: PhantomData }
   }
   }

   impl<'a> ParsleyParser for BinaryBuffer<'a> {
   type T<'a> = &'a [u8];

   fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
   buf.extract(self.len)
   }
   }
   */

pub struct CharParser {
    arg: char
}

impl CharParser {
    pub fn new(arg: char) -> CharParser {
        CharParser { arg }
    }
}

impl ParsleyParser for CharParser {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let bytes = buf.extract(1)?;
        let mut ret = Vec::new();
        let mut cmp = self.arg as u8; // type cast to an integer
        ret.extend_from_slice(bytes);
        let end = buf.get_cursor();
        if cmp == ret[0]
        {
            Ok(LocatedVal::new(ret, start, end))
        }
        else
        {
            panic!("Parsing failed, expected {:?} but got {:?}", cmp, ret[0]); 
            //Ok(LocatedVal::new(ret, start, end))
        }
    }
}

pub struct TokenParser<'a> {
    arg: &'a str,
    len: usize
}

impl TokenParser<'_> {
    pub fn new<'a>(arg: &'a str, len: usize) -> TokenParser {
        // TODO: verify the two lengths
        TokenParser { arg, len }
    }
}

impl ParsleyParser for TokenParser<'_> {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let bytes = buf.extract(self.len)?;
        let mut ret = Vec::new();
        let mut cmp = self.arg.to_string().into_bytes();
        ret.extend_from_slice(bytes);
        let end = buf.get_cursor();
        assert_eq!(cmp, ret, "TokenParser failed :{:?} and {:?} did not match.", cmp, ret);
        Ok(LocatedVal::new(ret, start, end))
    }
}

pub struct BitObj8 {
}

impl BitObj8 {
    pub fn new() -> BitObj8 {
        BitObj8 {}
    }
}

impl ParsleyParser for BitObj8 {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut bytes = buf.extract(1)?;
        println!("Bytes in BitVector {:?}", bytes);
        let other = BitSet::from_bytes(&[bytes[0]]);
        let mut bv = other.into_bit_vec();
        println!("Bitvector {:?}", bv);
        let end = buf.get_cursor();
        let result: Vec<u8> = Vec::new();

        Ok(LocatedVal::new(result, start, end))
    }
}

pub struct IntObj128 {
}

impl IntObj128 {
    pub fn new() -> IntObj128 {
        IntObj128 {}
    }
}

impl ParsleyParser for IntObj128 {
    type T = LocatedVal<Vec<u128>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut bytes = buf.extract(16)?;
        let mut result: Vec<u128> = Vec::new();
        // Convert and extract a 32 bit integer?
        for i in (0..16).step_by(16) {
            result.push(BigEndian::read_u128(&bytes[i..]));
        }
        let end = buf.get_cursor();

        Ok(LocatedVal::new(result, start, end))
    }
}
pub struct IntObj64 {
}

impl IntObj64 {
    pub fn new() -> IntObj64 {
        IntObj64 {}
    }
}

impl ParsleyParser for IntObj64 {
    type T = LocatedVal<Vec<u64>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut bytes = buf.extract(8)?;
        let mut result: Vec<u64> = Vec::new();
        // Convert and extract a 32 bit integer?
        for i in (0..8).step_by(8) {
            result.push(BigEndian::read_u64(&bytes[i..]));
        }
        let end = buf.get_cursor();

        Ok(LocatedVal::new(result, start, end))
    }
}
pub struct IntObj32 {
}

impl IntObj32 {
    pub fn new() -> IntObj32 {
        IntObj32 {}
    }
}

impl ParsleyParser for IntObj32 {
    type T = LocatedVal<Vec<u32>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut bytes = buf.extract(4)?;
        let mut result: Vec<u32> = Vec::new();
        // Convert and extract a 32 bit integer?
        for i in (0..4).step_by(4) {
            result.push(BigEndian::read_u32(&bytes[i..]));
        }
        let end = buf.get_cursor();

        Ok(LocatedVal::new(result, start, end))
    }
}

pub struct IntObj16 {
}

impl IntObj16 {
    pub fn new() -> IntObj16 {
        IntObj16 {}
    }
}

impl ParsleyParser for IntObj16 {
    type T = LocatedVal<Vec<u16>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut bytes = buf.extract(2)?;
        let mut result: Vec<u16> = Vec::new();
        let mut dst = [0; 1];
        // Convert and extract a 16 bit integer?
        for i in (0..2).step_by(2) {
            result.push(BigEndian::read_u16(&bytes[i..]));
        }
        //println!("{:?}", result);
        let end = buf.get_cursor();

        Ok(LocatedVal::new(result, start, end))
    }
}
pub struct IntObj8 {
}

impl IntObj8 {
    pub fn new() -> IntObj8 {
        IntObj8 {}
    }
}

impl ParsleyParser for IntObj8 {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut bytes = buf.extract(1)?;
        let mut result: Vec<u8> = Vec::new();
        // Convert and extract a 8 bit integer?
        result.push(bytes[0]);
        let end = buf.get_cursor();

        Ok(LocatedVal::new(result, start, end))
    }
}

pub struct BinaryBuffer {
    len: usize
}

impl BinaryBuffer {
    pub fn new(len: usize) -> BinaryBuffer {
        BinaryBuffer { len }
    }
}

impl ParsleyParser for BinaryBuffer {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let bytes = buf.extract(self.len)?;
        let mut ret = Vec::new();
        ret.extend_from_slice(bytes);
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ret, start, end))
    }
}

// unit tests

#[cfg(test)]
mod test_binary {
    use super::*;
    use super::super::parsebuffer::{ParseBuffer, ParsleyParser, LocatedVal, ErrorKind, make_error};

    #[test]
    fn scan() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = BinaryScanner::new("%PDF-".as_bytes());

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(0, 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("garbage %PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(8, 0, 8)));
        assert_eq!(pb.get_cursor(), 8);
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(0, 8, 8)));
        assert_eq!(pb.get_cursor(), 8);

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        let e = make_error(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn exact() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = BinaryMatcher::new("%PDF-".as_bytes());

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        let e = make_error(ErrorKind::GuardError("match"), 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(true, 0, 5)));
        assert_eq!(pb.get_cursor(), 5);

        let mut pb = ParseBuffer::new(Vec::from(" %PDF-".as_bytes()));
        let e = make_error(ErrorKind::GuardError("match"), 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn test_integers() {
        let mut s = IntObj8::new();
        let mut pb = ParseBuffer::new(Vec::from("\x05".as_bytes()));
        let mut val = vec![5];
        let mut e = Ok(LocatedVal::new(val, 0, 1));
        assert_eq!(s.parse(&mut pb), e);

        let mut s = IntObj8::new();
        let mut pb = ParseBuffer::new(Vec::from("\x05".as_bytes()));
        let mut val = vec![3];
        let mut e = Ok(LocatedVal::new(val, 0, 2));
        assert_ne!(s.parse(&mut pb), e);

        let mut s = IntObj16::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x05".as_bytes()));
        let mut val = vec![5];
        let mut e = Ok(LocatedVal::new(val, 0, 2));
        assert_eq!(s.parse(&mut pb), e);

        let mut s = IntObj16::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x05".as_bytes()));
        let mut val = vec![3];
        let mut e = Ok(LocatedVal::new(val, 0, 2));
        assert_ne!(s.parse(&mut pb), e);

        let mut s = IntObj32::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x00\x00\x05".as_bytes()));
        let mut val = vec![5];
        let mut e = Ok(LocatedVal::new(val, 0, 2));
        assert_eq!(s.parse(&mut pb), e);

        let mut s = IntObj32::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x00\x00\x05".as_bytes()));
        let mut val = vec![3];
        let mut e = Ok(LocatedVal::new(val, 0, 2));
        assert_ne!(s.parse(&mut pb), e);

        let mut s = IntObj64::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x00\x00\x00\x00\x00\x00\x05".as_bytes()));
        let mut val = vec![5];
        let mut e = Ok(LocatedVal::new(val, 0, 8));
        assert_eq!(s.parse(&mut pb), e);

        let mut s = IntObj64::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x00\x00\x00\x00\x00\x00\x05".as_bytes()));
        let mut val = vec![3];
        let mut e = Ok(LocatedVal::new(val, 0, 8));
        assert_ne!(s.parse(&mut pb), e);

        let mut s = IntObj128::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05".as_bytes()));
        let mut val = vec![5];
        let mut e = Ok(LocatedVal::new(val, 0, 16));
        assert_eq!(s.parse(&mut pb), e);

        let mut s = IntObj128::new();
        let mut pb = ParseBuffer::new(Vec::from("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05".as_bytes()));
        let mut val = vec![3];
        let mut e = Ok(LocatedVal::new(val, 0, 16));
        assert_ne!(s.parse(&mut pb), e);
    }

    #[test]
    fn char_parser() {
        let mut character = CharParser::new('\x00');
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("\x00".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = vec![0];
        let mut e = Ok(LocatedVal::new(val, 0, 1));
        assert_eq!(character.parse(&mut pb), e);
    }

    #[test]    
    #[should_panic]
    fn char_fail_parser() {
        let mut character = CharParser::new('\x00');
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("\x01".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = vec![0];
        let mut e = Ok(LocatedVal::new(val, 0, 1));
        assert_ne!(character.parse(&mut pb), e);
    }

    #[test]
    fn token_parser() {
        let mut character = TokenParser::new("\x01\x05\x06", 3);
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("\x01\x05\x06".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = vec![1, 5, 6];
        let mut e = Ok(LocatedVal::new(val, 0, 1));
        assert_eq!(character.parse(&mut pb), e);
    }

    #[test]
    #[should_panic]
    fn token_fail_parser() {
        let mut character = TokenParser::new("\x01\x04\x06", 3);
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("\x01\x05\x06".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = vec![1, 5, 6];
        let mut e = Ok(LocatedVal::new(val, 0, 1));
        assert_ne!(character.parse(&mut pb), e);
    }

    #[test]
    fn buffer() {
        let mut s = BinaryBuffer::new(3);
        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        let e = make_error(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));

        let mut s = BinaryBuffer::new(3);
        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        let v = s.parse(&mut pb).unwrap();
        let r = Vec::from("%PD".as_bytes());
        assert_eq!(*v.val(), r);
        assert_eq!(pb.get_cursor(), 3);
    }
}
