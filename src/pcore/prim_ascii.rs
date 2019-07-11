use std::convert::TryFrom;
use super::parsebuffer::{ParsleyPrimitive, ParsleyParser, ParseBuffer, ParseError, ErrorKind};

pub struct AsciiCharPrimitive;

impl ParsleyPrimitive for AsciiCharPrimitive {
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

// Convenience wrappers around the primitive interfaces, that allow
// use with the primitive combinators.

pub struct AsciiChar {
    guard: Option<Box<FnMut(&char) -> bool>>
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
    type T = char;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        match &mut self.guard {
            None    => buf.parse_prim::<AsciiCharPrimitive>(),
            Some(b) => buf.parse_guarded::<AsciiCharPrimitive>(b)
        }
    }
}

pub struct AsciiScanner {
    tag: Vec<u8>
}

impl AsciiScanner {
    pub fn new(tag: &str) -> AsciiScanner {
        let mut t = Vec::new();
        for c in tag.as_bytes().iter() { t.push(*c); }
        AsciiScanner { tag: t }
    }
}

impl ParsleyParser for AsciiScanner {
    type T = usize;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.scan(&self.tag)
    }
}

pub struct AsciiMatcher {
    tag: Vec<u8>
}

impl AsciiMatcher {
    pub fn new(tag: &str) -> AsciiMatcher {
        let mut t = Vec::new();
        for c in tag.as_bytes().iter() { t.push(*c); }
        AsciiMatcher { tag: t }
    }
}

impl ParsleyParser for AsciiMatcher {
    type T = usize;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.exact(&self.tag)
    }
}

// test the primitive parser
#[cfg(test)]
mod test_prim_ascii {
    use super::AsciiCharPrimitive;
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrimitive, ErrorKind, ParseError};

    // this raw interface would not normally be used; we would be
    // going via the ParseBuffer as in the remaining tests.
    #[test]
    fn raw() {
        let mut v = Vec::new();
        v.push(255);
        let r = <AsciiCharPrimitive as ParsleyPrimitive>::parse(&v);
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        assert_eq!(r, Err(pe));

        let mut w = Vec::new();
        w.push(65); // 'A'
        let r = <AsciiCharPrimitive as ParsleyPrimitive>::parse(&w);
        assert_eq!(r, Ok(('A', 1)));
    }

    #[test]
    fn empty() {
        let mut pb = ParseBuffer::new(Vec::new());
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(pb.parse_prim::<AsciiCharPrimitive>(), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn ascii() {
        let mut v : Vec<u8> = Vec::new();
        v.push(65);   // 'A'
        v.push(128);  // non-ascii
        v.push(0);    // nul; ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = pb.parse_prim::<AsciiCharPrimitive>();
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = pb.parse_prim::<AsciiCharPrimitive>();
        let pe = ParseError::new("ascii-prim: invalid ascii character");
        let e = Err(ErrorKind::PrimitiveError(pe));
        assert_eq!(r, e);
        // the cursor should not advance over the invalid char.
        assert_eq!(pb.get_cursor(), 1);

        // forcibly advance
        pb.set_cursor(2);
        let r = pb.parse_prim::<AsciiCharPrimitive>();
        assert_eq!(r, Ok('\u{0}'));
        assert_eq!(pb.get_cursor(), 3);
        assert_eq!(pb.parse_prim::<AsciiCharPrimitive>(), Err(ErrorKind::EndOfBuffer))
    }

    #[test]
    fn guard() {
        let mut v : Vec<u8> = Vec::new();
        v.push(65);  // 'A'
        v.push(66);  // 'B'
        v.push(128); // non-ascii
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.get_cursor(), 0);

        let r = pb.parse_guarded::<AsciiCharPrimitive>(&mut |c: &char| {*c == 'A'});
        assert_eq!(r, Ok('A'));
        assert_eq!(pb.get_cursor(), 1);

        let r = pb.parse_guarded::<AsciiCharPrimitive>(&mut |c: &char| {*c == 'A'});
        let e = Err(ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name()));
        assert_eq!(r, e);
        // the cursor should not advance if the guard fails
        assert_eq!(pb.get_cursor(), 1);
    }
}

// test the convenience wrapper
#[cfg(test)]
mod test_ascii {
    use super::{AsciiCharPrimitive, AsciiChar, AsciiScanner, AsciiMatcher};
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrimitive, ParsleyParser, ParseError, ErrorKind};

    #[test]
    fn empty() {
        let mut ascii_parser = AsciiChar::new();
        let mut pb = ParseBuffer::new(Vec::new());
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(ascii_parser.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn ascii() {
        let mut ascii_parser = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
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
        let e = Err(ErrorKind::PrimitiveError(pe));
        assert_eq!(r, e);
        // the cursor should not advance over the invalid char.
        assert_eq!(pb.get_cursor(), 1);

        // forcibly advance to nul
        pb.set_cursor(2);
        let r = ascii_parser.parse(&mut pb);
        // nul fails the guard test
        let e = Err(ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name()));
        assert_eq!(r, e);
        // the cursor should not advance over the failed guard
        assert_eq!(pb.get_cursor(), 2);

        // forcibly advance
        pb.set_cursor(3);
        assert_eq!(ascii_parser.parse(&mut pb), Err(ErrorKind::EndOfBuffer))
    }

    #[test]
    fn scan() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = AsciiScanner::new("%PDF-");

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(0));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("garbage %PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(8));
        assert_eq!(pb.get_cursor(), 8);
        assert_eq!(s.parse(&mut pb), Ok(0));
        assert_eq!(pb.get_cursor(), 8);

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn exact() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = AsciiMatcher::new("%PDF-");

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::GuardError("match")));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(5));
        assert_eq!(pb.get_cursor(), 5);

        let mut pb = ParseBuffer::new(Vec::from(" %PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::GuardError("match")));
        assert_eq!(pb.get_cursor(), 0);
    }
}
