// Basic primitives for PDF.

use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};

pub struct WhitespaceEOL;
impl ParsleyParser for WhitespaceEOL {
    type T = ();

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.parse_allowed_bytes(" \0\t\r\n\x0c".as_bytes())?;
        Ok(())
    }
}

pub struct WhitespaceNoEOL;
impl ParsleyParser for WhitespaceNoEOL {
    type T = ();

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let ws = buf.parse_allowed_bytes(" \0\t\r\x0c".as_bytes())?;
        // If the last character is '\r' (13), check if the next one
        // is '\n' (10).  If so, rewind by one character.
        if (ws.last() == Some(&13)) & (buf.peek() == Some(10)) {
            buf.decr_cursor();
        }
        Ok(())
    }
}

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

pub struct Integer;
impl ParsleyParser for Integer {
    type T = i64;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        let minus =
            if buf.peek() == Some(45) {
                buf.incr_cursor();
                true
            } else {
                false
            };
        let num_str = buf.parse_allowed_bytes("0123456789".as_bytes())?;
        if num_str.len() == 0 {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("not at integer"))
        }
        let mut num : i64 = 0;
        for c in num_str.iter() {
            num = num * 10 + i64::from(c - 48);
        }
        if minus { num *= -1; }
        Ok(num)
    }
}

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


#[cfg(test)]
mod test_pdf_prim {
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
    use super::{WhitespaceNoEOL, WhitespaceEOL, Comment, Integer, Boolean, Null};

    #[test]
    fn noeol() {
        let mut ws = WhitespaceNoEOL;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 3);

        let mut v = Vec::new();
        v.extend_from_slice(" \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 1);
    }

    #[test]
    fn eol() {
        let mut ws = WhitespaceEOL;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice(" \r ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(ws.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 3);

        let mut v = Vec::new();
        v.extend_from_slice(" \r\n".as_bytes());
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

        let mut v = Vec::new();
        v.extend_from_slice("% ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 2);

        let mut v = Vec::new();
        v.extend_from_slice("% \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(com.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test]
    fn integer() {
        let mut int = Integer;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at integer")));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at integer")));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice("-".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Err(ErrorKind::GuardError("not at integer")));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice("1".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Ok(1));
        assert_eq!(pb.get_cursor(), 1);

        let mut v = Vec::new();
        v.extend_from_slice("23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Ok(23));
        assert_eq!(pb.get_cursor(), 2);

        let mut v = Vec::new();
        v.extend_from_slice("-23 ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(int.parse(&mut pb), Ok(-23));
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    fn boolean() {
        let mut bl = Boolean;

        let v = Vec::new();
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Err(ErrorKind::GuardError("not at boolean")));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Err(ErrorKind::GuardError("not at boolean")));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice("true".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(bl.parse(&mut pb), Ok(true));
        assert_eq!(pb.get_cursor(), 4);

        let mut v = Vec::new();
        v.extend_from_slice("false ".as_bytes());
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

        let mut v = Vec::new();
        v.extend_from_slice(" ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Err(ErrorKind::GuardError("not at null")));
        assert_eq!(pb.get_cursor(), 0);

        let mut v = Vec::new();
        v.extend_from_slice("null".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 4);

        let mut v = Vec::new();
        v.extend_from_slice("null ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(null.parse(&mut pb), Ok(()));
        assert_eq!(pb.get_cursor(), 4);
    }
}
