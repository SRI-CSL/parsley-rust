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

// Basic primitive (non-compound or non-recursive) PDF objects.

use std::collections::HashSet;
use std::convert::TryFrom;

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser,
};

// There are two whitespace parsers.  This first one does not allow
// EOL as whitespace.
pub struct WhitespaceNoEOL {
    empty_ok: bool,
}

impl WhitespaceNoEOL {
    pub fn new(empty_ok: bool) -> WhitespaceNoEOL { WhitespaceNoEOL { empty_ok } }
}

impl ParsleyParser for WhitespaceNoEOL {
    type T = LocatedVal<()>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let ws = buf.parse_allowed_bytes(b" \0\t\r\x0c")?;
        if ws.len() == 0 && !self.empty_ok {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not at whitespace-noeol".to_string());
            return Err(LocatedVal::new(err, start, end))
        };
        // If the last character is '\r' (13), check if the next one
        // is '\n' (10).  If so, rewind by one character.
        if (ws.last() == Some(&13)) & (buf.peek() == Some(10)) {
            buf.decr_cursor();
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new((), start, end))
    }
}

// Comments

pub struct Comment;

impl ParsleyParser for Comment {
    type T = LocatedVal<Vec<u8>>;

    // The buffer should be positioned at the '%'; it consumes upto
    // and including end-of-line or upto end-of-buffer.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        if !(buf.peek() == Some(37)) {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not at comment".to_string());
            return Err(LocatedVal::new(err, start, end))
        }
        buf.incr_cursor();
        let c = buf.parse_bytes_until(b"\n")?;
        if buf.peek() == Some(10) {
            buf.incr_cursor();
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(c, start, end))
    }
}

// This is the second whitespace parser, that allows EOL.  Since
// comments can appear anywhere outside of strings, they are
// essentially whitespace, and are consumed by this parser.
pub struct WhitespaceEOL {
    empty_ok: bool,
}

impl WhitespaceEOL {
    pub fn new(empty_ok: bool) -> WhitespaceEOL { WhitespaceEOL { empty_ok } }
}

impl ParsleyParser for WhitespaceEOL {
    type T = LocatedVal<()>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        // loop to consume comments
        let mut is_empty = true;
        loop {
            let v = buf.parse_allowed_bytes(b" \0\t\r\n\x0c")?;
            if v.len() > 0 {
                is_empty = false
            }
            // Check if we are at a comment.
            if let Some(37) = buf.peek() {
                // '%'
                let mut c = Comment;
                c.parse(buf)?;
                is_empty = false;
                continue
            }
            break
        }

        if is_empty && !self.empty_ok {
            // we did not consume anything
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not at whitespace-eol".to_string());
            return Err(LocatedVal::new(err, start, end))
        };
        let end = buf.get_cursor();
        Ok(LocatedVal::new((), start, end))
    }
}

// Booleans are almost keywords, except that they have a semantic
// value.
pub struct Boolean;

impl ParsleyParser for Boolean {
    type T = LocatedVal<bool>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut b = buf.exact(b"true");
        if let Err(_) = b {
            b = buf.exact(b"false");
            if let Err(_) = b {
                let end = buf.get_cursor();
                let err = ErrorKind::GuardError("not at boolean".to_string());
                Err(LocatedVal::new(err, start, end))
            } else {
                let end = buf.get_cursor();
                Ok(LocatedVal::new(false, start, end))
            }
        } else {
            let end = buf.get_cursor();
            Ok(LocatedVal::new(true, start, end))
        }
    }
}

// null is an explicit PDF object.
pub struct Null;

impl ParsleyParser for Null {
    type T = LocatedVal<()>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let null = buf.exact(b"null");
        if let Err(_) = null {
            let end = buf.get_cursor();
            Err(locate_value(
                ErrorKind::GuardError("not at null".to_string()),
                start,
                end,
            ))
        } else {
            let end = buf.get_cursor();
            Ok(LocatedVal::new((), start, end))
        }
    }
}

