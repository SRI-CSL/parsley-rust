/// Basic parsing buffer manager, and the traits defining the parsing interface.

use std::error::Error;
use std::cmp::{PartialEq, PartialOrd, Ordering};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::fmt;

// The basic parsing buffer.
#[derive(Debug)]
pub struct ParseBuffer {
    buf: Vec<u8>,
    ofs: usize,
}

// Location information for objects returned by parsers.
pub trait Location {
    fn loc_start(&self) -> usize;
    fn loc_end(&self)   -> usize;
}

// Values returned by parsers should provide location annotations.
#[derive(Debug)]
pub struct LocatedVal<T>
where T : PartialEq
{
    val:   T,
    start: usize,
    end:   usize,
}
impl<T> LocatedVal<T>
where T : PartialEq
{
    pub fn new(val: T, start: usize, end: usize) -> LocatedVal<T> {
        LocatedVal { val, start, end }
    }
    pub fn val(&self)   -> &T { &self.val }
    pub fn unwrap(self) -> T  { self.val }
}
// Equality for LocatedVal<T> should not take into account the
// location.  Similarly, when a LocatedVal<T> is placed into a map
// (i.e. HashMap) as the key-type, the matching should be performed
// over the T.
impl<T> PartialEq for LocatedVal<T>
where T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.val, &other.val)
    }
}
impl<T> Eq for LocatedVal<T>
where T : PartialEq
{}
impl<T> PartialOrd for LocatedVal<T>
where T : PartialOrd
{
    fn partial_cmp(&self, other: &LocatedVal<T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.val, &other.val)
    }
}
impl<T> Ord for LocatedVal<T>
where T : Ord
{
    fn cmp(&self, other: &LocatedVal<T>) -> Ordering {
        Ord::cmp(&self.val, &other.val)
    }
}
impl<T> Hash for LocatedVal<T>
where T : PartialEq, T : Hash
{
    fn hash<H : Hasher>(&self, state: &mut H) { self.val.hash(state) }
}
// This allows HashMap lookup using T even when the HashMap is keyed
// by LocatedVal<T>.
impl<T> Borrow<T> for LocatedVal<T>
where T : PartialEq
{
    fn borrow(&self) -> &T { &self.val }
}
// Provide location information.
impl<T> Location for LocatedVal<T>
where T : PartialEq
{
    fn loc_start(&self) -> usize { self.start }
    fn loc_end(&self)   -> usize { self.end }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    msg: &'static str
}

impl ParseError {
    pub fn new(msg: &'static str) -> ParseError {
        ParseError { msg }
    }
}

// Convenience alias.
pub type ParseResult<T> = std::result::Result<T, ErrorKind>;

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
    type T : Location;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T>;
}

// Errors generated by this module.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    // Insufficient data
    EndOfBuffer,
    // Errors during unguarded primitive parsing.
    PrimitiveError(ParseError),
    // Errors during guarded primitive parsing.
    GuardError(&'static str),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::EndOfBuffer => write!(f, "end of buffer"),
            ErrorKind::PrimitiveError(ParseError{msg}) => write!(f, "primitive parse failure: {}", msg),
            ErrorKind::GuardError(prim) => write!(f, "primitive guard error on {}", prim),
        }
    }
}

impl Error for ErrorKind {
    fn description(&self) -> &str {
        match self {
            ErrorKind::EndOfBuffer => "end of buffer",
            ErrorKind::PrimitiveError(ParseError{msg}) => msg,
            ErrorKind::GuardError(_prim) => "primitive guard error",
        }
    }
}

impl From<ParseError> for ErrorKind {
    fn from(err: ParseError) -> ErrorKind {
        ErrorKind::PrimitiveError(err)
    }
}

impl ParseBuffer {
    pub fn new(buf: Vec<u8>) -> ParseBuffer {
        ParseBuffer { buf, ofs : 0 }
    }

    pub fn remaining(&self) -> usize {
        assert!(self.ofs <= self.buf.len());
        self.buf.len() - self.ofs
    }

    pub fn peek(&self) -> Option<u8> {
        if self.ofs < self.buf.len() {
            Some(self.buf[self.ofs])
        } else {
            None
        }
    }

    // Cursor management: get and set the parsing cursor; to allow
    // parsing to backtrack or rewind after an unsuccessful complex
    // parse.
    pub fn get_cursor(&self) -> usize {
        self.ofs
    }
    pub fn set_cursor(&mut self, ofs: usize) {
        assert!(ofs <= self.buf.len());
        self.ofs = ofs
    }

    // to be used only for low-level primitives.
    pub fn incr_cursor(&mut self) -> () {
        self.ofs += 1;
    }
    pub fn decr_cursor(&mut self) -> () {
        self.ofs -= 1;
    }

    // Parsing a single element of the Parsley primitive type P; it
    // returns a value of the Rust representation type P::T when successful.
    pub fn parse_prim<P : ParsleyPrimitive>(&mut self) -> ParseResult<P::T>
    {
        let (t, consumed) = P::parse(&self.buf[self.ofs..])?;
        self.ofs += consumed;
        Ok(t)
    }

