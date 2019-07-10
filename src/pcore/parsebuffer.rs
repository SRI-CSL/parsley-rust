/// Basic parsing buffer manager, and the traits defining the parsing interface.

use std::result::Result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseBuffer {
    buf: Vec<u8>,
    ofs: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    msg: &'static str
}

impl ParseError {
    pub fn new(msg: &'static str) -> ParseError {
        ParseError{msg}
    }
}

// The trait defining the interface for parsing a primitive
// *fixed-size* type.
pub trait ParsleyPrimitive {
    // The Rust type for the parsed value
    type T;

    // The name of the type, used for logging/error-messages
    fn name() -> &'static str;

    // The fixed-size this type consumes from the parsing buffer.
    fn size_bytes() -> usize;

    // Parses a single fixed-size value from the provided buffer, and
    // returns the value and the number of bytes consumed from the
    // buffer.  There may not be size_bytes() in the buffer.
    fn parse(buf: &[u8]) -> Result<(Self::T,usize), ParseError>;
}

// The trait defining a general Parsley parser.  This trait is
// intended to be compatible with the various parser combinators.
//
// The main difference between this trait and the Primitive trait
// above is that the general parsers do not consume a fixed size from
// the buffer.
pub trait ParsleyParser {
    // The Rust type for the parsed value
    type T;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind>;
}

// Errors generated by this module.
#[derive(Debug, PartialEq, Eq)]
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

    fn remaining(&self) -> usize {
        assert!(self.ofs <= self.buf.len());
        self.buf.len() - self.ofs
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

    // Parsing a single element of the Parsley primitive type P; it
    // returns a value of the Rust representation type P::T when successful.
    pub fn parse_prim<P : ParsleyPrimitive>(&mut self) ->
        Result<P::T, ErrorKind>
    {
        if self.remaining() < P::size_bytes() { return Err(ErrorKind::EndOfBuffer) }
        let (t, consumed) = P::parse(&self.buf[self.ofs..])?;
        assert_eq!(consumed, P::size_bytes());
        self.ofs += consumed;
        Ok(t)
    }

    // Parsing a single element of the Parsley primitive type P that
    // is constrained by a predicate 'guard'; it returns a value of
    // the Rust representation type P::T when successful.  The 'guard'
    // is specified in terms of the values of the representation type
    // P::T.
    pub fn parse_guarded<P : ParsleyPrimitive>(&mut self, guard: &mut dyn FnMut(&P::T) -> bool) ->
        Result<P::T, ErrorKind>
    {
        if self.remaining() < P::size_bytes() { return Err(ErrorKind::EndOfBuffer) }
        let (t, consumed) = P::parse(&self.buf[self.ofs..])?;
        assert_eq!(consumed, P::size_bytes());
        if !guard(&t) { return Err(ErrorKind::GuardError(P::name())) };
        self.ofs += consumed;
        Ok(t)
    }

    // Scanning for a constant tag.  The cursor is set to the *start*
    // of the tag when successful, and the number of bytes skipped
    // over is returned.  If the tag is not found, the cursor is not
    // moved.  This is a primitive since low-level access to the parse
    // buffer is needed.
    pub fn scan(&mut self, tag: &[u8]) -> Result<usize, ErrorKind> {
        let mut skip = 0;
        for w in self.buf[self.ofs..].windows(tag.len()) {
            if w.starts_with(tag) {
                self.ofs = self.ofs + skip;
                return Ok(skip)
            }
            skip = skip + 1;
        }
        Err(ErrorKind::EndOfBuffer)
    }
}