// Integer objects.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerT(i64);

impl IntegerT {
    pub fn new(i: i64) -> IntegerT { IntegerT(i) }
    pub fn int_val(&self) -> i64 { self.0 }
    pub fn is_usize(&self) -> bool {
        let u = <usize as TryFrom<i64>>::try_from(self.0);
        if let Err(_) = u {
            false
        } else {
            true
        }
    }
    pub fn usize_val(&self) -> usize {
        let u = <usize as TryFrom<i64>>::try_from(self.0);
        u.unwrap()
    }
    pub fn is_zero(&self) -> bool { self.0 == 0 }
    pub fn is_positive(&self) -> bool { self.0 > 0 }
}

pub struct IntegerP;

impl ParsleyParser for IntegerP {
    type T = LocatedVal<IntegerT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let minus = if buf.peek() == Some(45) {
            // '-'
            buf.incr_cursor();
            true
        } else if buf.peek() == Some(43) {
            // '+'
            buf.incr_cursor();
            false
        } else {
            false
        };
        let num_str = buf.parse_allowed_bytes(b"0123456789")?;
        if (num_str.len() == 0) && (buf.peek() != Some(46)) {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not at number".to_string());
            buf.set_cursor(start);
            return Err(locate_value(err, start, end))
        }
        let mut num: i64 = 0;
        for c in num_str.iter() {
            let tmp = i64::checked_mul(num, 10);
            if let None = tmp {
                let end = buf.get_cursor();
                let err = ErrorKind::GuardError("numerical overflow".to_string());
                buf.set_cursor(start);
                return Err(locate_value(err, start, end))
            }
            num = tmp.unwrap();
            let tmp = i64::checked_add(num, i64::from(c - 48));
            if let None = tmp {
                let end = buf.get_cursor();
                let err = ErrorKind::GuardError("numerical overflow".to_string());
                buf.set_cursor(start);
                return Err(locate_value(err, start, end))
            }
            num = tmp.unwrap();
        }
        if minus {
            num *= -1;
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(IntegerT(num), start, end))
    }
}

// Real objects.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealT(i128, i128); // rational number representation: (numerator, denominator)

impl RealT {
    pub fn new(n: i128, d: i128) -> RealT { RealT(n, d) }

    pub fn is_positive(&self) -> bool { self.0 >= 0 }
    pub fn is_zero(&self) -> bool { self.0 == 0 }

    // This predicate needs to return true when the value is
    // representable by IntegerT.
    pub fn is_integer(&self) -> bool {
        // for now, just ensure denominator is 1, and numerator is
        // representable.  this should handle almost all cases.
        let conv = <i64 as TryFrom<i128>>::try_from(self.0);
        if conv.is_ok() {
            self.1 == 1
        } else {
            false
        }
    }

    pub fn numerator(&self) -> i64 {
        let conv = <i64 as TryFrom<i128>>::try_from(self.0);
        conv.unwrap()
    }
}

pub struct RealP;

