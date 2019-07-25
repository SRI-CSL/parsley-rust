// Basic primitive (non-compound or non-recursive) PDF objects.

use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
use super::super::pcore::prim_binary::{BinaryMatcher};

// The whitespace parsers require at least one whitespace character
// for a successful parse.

pub struct WhitespaceEOL;
impl ParsleyParser for WhitespaceEOL {
    type T = ();

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let v = buf.parse_allowed_bytes(" \0\t\r\n\x0c".as_bytes())?;
        if v.len() == 0 { return Err(ErrorKind::GuardError("not at whitespace-eol")) };
        Ok(())
    }
}

pub struct WhitespaceNoEOL;
impl ParsleyParser for WhitespaceNoEOL {
    type T = ();

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let ws = buf.parse_allowed_bytes(" \0\t\r\x0c".as_bytes())?;
        if ws.len() == 0 { return Err(ErrorKind::GuardError("not at whitespace-noeol")) };
        // If the last character is '\r' (13), check if the next one
        // is '\n' (10).  If so, rewind by one character.
        if (ws.last() == Some(&13)) & (buf.peek() == Some(10)) {
            buf.decr_cursor();
        }
        Ok(())
    }
}

// Comments

pub struct Comment;
impl ParsleyParser for Comment {
    type T = ();

    // The buffer should be positioned at the '%'; it consumes upto
    // and including end-of-line or upto end-of-buffer.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        if !(buf.peek() == Some(37)) { return Err(ErrorKind::GuardError("not at comment")) }
        let _ = buf.parse_bytes_until("\n".as_bytes())?;
        if buf.peek() == Some(10) { buf.incr_cursor(); }
        Ok(())
    }
}

// Keyword matcher

pub struct Keyword {
    matcher: BinaryMatcher
}
impl Keyword {
    pub fn new(word: &[u8]) -> Keyword {
        Keyword { matcher: BinaryMatcher::new(word) }
    }
}
impl ParsleyParser for Keyword {
    type T = ();

    // The buffer should be positioned at the start of the keyword for a successful match.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        &self.matcher.parse(buf)?;
        Ok(())
    }
}

// Booleans are almost keywords, except that they have a semantic
// value.  Include Null here since it is an explicit PDF object.

pub struct Boolean;
impl ParsleyParser for Boolean {
    type T = bool;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let mut b = buf.skip_prefix("true".as_bytes())?;
        if b { return Ok(true) };
        b = buf.skip_prefix("false".as_bytes())?;
        if b { return Ok(false) };
        Err(ErrorKind::GuardError("not at boolean"))
    }
}

pub struct Null;
impl ParsleyParser for Null {
    type T = ();

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let null = buf.skip_prefix("null".as_bytes())?;
        if null { return Ok(()) };
        Err(ErrorKind::GuardError("not at null"))
    }
}

// Number objects, both integers and reals.

#[derive(Debug, PartialEq)]
pub enum NumberT {
    Integer(i64),
    Real(i64, i64) // rational number representation: (numerator, denominator)
}

impl NumberT {
    // constructors for each variant
    pub fn new_integer(i: i64) -> NumberT {
        NumberT::Integer(i)
    }

    pub fn new_real(n: i64, d: i64) -> NumberT {
        NumberT::Real(n, d)
    }

    // predicates for each variant
    pub fn is_integer(&self) -> bool {
        match &self {
            NumberT::Integer(_) => true,
            NumberT::Real(_,_)  => false
        }
    }
    pub fn is_real(&self) -> bool {
        match &self {
            NumberT::Integer(_) => false,
            NumberT::Real(_,_)  => true
        }
    }
    pub fn is_positive(&self) -> bool {
        match &self {
            NumberT::Integer(i) => i >= &0,
            NumberT::Real(n, _d) => n >= &0
        }
    }
    pub fn is_zero(&self) -> bool {
        match &self {
            NumberT::Integer(i) => i == &0,
            NumberT::Real(n, _d) => n == &0
        }
    }
}

