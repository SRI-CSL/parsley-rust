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

/// File structure of PDF

use std::io::Read;
// for read_to_string()
use std::convert::TryFrom;
use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, LocatedVal,
                                       ParseResult, ErrorKind, make_error};
use super::pdf_prim::{IntegerP, WhitespaceEOL, WhitespaceNoEOL, Comment};
use super::pdf_obj::{PDFObjContext, PDFObjP, PDFObjT, DictT, DictP};

// This doesn't yet fully specify a legal version string.
//
// Header h { version: typeof(CommentObj),
//            binary: option(typeof(CommentObj)) } :=
//
//    v=CommentObj b=(CommentObj)?
//    // '?' is a builtin that creates an option-type

#[derive(Debug, PartialEq)]
pub struct HeaderT {
    version: LocatedVal<Vec<u8>>,
    binary: Option<LocatedVal<Vec<u8>>>,
}

impl HeaderT {
    pub fn version(&self) -> &LocatedVal<Vec<u8>> { &self.version }
    pub fn binary(&self) -> &Option<LocatedVal<Vec<u8>>> { &self.binary }
}

pub struct HeaderP;

impl ParsleyParser for HeaderP {
    type T = LocatedVal<HeaderT>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let mut c = Comment;

        let start = buf.get_cursor();
        let version = c.parse(buf)?;
        let binary = if let Ok(s) = c.parse(buf) { Some(s) } else { None };
        let end = buf.get_cursor();
        Ok(LocatedVal::new(HeaderT { version, binary }, start, end))
    }
}

// XrefEnt x { offset: int, gen: int, status: xfree_t } :=
//     o=[[digit]*10] g=[[digit]*5]
//     { x.offset = $string_to_int(o);
//       x.gen    = $string_to_int(g);
//       x.status = Inuse
//     }
//     ( 'n' { x.status = Inuse }
//     | 'f' { x.status = Free  }
//     )
//     '\r\n' // This is required to be 'a two-character end-of-line
//            // sequence'.
//     ;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct XrefEntT {
    info: usize,
    gen: usize,
    in_use: bool
}

impl XrefEntT {
    pub fn info(&self)   -> usize { self.info }
    pub fn gen(&self)    -> usize { self.gen }
    pub fn in_use(&self) -> bool  { self.in_use }

    pub fn is_valid(&self) -> bool {
        self.gen() <= 65535
    }
}

struct XrefEntP;