impl ParsleyParser for RealP {
    type T = LocatedVal<RealT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let minus = if buf.peek() == Some(45) {
            // '-'
            buf.incr_cursor();
            true
        } else if buf.peek() == Some(43) {
            // '+'
            buf.incr_cursor();
            false
        } else {
            false
        };
        let num_str = buf.parse_allowed_bytes(b"0123456789")?;
        if (num_str.len() == 0) && (buf.peek() != Some(46)) {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not at number".to_string());
            buf.set_cursor(start);
            return Err(locate_value(err, start, end))
        }
        let mut num: i128 = 0;
        for c in num_str.iter() {
            let tmp = i128::checked_mul(num, 10);
            if let None = tmp {
                let end = buf.get_cursor();
                let err = ErrorKind::GuardError("numerical overflow".to_string());
                buf.set_cursor(start);
                return Err(locate_value(err, start, end))
            }
            num = tmp.unwrap();
            let tmp = i128::checked_add(num, i128::from(c - 48));
            if let None = tmp {
                let end = buf.get_cursor();
                let err = ErrorKind::GuardError("numerical overflow".to_string());
                buf.set_cursor(start);
                return Err(locate_value(err, start, end))
            }
            num = tmp.unwrap();
        }
        if buf.peek() == Some(46) {
            // '.'
            let mut den: i128 = 1;
            buf.incr_cursor();
            let s = buf.parse_allowed_bytes(b"0123456789");
            if let Ok(den_str) = s {
                for c in den_str.iter() {
                    let tmp = i128::checked_mul(num, 10);
                    if let None = tmp {
                        let end = buf.get_cursor();
                        let err = ErrorKind::GuardError("numerical overflow".to_string());
                        buf.set_cursor(start);
                        return Err(locate_value(err, start, end))
                    }
                    num = tmp.unwrap();
                    let tmp = i128::checked_add(num, i128::from(c - 48));
                    if let None = tmp {
                        let end = buf.get_cursor();
                        let err = ErrorKind::GuardError("numerical overflow".to_string());
                        buf.set_cursor(start);
                        return Err(locate_value(err, start, end))
                    }
                    num = tmp.unwrap();
                    let tmp = i128::checked_mul(den, 10);
                    if let None = tmp {
                        let end = buf.get_cursor();
                        let err = ErrorKind::GuardError("numerical overflow".to_string());
                        buf.set_cursor(start);
                        return Err(locate_value(err, start, end))
                    }
                    den = tmp.unwrap();
                }
            }
            if minus {
                num *= -1;
            }
            let end = buf.get_cursor();
            Ok(LocatedVal::new(RealT(num, den), start, end))
        } else {
            if minus {
                num *= -1;
            }
            let end = buf.get_cursor();
            Ok(LocatedVal::new(RealT(num, 1), start, end))
        }
    }
}

// Representation does not include the demarcating brackets.
pub struct HexString;

// assumes input is hex
fn int_of_hex(b: u8) -> u8 {
    assert!(b.is_ascii_hexdigit());
    if b'0' <= b && b <= b'9' {
        b - b'0'
    } else if b'a' <= b && b <= b'f' {
        b - b'a' + 10
    } else {
        b - b'A' + 10
    }
}

impl ParsleyParser for HexString {
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        if buf.peek() != Some(60) {
            let err = ErrorKind::GuardError("not at hex string".to_string());
            return Err(LocatedVal::new(err, start, start))
        };
        buf.incr_cursor();
        let bytes = buf.parse_allowed_bytes(b"0123456789abcdefABCDEF \n\r\t\0\x0c")?;
        if buf.peek() != Some(62) {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not at valid hex string".to_string());
            buf.set_cursor(start);
            return Err(locate_value(err, start, end))
        }
        buf.incr_cursor();

        let mut hx = Vec::new();
        let mut ws = HashSet::new();
        for c in [b' ', b'\r', b'\n', b'\t', b'\0', b'\x0c'].iter() {
            ws.insert(c);
        }
        for b in bytes.iter() {
            // skip over whitespace
            if !ws.contains(b) {
                hx.push(*b)
            }
        }
        if hx.len() % 2 != 0 {
            hx.push(b'0');
        }
        // Convert hex pairs to bytes
        let mut v = Vec::new();
        for i in 0 .. hx.len() / 2 {
            let b = 16 * int_of_hex(hx[2 * i]) + int_of_hex(hx[2 * i + 1]);
            v.push(b)
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(v, start, end))
    }
}

// Raw: does not perform any backslash processing or unescaping (other
// than properly accounting for escaped parentheses), normalization or
// unicode validation.  The representation does not include the
// demarcating parentheses.
pub struct RawLiteralString;

impl ParsleyParser for RawLiteralString {
    // since the literal could contain arbitrary bytes, the raw
    // version is represented as a byte vector.
    type T = LocatedVal<Vec<u8>>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        if buf.peek() != Some(40) {
            // '('
            let err = ErrorKind::GuardError("not at literal string".to_string());
            return Err(LocatedVal::new(err, start, start))
        };