pub struct Number;
impl ParsleyParser for Number {
    type T = NumberT;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        let minus =
            if buf.peek() == Some(45) {        // '-'
                buf.incr_cursor();
                true
            } else if buf.peek() == Some(43) { // '+'
                buf.incr_cursor();
                false
            } else {
                false
            };
        let num_str = buf.parse_allowed_bytes("0123456789".as_bytes())?;
        if (num_str.len() == 0) && (buf.peek() != Some(46)) {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("not at number"))
        }
        let mut num : i64 = 0;
        for c in num_str.iter() {
            num = num * 10 + i64::from(c - 48);
        }
        if buf.peek() == Some(46) {            // '.'
            let mut den : i64 = 1;
            buf.incr_cursor();
            let s = buf.parse_allowed_bytes("0123456789".as_bytes());
            if let Ok(den_str) = s {
                for c in den_str.iter() {
                    num = num * 10 + i64::from(c - 48);
                    den *= 10;
                }
            }
            if minus { num *= -1; }
            Ok(NumberT::Real(num, den))
        } else {
            if minus { num *= -1; }
            Ok(NumberT::Integer(num))
        }
    }
}

// Representation does not include the demarcating brackets.
pub struct HexString;
impl ParsleyParser for HexString {
    type T = String;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        if buf.peek() != Some(60) {
            return Err(ErrorKind::GuardError("not at hex string"))
        };
        buf.incr_cursor();
        let bytes = buf.parse_allowed_bytes("0123456789abcdefABCDEF".as_bytes())?;
        if buf.peek() != Some(62) {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("not at valid hex string"))
        }
        buf.incr_cursor();

        let mut s = String::new();
        for b in bytes.iter() { s.push(char::from(*b)); }
        if s.len() % 2 != 0 { s.push('0'); }
        Ok(s)
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
    type T = Vec<u8>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        if buf.peek() != Some(40) { // '('
            return Err(ErrorKind::GuardError("not at literal string"))
        };

        let mut v = Vec::new();
        let mut depth = 1;
        buf.incr_cursor();

        loop {
            let bytes = buf.parse_bytes_until("()".as_bytes())?;
            match buf.peek() {
                Some(40) => { // '('
                    buf.incr_cursor();
                    if let Some(last) = bytes.last() {
                        let escaped = *last == 92; // '\' escaped '('
                        v.extend_from_slice(&bytes);
                        v.extend_from_slice("(".as_bytes());
                        if !escaped { depth += 1 }
                    } else {
                        depth += 1;      // unescaped '('
                        v.extend_from_slice(&bytes);
                        v.extend_from_slice("(".as_bytes());
                    }
                },
                Some(41) => { // ')'
                    buf.incr_cursor();
                    if let Some(last) = bytes.last() {
                        let escaped = *last == 92;
                        v.extend_from_slice(&bytes);
                        if !escaped {
                            depth -= 1;
                            if depth == 0 { break }
                        }
                        v.extend_from_slice(")".as_bytes());
                    } else {
                        v.extend_from_slice(&bytes);
                        depth -= 1;
                        if depth == 0 { break; }
                        v.extend_from_slice(")".as_bytes());
                    }
                },
                Some(_) => {
                    // can never happen
                    panic!("unexpected lit string");
                }
                None => {
                    buf.set_cursor(cursor);
                    return Err(ErrorKind::EndOfBuffer)
                }
            }
        }
        Ok(v)
    }
}

// Raw names: does not perform normalization or UTF decoding, and
// the representation does not include the leading '/'.
pub struct RawName;
impl ParsleyParser for RawName {
    type T = Vec<u8>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        if buf.peek() != Some(47) {
            return Err(ErrorKind::GuardError("not at name object"))
        }
        buf.incr_cursor();

        // terminated by whitespace or delimiter characters.  empty
        // names are considered valid.
        let v = buf.parse_bytes_until(" \0\t\r\n\x0c()<>[]{}/%".as_bytes())?;
        Ok(v)
    }
}

// Stream object content
pub struct StreamContent {
    len: usize
}

impl StreamContent {
    pub fn new(len: usize) -> StreamContent {
        StreamContent { len }
    }
}

impl ParsleyParser for StreamContent {
    // This involves a copy from the parsebuffer, which is
    // inefficient.  We should add an interface that returns a
    // reference, but this runs into the same issues as the
    // BinaryBuffer parser: viz, parameterizing lifetimes on
    // associated types.
    type T = Vec<u8>;

