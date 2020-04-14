// Copyright (c) 2019-2020 SRI International.
// All rights reserved.
//
//    This file is part of the Parsley parser.
//
//    Parsley is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    Parsley is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Basic combinators (sequence, alternation, and Kleene/star closure).

use super::parsebuffer::{ParseBufferT, ParsleyParser, LocatedVal, ParseResult, ErrorKind, make_error};

pub struct Sequence<'a, P1: ParsleyParser, P2: ParsleyParser> {
    p1: &'a mut P1,
    p2: &'a mut P2,
}

impl<'a, P1, P2> Sequence<'a, P1, P2>
    where P1: ParsleyParser,
          P2: ParsleyParser
{
    pub fn new(p1: &'a mut P1, p2: &'a mut P2) -> Sequence<'a, P1, P2> {
        Sequence { p1, p2 }
    }
}

impl<'a, P1: ParsleyParser, P2: ParsleyParser> ParsleyParser for Sequence<'a, P1, P2>
    where P1::T: PartialEq,
          P2::T: PartialEq
{
    type T = LocatedVal<(P1::T, P2::T)>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let o1 = self.p1.parse(buf);
        if let Err(err) = o1 {
            buf.set_cursor(start);
            return Err(err)
        }
        let o1 = o1.unwrap();

        let o2 = self.p2.parse(buf);
        if let Err(err) = o2 {
            buf.set_cursor(start);
            return Err(err)
        }
        let o2 = o2.unwrap();
        let end = buf.get_cursor();

        Ok(LocatedVal::new((o1, o2), start, end))
    }
}

#[derive(Debug, PartialEq)]
pub enum Alt<T1, T2> {
    Left(T1),
    Right(T2),
}

pub struct Alternate<'a, P1: ParsleyParser, P2: ParsleyParser> {
    p1: &'a mut P1,
    p2: &'a mut P2,
}

impl<'a, P1, P2> Alternate<'a, P1, P2>
    where P1: ParsleyParser,
          P2: ParsleyParser
{
    pub fn new(p1: &'a mut P1, p2: &'a mut P2) -> Alternate<'a, P1, P2> {
        Alternate { p1, p2 }
    }
}

impl<'a, P1: ParsleyParser, P2: ParsleyParser> ParsleyParser for Alternate<'a, P1, P2>
    where P1::T: PartialEq,
          P2::T: PartialEq
{
    type T = LocatedVal<Alt<P1::T, P2::T>>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let o1 = self.p1.parse(buf);
        if let Ok(o) = o1 {
            let end = buf.get_cursor();
            return Ok(LocatedVal::new(Alt::Left(o), start, end))
        }

        buf.set_cursor(start);
        let o2 = self.p2.parse(buf);
        if let Err(err) = o2 {
            buf.set_cursor(start);
            return Err(err)
        }

        let o2 = o2.unwrap();
        let end = buf.get_cursor();
        Ok(LocatedVal::new(Alt::Right(o2), start, end))
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

impl<'a, P: ParsleyParser> ParsleyParser for Star<'a, P>
    where P::T: PartialEq
{
    type T = LocatedVal<Vec<P::T>>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut c = start;
        let mut v = Vec::new();

        let mut r = self.p.parse(buf);
        while let Ok(o) = r {
            v.push(o);
            c = buf.get_cursor();
            r = self.p.parse(buf)
        }

        buf.set_cursor(c);
        let end = c;
        Ok(LocatedVal::new(v, start, end))
    }
}

pub struct Not<'a, P: ParsleyParser> {
    p: &'a mut P
}

impl<'a, P> Not<'a, P>
    where P: ParsleyParser
{
    pub fn new(p: &'a mut P) -> Not<'a, P> {
        Not { p }
    }
}

impl<'a, P: ParsleyParser> ParsleyParser for Not<'a, P>
    where P::T: PartialEq
{
    type T = LocatedVal<()>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let r = self.p.parse(buf);
        let end = buf.get_cursor();
        buf.set_cursor(start);

        if let Ok(_) = r {
            Err(make_error(ErrorKind::GuardError("not".to_string()), start, end))
        } else {
            Ok(LocatedVal::new((), start, start))
        }
    }
}

// combinator tests

#[cfg(test)]
mod test_sequence {
    use super::super::parsebuffer::{ParseBuffer, ParseBufferT, ParsleyPrimitive, ParsleyParser,
                                    ErrorKind, make_error};
    use super::super::prim_ascii::{AsciiChar, AsciiCharPrimitive};
    use super::Sequence;