impl XrefEntP {
    fn parse(&self, buf: &mut ParseBuffer) -> ParseResult<LocatedVal<XrefEntT>> {
        let start = buf.get_cursor();

        // Due to borrow checking of the mutable buf, each extracted
        // segment has to be processed completely before the next
        // extraction.

        // The offset (inuse case) or next object (free case) is 10 digits.
        let mut inf = buf.extract(10)?;
        let mut infs = String::new();
        if let Err(_) = inf.read_to_string(&mut infs) {
            let err = ErrorKind::GuardError("bad xref ofs".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        // check if all bytes read are digits
        if infs.matches(&mut |c: char| { c.is_ascii_digit() }).count() != 10 {
            let err = ErrorKind::GuardError("bad format for xref ofs".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let info = usize::from_str_radix(&infs, 10);
        if let Err(_) = info {
            let err = ErrorKind::GuardError("bad xref ofs conversion".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let info = info.unwrap();

        // Single space seperator.
        let _ = buf.exact(" ".as_bytes())?;

        // The generation number is 5 digits.
        let mut gen = buf.extract(5)?;
        let mut gens = String::new();
        if let Err(_) = gen.read_to_string(&mut gens) {
            let err = ErrorKind::GuardError("bad xref gen".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        // check if all bytes read are digits
        if gens.matches(&mut |c: char| { c.is_ascii_digit() }).count() != 5 {
            let err = ErrorKind::GuardError("bad format for xref gen".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let gen = usize::from_str_radix(&gens, 10);
        if let Err(_) = gen {
            let err = ErrorKind::GuardError("bad xref gen".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let gen = gen.unwrap();

        // Single space seperator.
        let _ = buf.exact(" ".as_bytes())?;

        // Entry type is a single character.
        let flg = buf.extract(1)?;

        let in_use = match flg[0] {
            102 => false,  // 'f'
            110 => true,   // 'n'
            _ => {
                let err = ErrorKind::GuardError("bad xref code".to_string());
                let end = buf.get_cursor();
                return Err(make_error(err, start, end));
            }
        };

        // Xrefent-specific EOL.
        let eol = buf.extract(2)?;
        if eol != " \r".as_bytes() && eol != " \n".as_bytes() && eol != "\r\n".as_bytes() {
            let err = ErrorKind::GuardError("bad eol gen".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }

        let ent = XrefEntT { info, gen, in_use };
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ent, start, end))
    }
}

// XrefSubSect x { start: int, count: int, ents: [XrefEnt] } :=
//
//    s=IntegerObj [ s.val >= 0 ]
//    c=IntegerObj [ s.val >  0 ]
//
//   '\n'  // EOL-type is not specified in the spec.
//
//   { x.start := s.val;
//     x.count := c.val }
//
//   ( e=XrefEnt [ x.ents.len() < x.count() ] { x.ents.append(e) } )* ;

#[derive(Debug, PartialEq)]
pub struct XrefSubSectT {
    start: usize,
    count: usize,
    ents: Vec<LocatedVal<XrefEntT>>,
}

#[derive(Debug, PartialEq)]
pub enum InvalidXrefSubSect {
    Entry(usize),
    Count(usize,usize),
}

impl XrefSubSectT {
    pub fn start(&self) -> usize { self.start }
    pub fn count(&self) -> usize { self.count }
    pub fn ents(&self)  -> &[LocatedVal<XrefEntT>] { self.ents.as_slice() }

    pub fn is_valid(&self) -> Option<InvalidXrefSubSect> {
        for (i, e) in self.ents().iter().enumerate() {
            if e.val().is_valid() { continue }
            return Some(InvalidXrefSubSect::Entry(i))
        }
        // Each subsection should have the advertized number of entries.
        if self.ents.len() != self.count {
            return Some(InvalidXrefSubSect::Count(self.ents.len(), self.count))
        }
        None
    }
}

struct XrefSubSectP;

impl XrefSubSectP {
    fn parse(&self, buf: &mut ParseBuffer) -> ParseResult<LocatedVal<XrefSubSectT>> {
        let start = buf.get_cursor();

        // The spec is not clear whether there is any leading or
        // trailing whitespace on the first line of a subsection.
        // Assume there can be possibly empty leading whitespace for now.
        let mut ws = WhitespaceNoEOL::new(true);
        ws.parse(buf)?;

        let mut int = IntegerP;
        let xstart = usize::try_from(int.parse(buf)?.val().int_val());
        if let Err(_) = xstart {
            let err = ErrorKind::GuardError("conversion error on xref-subsect start".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let xstart = xstart.unwrap();

        let _ = buf.exact(" ".as_bytes())?;
        let xcount = usize::try_from(int.parse(buf)?.val().int_val());
        if let Err(_) = xcount {
            let err = ErrorKind::GuardError("conversion error on xref-subsect count".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let xcount = xcount.unwrap();

        // Again, no clarity on the type of EOL terminating this line.
        // Assume we need one for now.
        let mut ws = WhitespaceEOL::new(false);
        ws.parse(buf)?;

        // Now get the specified number of entries.
        let p = XrefEntP;
        let mut ents = Vec::new();
        for _ in 0..xcount {
            let ent = p.parse(buf)?;
            // TODO: constrain object 0 to always be free
            ents.push(ent);
        }

        let end = buf.get_cursor();
        Ok(LocatedVal::new(XrefSubSectT { start: xstart, count: xcount, ents }, start, end))
    }
}

// XrefSect x { sects: [XrefSubSect] } :=
//
//    'xref' ( s=XrefSubSect { x.sects.append(s) } )* ;

#[derive(Debug, PartialEq)]
pub struct XrefSectT {
    sects: Vec<LocatedVal<XrefSubSectT>>
}

#[derive(Debug, PartialEq)]
pub enum InvalidXrefSect {
    SubSect(usize, InvalidXrefSubSect),  // invalid subsection
    ObjectNotFree(usize),                // in-use object that is on free linked list
    BadDeadObject(usize),                // invalid link or generation
    NonCircularFreeList                  // invalid tail pointer
}

pub struct XrefSectEntIterator<'a> {
    subsect: usize,
    idx: usize,
    xref: &'a XrefSectT
}

impl XrefSectT {
    pub fn sects(&self) -> &[LocatedVal<XrefSubSectT>] { self.sects.as_slice() }

    pub fn ent_iter(&self) -> XrefSectEntIterator {
        XrefSectEntIterator { subsect: 0, idx: 0, xref: self }
    }

    pub fn is_valid(&self) -> Option<InvalidXrefSect> {
        // Each subsection should be valid.
        for (i, ls) in self.sects.iter().enumerate() {
            let ss = ls.val();
            let v = ss.is_valid();
            if v.is_some() {
                return Some(InvalidXrefSect::SubSect(i, v.unwrap()))
            }
        }

        // First entry should be free and the head of the free entry linked list.
        // This linked list should be circular, with the tail pointing to the 0'th entry.
        // Each free entry not on linked list should have gen == 65535, and link to entry 0
        let mut next_free = 0;
        for (o, ent) in self.ent_iter() {
            if o == next_free {
                if ent.in_use() {
                    return Some(InvalidXrefSect::ObjectNotFree(o))
                }
                next_free = ent.info();
                continue;
            }
            if !ent.in_use() {
                // Not on the linked list.
                if ent.gen() != 65535 || ent.info() != 0 {
                    return Some(InvalidXrefSect::BadDeadObject(o))
                }
            }
        }
        if next_free != 0 {
            return Some(InvalidXrefSect::NonCircularFreeList)
        }
        None
    }
}

impl<'a> Iterator for XrefSectEntIterator<'a> {
    type Item = (usize, XrefEntT);  // object number and its entry

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.subsect >= self.xref.sects().len() {
                break;
            }
            let ss = self.xref.sects()[self.subsect].val();
            if self.idx >= ss.count() {
                self.subsect += 1;
                self.idx = 0;
                continue;
            }
            let ent = ss.ents()[self.idx].val();
            let obj = ss.start() + self.idx;
            self.idx += 1;
            return Some((obj, *ent))
        }
        None
    }
}

pub struct XrefSectP;

impl ParsleyParser for XrefSectP {
    type T = LocatedVal<XrefSectT>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        // First, consume possibly empty whitespace.  TODO: Check with
        // the upcoming update to the standard, intimated at the
        // hackathon.
        let mut ws = WhitespaceEOL::new(true);
        ws.parse(buf)?;

        if let Err(_) = buf.exact("xref".as_bytes()) {
            let err = ErrorKind::GuardError("not at xref".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        // Since the xref subsections follow this line, there is an
        // implied EOL.
        let mut ws = WhitespaceEOL::new(false);
        ws.parse(buf)?;

        // There is no specified terminator for an xref section, so
        // keep consuming xref subsections until we have an error.
        let mut sects = Vec::new();
        loop {
            let p = XrefSubSectP;
            let sect = p.parse(buf);
            if let Err(_) = sect { break; }
            sects.push(sect.unwrap());
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(XrefSectT { sects }, start, end))
    }
}

// Body b { objs = [PDFObj] } :=
//
//    ( o=PDFObj [ o ~~ IndirectObj ] { b.objs.append(o) } )* ;

#[derive(Debug, PartialEq)]
pub struct BodyT {
    objs: Vec<LocatedVal<PDFObjT>>
}

impl BodyT {
    pub fn objs(&self) -> &[LocatedVal<PDFObjT>] { self.objs.as_slice() }
}

pub struct BodyP<'a> {
    ctxt: &'a mut PDFObjContext
}

impl BodyP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> BodyP<'a> {
        BodyP { ctxt }
    }
}

impl ParsleyParser for BodyP<'_> {
    type T = LocatedVal<BodyT>;

    // PDF documents will almost never be parsed in this linear
    // fashion, but will instead be parsed by seeking to offsets
    // specified in the xref table.  Nevertheless, this function can
    // be used with simple files for debugging purposes.
    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut op = PDFObjP::new(&mut self.ctxt);
        // The outermost objects should all be indirect objects.  In
        // the simplest case, we just terminate when we can't parse
        // any more objects.
        let mut objs = Vec::new();
        loop {
            let o = op.parse(buf);
            if let Err(_) = o { break; }
            let o = o.unwrap();
            if let PDFObjT::Indirect(_) = o.val() {
                objs.push(o)
            } else {
                let err = ErrorKind::GuardError("non-indirect object in body".to_string());
                let end = buf.get_cursor();
                return Err(make_error(err, start, end));
            }
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(BodyT { objs }, start, end))
    }
}

// Trailer t { dict: DictObj } := 'trailer' d=DictObj { t.dict = d } ;
#[derive(Debug, PartialEq)]
pub struct TrailerT {
    dict: DictT
}

impl TrailerT {
    pub fn dict(&self) -> &DictT { &self.dict }
}

pub struct TrailerP<'a> {
    ctxt: &'a mut PDFObjContext
}

impl TrailerP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> TrailerP<'a> {
        TrailerP { ctxt }
    }
}

impl ParsleyParser for TrailerP<'_> {
    type T = LocatedVal<TrailerT>;

    // This assumes we are positioned at 'trailer'.
    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        if let Err(_) = buf.exact("trailer".as_bytes()) {
            let err = ErrorKind::GuardError("not at trailer".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let mut ws = WhitespaceEOL::new(false); // need to consume an EOL
        ws.parse(buf)?;

        let mut dp = DictP::new(&mut self.ctxt);
        let dict = dp.parse(buf);
        if let Err(_) = dict {
            let err = ErrorKind::GuardError("error parsing trailer dictionary".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let dict = dict.unwrap();
        let end = buf.get_cursor();
        Ok(LocatedVal::new(TrailerT { dict }, start, end))
    }
}

#[derive(Debug, PartialEq)]
pub struct StartXrefT {
    offset: usize
}

impl StartXrefT {
    pub fn offset(&self) -> usize { self.offset }
}

pub struct StartXrefP;

impl ParsleyParser for StartXrefP {
    type T = LocatedVal<StartXrefT>;

    // This assumes we are positioned at 'startxref', which is
    // typically found by scanning backwards from EOF.
    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        if let Err(_) = buf.exact("startxref".as_bytes()) {
            let err = ErrorKind::GuardError("not at startxref".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let mut ws = WhitespaceEOL::new(false); // need to consume an EOL
        ws.parse(buf)?;

        let mut int = IntegerP;
        let offset = usize::try_from(int.parse(buf)?.val().int_val());
        if let Err(_) = offset {
            let err = ErrorKind::GuardError("conversion error on startxref".to_string());
            let end = buf.get_cursor();
            return Err(make_error(err, start, end));
        }
        let offset = offset.unwrap();
        let end = buf.get_cursor();
        Ok(LocatedVal::new(StartXrefT { offset }, start, end))
    }
}

#[cfg(test)]
mod test_pdf_file {
    use std::rc::Rc;
    use std::collections::BTreeMap;
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, LocatedVal,
                                                  ErrorKind, make_error};
    use super::super::super::pdf_lib::pdf_obj::{PDFObjContext, PDFObjT, ReferenceT, DictT};
    use super::super::super::pdf_lib::pdf_prim::IntegerT;
    use super::{HeaderT, HeaderP, BodyT, BodyP, StartXrefT, StartXrefP, TrailerT, TrailerP};
    use super::{XrefEntT, XrefEntP, XrefSubSectT, XrefSubSectP, XrefSectT, XrefSectP};
    use super::{InvalidXrefSect};

    #[test]
    fn test_header() {
        let mut p = HeaderP;

        //                 012345678901234567890
        let v = Vec::from("%PDF-1.0 \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let hdr = HeaderT {
            version: LocatedVal::new(Vec::from("PDF-1.0 \r".as_bytes()), 0, 11),
            binary: None,
        };
        assert_eq!(val, Ok(LocatedVal::new(hdr, 0, 11)));
        assert_eq!(pb.get_cursor(), 11);
        //                 01234567890123456789012345678
        let v = Vec::from("%PDF-1.0 \r\n%binary_bytes\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let hdr = HeaderT {
            version: LocatedVal::new(Vec::from("PDF-1.0 \r".as_bytes()), 0, 11),
            binary: Some(LocatedVal::new(Vec::from("binary_bytes".as_bytes()), 11, 25)),
        };
        assert_eq!(val, Ok(LocatedVal::new(hdr, 0, 25)));
        assert_eq!(pb.get_cursor(), 25);
    }

    #[test]
    fn test_xref_ent() {
        let p = XrefEntP;
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefEntT { info: 1234567890, gen: 12345, in_use: false };
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));
        assert!(ent.unwrap().val().is_valid());
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 12345 n \n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefEntT { info: 1234567890, gen: 12345, in_use: true };
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));
        assert!(ent.unwrap().val().is_valid());
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 12345 f \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefEntT { info: 1234567890, gen: 12345, in_use: false };
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));
        assert!(ent.unwrap().val().is_valid());

        // bad eol
        let v = Vec::from("1234567890 12345 f  \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let e = make_error(ErrorKind::GuardError("bad eol gen".to_string()), 0, 0);
        assert_eq!(ent, Err(e));

        // bad generation
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 65536 f \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefEntT { info: 1234567890, gen: 65536, in_use: false };
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));
        assert!(!ent.unwrap().val().is_valid());
    }

