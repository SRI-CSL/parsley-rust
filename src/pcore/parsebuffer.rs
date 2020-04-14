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

/// Basic parsing buffer manager, and the traits defining the parsing interface.

use std::cmp::{PartialEq, PartialOrd, Ordering};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::fmt;

// Location information for objects returned by parsers.
pub trait Location {
    fn loc_start(&self) -> usize;
    fn loc_end(&self) -> usize;
}

// Values returned by parsers should provide location annotations.
#[derive(Debug)]
pub struct LocatedVal<T>
{
    val: T,
    start: usize,
    end: usize,
}

impl<T> LocatedVal<T>
    where T: PartialEq
{
    pub fn new(val: T, start: usize, end: usize) -> LocatedVal<T> {
        LocatedVal { val, start, end }
    }
    pub fn val(&self) -> &T { &self.val }
    pub fn unwrap(self) -> T { self.val }
    pub fn start(&self) -> usize { self.start }
    pub fn end(&self) -> usize { self.end }
}

// Equality for LocatedVal<T> should not take into account the
// location.  Similarly, when a LocatedVal<T> is placed into a map
// (i.e. HashMap) as the key-type, the matching should be performed
// over the T.
impl<T> PartialEq for LocatedVal<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.val, &other.val)
    }
}

impl<T> Eq for LocatedVal<T>
    where T: PartialEq
{}

impl<T> PartialOrd for LocatedVal<T>
    where T: PartialOrd
{
    fn partial_cmp(&self, other: &LocatedVal<T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.val, &other.val)
    }
}

impl<T> Ord for LocatedVal<T>
    where T: Ord
{
    fn cmp(&self, other: &LocatedVal<T>) -> Ordering {
        Ord::cmp(&self.val, &other.val)
    }
}

impl<T> Hash for LocatedVal<T>
    where T: PartialEq, T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.val.hash(state) }
}

// This allows HashMap lookup using T even when the HashMap is keyed
// by LocatedVal<T>.
impl<T> Borrow<T> for LocatedVal<T>
    where T: PartialEq
{
    fn borrow(&self) -> &T { &self.val }
}

// Provide location information.
impl<T> Location for LocatedVal<T>
    where T: PartialEq
{
    fn loc_start(&self) -> usize { self.start }
    fn loc_end(&self) -> usize { self.end }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    msg: String
}

impl ParseError {
    pub fn new(s: &str) -> ParseError {
        ParseError { msg: String::from(s) }
    }
}

// Convenience alias.
pub type ParseResult<T> = std::result::Result<T, LocatedVal<ErrorKind>>;

// The trait defining the interface for parsing a primitive type of
// unknown size.  The parsers implementing this trait are usually
// associated with a specific primitive type, specified by name().
// They cannot be the output of combinators.
pub trait ParsleyPrimitive {
    // The Rust type for the parsed value
    type T;

    // The name of the type, used for logging/error-messages
    fn name() -> &'static str;

    // Parses a single value from the provided buffer, and returns the
    // value and the number of bytes consumed from the buffer.
    fn parse(buf: &[u8]) -> ParseResult<(Self::T, usize)>;
}

// The trait defining a general Parsley parser.  This trait is
// intended to be compatible with the various parser combinators.
// That is, parsers implementing this trait can be input into parser
// combinators, and are output from parser combinators.
//
// Unlike parsers implementing the Primitive trait above, these
// general parsers do not have a defined name.
pub trait ParsleyParser {
    // The Rust type for the parsed value
    type T: Location;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T>;
}

// Errors generated by this module.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    // Insufficient data
    EndOfBuffer,
    // Errors during unguarded primitive parsing.
    PrimitiveError(ParseError),
    // Errors during guarded primitive parsing.
    GuardError(String),
}

// function to report located errors with sensible location
pub fn make_error(val: ErrorKind, s: usize, e: usize) -> LocatedVal<ErrorKind> {
    if s < e {
        LocatedVal::new(val, s, e)
    } else {
        LocatedVal::new(val, e, s)
    }
}

