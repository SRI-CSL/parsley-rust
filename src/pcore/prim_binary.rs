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

/// Primitives for handling binary data.
use super::parsebuffer::{ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser};

pub struct BinaryScanner {
    tag: Vec<u8>,
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

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let val = buf.scan(&self.tag)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(val, start, end))
    }
}

pub struct BinaryMatcher {
    tag: Vec<u8>,
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

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let val = buf.exact(&self.tag)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(val, start, end))
    }
}

pub struct ByteVecP {
    len: usize,
}
impl ByteVecP {
    pub fn new(len: usize) -> Self { Self { len } }
}
impl ParsleyParser for ByteVecP {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let vec = buf.extract(self.len)?;
        Ok(LocatedVal::new(Vec::from(vec), start, buf.get_cursor()))
    }
}

// Binary integers

#[derive(Debug, Clone, Copy)]
pub enum Endian {
    Little,
    Big,
}

pub struct UInt8P;
impl ParsleyParser for UInt8P {
    type T = LocatedVal<u8>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        match buf.peek() {
            Some(b) => {
                buf.incr_cursor_unsafe();
                let end = buf.get_cursor();
                Ok(LocatedVal::new(b, start, end))
            },
            None => Err(LocatedVal::new(ErrorKind::EndOfBuffer, start, start)),
        }
    }
}

pub struct UInt16P {
    endian: Endian,
}
impl UInt16P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for UInt16P {
    type T = LocatedVal<u16>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = UInt8P;
        let v1 = p.parse(buf)?;
        match p.parse(buf) {
            Err(e) => {
                buf.set_cursor_unsafe(start);
                Err(e)
            },
            Ok(v2) => {
                let v: u16 = match self.endian {
                    Endian::Big => ((*v1.val() as u16) << 8) + (*v2.val() as u16),
                    Endian::Little => ((*v2.val() as u16) << 8) + (*v1.val() as u16),
                };
                Ok(LocatedVal::new(v, start, buf.get_cursor()))
            },
        }
    }
}

pub struct UInt32P {
    endian: Endian,
}
impl UInt32P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for UInt32P {
    type T = LocatedVal<u32>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = UInt16P::new(self.endian);
        let v1 = p.parse(buf)?;
        match p.parse(buf) {
            Err(e) => {
                buf.set_cursor_unsafe(start);
                Err(e)
            },
            Ok(v2) => {
                let v: u32 = match self.endian {
                    Endian::Big => ((*v1.val() as u32) << 16) + (*v2.val() as u32),
                    Endian::Little => ((*v2.val() as u32) << 16) + (*v1.val() as u32),
                };
                Ok(LocatedVal::new(v, start, buf.get_cursor()))
            },
        }
    }
}

pub struct F32P {
    endian: Endian,
}
impl F32P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for F32P {
    type T = LocatedVal<f32>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = UInt32P::new(self.endian);
        match p.parse(buf) {
            Err(e) => {
                buf.set_cursor_unsafe(start);
                Err(e)
            },
            Ok(v2) => {
                let v: f32 = match self.endian {
                    Endian::Big => {
                        let bytes = v2.val().to_be_bytes();
                        f32::from_be_bytes(bytes)
                    },
                    Endian::Little => {
                        let bytes = v2.val().to_be_bytes();
                        f32::from_be_bytes(bytes)
                    },
                };
                Ok(LocatedVal::new(v, start, buf.get_cursor()))
            },
        }
    }
}

pub struct UInt64P {
    endian: Endian,
}
impl UInt64P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for UInt64P {
    type T = LocatedVal<u64>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = UInt32P::new(self.endian);
        let v1 = p.parse(buf)?;
        match p.parse(buf) {
            Err(e) => {
                buf.set_cursor_unsafe(start);
                Err(e)
            },
            Ok(v2) => {
                let v: u64 = match self.endian {
                    Endian::Big => ((*v1.val() as u64) << 32) + (*v2.val() as u64),
                    Endian::Little => ((*v2.val() as u64) << 32) + (*v1.val() as u64),
                };
                Ok(LocatedVal::new(v, start, buf.get_cursor()))
            },
        }
    }
}