    #[test]
    pub fn test() {
        let mut ap = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Sequence::new(&mut ap, &mut bp);

        // empty
        let mut pb = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb);
        let e = Err(make_error(ErrorKind::EndOfBuffer, 0, 0));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);

        // pre invalid
        let mut v = Vec::new();
        v.extend_from_slice(b"C");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name().to_string());
        let e = Err(make_error(e, 0, 0));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);

        // mid invalid
        let mut v = Vec::new();
        v.extend_from_slice(b"AC");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name().to_string());
        let e = Err(make_error(e, 1, 1));
        assert_eq!(r, e);
        // the cursor should not advance for partial matches
        assert_eq!(pb.get_cursor(), 0);

        // valid
        let mut v = Vec::new();
        v.extend_from_slice(b"AB");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb).unwrap();
        let r = r.unwrap();
        assert_eq!((*r.0.val(), *r.1.val()), ('A', 'B'));
        // the cursor should have advanced past the match
        assert_eq!(pb.get_cursor(), 2);
    }
}

#[cfg(test)]
mod test_alternate {
    use super::super::parsebuffer::{ParseBuffer, ParseBufferT, ParsleyPrimitive, ParsleyParser,
                                    LocatedVal, ErrorKind, make_error};
    use super::super::prim_ascii::{AsciiChar, AsciiCharPrimitive};
    use super::{Alternate, Alt};

    #[test]
    pub fn test() {
        let mut ap = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut bp = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut seq = Alternate::new(&mut ap, &mut bp);

        // empty
        let mut pb = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb);
        let e = Err(make_error(ErrorKind::EndOfBuffer, 0, 0));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);

        // pre invalid
        let mut v = Vec::new();
        v.extend_from_slice(b"CA");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name().to_string());
        let e = Err(make_error(e, 0, 0));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);

        // left valid
        let mut v = Vec::new();
        v.extend_from_slice(b"A");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb).unwrap();
        assert_eq!(*r.val(), Alt::Left(LocatedVal::new('A', 0, 1)));
        assert_eq!(pb.get_cursor(), 1);

        // right valid
        let mut v = Vec::new();
        v.extend_from_slice(b"B");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb).unwrap();
        assert_eq!(*r.val(), Alt::Right(LocatedVal::new('B', 0, 1)));
        assert_eq!(pb.get_cursor(), 1);
    }
}

#[cfg(test)]
mod test_star {
    use super::super::parsebuffer::{ParseBuffer, ParseBufferT, ParsleyParser, LocatedVal};
    use super::super::prim_ascii::AsciiChar;
    use super::Star;

    #[test]
    pub fn guarded() {
        let mut p = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut seq = Star::new(&mut p);

        // empty
        let mut pb = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb).unwrap();
        // We should get an empty vector
        let e: Vec<LocatedVal<char>> = vec![];
        assert_eq!(*r.val(), e);
        assert_eq!(pb.get_cursor(), 0);