pub fn make_error_with_loc(val: ErrorKind, l: &dyn Location) -> LocatedVal<ErrorKind> {
    LocatedVal::new(val, l.loc_start(), l.loc_end())
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::EndOfBuffer =>
                write!(f, "end of buffer"),
            ErrorKind::PrimitiveError(ParseError { msg }) =>
                write!(f, "primitive parse failure: {}", msg),
            ErrorKind::GuardError(prim) =>
                write!(f, "primitive guard error: {}", prim),
        }
    }
}

impl From<ParseError> for ErrorKind {
    fn from(err: ParseError) -> ErrorKind {
        ErrorKind::PrimitiveError(err)
    }
}

// The interface provided by every parsebuffer view.  All cursor
// management and query operations are done with respect to the view,
// and not the underlying buffer.

pub trait ParseBufferT {
    // current state queries

    fn size(&self) -> usize;
    fn remaining(&self) -> usize;
    fn get_cursor(&self) -> usize;

    // buffer access

    fn peek(&self) -> Option<u8>;
    fn buf(&self) -> &[u8];

    // internal api, used for view restrictions.  ideally, this would have visibility
    //   pub(in self::ParseBuffer)
    // but this is not supported (surprisingly).
    fn rc_buf(&self) -> Rc<Vec<u8>>;
    fn start(&self) -> usize;

    // cursor management

    fn set_cursor(&mut self, ofs: usize);
    fn incr_cursor(&mut self);
    fn decr_cursor(&mut self);

    // parsing primitives

    // Checks for a tag at the current location without moving the cursor.
    fn check_prefix(&mut self, prefix: &[u8]) -> ParseResult<bool>;

    // The match stops at the first disallowed byte.
    fn parse_allowed_bytes(&mut self, allow: &[u8]) -> ParseResult<Vec<u8>>;

    // The match stops at the first disallowed byte.
    //
    // TODO: a version that returns ErrorKind::EndOfBuffer if the
    // terminator is not found.
    fn parse_bytes_until(&mut self, terminators: &[u8]) -> ParseResult<Vec<u8>>;

    // Scanning for a tag.  The cursor is set to the *start* of the
    // tag when successful, and the number of bytes skipped over is
    // returned.  If the tag is not found, the cursor is not moved.
    // It includes the current position in the match, so the returned
    // relative offset (if any) could be zero if the cursor is
    // positioned at the tag.  This is a primitive since low-level
    // access to the parse buffer is needed.
    fn scan(&mut self, tag: &[u8]) -> ParseResult<usize>;

    // Scan backwards for a tag. As above, the cursor is set to the
    // *start* of the tag when successful.  Since it skips the current
    // position, the returned relative offset (if any) is always
    // positive.
    fn backward_scan(&mut self, tag: &[u8]) -> ParseResult<usize>;

    // Exact match on a tag at the current cursor location.  On
    // success, cursor is advanced past the exact match, but not moved
    // on failure.
    fn exact(&mut self, tag: &[u8]) -> ParseResult<bool>;

    // Extract binary stream of specified length.
    fn extract<'a>(&'a mut self, len: usize) -> ParseResult<&'a [u8]>;
}

// The basic parsing buffer.  This implements a (possibly restricted)
// view of length 'size' into the underlying buffer 'buf'.  The first
// (0'th byte) of the view corresponds to byte 'start', and the last
// valid byte of the view is at byte 'start + size - 1'.

#[derive(Debug)]
pub struct ParseBuffer {
    buf:   Rc<Vec<u8>>,
    start: usize,
    size:  usize,
    ofs:   usize, // index into the view (added to 'start' for buffer index).
}

impl ParseBuffer {
    // Creates a default view into the parse buffer.
    pub fn new(buf: Vec<u8>) -> ParseBuffer {
        let size = buf.len();
        ParseBuffer { buf: Rc::new(buf), start: 0, ofs: 0, size }
    }

    // Returns a new view restricted as specified if the bounds
    // permit.  In the returned view, the offset is reset to 0.
    pub fn restrict_view(buf: &dyn ParseBufferT, start: usize, size: usize)
                         -> Option<ParseBuffer> {
        if size > 0 && start + size <= buf.size() {
            Some(ParseBuffer { buf: buf.rc_buf(),
                               start: buf.start() + start, ofs: 0, size })
        } else {
            None
        }
    }

