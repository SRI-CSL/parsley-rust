use std::result::Result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseBuffer {
    buf: Vec<u8>,
    ofs: usize
}

#[derive(Debug)]
pub struct ParseError<'a> {
    msg: &'a str
}

impl<'a> ParseError<'a> {
    pub fn new(msg: &'a str) -> ParseError {
        ParseError{msg}
    }
}

pub trait ParsleyPrim {
    type T;
    fn prim_name() -> &'static str;
    fn parse_one(buf: &[u8]) -> Result<(Self::T,usize), ParseError>;
}

#[derive(Debug)]
pub enum ErrorKind<'a> {
    GuardError(&'a str),
    PrimError(ParseError<'a>),
}

impl<'a> fmt::Display for ErrorKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::GuardError(prim) => write!(f, "primitive guard error on {}", prim),
            ErrorKind::PrimError(ParseError{msg}) => write!(f, "primitive parse failure: {}", msg),
        }
    }
}

impl<'a> Error for ErrorKind<'a> {
    fn description(&self) -> &str {
        match self {
            ErrorKind::GuardError(_prim) => "primitive guard error",
            ErrorKind::PrimError(ParseError{msg}) => msg,
        }
    }
}

impl<'a> From<ParseError<'a>> for ErrorKind<'a> {
    fn from(err: ParseError) -> ErrorKind {
        ErrorKind::PrimError(err)
    }
}

impl ParseBuffer {
    pub fn new(buf: Vec<u8>) -> ParseBuffer {
        ParseBuffer { buf, ofs : 0 }
    }

    pub fn parse_prim<T : ParsleyPrim>(&mut self) ->
        Result<T::T, ErrorKind>
    {
        let (t, consumed) = T::parse_one(&self.buf[self.ofs..])?;
        self.ofs += consumed;
        Ok(t)
    }

    pub fn parse_guarded<T : ParsleyPrim>(&mut self, guard: Box<Fn(&T::T) -> bool>) ->
        Result<T::T, ErrorKind>
    {
        let (t, consumed) = T::parse_one(&self.buf[self.ofs..])?;
        if !guard(&t) {
            return Err(ErrorKind::GuardError(T::prim_name()));
        };
        self.ofs += consumed;
        Ok(t)
    }
}
