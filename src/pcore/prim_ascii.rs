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

use std::convert::TryFrom;
use super::parsebuffer::{ParsleyPrimitive, ParsleyParser, ParseBuffer,
                         ParseResult, ParseError, LocatedVal, ErrorKind};

pub struct AsciiCharPrimitive;

impl ParsleyPrimitive for AsciiCharPrimitive {
    type T = char;

    fn name() -> &'static str { "ascii-prim" }

    fn parse(buf: &[u8]) -> ParseResult<(Self::T, usize)> {
        if buf.len() < 1 {
            return Err(LocatedVal::new(ErrorKind::EndOfBuffer, 0, 0));
        }
        let c = char::try_from(buf[0]);
        // check this: we should never get the below error from
        // non-empty buffers, as all u8 should be convertible to char.
        if c.is_err() {
            let err = ErrorKind::PrimitiveError(ParseError::new("ascii-prim: invalid character"));
            return Err(LocatedVal::new(err, 0, 0));
        }
        let c = c.unwrap();
        if !c.is_ascii() {
            let err = ErrorKind::PrimitiveError(ParseError::new("ascii-prim: invalid ascii character"));
            return Err(LocatedVal::new(err, 0, 0));
        }
        Ok((c, 1))
    }
}

// Convenience wrappers around the primitive interfaces, that allow
// use with the primitive combinators.

pub struct AsciiChar {
    guard: Option<Box<dyn FnMut(&char) -> bool>>
}

impl AsciiChar {
    pub fn new() -> AsciiChar {
        AsciiChar { guard: None }
    }

    pub fn new_guarded(g: Box<dyn FnMut(&char) -> bool>) -> AsciiChar {
        AsciiChar { guard: Some(g) }
    }
}

impl ParsleyParser for AsciiChar {
    type T = LocatedVal<char>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let c = match &mut self.guard {
            None => buf.parse_prim::<AsciiCharPrimitive>()?,
            Some(b) => buf.parse_guarded::<AsciiCharPrimitive>(b)?
        };
        let end = buf.get_cursor();
        Ok(LocatedVal::new(c, start, end))
    }
}

// test the primitive parser
#[cfg(test)]
mod test_prim_ascii {
    use super::AsciiCharPrimitive;
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrimitive, ErrorKind, ParseError, make_error};

    // this raw interface would not normally be used; we would be
    // going via the ParseBuffer as in the remaining tests.
    #[test]
    fn raw() {
        let mut v = Vec::new();
        v.push(255);
        let r = <AsciiCharPrimitive as ParsleyPrimitive>::parse(&v);
        let pe = ErrorKind::PrimitiveError(ParseError::new("ascii-prim: invalid ascii character"));
        let pe = make_error(pe, 0, 0);
        assert_eq!(r, Err(pe));

        let mut w = Vec::new();
        w.extend_from_slice("A".as_bytes());
        let r = <AsciiCharPrimitive as ParsleyPrimitive>::parse(&w);
        assert_eq!(r, Ok(('A', 1)));
    }

    #[test]
    fn empty() {
        let mut pb = ParseBuffer::new(Vec::new());
        assert_eq!(pb.get_cursor(), 0);
        let e = make_error(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(pb.parse_prim::<AsciiCharPrimitive>(), Err(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn ascii() {
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("A".as_bytes());
        v.push(128);  // non-ascii
        v.push(0);    // nul; ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = pb.parse_prim::<AsciiCharPrimitive>();
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = pb.parse_prim::<AsciiCharPrimitive>();
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        let pe = make_error(ErrorKind::PrimitiveError(pe), 1, 1);
        let e = Err(pe);
        assert_eq!(r, e);
        // the cursor should not advance over the invalid char.
        assert_eq!(pb.get_cursor(), 1);

        // forcibly advance
        pb.set_cursor(2);
        let r = pb.parse_prim::<AsciiCharPrimitive>();
        assert_eq!(r, Ok('\u{0}'));
        assert_eq!(pb.get_cursor(), 3);
        let e = make_error(ErrorKind::EndOfBuffer, 3, 3);
        assert_eq!(pb.parse_prim::<AsciiCharPrimitive>(), Err(e))
    }

    #[test]
    fn guard() {
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("AB".as_bytes());
        v.push(128); // non-ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = pb.parse_guarded::<AsciiCharPrimitive>(&mut |c: &char| { *c == 'A' });
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = pb.parse_guarded::<AsciiCharPrimitive>(&mut |c: &char| { *c == 'A' });
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name());
        let e = Err(make_error(e, 1, 1));
        assert_eq!(r, e);
        // the cursor should not advance if the guard fails
        assert_eq!(pb.get_cursor(), 1);
    }
}

// test the convenience wrapper
#[cfg(test)]
mod test_ascii {
    use super::{AsciiCharPrimitive, AsciiChar};
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrimitive, ParsleyParser, ParseError,
                                    LocatedVal, ErrorKind, make_error};

    #[test]
    fn empty() {
        let mut ascii_parser = AsciiChar::new();
        let mut pb = ParseBuffer::new(Vec::new());
        assert_eq!(pb.get_cursor(), 0);
        let e = Err(make_error(ErrorKind::EndOfBuffer, 0, 0));
        assert_eq!(ascii_parser.parse(&mut pb), e);
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn ascii() {
        let mut ascii_parser = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice("A".as_bytes());
        v.push(128);  // non-ascii
        v.push(0);    // nul; ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = ascii_parser.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new('A', 0, 1)));
        assert_eq!(pb.get_cursor(), 1);

        let r = ascii_parser.parse(&mut pb);
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        let pe = make_error(ErrorKind::PrimitiveError(pe), 1, 1);
        let e = Err(pe);
        assert_eq!(r, e);
        // the cursor should not advance over the invalid char.
        assert_eq!(pb.get_cursor(), 1);

        // forcibly advance to nul
        pb.set_cursor(2);
        let r = ascii_parser.parse(&mut pb);
        // nul fails the guard test
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name());
        let e = Err(make_error(e, 2, 2));
        assert_eq!(r, e);
        // the cursor should not advance over the failed guard
        assert_eq!(pb.get_cursor(), 2);

        // forcibly advance
        pb.set_cursor(3);
        let e = Err(make_error(ErrorKind::EndOfBuffer, 3, 3));
        assert_eq!(ascii_parser.parse(&mut pb), e)
    }
}