    pub fn restrict_view_from(buf: &dyn ParseBufferT, start: usize)
                              -> Option<ParseBuffer> {
        if start < buf.size() {
            ParseBuffer::restrict_view(buf, start, buf.size() - start)
        } else {
            None
        }
    }
}

// Parsing a single element of the Parsley primitive type P; it
// returns a value of the Rust representation type P::T when
// successful.
pub fn parse_prim<P: ParsleyPrimitive>(buf: &mut dyn ParseBufferT) -> ParseResult<P::T> {
    let start = buf.get_cursor();
    let (t, consumed) = P::parse(buf.buf())?;
    buf.set_cursor(start + consumed);
    Ok(t)
}

// Parsing a single element of the Parsley primitive type P that is
// constrained by a predicate 'guard'; it returns a value of the Rust
// representation type P::T when successful.  The 'guard' is specified
// in terms of the values of the representation type P::T.
pub fn parse_guarded<P: ParsleyPrimitive>(buf: &mut dyn ParseBufferT,
                                          guard: &mut dyn FnMut(&P::T) -> bool)
                                          -> ParseResult<P::T> {
    let start = buf.get_cursor();
    let (t, consumed) = P::parse(buf.buf())?;
    if !guard(&t) {
        let end = buf.get_cursor();
        let err = ErrorKind::GuardError(P::name().to_string());
        return Err(make_error(err, start, end))
    };
    buf.set_cursor(start + consumed);
    Ok(t)
}

impl ParseBufferT for ParseBuffer {
    fn size(&self) -> usize {
        self.size
    }

    fn remaining(&self) -> usize {
        assert!(self.ofs <= self.size);
        self.size - self.ofs
    }

    fn peek(&self) -> Option<u8> {
        if self.ofs < self.size {
            Some(self.buf[self.start + self.ofs])
        } else {
            None
        }
    }

    fn buf(&self) -> &[u8] {
        &self.buf[(self.start + self.ofs) .. (self.start + self.size)]
    }

    fn rc_buf(&self) -> Rc<Vec<u8>> {
        Rc::clone(&self.buf)
    }
    fn start(&self) -> usize {
        self.start
    }

    fn get_cursor(&self) -> usize {
        self.ofs
    }
    fn set_cursor(&mut self, ofs: usize) {
        assert!(ofs <= self.size);
        self.ofs = ofs
    }

    fn incr_cursor(&mut self) -> () {
        assert!(self.ofs < self.size);
        self.ofs += 1;
    }
    fn decr_cursor(&mut self) -> () {
        assert!(self.ofs > 0);
        self.ofs -= 1;
    }

    fn parse_allowed_bytes(&mut self, allow: &[u8]) -> ParseResult<Vec<u8>> {
        let mut consumed = 0;
        let mut r = Vec::new();
        for b in self.buf[(self.start + self.ofs) .. (self.start + self.size)].iter() {
            if !allow.contains(b) { break }
            r.push(*b);
            consumed += 1;
        }
        self.ofs += consumed;
        Ok(r)
    }

    fn parse_bytes_until(&mut self, terminators: &[u8]) -> ParseResult<Vec<u8>> {
        let mut consumed = 0;
        let mut r = Vec::new();
        for b in self.buf[(self.start + self.ofs) .. (self.start + self.size)].iter() {
            if terminators.contains(b) { break }
            r.push(*b);
            consumed += 1;
        }
        self.ofs += consumed;
        Ok(r)
    }

    fn check_prefix(&mut self, prefix: &[u8]) -> ParseResult<bool> {
        Ok(self.buf[(self.start + self.ofs) .. (self.start + self.size)].starts_with(prefix))
    }

    fn scan(&mut self, tag: &[u8]) -> ParseResult<usize> {
        let start = self.get_cursor();
        let mut skip = 0;
        for w in self.buf[(self.start + self.ofs) .. (self.start + self.size)].windows(tag.len()) {
            if w.starts_with(tag) {
                self.ofs = self.ofs + skip;
                return Ok(skip)
            }
            skip += 1;
        }
        Err(make_error(ErrorKind::EndOfBuffer, start, start))
    }