    #[test]
    fn test_xref_subsect() {
        let p = XrefSubSectP;
        //                 01234567890123456789012345678
        let v = Vec::from("0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = LocatedVal::new(XrefEntT { info: 1234567890, gen: 12345, in_use: false }, 4, 24);
        let s = XrefSubSectT { start: 0, count: 1, ents: vec![xref] };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 24)));
        assert!(val.unwrap().val().is_valid().is_none());

        // leading and trailing space on leading line
        //                 01234567890123456789012345678901
        let v = Vec::from(" 0 1 \r\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = LocatedVal::new(XrefEntT { info: 1234567890, gen: 12345, in_use: false }, 7, 27);
        let s = XrefSubSectT { start: 0, count: 1, ents: vec![xref] };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 27)));
        assert!(val.unwrap().val().is_valid().is_none());
    }

    #[test]
    fn test_xref_sect() {
        let mut p = XrefSectP;
        //                           1         2         3
        //                 0123456789012345678901234567890123
        let v = Vec::from("xref\n0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = LocatedVal::new(XrefEntT { info: 1234567890, gen: 12345, in_use: false }, 9, 29);
        let ssect = LocatedVal::new(XrefSubSectT { start: 0, count: 1, ents: vec![xref] }, 5, 29);
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 29)));
        assert_eq!(val.unwrap().val().is_valid(), Some(InvalidXrefSect::NonCircularFreeList));

        // different leading eol, and terminate with trailer
        //                           1         2         3         4
        //                 0123456789012345678901234567890123456789012
        let v = Vec::from("xref\r\n0 1\n1234567890 12345 f\r\ntrailer".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = LocatedVal::new(XrefEntT { info: 1234567890, gen: 12345, in_use: false }, 10, 30);
        let ssect = LocatedVal::new(XrefSubSectT { start: 0, count: 1, ents: vec![xref] }, 6, 30);
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 30)));
        assert_eq!(val.unwrap().val().is_valid(), Some(InvalidXrefSect::NonCircularFreeList));

        // snippet from hello-world pdf.