    // This assumes that the whitespace before the 'stream' keyword
    // has been consumed.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        if !buf.skip_prefix("stream".as_bytes()).unwrap() {
            return Err(ErrorKind::GuardError("not at stream content"))
        }
        if buf.peek() == Some(13) { // '\r'
            buf.incr_cursor();
        }
        if buf.peek() == Some(10) { // '\n'
            buf.incr_cursor();
        } else {
            // TODO: check if the '\n' is a requirement.
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("not a valid stream marker"))
        }
        let content_res = buf.extract(self.len);
        if let Err(e) = content_res {
            buf.set_cursor(cursor);
            return Err(e)
        }
        let v = Vec::from(content_res.unwrap());
        if buf.peek() == Some(13) { // '\r'
            buf.incr_cursor();
        }
        if buf.peek() == Some(10) { // '\n'
            buf.incr_cursor();
        }
        if !buf.skip_prefix("endstream".as_bytes()).unwrap() {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid endstream"))
        }
        Ok(v)
    }
}

#[cfg(test)]
mod test_pdf_prim {
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
    use super::{WhitespaceNoEOL, WhitespaceEOL, Comment, Boolean, Null};
    use super::{Number, NumberT};
    use super::{HexString, RawLiteralString, RawName, StreamContent};
    use super::{Keyword};

    #[test]
    fn noeol() {
        let mut ws = WhitespaceNoEOL;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Err(ErrorKind::GuardError("not at whitespace-noeol")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 1);
    }