    fn backward_scan(&mut self, tag: &[u8]) -> ParseResult<usize> {
        let start = self.get_cursor();
        let mut skip = 1;
        for w in self.buf[self.start .. (self.start + self.ofs)].windows(tag.len()).rev() {
            if w.starts_with(tag) {
                skip = skip + tag.len() - 1;
                self.ofs = self.ofs - skip;
                return Ok(skip)
            }
            skip += 1;
        }
        Err(make_error(ErrorKind::EndOfBuffer, start, start))
    }

    fn exact(&mut self, tag: &[u8]) -> ParseResult<bool> {
        let start = self.get_cursor();
        if self.buf[(self.start + self.ofs) .. (self.start + self.size)].starts_with(tag) {
            self.ofs = self.ofs + tag.len();
            Ok(true)
        } else {
            Err(make_error(ErrorKind::GuardError("match".to_string()), start, start))
        }
    }

    fn extract<'a>(&'a mut self, len: usize) -> ParseResult<&'a [u8]> {
        if self.remaining() < len {
            let start = self.get_cursor();
            Err(make_error(ErrorKind::EndOfBuffer, start, start))
        } else {
            let ret = &self.buf[(self.start + self.ofs) .. (self.start + self.ofs + len)];
            self.ofs += len;
            Ok(ret)
        }
    }
}

#[cfg(test)]
mod test_parsebuffer {
    use std::collections::HashMap;
    use super::{ParseBuffer, ParseBufferT, LocatedVal, ErrorKind, make_error};

    #[test]
    fn test_scan() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan(b"56"), Ok(5));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.scan(b"56"), Ok(0));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.scan(b"0"),
                   Err(make_error(ErrorKind::EndOfBuffer, 5, 5)));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn test_backward_scan() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan(b"56"), Ok(5));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.backward_scan(b"56"),
                   Err(make_error(ErrorKind::EndOfBuffer, 5, 5)));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.backward_scan(b"012"), Ok(5));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn test_locatedval_eq() {
        let v = LocatedVal::new(1, 0, 0);
        let w = LocatedVal::new(1, 1, 2);
        assert_eq!(v, w);

        let mut map = HashMap::new();
        map.insert(v, 0);
        assert!(map.contains_key(&1));
    }

    #[test]
    fn test_restrict_view() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan(b"9"), Ok(9));
        pb.set_cursor(0);
        assert_eq!(pb.scan(b"56"), Ok(5));
        let size = pb.remaining();
        pb = ParseBuffer::restrict_view(&pb, 5, size).unwrap();
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(pb.remaining(), size);
        assert_eq!(pb.scan(b"56"), Ok(0));
        pb.set_cursor(0);
        assert_eq!(pb.scan(b"9"), Ok(4));

        // identical view
        pb.set_cursor(0);
        let size = pb.remaining();
        pb = ParseBuffer::restrict_view(&pb, 0, size).unwrap();
        assert_eq!(pb.remaining(), size);
        assert_eq!(pb.scan(b"9"), Ok(4));

        // identical view
        pb = ParseBuffer::restrict_view_from(&pb, 0).unwrap();
        assert_eq!(pb.remaining(), size);
        assert_eq!(pb.scan(b"9"), Ok(4));

        // view from
        pb = ParseBuffer::restrict_view_from(&pb, pb.size() - 1).unwrap();
        assert_eq!(pb.remaining(), 1);
        assert_eq!(pb.scan(b"9"), Ok(0));
        let pb = ParseBuffer::restrict_view_from(&pb, 1);
        assert!(pb.is_none());
    }

    #[test]
    fn test_multiple_view() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan(b"9"), Ok(9));
        pb.set_cursor(0);
        assert_eq!(pb.scan(b"56"), Ok(5));

        // create a new view
        let size = pb.remaining();
        let mut pb_new = ParseBuffer::restrict_view(&pb, 5, size).unwrap();
        assert_eq!(pb_new.scan(b"56"), Ok(0));
        pb_new.set_cursor(0);
        assert_eq!(pb_new.scan(b"9"), Ok(4));

        // ensure the old view is still usable
        pb.set_cursor(0);
        assert_eq!(pb.scan(b"56"), Ok(5));
    }
}
