use std::convert::TryFrom;
use super::pbuf;

pub struct AsciiChar {
    c : char
}

impl AsciiChar {
    pub fn value(&self) -> char { self.c }
}

impl pbuf::ParsleyPrim for AsciiChar {
    type T = char;

    fn prim_name() -> &'static str { "ascii" }

    fn parse_one(buf: &[u8]) -> Result<(Self::T, usize), pbuf::ParseError> {
        if buf.len() == 0 { return Err(pbuf::ParseError::new("ascii: end-of-buffer")) };
        let c = char::try_from(buf[0]);
        if c.is_err() { return Err(pbuf::ParseError::new("ascii: invalid character")) };
        let c = c.unwrap();
        if !c.is_ascii() { return Err(pbuf::ParseError::new("ascii: invalid ascii character")) };
        // ascii consumes a single byte
        Ok((c, 1))
    }
}