        // match the backslashes and parens we see
        let mut last_slash: Option<usize> = None;
        let mut depth = 1;

        buf.incr_cursor();
        let mut v = Vec::new();
        loop {
            let bytes = buf.parse_bytes_until(b"()\\")?;
            match buf.peek() {
                Some(40) => {
                    // '('
                    let curpos = buf.get_cursor();
                    buf.incr_cursor();
                    v.extend_from_slice(&bytes);
                    v.extend_from_slice(b"(");

                    match last_slash {
                        Some(pos) if pos + 1 == curpos => {
                            // this is an escaped paren
                        },
                        _ => {
                            // this is not an escaped paren
                            depth += 1;
                            // reset the slash state
                            last_slash = None
                        },
                    }
                },
                Some(41) => {
                    // ')'
                    let curpos = buf.get_cursor();
                    buf.incr_cursor();
                    v.extend_from_slice(&bytes);

                    // append the ')' only if it does not terminate the string.
                    match last_slash {
                        Some(pos) if pos + 1 == curpos => {
                            // this is an escaped paren
                        },
                        _ => {
                            depth -= 1;
                            if depth == 0 {
                                // this terminates the string, so break the loop
                                break
                            } else {
                                // reset the slash state as optimization
                                last_slash = None
                            }
                        },
                    };
                    v.extend_from_slice(b")");
                },
                Some(92) => {
                    // '\'
                    let curpos = buf.get_cursor();
                    buf.incr_cursor();
                    v.extend_from_slice(&bytes);
                    v.extend_from_slice(b"\\");

                    // Toggle the backslash state.
                    match last_slash {
                        Some(pos) => {
                            if pos + 1 == curpos {
                                // this is a \\, so reset the state.
                                last_slash = None
                            } else {
                                // we're beginning a new escape.
                                last_slash = Some(curpos)
                            }
                        },
                        None =>
                        // we're beginning a new escape.
                        {
                            last_slash = Some(curpos)
                        },
                    }
                },
                Some(_) => {
                    // can never happen
                    panic!("unexpected lit string");
                },
                None => {
                    let end = buf.get_cursor();
                    buf.set_cursor(start);
                    return Err(locate_value(ErrorKind::EndOfBuffer, start, end))
                },
            }
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(v, start, end))
    }
}

// Raw names: does not perform UTF decoding, and the representation
// does not include the leading '/'.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct NameT {
    raw_bytes: Vec<u8>,
}

impl NameT {
    pub fn new(raw_bytes: Vec<u8>) -> NameT { NameT { raw_bytes } }
    pub fn val(&self) -> &[u8] { &self.raw_bytes }
    pub fn normalize(&self) -> Vec<u8> {
        // TODO: normalize according to PDF spec.
        self.raw_bytes.clone()
    }
    // TODO: allow dereferencing to Vec
    pub fn len(&self) -> usize { self.raw_bytes.len() }
    pub fn as_string(&self) -> String {
        match std::str::from_utf8(&self.raw_bytes) {
            Ok(v) => v.to_string(),
            Err(e) => format!("(cannot convert name: {})", e),
        }
    }
}

pub struct NameP;

impl ParsleyParser for NameP {
    type T = LocatedVal<NameT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        if buf.peek() != Some(47) {
            // '/'
            let err = ErrorKind::GuardError("not at name object".to_string());
            return Err(LocatedVal::new(err, start, start))
        }
        buf.incr_cursor();

        // terminated by whitespace or delimiter characters.  empty
        // names are considered valid.
        let v = buf.parse_bytes_until(b" \0\t\r\n\x0c()<>[]{}/%")?;
        let end = buf.get_cursor();

