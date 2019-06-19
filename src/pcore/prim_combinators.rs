/// Basic combinators (sequence, alternation, and Kleene/star closure), and
/// the parsing trait expected by the combinators.

use std::result::Result;
use super::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};

pub struct Sequence<'a, P1: ParsleyParser, P2: ParsleyParser> {
    p1: &'a mut P1,
    p2: &'a mut P2
}

impl<'a, P1, P2> Sequence<'a, P1, P2>
where P1: ParsleyParser,
      P2: ParsleyParser
{
    pub fn new(p1: &'a mut P1, p2: &'a mut P2) -> Sequence<'a, P1, P2> {
        Sequence { p1, p2 }
    }
}

impl<'a, P1: ParsleyParser, P2: ParsleyParser> ParsleyParser for Sequence<'a, P1, P2> {
    type T = (P1::T, P2::T);

    // TODO: fix the name api
    fn name() -> &'static str { "sequence" }

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        let o1 = self.p1.parse(buf);
        if let Err(err) = o1 {
            buf.set_cursor(cursor);
            return Err(err)
        }
        let o1 = o1.unwrap();

        let o2 = self.p2.parse(buf);
        if let Err(err) = o2 {
            buf.set_cursor(cursor);
            return Err(err)
        }
        let o2 = o2.unwrap();

        Ok((o1, o2))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Alt<T1, T2> {
    Left(T1),
    Right(T2),
}

pub struct Alternate<'a, P1: ParsleyParser, P2: ParsleyParser> {
    p1: &'a mut P1,
    p2: &'a mut P2
}

impl<'a, P1, P2> Alternate<'a, P1, P2>
where P1: ParsleyParser,
      P2: ParsleyParser
{
    pub fn new(p1: &'a mut P1, p2: &'a mut P2) -> Alternate<'a, P1, P2> {
        Alternate { p1, p2 }
    }
}

impl<'a, P1: ParsleyParser, P2: ParsleyParser> ParsleyParser for Alternate<'a, P1, P2> {
    type T = Alt<P1::T, P2::T>;

    // TODO: fix the name api
    fn name() -> &'static str { "alternate" }

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let cursor = buf.get_cursor();
        let o1 = self.p1.parse(buf);
        if let Ok(o) = o1 {
            return Ok(Alt::Left(o))
        }

        buf.set_cursor(cursor);
        let o2 = self.p2.parse(buf);
        if let Err(err) = o2 {
            buf.set_cursor(cursor);
            return Err(err)
        }

        let o2 = o2.unwrap();
        Ok(Alt::Right(o2))
    }
}

pub struct Star<'a, P: ParsleyParser> {
    p: &'a mut P
}

impl<'a, P> Star<'a, P>
where P: ParsleyParser
{
    pub fn new(p: &'a mut P) -> Star<'a, P> {
        Star { p }
    }
}

impl<'a, P: ParsleyParser> ParsleyParser for Star<'a, P> {
    type T = Vec<P::T>;

    // TODO: fix the name api
    fn name() -> &'static str { "star" }

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let mut c = buf.get_cursor();
        let mut v = Vec::new();

        let mut r = self.p.parse(buf);
        while let Ok(o) = r {
            v.push(o);
            c = buf.get_cursor();
            r = self.p.parse(buf)
        }

        buf.set_cursor(c);
        Ok(v)
    }
}

// combinator tests

#[cfg(test)]
mod test_sequence {
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrim, ParsleyParser, ErrorKind};
    use super::super::prim_ascii::{AsciiChar, AsciiCharPrim};
    use super::{Sequence};

    #[test]
    pub fn empty() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Sequence::new(&mut ap, &mut bp);

        let mut pb  = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb);
        let e = Err(ErrorKind::EndOfBuffer);
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    pub fn pre_invalid() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Sequence::new(&mut ap, &mut bp);

        let mut v   = Vec::new();
        v.push(67);  // 'C'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = Err(ErrorKind::GuardError(<AsciiCharPrim as ParsleyPrim>::name()));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);

    }

    #[test]
    pub fn mid_invalid() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Sequence::new(&mut ap, &mut bp);

        let mut v   = Vec::new();
        v.push(65);  // 'A'
        v.push(67);  // 'C'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = Err(ErrorKind::GuardError(<AsciiCharPrim as ParsleyPrim>::name()));
        assert_eq!(r, e);
        // the cursor should not advance for partial matches
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    pub fn valid() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Sequence::new(&mut ap, &mut bp);

        let mut v   = Vec::new();
        v.push(65);  // 'A'
        v.push(66);  // 'B'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        assert_eq!(r, Ok(('A', 'B')));
        // the cursor should have advanced past the match
        assert_eq!(pb.get_cursor(), 2);
    }
}

#[cfg(test)]
mod test_alternate {
    use super::super::parsebuffer::{ParseBuffer, ParsleyPrim, ParsleyParser, ErrorKind};
    use super::super::prim_ascii::{AsciiChar, AsciiCharPrim};
    use super::{Alternate, Alt};

    #[test]
    pub fn empty() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Alternate::new(&mut ap, &mut bp);

        let mut pb  = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb);
        let e = Err(ErrorKind::EndOfBuffer);
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    pub fn pre_invalid() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Alternate::new(&mut ap, &mut bp);

        let mut v   = Vec::new();
        v.push(67);  // 'C'
        v.push(65);  // 'A'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = Err(ErrorKind::GuardError(<AsciiCharPrim as ParsleyPrim>::name()));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    pub fn left_valid() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Alternate::new(&mut ap, &mut bp);

        let mut v   = Vec::new();
        v.push(65);  // 'A'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        assert_eq!(r, Ok(Alt::Left('A')));
        assert_eq!(pb.get_cursor(), 1);
    }

    #[test]
    pub fn right_valid() {
        let mut ap  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp  = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Alternate::new(&mut ap, &mut bp);

        let mut v   = Vec::new();
        v.push(66);  // 'B'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        assert_eq!(r, Ok(Alt::Right('B')));
        assert_eq!(pb.get_cursor(), 1);
    }
}

#[cfg(test)]
mod test_star {
    use super::super::parsebuffer::{ParseBuffer, ParsleyParser};
    use super::super::prim_ascii::{AsciiChar};
    use super::{Star};

    #[test]
    pub fn empty() {
        let mut p   = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut seq = Star::new(&mut p);

        let mut pb  = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb);
        // We should get an empty vector
        let e : Vec<char> = vec![];
        assert_eq!(r, Ok(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    pub fn valid() {
        let mut p   = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut seq = Star::new(&mut p);

        let mut v   = Vec::new();
        v.push(65);  // 'A'
        v.push(65);  // 'A'
        v.push(65);  // 'A'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e : Vec<char> = vec!['A', 'A', 'A'];
        assert_eq!(r, Ok(e));
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    pub fn valid_trailer() {
        let mut p   = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut seq = Star::new(&mut p);

        let mut v   = Vec::new();
        v.push(65);  // 'A'
        v.push(65);  // 'A'
        v.push(65);  // 'A'
        v.push(66);  // 'B'

        let mut pb  = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e : Vec<char> = vec!['A', 'A', 'A'];
        assert_eq!(r, Ok(e));
        assert_eq!(pb.get_cursor(), 3);
    }
}