pub struct Int8P;
impl ParsleyParser for Int8P {
    type T = LocatedVal<i8>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut p = UInt8P;
        let v = p.parse(buf)?;
        Ok(v.place(*v.val() as i8))
    }
}

pub struct Int16P {
    endian: Endian,
}
impl Int16P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for Int16P {
    type T = LocatedVal<i16>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut p = UInt16P::new(self.endian);
        let v = p.parse(buf)?;
        Ok(v.place(*v.val() as i16))
    }
}

pub struct Int32P {
    endian: Endian,
}
impl Int32P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for Int32P {
    type T = LocatedVal<i32>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut p = UInt32P::new(self.endian);
        let v = p.parse(buf)?;
        Ok(v.place(*v.val() as i32))
    }
}

pub struct Int64P {
    endian: Endian,
}
impl Int64P {
    pub fn new(endian: Endian) -> Self { Self { endian } }
}
impl ParsleyParser for Int64P {
    type T = LocatedVal<i64>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut p = UInt64P::new(self.endian);
        let v = p.parse(buf)?;
        Ok(v.place(*v.val() as i64))
    }
}

// unit tests

#[cfg(test)]
mod test_binary {
    use super::super::parsebuffer::{
        locate_value, ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParsleyParser,
    };
    use super::{BinaryMatcher, BinaryScanner};
    use super::{Endian, Int16P, Int32P, Int64P, Int8P, UInt16P, UInt32P, UInt64P, UInt8P, F32P};

