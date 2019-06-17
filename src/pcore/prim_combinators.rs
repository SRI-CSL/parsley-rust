/// Basic combinators (sequence, alternation, and Kleene/star closure), and
/// the parsing trait expected by the combinators.

use std::result::Result;
use super::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};

pub fn sequence<'a, O1: ParsleyParser<'a>, O2: ParsleyParser<'a>>(buf: &mut ParseBuffer, f: &mut O1, g: &mut O2) ->
    Result <(O1::T, O2::T), ErrorKind<'a>>
{
    let cursor = buf.get_cursor();
    let o1 = f.parse(buf);
    if let Err(err) = o1 {
        buf.set_cursor(cursor);
        return Err(err)
    }
    let o1 = o1.unwrap();
    let o2 = g.parse(buf);
    if let Err(err) = o2 {
        buf.set_cursor(cursor);
        return Err(err)
    }
    let o2 = o2.unwrap();
    Ok((o1, o2))
}

pub enum Alt<T1, T2> {
    Left(T1),
    Right(T2),
}

pub fn alternate<'a, O1: ParsleyParser<'a>, O2: ParsleyParser<'a>>(buf: &mut ParseBuffer, f: &mut O1, g: &mut O2) ->
    Result <Alt<O1::T, O2::T>, ErrorKind<'a>>
{
    let cursor = buf.get_cursor();
    let o1 = f.parse(buf);
    if let Ok(o) = o1 {
        return Ok(Alt::Left(o))
    }
    buf.set_cursor(cursor);
    let o2 = g.parse(buf);
    if let Err(err) = o2 {
        buf.set_cursor(cursor);
        return Err(err)
    }
    let o2 = o2.unwrap();
    Ok(Alt::Right(o2))
}

pub fn star<'a, O: ParsleyParser<'a>>(buf: &mut ParseBuffer, p: &mut O) ->
    Result <Vec<O::T>, ErrorKind<'a>>
{
    let mut c = buf.get_cursor();
    let mut v = Vec::new();
    let mut r = p.parse(buf);
    while let Ok(o) = r {
        v.push(o);
        c = buf.get_cursor();
        r = p.parse(buf)
    }
    buf.set_cursor(c);
    Ok(v)
}