        // Normalize hex-codes if length permits.
        let ret = if v.len() < 3 {
            v
        } else {
            let mut r = Vec::new();
            let mut iter = v.windows(3);
            let mut w = iter.next();

            while w.is_some() {
                fn from_hex(b: u8) -> u8 {
                    if b'0' <= b && b <= b'9' {
                        b - b'0'
                    } else {
                        b - b'a' + 10
                    }
                }
                let triple = w.unwrap();
                if triple[0] == 35  // '#'
                        && triple[1].is_ascii_hexdigit() && triple[2].is_ascii_hexdigit()
                {
                    let h = from_hex(triple[1].to_ascii_lowercase());
                    let l = from_hex(triple[2].to_ascii_lowercase());
                    let ch = 16 * h + l;
                    if ch == 0 {
                        let err = ErrorKind::GuardError("null char in name".to_string());
                        buf.set_cursor(start);
                        return Err(locate_value(err, start, end))
                    }
                    r.push(ch);
                    // adjust iterator to skip the next two windows if present.
                    // if not present, properly handle any trailing bytes.
                    let x = iter.next();
                    if x.is_none() {
                        break
                    }
                    let y = iter.next();
                    if y.is_none() {
                        let x = x.unwrap();
                        r.push(x[2]);
                        break
                    }
                    w = iter.next();
                    if w.is_none() {
                        let y = y.unwrap();
                        r.push(y[1]);
                        r.push(y[2]);
                        break
                    }
                } else {
                    r.push(triple[0]);
                    w = iter.next();
                    if w.is_none() {
                        // push trailing bytes
                        r.push(triple[1]);
                        r.push(triple[2]);
                    }
                }
            }
            r
        };
        let name = NameT::new(ret);
        Ok(LocatedVal::new(name, start, end))
    }
}

// Stream object content.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StreamContentT {
    start:   usize, // locations that exclude the 'stream'/'endstream' tokens.
    size:    usize,
    content: Vec<u8>, // this should be processed using a view; deprecated.
}

impl StreamContentT {
    pub fn new(start: usize, size: usize, content: Vec<u8>) -> StreamContentT {
        Self {
            start,
            size,
            content,
        }
    }
    pub fn start(&self) -> usize { self.start }
    pub fn size(&self) -> usize { self.size }
}

pub struct StreamContentP;

impl ParsleyParser for StreamContentP {
    type T = LocatedVal<StreamContentT>;

    // This assumes that the whitespace before the 'stream' keyword
    // has been consumed.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let is_stream = buf.exact(b"stream");
        if let Err(_) = is_stream {
            let err = ErrorKind::GuardError("not at stream content".to_string());
            return Err(locate_value(err, start, start))
        }
        if buf.peek() == Some(13) {
            // '\r'
            buf.incr_cursor();
        }
        if buf.peek() == Some(10) {
            // '\n'
            buf.incr_cursor();
        } else {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("not a valid stream marker".to_string());
            buf.set_cursor(start);
            return Err(locate_value(err, start, end))
        }
        let stream_start_cursor = buf.get_cursor();

        let len = buf.scan(b"endstream");
        if let Err(e) = len {
            buf.set_cursor(start);
            return Err(e)
        }
        let stream_end_cursor = buf.get_cursor();
        let mut stream_len = stream_end_cursor - stream_start_cursor;

        // Extract the stream content
        buf.set_cursor(stream_start_cursor);
        let content_res = buf.extract(stream_end_cursor - stream_start_cursor);
        if let Err(e) = content_res {
            buf.set_cursor(start);
            return Err(e)
        }
        let mut v = Vec::from(content_res.unwrap());
        // Remove the trailing EOL.
        //
        // FIXME: If we find a trailing '\r\n', it is not clear from
        // the spec whether the '\r' is part of the data and EOL is
        // '\n'.  For now, just remove a trailing '\n'.
        match v.pop() {
            None => (),
            Some(10) => stream_len -= 1,
            Some(c) => v.push(c),
        }

        // Go back to the end of the content
        buf.set_cursor(stream_end_cursor);
        if let Err(_) = buf.exact(b"endstream") {
            let end = buf.get_cursor();
            let err = ErrorKind::GuardError("invalid endstream".to_string());
            buf.set_cursor(start);
            return Err(locate_value(err, start, end))
        }
        let stream = StreamContentT::new(stream_start_cursor, stream_len, v);
        let end = buf.get_cursor();
        Ok(LocatedVal::new(stream, start, end))
    }
}