    #[test]
    fn scan() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = BinaryScanner::new(b"%PDF-");

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(0, 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("garbage %PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(8, 0, 8)));
        assert_eq!(pb.get_cursor(), 8);
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(0, 8, 8)));
        assert_eq!(pb.get_cursor(), 8);

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        let e = locate_value(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn exact() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = BinaryMatcher::new(b"%PDF-");

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        let e = locate_value(ErrorKind::GuardError("match".to_string()), 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(LocatedVal::new(true, 0, 5)));
        assert_eq!(pb.get_cursor(), 5);

        let mut pb = ParseBuffer::new(Vec::from(" %PDF-".as_bytes()));
        let e = locate_value(ErrorKind::GuardError("match".to_string()), 0, 0);
        assert_eq!(s.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn uint8() {
        let mut p = UInt8P;
        let mut pb = ParseBuffer::new(vec![0, 127, 128, 255]);

        let v = LocatedVal::new(0, 0, 1);
        assert_eq!(p.parse(&mut pb), Ok(v));

        let v = LocatedVal::new(127, 1, 2);
        assert_eq!(p.parse(&mut pb), Ok(v));

        let v = LocatedVal::new(128, 2, 3);
        assert_eq!(p.parse(&mut pb), Ok(v));

        let v = LocatedVal::new(255, 3, 4);
        assert_eq!(p.parse(&mut pb), Ok(v));
    }

    #[test]
    fn int8() {
        let mut p = Int8P;
        let mut pb = ParseBuffer::new(vec![0, 127, 128, 255]);

        let v = LocatedVal::new(0, 0, 1);
        assert_eq!(p.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(127, 1, 2);
        assert_eq!(p.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(-128, 2, 3);
        assert_eq!(p.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(-1, 3, 4);
        assert_eq!(p.parse(&mut pb), Ok(v));
    }

    #[test]
    fn uint16() {
        let mut bp = UInt16P::new(Endian::Big);
        let mut lp = UInt16P::new(Endian::Little);

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255]);
        let v = LocatedVal::new(127 * 256, 0, 2);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128 * 256 + 255, 2, 4);
        assert_eq!(bp.parse(&mut pb), Ok(v));

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255]);
        let v = LocatedVal::new(127, 0, 2);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128 + 255 * 256, 2, 4);
        assert_eq!(lp.parse(&mut pb), Ok(v));
    }

    #[test]
    fn int16() {
        let mut bp = Int16P::new(Endian::Big);
        let mut lp = Int16P::new(Endian::Little);

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255]);
        let v = LocatedVal::new(127 * 256, 0, 2);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new((128 * 256 + 255) as i16, 2, 4);
        assert_eq!(bp.parse(&mut pb), Ok(v));

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255]);
        let v = LocatedVal::new(127, 0, 2);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new((128 + 255 * 256) as i16, 2, 4);
        assert_eq!(lp.parse(&mut pb), Ok(v));
    }

    #[test]
    fn uint32() {
        let mut bp = UInt32P::new(Endian::Big);
        let mut lp = UInt32P::new(Endian::Little);

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255, 128, 0, 0, 0, 255, 255, 255, 255]);
        let v = LocatedVal::new((127 << 24) + (128 << 8) + 255, 0, 4);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128 << 24, 5, 8);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(u32::MAX, 9, 12);
        assert_eq!(bp.parse(&mut pb), Ok(v));

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255, 128, 0, 0, 0, 255, 255, 255, 255]);
        let v = LocatedVal::new((255 << 24) + (128 << 16) + 127, 0, 4);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128, 5, 8);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(u32::MAX, 9, 12);
        assert_eq!(lp.parse(&mut pb), Ok(v));
    }

    #[test]
    fn int32() {
        let mut bp = Int32P::new(Endian::Big);
        let mut lp = Int32P::new(Endian::Little);

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255, 128, 0, 0, 0, 255, 255, 255, 255]);
        let v = LocatedVal::new((127 << 24) + (128 << 8) + 255, 0, 4);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new((128 << 24) as i32, 5, 8);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(-1, 9, 12);
        assert_eq!(bp.parse(&mut pb), Ok(v));

        let mut pb = ParseBuffer::new(vec![127, 0, 128, 255, 128, 0, 0, 0, 255, 255, 255, 255]);
        let v = LocatedVal::new((255 << 24) + (128 << 16) + 127, 0, 4);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128, 5, 8);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(-1, 9, 12);
        assert_eq!(lp.parse(&mut pb), Ok(v));
    }

    #[test]
    fn uint64() {
        let mut bp = UInt64P::new(Endian::Big);
        let mut lp = UInt64P::new(Endian::Little);

        let mut pb = ParseBuffer::new(vec![
            127, 0, 0, 0, 0, 0, 0, 255, 128, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            255, 255,
        ]);
        let v = ((127 as u64) << 56) + 255;
        let v = LocatedVal::new(v, 0, 8);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128 << 56, 9, 16);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(u64::MAX, 17, 24);
        assert_eq!(bp.parse(&mut pb), Ok(v));

        let mut pb = ParseBuffer::new(vec![
            127, 0, 0, 0, 0, 0, 0, 255, 128, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            255, 255,
        ]);
        let v = 127 + ((255 as u64) << 56);
        let v = LocatedVal::new(v, 0, 8);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128, 9, 16);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(u64::MAX, 17, 24);
        assert_eq!(lp.parse(&mut pb), Ok(v));
    }

    #[test]
    fn int64() {
        let mut bp = Int64P::new(Endian::Big);
        let mut lp = Int64P::new(Endian::Little);

        let mut pb = ParseBuffer::new(vec![
            127, 0, 0, 0, 0, 0, 0, 255, 128, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            255, 255,
        ]);
        let v = ((127 as i64) << 56) + 255;
        let v = LocatedVal::new(v, 0, 8);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(((128 as u64) << 56) as i64, 9, 16);
        assert_eq!(bp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(-1, 17, 24);
        assert_eq!(bp.parse(&mut pb), Ok(v));

        let mut pb = ParseBuffer::new(vec![
            127, 0, 0, 0, 0, 0, 0, 255, 128, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            255, 255,
        ]);
        let v = 127 + ((255 as i64) << 56);
        let v = LocatedVal::new(v, 0, 8);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(128 as i64, 9, 16);
        assert_eq!(lp.parse(&mut pb), Ok(v));
        let v = LocatedVal::new(-1, 17, 24);
        assert_eq!(lp.parse(&mut pb), Ok(v));
    }
    #[test]
    fn f32() {
        let pi = 3.1456_f32;
        let mut bp = F32P::new(Endian::Big);
        let mut pb = ParseBuffer::new(vec![64, 73, 81, 131]);

        let bytes = bp.parse(&mut pb);
        assert_eq!(pi, bytes.unwrap().unwrap());

        let pi1 = -0.01_f32;
        pb = ParseBuffer::new(vec![188, 35, 215, 10]);

        let bytes1 = bp.parse(&mut pb);
        assert_eq!(pi1, bytes1.unwrap().unwrap());
    }
}