    #[test]
    fn eol() {
        let mut ws = WhitespaceEOL;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Err(ErrorKind::GuardError("not at whitespace-eol")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    fn comment() {
        let mut com = Comment;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Err(ErrorKind::GuardError("not at comment")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("% ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("% \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test]
    fn boolean() {
        let mut bl = Boolean;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Err(ErrorKind::GuardError("not at boolean")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Err(ErrorKind::GuardError("not at boolean")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("true".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Ok(true));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("false ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Ok(false));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn null() {
        let mut null = Null;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Err(ErrorKind::GuardError("not at null")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Err(ErrorKind::GuardError("not at null")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("null".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("null ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 4);
    }
    #[test]
    fn integer() {
        let mut int = Number;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" 1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("-".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("+".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(1)));
        assert!(i.unwrap().is_integer());
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(23)));
        assert!(i.unwrap().is_integer());
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("+23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(23)));
        assert!(i.unwrap().is_positive());
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("-23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(-23)));
        assert!(!i.unwrap().is_positive());
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(0)));
        assert!(i.unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("-0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(0)));
        assert!(i.unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("+0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = int.parse(&mut pb);
        assert_eq!(i, Ok(NumberT::Integer(0)));
        assert!(i.unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 2);
    }

    #[test]
    fn real() {
        let mut real = Number;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(real.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" 1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(real.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("-".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(real.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("+".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(real.parse(&mut pb), Err(ErrorKind::GuardError("not at number")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Integer(1)));
        assert!(! r.unwrap().is_real());
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("23.01 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Real(2301, 100)));
        assert!(r.unwrap().is_real());
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("+23.10 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Real(2310, 100)));
        assert!(r.unwrap().is_positive());
        assert_eq!(pb.get_cursor(), 6);

        let v = Vec::from("-23.10 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Real(-2310, 100)));
        assert!(!r.unwrap().is_positive());
        assert_eq!(pb.get_cursor(), 6);

        let v = Vec::from("0.0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Real(0, 10)));
        assert!(r.unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("+0.0 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Real(0, 10)));
        assert!(r.unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("-0.00 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let r = real.parse(&mut pb);
        assert_eq!(r, Ok(NumberT::Real(0, 100)));
        assert!(r.unwrap().is_zero());
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn hex_string() {
        let mut hex = HexString;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Err(ErrorKind::GuardError("not at hex string")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Err(ErrorKind::GuardError("not at hex string")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("< ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Err(ErrorKind::GuardError("not at valid hex string")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("<> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb).unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("<1a9> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Ok(String::from("1a90")));
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("<1a90> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Ok(String::from("1a90")));
        assert_eq!(pb.get_cursor(), 6);

        let v = Vec::from("<1a9q> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(hex.parse(&mut pb), Err(ErrorKind::GuardError("not at valid hex string")));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn raw_lit_string() {
        let mut lit = RawLiteralString;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Err(ErrorKind::GuardError("not at literal string")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Err(ErrorKind::GuardError("not at literal string")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("( ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("() ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb).unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("()) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb).unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 2);
        assert_eq!(lit.parse(&mut pb), Err(ErrorKind::GuardError("not at literal string")));
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("(()) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("()")));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("(() ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("(1a9) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("1a9".as_bytes())));
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("(1a(9)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("1a(9)0".as_bytes())));
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a(90) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("(1a9)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("1a9".as_bytes())));
        assert_eq!(pb.get_cursor(), 5);

        let v = Vec::from("(1a\\(90) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("1a\\(90".as_bytes())));
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a9\\)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("1a9\\)0".as_bytes())));
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("(1a\\(90\\)) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(lit.parse(&mut pb), Ok(Vec::from("1a\\(90\\)".as_bytes())));
        assert_eq!(pb.get_cursor(), 10);
    }

    #[test]
    fn raw_name() {
        let mut name = RawName;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb), Err(ErrorKind::GuardError("not at name object")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb), Err(ErrorKind::GuardError("not at name object")));
        assert_eq!(pb.get_cursor(), 0);

        let v = Vec::from("/ ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb).unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("/{ ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb).unwrap().len(), 0);
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("/1a9) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb), Ok(Vec::from("1a9".as_bytes())));
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from("/(1a(9)0) ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(name.parse(&mut pb), Ok(Vec::from("".as_bytes())));
        assert_eq!(pb.get_cursor(), 1);
    }

    #[test]
    fn stream_content() {
        let mut sc = StreamContent::new(2);

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Err(ErrorKind::GuardError("not at stream content")));
        assert_eq!(pb.get_cursor(), 0);

        let mut sc = StreamContent::new(2);
        let v = Vec::from(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Err(ErrorKind::GuardError("not at stream content")));
        assert_eq!(pb.get_cursor(), 0);

        let mut sc = StreamContent::new(2);
        let v = Vec::from("strea ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Err(ErrorKind::GuardError("not at stream content")));
        assert_eq!(pb.get_cursor(), 0);

        let mut sc = StreamContent::new(2);
        let v = Vec::from("stream\n  endstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Ok(Vec::from("  ")));
        assert_eq!(pb.get_cursor(), 18);

        let mut sc = StreamContent::new(2);
        let v = Vec::from("stream\r\n  endstream".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Ok(Vec::from("  ")));
        assert_eq!(pb.get_cursor(), 19);

        let mut sc = StreamContent::new(2);
        let v = Vec::from("stream\r\n  \nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Ok(Vec::from("  ")));
        assert_eq!(pb.get_cursor(), 20);

        let mut sc = StreamContent::new(2);
        let v = Vec::from("stream\r\n  \r\nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Ok(Vec::from("  ")));
        assert_eq!(pb.get_cursor(), 21);

        // wrong length
        let mut sc = StreamContent::new(2);
        let v = Vec::from("stream\r\n   \r\nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Err(ErrorKind::GuardError("invalid endstream")));
        assert_eq!(pb.get_cursor(), 0);

        // wrong eol
        let mut sc = StreamContent::new(2);
        let v = Vec::from("stream\r  \r\nendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Err(ErrorKind::GuardError("not a valid stream marker")));
        assert_eq!(pb.get_cursor(), 0);
        let mut sc = StreamContent::new(2);

        // flexible endstream eol
        let v = Vec::from("stream\r\n  \rendstream ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(sc.parse(&mut pb), Ok(Vec::from("  ")));
        assert_eq!(pb.get_cursor(), 20);
    }

    #[test]
    fn keyword() {
        let mut k = Keyword::new("obj".as_bytes());

        let v = Vec::from("obj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(k.parse(&mut pb), Ok(()));

        let v = Vec::from(" obj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_ne!(k.parse(&mut pb), Ok(()));
    }
}
