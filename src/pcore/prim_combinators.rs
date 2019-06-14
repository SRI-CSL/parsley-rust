/// Basic combinators (sequence, alternation, and Kleene/star closure), and
/// the parsing trait expected by the combinators.

use super::pbuf;
use std::result::Result;

#[derive(Debug)]
pub enum ErrorKind {
    // TODO
}

pub trait ParsleyParser<O> {
    fn parse(&mut self, buf: &mut pbuf::ParseBuffer) -> Result<O, ErrorKind>;
}

pub fn sequence<O1, O2, F, G>(buf: &mut pbuf::ParseBuffer, f: &mut F, g: &mut G) ->
    Result <(O1, O2), ErrorKind>
where
    F : ParsleyParser<O1>,
    G : ParsleyParser<O2>,
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

pub enum Alt<O1, O2> {
    Left(O1),
    Right(O2),
}

pub fn alternate<O1, O2, F, G>(buf: &mut pbuf::ParseBuffer, f: &mut F, g: &mut G) ->
    Result <Alt<O1, O2>, ErrorKind>
where
    F : ParsleyParser<O1>,
    G : ParsleyParser<O2>,
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

pub fn star<O>(buf: &mut pbuf::ParseBuffer, p: &mut ParsleyParser<O>) ->
    Result <Vec<O>, ErrorKind>
{
    let mut v = Vec::new();
    let mut r = p.parse(buf);
    while let Ok(o) = r {
        v.push(o);
        r = p.parse(buf)
    }
    Ok(v)
}