    // Parsing a single element of the Parsley primitive type P that
    // is constrained by a predicate 'guard'; it returns a value of
    // the Rust representation type P::T when successful.  The 'guard'
    // is specified in terms of the values of the representation type
    // P::T.
    pub fn parse_guarded<P : ParsleyPrimitive>(&mut self, guard: &mut dyn FnMut(&P::T) -> bool) ->
        ParseResult<P::T>
    {
        let (t, consumed) = P::parse(&self.buf[self.ofs..])?;
        if !guard(&t) { return Err(ErrorKind::GuardError(P::name())) };
        self.ofs += consumed;
        Ok(t)
    }

    // A specialized form of guarded to avoid creation of boxed FnMut
    // closures, which seem to require 'static lifetimes.  The match
    // stops at the first disallowed byte.
    pub fn parse_allowed_bytes(&mut self, allow: &[u8]) -> ParseResult<Vec<u8>>
    {
        let mut consumed = 0;
        let mut r = Vec::new();
        for b in self.buf[self.ofs..].iter() {
            if !allow.contains(b) { break; }
            r.push(*b);
            consumed += 1;
        }
        self.ofs += consumed;
        Ok(r)
    }

    // A specialized form of guarded to avoid creation of boxed FnMut
    // closures, which seem to require 'static lifetimes.  The match
    // stops at the first disallowed byte.
    //
    // TODO: a version that returns ErrorKind::EndOfBuffer if the
    // terminator is not found.
    pub fn parse_bytes_until(&mut self, terminators: &[u8]) -> ParseResult<Vec<u8>>
    {
        let mut consumed = 0;
        let mut r = Vec::new();
        for b in self.buf[self.ofs..].iter() {
            if terminators.contains(b) { break; }
            r.push(*b);
            consumed += 1;
        }
        self.ofs += consumed;
        Ok(r)
    }

    // Checks for a tag at the current location without moving the cursor.
    pub fn check_prefix(&mut self, prefix: &[u8]) -> ParseResult<bool>
    {
        Ok(self.buf[self.ofs..].starts_with(prefix))
    }

    // Scanning for a tag.  The cursor is set to the *start* of the
    // tag when successful, and the number of bytes skipped over is
    // returned.  If the tag is not found, the cursor is not moved.
    // It includes the current position in the match, so the returned
    // relative offset (if any) could be zero if the cursor is
    // positioned at the tag.  This is a primitive since low-level
    // access to the parse buffer is needed.
    pub fn scan(&mut self, tag: &[u8]) -> ParseResult<usize> {
        let mut skip = 0;
        for w in self.buf[self.ofs..].windows(tag.len()) {
            if w.starts_with(tag) {
                self.ofs = self.ofs + skip;
                return Ok(skip)
            }
            skip += 1;
        }
        Err(ErrorKind::EndOfBuffer)
    }

    // Scan backwards for a tag. As above, the cursor is set to the
    // *start* of the tag when successful.  Since it skips the current
    // position, the returned relative offset (if any) is always
    // positive.
    pub fn backward_scan(&mut self, tag: &[u8]) -> ParseResult<usize> {
        let mut skip = 1;
        for w in self.buf[..self.ofs].windows(tag.len()).rev() {
            if w.starts_with(tag) {
                skip = skip + tag.len() - 1;
                self.ofs = self.ofs - skip;
                return Ok(skip)
            }
            skip += 1;
        }
        Err(ErrorKind::EndOfBuffer)
    }

    // Exact match on a tag at the current cursor location.  On
    // success, cursor is advanced past the exact match, but not moved
    // on failure.
    pub fn exact(&mut self, tag: &[u8]) -> ParseResult<bool> {
        if self.buf[self.ofs..].starts_with(tag) {
            self.ofs = self.ofs + tag.len();
            Ok(true)
        } else {
            Err(ErrorKind::GuardError("match"))
        }
    }

    // Extract binary stream of specified length.
    pub fn extract<'a>(&'a mut self, len: usize) -> ParseResult<&'a [u8]> {
        if self.buf[self.ofs..].len() < len {
            Err(ErrorKind::EndOfBuffer)
        } else {
            let ret = &self.buf[self.ofs..(self.ofs+len)];
            self.ofs += len;
            Ok(ret)
        }
    }

    // Destructive modification of parsing buffer by dropping content
    // before the cursor.  The cursor will then have offset 0.
    pub fn drop_upto(&mut self) {
        self.buf = self.buf.split_off(self.ofs);
        self.ofs = 0
    }
}

#[cfg(test)]
mod test_parsebuffer {
    use std::collections::HashMap;
    use super::{ParseBuffer, LocatedVal, ErrorKind};

    #[test]
    fn test_scan() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan("56".as_bytes()), Ok(5));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.scan("56".as_bytes()), Ok(0));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.scan("0".as_bytes()), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn test_backward_scan() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan("56".as_bytes()), Ok(5));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.backward_scan("56".as_bytes()), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 5);
        assert_eq!(pb.backward_scan("012".as_bytes()), Ok(5));
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
    fn test_drop_upto() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan("56".as_bytes()), Ok(5));
        pb.drop_upto();
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(pb.scan("56".as_bytes()), Ok(0));
    }
}
