use std::convert::TryFrom;
use super::parsebuffer::{ParsleyPrim, ParsleyParser, ParseBuffer, ParseError, ErrorKind};

pub struct AsciiChar {}

impl ParsleyPrim for AsciiChar {
    type T = char;

    fn name() -> &'static str { "ascii-prim" }

    fn size_bytes() -> usize { 1 }

    fn parse(buf: &[u8]) -> Result<(Self::T, usize), ParseError> {
        // bounds check is done by  ParseBuffer, so we don't need to do it here.
        let c = char::try_from(buf[0]);
        // check this: we should never get the below error from
        // non-empty buffers, as all u8 should be convertible to char.
        if c.is_err() { return Err(ParseError::new("ascii-prim: invalid character")) }
        let c = c.unwrap();
        if !c.is_ascii() { return Err(ParseError::new("ascii-prim: invalid ascii character")) }
        Ok((c, 1))
    }
}

// A convenience wrapper around the primitive interface, that allows
// use with the primitive combinators.
impl ParsleyParser for AsciiChar {
    type T = char;

    fn name() -> &'static str { "ascii" }

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.parse_prim::<AsciiChar>()
    }
}

// test the primitive parser
#[cfg(test)]
mod test_prim_ascii {
    use super::AsciiChar;
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrim, ErrorKind, ParseError};

    // this raw interface would not normally be used; we would be
    // going via the ParseBuffer as in the remaining tests.
    #[test]
    fn test_prim() {
        let mut v = Vec::new();
        v.push(255);
        let r = <AsciiChar as ParsleyPrim>::parse(&v);
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        assert_eq!(r, Err(pe));

        let mut w = Vec::new();
        w.push(65); // 'A'
        let r = <AsciiChar as ParsleyPrim>::parse(&w);
        assert_eq!(r, Ok(('A', 1)));
    }

    #[test]
    fn test_empty() {
        let mut pb = ParseBuffer::new(Vec::new());
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(pb.parse_prim::<AsciiChar>(), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn test_ascii() {
        let mut v : Vec<u8> = Vec::new();
        v.push(65);   // 'A'
        v.push(128);  // non-ascii
        v.push(0);    // nul; ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = pb.parse_prim::<AsciiChar>();
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = pb.parse_prim::<AsciiChar>();
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        let e = Err(ErrorKind::PrimError(pe));
        assert_eq!(r, e);
        // the cursor should not advance over the invalid char.
        assert_eq!(pb.get_cursor(), 1);

        // forcibly advance
        pb.set_cursor(2);
        let r = pb.parse_prim::<AsciiChar>();
        assert_eq!(r, Ok('\u{0}'));
        assert_eq!(pb.get_cursor(), 3);
        assert_eq!(pb.parse_prim::<AsciiChar>(), Err(ErrorKind::EndOfBuffer))
    }

    #[test]
    fn test_guard() {
        let mut v : Vec<u8> = Vec::new();
        v.push(65);  // 'A'
        v.push(66);  // 'B'
        v.push(128); // non-ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = pb.parse_guarded::<AsciiChar>(Box::new(|c| {*c == 'A'}));
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = pb.parse_guarded::<AsciiChar>(Box::new(|c| {*c == 'A'}));
        let e = Err(ErrorKind::GuardError(<AsciiChar as ParsleyPrim>::name()));
        assert_eq!(r, e);
        // the cursor should not advance if the guard fails
        assert_eq!(pb.get_cursor(), 1);
    }
}

// test the convenience wrapper
#[cfg(test)]
mod test_ascii {
    use super::AsciiChar;
    use super::super::parsebuffer::{ParseBuffer, ParsleyParser, ParseError, ErrorKind};

    #[test]
    fn test_empty() {
        let mut ascii_parser = AsciiChar {};
        let mut pb = ParseBuffer::new(Vec::new());
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(ascii_parser.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn test_ascii() {
        let mut ascii_parser = AsciiChar {};
        let mut v : Vec<u8> = Vec::new();
        v.push(65);   // 'A'
        v.push(128);  // non-ascii
        v.push(0);    // nul; ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = ascii_parser.parse(&mut pb);
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = ascii_parser.parse(&mut pb);
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        let e = Err(ErrorKind::PrimError(pe));
        assert_eq!(r, e);
        // the cursor should not advance over the invalid char.
        assert_eq!(pb.get_cursor(), 1);

        // forcibly advance
        pb.set_cursor(2);
        let r = ascii_parser.parse(&mut pb);
        assert_eq!(r, Ok('\u{0}'));
        assert_eq!(pb.get_cursor(), 3);
        assert_eq!(ascii_parser.parse(&mut pb), Err(ErrorKind::EndOfBuffer))
    }
}
