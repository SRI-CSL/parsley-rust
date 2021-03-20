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
use super::parsebuffer::{LocatedVal, ParseBufferT, ParseResult, ParsleyParser};

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

// unit tests

#[cfg(test)]
mod test_binary {
    use super::super::parsebuffer::{
        locate_value, ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParsleyParser,
    };
    use super::{BinaryMatcher, BinaryScanner};

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
}