#[cfg(test)]
mod test_pdf_prim {
    use super::super::super::pcore::parsebuffer::{
        locate_value, ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParsleyParser,
    };
    use super::{Boolean, Comment, Null, WhitespaceEOL, WhitespaceNoEOL};
    use super::{HexString, RawLiteralString, StreamContentP, StreamContentT};
    use super::{IntegerP, IntegerT, NameP, NameT, RealP, RealT};

    #[test]
    fn noeol() {
        let mut ws = WhitespaceNoEOL::new(false);

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at whitespace-noeol".to_string()),
            0,
            0,
        );
        assert_eq!(ws.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 1)));
        assert_eq!(pb.get_cursor(), 1);
    }

    #[test]
    fn eol() {
        let mut ws = WhitespaceEOL::new(false);

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at whitespace-eol".to_string()),
            0,
            0,
        );
        assert_eq!(ws.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    fn noeol_empty() {
        let mut ws = WhitespaceNoEOL::new(true);

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 1)));
        assert_eq!(pb.get_cursor(), 1);
    }

    #[test]
    fn eol_empty() {
        let mut ws = WhitespaceEOL::new(true);

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(LocatedVal::new((), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    fn comment() {
        let mut com = Comment;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at comment".to_string()), 0, 0);
        assert_eq!(com.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("% ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Ok(LocatedVal::new(vec![32], 0, 2)));
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("% \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Ok(LocatedVal::new(vec![32, 13], 0, 4)));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("%PDF-1.0 \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            com.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("PDF-1.0 \r".as_bytes()), 0, 11))
        );
        assert_eq!(pb.get_cursor(), 11);
    }

    #[test]
    fn boolean() {
        let mut bl = Boolean;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at boolean".to_string()), 0, 0);
        assert_eq!(bl.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at boolean".to_string()), 0, 0);
        assert_eq!(bl.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("true".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Ok(LocatedVal::new(true, 0, 4)));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("false ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Ok(LocatedVal::new(false, 0, 5)));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn null() {
        let mut null = Null;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at null".to_string()), 0, 0);
        assert_eq!(null.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at null".to_string()), 0, 0);
        assert_eq!(null.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("null".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Ok(LocatedVal::new((), 0, 4)));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("null ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Ok(LocatedVal::new((), 0, 4)));
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test]
    fn integer() {
        let mut int = IntegerP;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(int.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" 1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(int.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("-".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(int.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("+".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(int.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(1), 0, 1)));
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(23), 0, 2)));
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("+23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(23), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("-23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(-23), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(0), 0, 1)));
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("-0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(0), 0, 2)));
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("+0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(LocatedVal::new(IntegerT(0), 0, 2)));
        assert_eq!(pb.get_cursor(), 2);
    }

    #[test]
    fn real() {
        let mut real = RealP;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(real.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" 1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(real.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("-".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(real.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("+".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at number".to_string()), 0, 0);
        assert_eq!(real.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(1, 1), 0, 1)));
        let r = r.unwrap();
        assert!(r.unwrap().is_integer());
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("23.01 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(2301, 100), 0, 5)));
        let r = r.unwrap().unwrap();
        assert!(!r.is_integer());
        assert_eq!(r.numerator(), 2301);
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("+23.10 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(2310, 100), 0, 6)));
        let r = r.unwrap().unwrap();
        assert!(r.is_positive());
        assert!(!r.is_integer());
        assert_eq!(pb.get_cursor(), 6);

        let v = Vec::from("-23.10 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(-2310, 100), 0, 6)));
        let r = r.unwrap().unwrap();
        assert!(!r.is_positive());
        assert!(!r.is_integer());
        assert_eq!(pb.get_cursor(), 6);

        let v = Vec::from("0.0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(0, 10), 0, 3)));
        assert!(r.unwrap().unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(".01 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(1, 100), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("+0.0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(0, 10), 0, 4)));
        let r = r.unwrap().unwrap();
        assert!(r.is_zero());
        assert!(!r.is_integer());
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("-0.00 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new(RealT(0, 100), 0, 5)));
        let r = r.unwrap().unwrap();
        assert!(r.is_zero());
        assert!(!r.is_integer());
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn hex_string() {
        let mut hex = HexString;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at hex string".to_string()), 0, 0);
        assert_eq!(hex.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("not at hex string".to_string()), 0, 0);
        assert_eq!(hex.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("< ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at valid hex string".to_string()),
            0,
            0,
        );
        assert_eq!(hex.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("<> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb).unwrap().unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("<1a9> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Ok(LocatedVal::new(vec![26, 144], 0, 5)));
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("<1a90> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Ok(LocatedVal::new(vec![26, 144], 0, 6)));
        assert_eq!(pb.get_cursor(), 6);

        let v = Vec::from("<1a9q> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at valid hex string".to_string()),
            0,
            0,
        );
        assert_eq!(hex.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("< 1\na\r9\t1\r> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            hex.parse(&mut pb),
            Ok(LocatedVal::new(vec![26, 145], 0, 11))
        );
        assert_eq!(pb.get_cursor(), 11);
    }

    #[test]
    fn raw_lit_string() {
        let mut lit = RawLiteralString;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at literal string".to_string()),
            0,
            0,
        );
        assert_eq!(lit.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at literal string".to_string()),
            0,
            0,
        );
        assert_eq!(lit.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("( ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(lit.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("() ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb).unwrap().unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("()) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb).unwrap().unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 2);
        let e = locate_value(
            ErrorKind::GuardError("not at literal string".to_string()),
            2,
            2,
        );
        assert_eq!(lit.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("(()) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("()"), 0, 4))
        );
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("(() ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(lit.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("(1a9) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a9".as_bytes()), 0, 5))
        );
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("(1a(9)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a(9)0".as_bytes()), 0, 8))
        );
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a(90) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(lit.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("(1a9)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a9".as_bytes()), 0, 5))
        );
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("(1a\\(90) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a\\(90".as_bytes()), 0, 8))
        );
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a9\\)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a9\\)0".as_bytes()), 0, 8))
        );
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a\\(90\\)) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a\\(90\\)".as_bytes()), 0, 10))
        );
        assert_eq!(pb.get_cursor(), 10);
    }

    #[test]
    fn raw_lit_string_escaped_escapes() {
        let mut lit = RawLiteralString;

        let v = Vec::from("(1a\\\\\\(90\\\\) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(
                Vec::from("1a\\\\\\(90\\\\".as_bytes()),
                0,
                10
            ))
        );
        assert_eq!(pb.get_cursor(), 12);

        let v = Vec::from("(1a90\\\\) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(Vec::from("1a90\\\\".as_bytes()), 0, 10))
        );
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a\\\\(90\\\\)) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            lit.parse(&mut pb),
            Ok(LocatedVal::new(
                Vec::from("1a\\\\(90\\\\)".as_bytes()),
                0,
                10
            ))
        );
        assert_eq!(pb.get_cursor(), 12);
    }

    #[test]
    fn raw_name() {
        let mut name = NameP;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at name object".to_string()),
            0,
            0,
        );
        assert_eq!(name.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at name object".to_string()),
            0,
            0,
        );
        assert_eq!(name.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("/ ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb).unwrap().unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("/{ ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb).unwrap().unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("/1a9) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(Vec::from("1a9".as_bytes()));
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 4)));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("/(1a(9)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(Vec::from("".as_bytes()));
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 1)));
        assert_eq!(pb.get_cursor(), 1);

        // embedded null-character in name

        let v = vec![47, 65, 0, 66, 32];
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![65]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 2)));
        assert_eq!(pb.get_cursor(), 2);

        let v = vec![47, 0, 66, 32];
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 1)));
        assert_eq!(pb.get_cursor(), 1);

        // embedded character codes

        let v = vec![47, 35, 48, 48];
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::GuardError("null char in name".to_string()), 0, 2);
        assert_eq!(name.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = vec![47, 35, 48, 49];
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![1]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 4)));
        assert_eq!(pb.get_cursor(), 4);

        let v = vec![47, 65, 35, 48, 49];
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![65, 1]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 5)));
        assert_eq!(pb.get_cursor(), 5);

        let v = vec![47, 35, 48, 49, 35, 48, 49];
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![1, 1]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 7)));
        assert_eq!(pb.get_cursor(), 7);

        let v = vec![47, 35, 48, 49, 65]; // code '01'
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![1, 65]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 5)));
        assert_eq!(pb.get_cursor(), 5);

        let v = vec![47, 35, 48, 49, 65, 66]; // code '01'
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![1, 65, 66]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 6)));
        assert_eq!(pb.get_cursor(), 6);

        let v = vec![47, 35, 48, 65, 65, 66]; // code '0A'
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![10, 65, 66]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 6)));
        assert_eq!(pb.get_cursor(), 6);

        let v = vec![47, 35, 48, 97, 65, 66]; // code '0a'
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![10, 65, 66]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 6)));
        assert_eq!(pb.get_cursor(), 6);

        let v = vec![47, 35, 102, 70, 65, 66]; // code 'fF'
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![255, 65, 66]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 6)));
        assert_eq!(pb.get_cursor(), 6);

        let v = vec![47, 35, 102, 102, 65, 66]; // code 'ff'
        let mut pb = ParseBuffer::new(v);
        let val = NameT::new(vec![255, 65, 66]);
        assert_eq!(name.parse(&mut pb), Ok(LocatedVal::new(val, 0, 6)));
        assert_eq!(pb.get_cursor(), 6);
    }

    #[test]
    fn stream_content() {
        let mut sc = StreamContentP;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at stream content".to_string()),
            0,
            0,
        );
        assert_eq!(sc.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at stream content".to_string()),
            0,
            0,
        );
        assert_eq!(sc.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("strea ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not at stream content".to_string()),
            0,
            0,
        );
        assert_eq!(sc.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        //                 012345 678901234567890
        let v = Vec::from("stream\n  endstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            sc.parse(&mut pb),
            Ok(LocatedVal::new(
                StreamContentT::new(7, 2, Vec::from("  ")),
                0,
                18
            ))
        );
        assert_eq!(pb.get_cursor(), 18);

        //                 012345 6 78901234567890
        let v = Vec::from("stream\r\n  endstream".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            sc.parse(&mut pb),
            Ok(LocatedVal::new(
                StreamContentT::new(8, 2, Vec::from("  ")),
                0,
                19
            ))
        );
        assert_eq!(pb.get_cursor(), 19);

        //                 012345 6 789 012345678901
        let v = Vec::from("stream\r\n  \nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            sc.parse(&mut pb),
            Ok(LocatedVal::new(
                StreamContentT::new(8, 2, Vec::from("  ")),
                0,
                20
            ))
        );
        assert_eq!(pb.get_cursor(), 20);

        //                 012345 6 789 0 123456789012
        let v = Vec::from("stream\r\n  \r\nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            sc.parse(&mut pb),
            Ok(LocatedVal::new(
                StreamContentT::new(8, 3, Vec::from("  \r")),
                0,
                21
            ))
        ); // spec FIXME
        assert_eq!(pb.get_cursor(), 21);

        // wrong starting eol
        let v = Vec::from("stream\r  \r\nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("not a valid stream marker".to_string()),
            0,
            0,
        );
        assert_eq!(sc.parse(&mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);

        // endstream eol
        //                 012345 6 789 01234567890
        let v = Vec::from("stream\r\n  \rendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(
            sc.parse(&mut pb),
            Ok(LocatedVal::new(
                StreamContentT::new(8, 3, Vec::from("  \r")),
                0,
                20
            ))
        );
        assert_eq!(pb.get_cursor(), 20);
    }
}
