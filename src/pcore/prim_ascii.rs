use std::convert::TryFrom;
use super::parsebuffer::{ParsleyPrim, ParseError};

pub struct AsciiChar {
    c : char
}

impl AsciiChar {
    pub fn value(&self) -> char { self.c }
}

impl ParsleyPrim for AsciiChar {
    type T = char;

    fn name() -> &'static str { "ascii" }

    fn size_bytes() -> usize { 1 }

    fn parse(buf: &[u8]) -> Result<(Self::T, usize), ParseError> {
        let c = char::try_from(buf[0]);
        if c.is_err() { return Err(ParseError::new("ascii: invalid character")) }
        let c = c.unwrap();
        if !c.is_ascii() { return Err(ParseError::new("ascii: invalid ascii character")) }
        Ok((c, 1))
    }
}