//01234
//5678
//9
//29
//49
//69
//89
//109
//129
        let v = Vec::from(
            "xref
0 6
0000000000 65535 f 
0000000010 00000 n 
0000000079 00000 n 
0000000173 00000 n 
0000000301 00000 n 
0000000380 00000 n 
".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);

        let mut ents = Vec::new();
        ents.push(LocatedVal::new(XrefEntT { info: 0, gen: 65535, in_use: false }, 9, 29));
        ents.push(LocatedVal::new(XrefEntT { info: 10, gen: 0, in_use: true }, 29, 49));
        ents.push(LocatedVal::new(XrefEntT { info: 79, gen: 0, in_use: true }, 49, 69));
        ents.push(LocatedVal::new(XrefEntT { info: 173, gen: 0, in_use: true }, 69, 89));
        ents.push(LocatedVal::new(XrefEntT { info: 301, gen: 0, in_use: true }, 89, 109));
        ents.push(LocatedVal::new(XrefEntT { info: 380, gen: 0, in_use: true }, 109, 129));
        let ssect = LocatedVal::new(XrefSubSectT { start: 0, count: 6, ents }, 5, 129);
        let s = LocatedVal::new(XrefSectT { sects: vec![ssect] }, 0, 129);
        assert_eq!(val, Ok(s));
        assert!(val.unwrap().val().is_valid().is_none());
    }

    #[test]
    fn test_body_without_comments() {
        let mut ctxt = PDFObjContext::new();
        let mut p = BodyP::new(&mut ctxt);
        // body snippet from hello world, but with embedded comments removed
        let v = Vec::from(
            "1 0 obj
<<
  /Type /Catalog
  /Pages 2 0 R
>>
endobj

2 0 obj
<<
  /Type /Pages
  /MediaBox [ 0 0 200 200 ]
  /Count 1
  /Kids [ 3 0 R ]
>>
endobj

3 0 obj
<<
  /Type /Page
  /Parent 2 0 R
  /Resources <<
    /Font <<
      /F1 4 0 R
    >>
  >>
  /Contents 5 0 R
>>
endobj

4 0 obj
<<
  /Type /Font
  /Subtype /Type1
  /BaseFont /Times-Roman
>>
endobj

5 0 obj
<<
  /Length 44
>>
stream
BT
70 50 TD
/F1 12 Tf
(Hello, world!) Tj
ET
endstream
endobj
".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb).unwrap();

        // quick and dirty test.
        let BodyT { objs } = val.unwrap();
        assert_eq!(objs.len(), 5);
        for o in objs {
            if let PDFObjT::Indirect(_) = o.unwrap() {} else {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_body_with_comments() {
        let mut ctxt = PDFObjContext::new();
        let mut p = BodyP::new(&mut ctxt);
        // original body snippet from hello world (with embedded comments)
        let v = Vec::from(
            "1 0 obj  % entry point
<<
  /Type /Catalog
  /Pages 2 0 R
>>
endobj

2 0 obj
<<
  /Type /Pages
  /MediaBox [ 0 0 200 200 ]
  /Count 1
  /Kids [ 3 0 R ]
>>
endobj

3 0 obj
<<
  /Type /Page
  /Parent 2 0 R
  /Resources <<
    /Font <<
      /F1 4 0 R
    >>
  >>
  /Contents 5 0 R
>>
endobj

4 0 obj
<<
  /Type /Font
  /Subtype /Type1
  /BaseFont /Times-Roman
>>
endobj

5 0 obj  % page content
<<
  /Length 44
>>
stream
BT
70 50 TD
/F1 12 Tf
(Hello, world!) Tj
ET
endstream
endobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb).unwrap();

        // quick and dirty test.
        let BodyT { objs } = val.unwrap();
        assert_eq!(objs.len(), 5);
        for o in objs {
            if let PDFObjT::Indirect(_) = o.unwrap() {} else {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_trailer() {
        let mut ctxt = PDFObjContext::new();
        let mut p = TrailerP::new(&mut ctxt);
//01234567
//890
//123456789
//0123456789012
//345
        let v = Vec::from(
            "trailer
<<
 /Size 8
 /Root 1 0 R
>>
".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);

        let mut map = BTreeMap::new();
        map.insert(LocatedVal::new(Vec::from("Size".as_bytes()), 12, 17),
                   Rc::new(LocatedVal::new(PDFObjT::Integer(IntegerT::new(8)), 18, 19)));
        map.insert(LocatedVal::new(Vec::from("Root".as_bytes()), 21, 26),
                   Rc::new(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 27, 32)));
        let dict = DictT::new(map);
        assert_eq!(val, Ok(LocatedVal::new(TrailerT { dict }, 0, 35)));
    }

    #[test]
    fn test_startxref() {
        let mut p = StartXrefP;
        //                 01234567890123456789012345678
        let v = Vec::from("startxref \n642\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        assert_eq!(val, Ok(LocatedVal::new(StartXrefT { offset: 642 }, 0, 14)));
        //                 0123456789012345678901234567890123
        let v = Vec::from("startxref %absurd comment \n642\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        assert_eq!(val, Ok(LocatedVal::new(StartXrefT { offset: 642 }, 0, 30)));
    }
}