        // valid
        let mut v = Vec::new();
        v.extend_from_slice(b"AAA");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = vec![LocatedVal::new('A', 0, 1),
                     LocatedVal::new('A', 1, 2),
                     LocatedVal::new('A', 2, 3)];
        assert_eq!(r, Ok(LocatedVal::new(e, 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        // valid with trailer
        let mut v = Vec::new();
        v.extend_from_slice(b"AAAB");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = vec![LocatedVal::new('A', 0, 1),
                     LocatedVal::new('A', 1, 2),
                     LocatedVal::new('A', 2, 3)];
        assert_eq!(r, Ok(LocatedVal::new(e, 0, 3)));
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    pub fn all() {
        let mut p = AsciiChar::new();
        let mut seq = Star::new(&mut p);

        let mut v = Vec::new();
        v.extend_from_slice(b"AAAB");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let e = vec![LocatedVal::new('A', 0, 1),
                     LocatedVal::new('A', 1, 2),
                     LocatedVal::new('A', 2, 3),
                     LocatedVal::new('B', 3, 4)];
        assert_eq!(r, Ok(LocatedVal::new(e, 0, 4)));
        assert_eq!(pb.get_cursor(), 4);
    }
}

#[cfg(test)]
mod test_combined {
    use super::super::parsebuffer::{ParseBuffer, ParseBufferT, ParsleyParser, ParsleyPrimitive,
                                    LocatedVal, ErrorKind, make_error};
    use super::super::prim_ascii::{AsciiChar, AsciiCharPrimitive};
    use super::{Star, Sequence, Alternate, Alt, Not};

    #[test] // a*b*
    pub fn astar_bstar() {
        let mut a = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut astar = Star::new(&mut a);
        let mut b = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut bstar = Star::new(&mut b);
        let mut seq = Sequence::new(&mut astar, &mut bstar);

        // empty
        let mut pb = ParseBuffer::new(Vec::new());
        let r = seq.parse(&mut pb);
        // We should get a tuple of empty matches.
        let a = LocatedVal::new(vec![], 0, 0);
        let b = LocatedVal::new(vec![], 0, 0);
        assert_eq!(r, Ok(LocatedVal::new((a, b), 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        // only a
        let mut v = Vec::new();
        v.extend_from_slice(b"AAA");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let a = LocatedVal::new(vec![LocatedVal::new('A', 0, 1),
                                     LocatedVal::new('A', 1, 2),
                                     LocatedVal::new('A', 2, 3)], 0, 3);
        let b = LocatedVal::new(vec![], 3, 3);
        assert_eq!(r, Ok(LocatedVal::new((a, b), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        // only b
        let mut v = Vec::new();
        v.extend_from_slice(b"BBBA");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let a = LocatedVal::new(vec![], 0, 0);
        let b = LocatedVal::new(vec![LocatedVal::new('B', 0, 1),
                                     LocatedVal::new('B', 1, 2),
                                     LocatedVal::new('B', 2, 3)], 0, 3);
        assert_eq!(r, Ok(LocatedVal::new((a, b), 0, 3)));
        assert_eq!(pb.get_cursor(), 3);

        // a then b
        let mut v = Vec::new();
        v.extend_from_slice(b"AAABBB");
        let mut pb = ParseBuffer::new(v);
        let r = seq.parse(&mut pb);
        let a = LocatedVal::new(vec![LocatedVal::new('A', 0, 1),
                                     LocatedVal::new('A', 1, 2),
                                     LocatedVal::new('A', 2, 3)], 0, 3);
        let b = LocatedVal::new(vec![LocatedVal::new('B', 3, 4),
                                     LocatedVal::new('B', 4, 5),
                                     LocatedVal::new('B', 5, 6)], 3, 6);
        assert_eq!(r, Ok(LocatedVal::new((a, b), 0, 6)));
        assert_eq!(pb.get_cursor(), 6);
    }

    #[test] // (a|b)*
    pub fn a_or_b_star() {
        let mut a = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut a_or_b = Alternate::new(&mut a, &mut b);
        let mut star = Star::new(&mut a_or_b);

        // empty
        let mut pb = ParseBuffer::new(Vec::new());
        let r = star.parse(&mut pb);
        // We should get a tuple of empty matches.
        assert_eq!(r, Ok(LocatedVal::new(vec![], 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"BAABC");
        let mut pb = ParseBuffer::new(v);
        let r = star.parse(&mut pb);
        let v = vec![LocatedVal::new(Alt::Right(LocatedVal::new('B', 0, 1)), 0, 1),
                     LocatedVal::new(Alt::Left(LocatedVal::new('A', 1, 2)), 1, 2),
                     LocatedVal::new(Alt::Left(LocatedVal::new('A', 2, 3)), 2, 3),
                     LocatedVal::new(Alt::Right(LocatedVal::new('B', 3, 4)), 3, 4)];
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 4)));
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test] // (ab)*
    pub fn a_b_star() {
        let mut a = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut a_b = Sequence::new(&mut a, &mut b);
        let mut star = Star::new(&mut a_b);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"ABABC");
        let mut pb = ParseBuffer::new(v);
        let r = star.parse(&mut pb);
        let v = vec![LocatedVal::new((LocatedVal::new('A', 0, 1),
                                      LocatedVal::new('B', 1, 2)),
                                     0, 2),
                     LocatedVal::new((LocatedVal::new('A', 2, 3),
                                      LocatedVal::new('B', 3, 4)),
                                     2, 4)];
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 4)));
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test] // a*|b*
    pub fn a_star_or_b_star() {
        let mut a = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut astar = Star::new(&mut a);
        let mut bstar = Star::new(&mut b);
        let mut abs = Alternate::new(&mut astar, &mut bstar);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"AABBC");
        let mut pb = ParseBuffer::new(v);
        let r = abs.parse(&mut pb);
        let v = Alt::Left(LocatedVal::new(vec![LocatedVal::new('A', 0, 1),
                                               LocatedVal::new('A', 1, 2)], 0, 2));
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 2)));
        assert_eq!(pb.get_cursor(), 2);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"BBAAC");
        let mut pb = ParseBuffer::new(v);
        let r = abs.parse(&mut pb);
        let v = Alt::Left(LocatedVal::new(vec![], 0, 0));
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 0)));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test] // (ab)|(ba)
    pub fn ab_or_ba() {
        let mut a1 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b1 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut a2 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b2 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut ab = Sequence::new(&mut a1, &mut b1);
        let mut ba = Sequence::new(&mut b2, &mut a2);
        let mut p = Alternate::new(&mut ab, &mut ba);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"ABABC");
        let mut pb = ParseBuffer::new(v);
        let r = p.parse(&mut pb);
        let v = Alt::Left(LocatedVal::new((LocatedVal::new('A', 0, 1),
                                           LocatedVal::new('B', 1, 2)), 0, 2));
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 2)));
        assert_eq!(pb.get_cursor(), 2);
        let r = p.parse(&mut pb);
        let v = Alt::Left(LocatedVal::new((LocatedVal::new('A', 2, 3),
                                           LocatedVal::new('B', 3, 4)), 2, 4));
        assert_eq!(r, Ok(LocatedVal::new(v, 2, 4)));
        assert_eq!(pb.get_cursor(), 4);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"BABAC");
        let mut pb = ParseBuffer::new(v);
        let r = p.parse(&mut pb);
        let v = Alt::Right(LocatedVal::new((LocatedVal::new('B', 0, 1),
                                            LocatedVal::new('A', 1, 2)), 0, 2));
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 2)));
        assert_eq!(pb.get_cursor(), 2);
        let r = p.parse(&mut pb);
        let v = Alt::Right(LocatedVal::new((LocatedVal::new('B', 2, 3),
                                            LocatedVal::new('A', 3, 4)), 2, 4));
        assert_eq!(r, Ok(LocatedVal::new(v, 2, 4)));
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test] // (a|b)(b|a)
    pub fn aorb_bora() {
        let mut a1 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b1 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut a2 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b2 = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut ab = Alternate::new(&mut a1, &mut b1);
        let mut ba = Alternate::new(&mut b2, &mut a2);
        let mut p = Sequence::new(&mut ab, &mut ba);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"ABABC");
        let mut pb = ParseBuffer::new(v);
        let r = p.parse(&mut pb);
        let v = (LocatedVal::new(Alt::Left(LocatedVal::new('A', 0, 1)), 0, 1),
                 LocatedVal::new(Alt::Left(LocatedVal::new('B', 1, 2)), 1, 2));
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 2)));
        assert_eq!(pb.get_cursor(), 2);
        let r = p.parse(&mut pb);
        let v = (LocatedVal::new(Alt::Left(LocatedVal::new('A', 2, 3)), 2, 3),
                 LocatedVal::new(Alt::Left(LocatedVal::new('B', 3, 4)), 3, 4));
        assert_eq!(r, Ok(LocatedVal::new(v, 2, 4)));
        assert_eq!(pb.get_cursor(), 4);
        let r = p.parse(&mut pb);
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name().to_string());
        let e = Err(make_error(e, 4, 4));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 4);

        // match
        let mut v = Vec::new();
        v.extend_from_slice(b"BAABC");

        let mut pb = ParseBuffer::new(v);
        let r = p.parse(&mut pb);
        let v = (LocatedVal::new(Alt::Right(LocatedVal::new('B', 0, 1)), 0, 1),
                 LocatedVal::new(Alt::Right(LocatedVal::new('A', 1, 2)), 1, 2));
        assert_eq!(r, Ok(LocatedVal::new(v, 0, 2)));
        assert_eq!(pb.get_cursor(), 2);
        let r = p.parse(&mut pb);
        let v = (LocatedVal::new(Alt::Left(LocatedVal::new('A', 2, 3)), 2, 3),
                 LocatedVal::new(Alt::Left(LocatedVal::new('B', 3, 4)), 3, 4));
        assert_eq!(r, Ok(LocatedVal::new(v, 2, 4)));
        assert_eq!(pb.get_cursor(), 4);
        let r = p.parse(&mut pb);
        let e = ErrorKind::GuardError(<AsciiCharPrimitive as ParsleyPrimitive>::name().to_string());
        let e = Err(make_error(e, 4, 4));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 4);
    }

    #[test] // !(a|b)
    pub fn not() {
        let mut a = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
        let mut b = AsciiChar::new_guarded(Box::new(|c: &char| *c == 'B'));
        let mut a_or_b = Alternate::new(&mut a, &mut b);
        let mut not = Not::new(&mut a_or_b);

        // empty
        let mut pb = ParseBuffer::new(Vec::new());
        let r = not.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new((), 0, 0)));
        assert_eq!(pb.get_cursor(), 0);

        // match-case
        let mut v = Vec::new();
        v.extend_from_slice(b"B");
        let mut pb = ParseBuffer::new(v);
        let r = not.parse(&mut pb);
        let e = Err(make_error(ErrorKind::GuardError("not".to_string()), 0, 0));
        assert_eq!(r, e);
        assert_eq!(pb.get_cursor(), 0);

        // not-case
        let mut v = Vec::new();
        v.extend_from_slice(b"C");
        let mut pb = ParseBuffer::new(v);
        let r = not.parse(&mut pb);
        assert_eq!(r, Ok(LocatedVal::new((), 0, 0)));
        assert_eq!(pb.get_cursor(), 0);
    }
}
