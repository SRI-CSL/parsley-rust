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

// File structure of PDF

use std::convert::TryFrom;
use std::fmt;
use std::io::Read; // for read_to_string()

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser,
};
use super::pdf_obj::{DictP, DictT, IndirectP, IndirectT, PDFObjContext};
use super::pdf_prim::{Comment, IntegerP, WhitespaceEOL, WhitespaceNoEOL};
use super::pdf_streams::{XrefEntStatus, XrefEntT};

#[derive(Debug, PartialEq)]
pub struct HeaderT {
    version: LocatedVal<Vec<u8>>,
    binary:  Option<LocatedVal<Vec<u8>>>,
}

impl HeaderT {
    pub fn version(&self) -> &LocatedVal<Vec<u8>> { &self.version }
    pub fn binary(&self) -> &Option<LocatedVal<Vec<u8>>> { &self.binary }
}

pub struct HeaderP;

impl ParsleyParser for HeaderP {
    type T = LocatedVal<HeaderT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut c = Comment;

        let start = buf.get_cursor();
        let version = c.parse(buf)?;
        let binary = if let Ok(s) = c.parse(buf) {
            Some(s)
        } else {
            None
        };
        let end = buf.get_cursor();
        Ok(LocatedVal::new(HeaderT { version, binary }, start, end))
    }
}

struct XrefEntP {
    ent_idx: usize,
}

impl XrefEntP {
    pub fn new(ent_idx: usize) -> XrefEntP { XrefEntP { ent_idx } }
}

impl ParsleyParser for XrefEntP {
    type T = LocatedVal<XrefEntT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<LocatedVal<XrefEntT>> {
        let start = buf.get_cursor();

        // Due to borrow checking of the mutable buf, each extracted
        // segment has to be processed completely before the next
        // extraction.

        // The offset (inuse case) or next object (free case) is 10 digits.
        let mut inf = buf.extract(10)?;
        let mut infs = String::new();
        if let Err(e) = inf.read_to_string(&mut infs) {
            let msg = format!("bad xref offset: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        // check if all bytes read are digits
        if infs.matches(&mut |c: char| c.is_ascii_digit()).count() != 10 {
            let msg = format!("bad format for xref offset: {}", infs);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let info = usize::from_str_radix(&infs, 10);
        if let Err(e) = info {
            let msg = format!("bad xref offset conversion: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let info = info.unwrap();

        // Single space seperator.
        let _ = buf.exact(b" ")?;

        // The generation number is 5 digits.
        let mut gen = buf.extract(5)?;
        let mut gens = String::new();
        if let Err(e) = gen.read_to_string(&mut gens) {
            let msg = format!("bad xref generation: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        // check if all bytes read are digits
        if gens.matches(&mut |c: char| c.is_ascii_digit()).count() != 5 {
            let err = ErrorKind::GuardError("bad format for xref generation".to_string());
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let gen = usize::from_str_radix(&gens, 10);
        if let Err(e) = gen {
            let msg = format!("bad xref generation: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let gen = gen.unwrap();
        // Check generation constraint
        if gen > 65535 {
            let msg = format!("invalid xref generation: {}", gen);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }

        // Single space seperator.
        let _ = buf.exact(b" ")?;

        // Entry type is a single character.
        let flg = buf.extract(1)?;

        let in_use = match flg[0] {
            102 => false, // 'f'
            110 => true,  // 'n'
            _ => {
                let err = ErrorKind::GuardError("bad xref code".to_string());
                let end = buf.get_cursor();
                return Err(locate_value(err, start, end))
            },
        };

        // Xrefent-specific EOL.
        let eol = buf.extract(2)?;
        if eol != b" \r" && eol != b" \n" && eol != b"\r\n" {
            let err = ErrorKind::GuardError("bad eol gen".to_string());
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }

        let status = if in_use {
            XrefEntStatus::InUse { file_ofs: info }
        } else {
            XrefEntStatus::Free { next: info }
        };

        let ent = XrefEntT::new(self.ent_idx, gen, status);
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ent, start, end))
    }
}

#[derive(Debug, PartialEq)]
pub struct XrefSubSectT {
    start: usize,
    count: usize,
    ents:  Vec<LocatedVal<XrefEntT>>,
}

impl XrefSubSectT {
    pub fn start(&self) -> usize { self.start }
    pub fn count(&self) -> usize { self.count }
    pub fn ents(&self) -> &[LocatedVal<XrefEntT>] { self.ents.as_slice() }
}

struct XrefSubSectP;

impl ParsleyParser for XrefSubSectP {
    type T = LocatedVal<XrefSubSectT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<LocatedVal<XrefSubSectT>> {
        let start = buf.get_cursor();

        // The spec is not clear whether there is any leading or
        // trailing whitespace on the first line of a subsection.
        // Assume there can be possibly empty leading whitespace for now.
        let mut ws = WhitespaceNoEOL::new(true);
        ws.parse(buf)?;

        let mut int = IntegerP;
        let xstart = usize::try_from(int.parse(buf)?.val().int_val());
        if let Err(e) = xstart {
            let msg = format!("conversion error on xref-subsect start: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let xstart = xstart.unwrap();

        let _ = buf.exact(b" ")?;
        let xcount = usize::try_from(int.parse(buf)?.val().int_val());
        if let Err(e) = xcount {
            let msg = format!("conversion error on xref-subsect count: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let xcount = xcount.unwrap();

        // Again, no clarity on the type of EOL terminating this line.
        // Assume we need one for now.
        let mut ws = WhitespaceEOL::new(false);
        ws.parse(buf)?;

        // Now get the specified number of entries.
        let mut ents = Vec::new();
        for idx in 0 .. xcount {
            let mut p = XrefEntP::new(xstart + idx);
            let ent = p.parse(buf)?;
            // Object 0 should always be free; we don't check it here,
            // but in a separate validation pass, which is easier
            // since xref table validity is a non-local property.
            ents.push(ent);
        }

        let end = buf.get_cursor();
        Ok(LocatedVal::new(
            XrefSubSectT {
                start: xstart,
                count: xcount,
                ents,
            },
            start,
            end,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct XrefSectT {
    sects: Vec<LocatedVal<XrefSubSectT>>,
}

#[derive(Debug, PartialEq)]
pub enum InvalidXrefSect {
    ObjectNotFree(usize), // in-use object that is on free linked list
    BadDeadObject(usize), // invalid link or generation
    NonCircularFreeList,  // invalid tail pointer
}

impl fmt::Display for InvalidXrefSect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ObjectNotFree(n) => write!(f, "in-use object {} is on xref free list", n),
            Self::BadDeadObject(n) => write!(
                f,
                "free object {} not on free list is not a valid dead object",
                n
            ),
            Self::NonCircularFreeList => write!(f, "xref free entry list is not circular"),
        }
    }
}

pub struct XrefSectEntIterator<'a> {
    subsect: usize,
    idx:     usize,
    xref:    &'a XrefSectT,
}

impl XrefSectT {
    pub fn sects(&self) -> &[LocatedVal<XrefSubSectT>] { self.sects.as_slice() }

    pub fn ents(&self) -> Vec<LocatedVal<XrefEntT>> {
        let mut ents = Vec::new();
        for (_, ent) in self.ent_iter() {
            ents.push(ent)
        }
        ents
    }

    pub fn ent_iter(&self) -> XrefSectEntIterator {
        XrefSectEntIterator {
            subsect: 0,
            idx:     0,
            xref:    self,
        }
    }

    pub fn is_valid(&self) -> Option<InvalidXrefSect> {
        // There are various non-local top-level validity constraints:
        //
        // . The first entry should be free and the head of the free
        //   entry linked list.
        // . This linked list should be circular, with the tail
        //   pointing to the 0'th entry.
        // . Each free entry not on the linked list should be dead,
        //   i,e, have gen == 65535, and a link to entry 0.
        //
        // TODO: this assumes a linearly ordered linked list, which is
        // the most common case. Generalize this later.
        let mut next_free = 0;
        for (o, ent) in self.ent_iter() {
            assert!(ent.val().obj() == o);
            if o == next_free {
                match ent.val().status() {
                    XrefEntStatus::InUse { .. } => return Some(InvalidXrefSect::ObjectNotFree(o)),
                    XrefEntStatus::Free { next } => {
                        next_free = *next;
                        continue
                    },
                    XrefEntStatus::InStream { .. } => {
                        // This should never happen in xref tables.
                        assert!(false)
                    },
                }
            }
            if !ent.val().in_use() {
                // This should be a dead object.
                match (ent.val().gen(), ent.val().status()) {
                    (65535, XrefEntStatus::Free { next: 0 }) => (),
                    _ => return Some(InvalidXrefSect::BadDeadObject(o)),
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
    type Item = (usize, LocatedVal<XrefEntT>); // object number and its entry

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.subsect >= self.xref.sects().len() {
                break
            }
            let ss = self.xref.sects()[self.subsect].val();
            if self.idx >= ss.count() {
                self.subsect += 1;
                self.idx = 0;
                continue
            }
            let ent = ss.ents()[self.idx];
            let obj = ss.start() + self.idx;
            self.idx += 1;
            return Some((obj, ent))
        }
        None
    }
}

pub struct XrefSectP;

impl ParsleyParser for XrefSectP {
    type T = LocatedVal<XrefSectT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        // First, consume possibly empty whitespace.  TODO: Check with
        // the upcoming update to the standard, intimated at the
        // hackathon.
        let mut ws = WhitespaceEOL::new(true);
        ws.parse(buf)?;

        if let Err(e) = buf.exact(b"xref") {
            let msg = format!("not at xref: {}", e.val());
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        // Since the xref subsections follow this line, there is an
        // implied EOL.
        let mut ws = WhitespaceEOL::new(false);
        ws.parse(buf)?;

        // There is no specified terminator for an xref section, so
        // keep consuming xref subsections until we have an error.
        let mut sects = Vec::new();
        loop {
            let mut p = XrefSubSectP;
            let sect = p.parse(buf);
            if let Err(e) = sect {
                // If this is an error on the first subsection, report
                // this error as the result.
                if sects.len() == 0 {
                    return Err(e)
                }
                break
            }
            sects.push(sect.unwrap());
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(XrefSectT { sects }, start, end))
    }
}

#[derive(Debug, PartialEq)]
pub struct BodyT {
    objs: Vec<LocatedVal<IndirectT>>,
}

impl BodyT {
    pub fn objs(&self) -> &[LocatedVal<IndirectT>] { self.objs.as_slice() }
}

pub struct BodyP<'a> {
    ctxt: &'a mut PDFObjContext,
}

impl BodyP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> BodyP<'a> { BodyP { ctxt } }
}

impl ParsleyParser for BodyP<'_> {
    type T = LocatedVal<BodyT>;

    // PDF documents will almost never be parsed in this linear
    // fashion, but will instead be parsed by seeking to offsets
    // specified in the xref table.  Nevertheless, this function can
    // be used with simple files for debugging purposes.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        // We can only find indirect objects at the top-level.
        let mut op = IndirectP::new(&mut self.ctxt);
        let mut objs = Vec::new();
        // In the simplest case, we just terminate when we can't parse
        // any more objects.
        loop {
            let o = op.parse(buf);
            if let Err(_) = o {
                break
            }
            let o = o.unwrap();
            objs.push(o);
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(BodyT { objs }, start, end))
    }
}

#[derive(Debug, PartialEq)]
pub struct TrailerT {
    dict: DictT,
}

impl TrailerT {
    pub fn dict(&self) -> &DictT { &self.dict }
}

pub struct TrailerP<'a> {
    ctxt: &'a mut PDFObjContext,
}

impl TrailerP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> TrailerP<'a> { TrailerP { ctxt } }
}

impl ParsleyParser for TrailerP<'_> {
    type T = LocatedVal<TrailerT>;

    // This assumes we are positioned at 'trailer'.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        if let Err(e) = buf.exact(b"trailer") {
            let msg = format!("not at trailer: {}", e.val());
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let mut ws = WhitespaceEOL::new(true); // optional whitespace
        ws.parse(buf)?;

        let mut dp = DictP::new(&mut self.ctxt);
        let dict = dp.parse(buf);
        if let Err(e) = dict {
            let msg = format!("error parsing trailer dictionary: {}", e.val());
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let dict = dict.unwrap();
        let end = buf.get_cursor();
        Ok(LocatedVal::new(TrailerT { dict }, start, end))
    }
}

#[derive(Debug, PartialEq)]
pub struct StartXrefT {
    offset: usize,
}

impl StartXrefT {
    pub fn offset(&self) -> usize { self.offset }
}

pub struct StartXrefP;

impl ParsleyParser for StartXrefP {
    type T = LocatedVal<StartXrefT>;

    // This assumes we are positioned at 'startxref', which is
    // typically found by scanning backwards from EOF.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        if let Err(e) = buf.exact(b"startxref") {
            let msg = format!("not at startxref: {}", e.val());
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let mut ws = WhitespaceEOL::new(false); // need to consume an EOL
        ws.parse(buf)?;

        let mut int = IntegerP;
        let offset = usize::try_from(int.parse(buf)?.val().int_val());
        if let Err(e) = offset {
            let msg = format!("conversion error on startxref: {}", e);
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
        }
        let offset = offset.unwrap();
        let end = buf.get_cursor();
        Ok(LocatedVal::new(StartXrefT { offset }, start, end))
    }
}

#[cfg(test)]
mod test_pdf_file {
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use super::super::super::pcore::parsebuffer::{
        locate_value, ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParsleyParser,
    };
    use super::super::super::pdf_lib::pdf_obj::{DictT, PDFObjContext, PDFObjT, ReferenceT};
    use super::super::super::pdf_lib::pdf_prim::{IntegerT, NameT};
    use super::super::pdf_streams::XrefEntStatus;
    use super::InvalidXrefSect;
    use super::{BodyP, BodyT, HeaderP, HeaderT, StartXrefP, StartXrefT, TrailerP, TrailerT};
    use super::{XrefEntP, XrefEntT, XrefSectP, XrefSectT, XrefSubSectP, XrefSubSectT};

    #[test]
    fn test_header() {
        let mut p = HeaderP;

        //                 012345678901234567890
        let v = Vec::from("%PDF-1.0 \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let hdr = HeaderT {
            version: LocatedVal::new(Vec::from("PDF-1.0 \r".as_bytes()), 0, 11),
            binary:  None,
        };
        assert_eq!(val, Ok(LocatedVal::new(hdr, 0, 11)));
        assert_eq!(pb.get_cursor(), 11);
        //                 01234567890123456789012345678
        let v = Vec::from("%PDF-1.0 \r\n%binary_bytes\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let hdr = HeaderT {
            version: LocatedVal::new(Vec::from("PDF-1.0 \r".as_bytes()), 0, 11),
            binary:  Some(LocatedVal::new(
                Vec::from("binary_bytes".as_bytes()),
                11,
                25,
            )),
        };
        assert_eq!(val, Ok(LocatedVal::new(hdr, 0, 25)));
        assert_eq!(pb.get_cursor(), 25);
    }

    #[test]
    fn test_xref_ent() {
        let mut p = XrefEntP::new(0);
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let status = XrefEntStatus::Free { next: 1234567890 };
        let xref = XrefEntT::new(0, 12345, status);
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 12345 n \n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let status = XrefEntStatus::InUse {
            file_ofs: 1234567890,
        };
        let xref = XrefEntT::new(0, 12345, status);
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 12345 f \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let status = XrefEntStatus::Free { next: 1234567890 };
        let xref = XrefEntT::new(0, 12345, status);
        assert_eq!(ent, Ok(LocatedVal::new(xref, 0, 20)));

        // bad eol
        let v = Vec::from("1234567890 12345 f  \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let e = locate_value(ErrorKind::GuardError("bad eol gen".to_string()), 0, 0);
        assert_eq!(ent, Err(e));

        // bad generation
        //                 01234567890123456789012345678
        let v = Vec::from("1234567890 65536 f \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        assert_eq!(
            ent,
            Err(LocatedVal::new(
                ErrorKind::GuardError("invalid xref generation: 65536".to_string()),
                0,
                16
            ))
        );
    }

    #[test]
    fn test_xref_subsect() {
        let mut p = XrefSubSectP;
        //                 01234567890123456789012345678
        let v = Vec::from("0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let status = XrefEntStatus::Free { next: 1234567890 };
        let xref = LocatedVal::new(XrefEntT::new(0, 12345, status), 4, 24);
        let s = XrefSubSectT {
            start: 0,
            count: 1,
            ents:  vec![xref],
        };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 24)));

        // leading and trailing space on leading line
        //                 01234567890123456789012345678901
        let v = Vec::from(" 0 1 \r\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let status = XrefEntStatus::Free { next: 1234567890 };
        let xref = LocatedVal::new(XrefEntT::new(0, 12345, status), 7, 27);
        let s = XrefSubSectT {
            start: 0,
            count: 1,
            ents:  vec![xref],
        };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 27)));
    }

    #[test]
    fn test_xref_sect() {
        let mut p = XrefSectP;
        //                           1         2         3
        //                 0123456789012345678901234567890123
        let v = Vec::from("xref\n0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let status = XrefEntStatus::Free { next: 1234567890 };
        let xref = LocatedVal::new(XrefEntT::new(0, 12345, status), 9, 29);
        let ssect = LocatedVal::new(
            XrefSubSectT {
                start: 0,
                count: 1,
                ents:  vec![xref],
            },
            5,
            29,
        );
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 29)));
        assert_eq!(
            val.unwrap().val().is_valid(),
            Some(InvalidXrefSect::NonCircularFreeList)
        );

        // different leading eol, and terminate with trailer
        //                           1         2         3         4
        //                 0123456789012345678901234567890123456789012
        let v = Vec::from("xref\r\n0 1\n1234567890 12345 f\r\ntrailer".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let status = XrefEntStatus::Free { next: 1234567890 };
        let xref = LocatedVal::new(XrefEntT::new(0, 12345, status), 10, 30);
        let ssect = LocatedVal::new(
            XrefSubSectT {
                start: 0,
                count: 1,
                ents:  vec![xref],
            },
            6,
            30,
        );
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(LocatedVal::new(s, 0, 30)));
        assert_eq!(
            val.unwrap().val().is_valid(),
            Some(InvalidXrefSect::NonCircularFreeList)
        );

        // snippet from hello-world pdf.
        let v = Vec::from(
            "xref
0 6
0000000000 65535 f 
0000000010 00000 n 
0000000079 00000 n 
0000000173 00000 n 
0000000301 00000 n 
0000000380 00000 n 
"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);

        let mut ents = Vec::new();
        let status = XrefEntStatus::Free { next: 0 };
        ents.push(LocatedVal::new(XrefEntT::new(0, 65535, status), 9, 29));
        let status = XrefEntStatus::InUse { file_ofs: 10 };
        ents.push(LocatedVal::new(XrefEntT::new(1, 0, status), 29, 49));
        let status = XrefEntStatus::InUse { file_ofs: 79 };
        ents.push(LocatedVal::new(XrefEntT::new(2, 0, status), 49, 69));
        let status = XrefEntStatus::InUse { file_ofs: 173 };
        ents.push(LocatedVal::new(XrefEntT::new(3, 0, status), 69, 89));
        let status = XrefEntStatus::InUse { file_ofs: 301 };
        ents.push(LocatedVal::new(XrefEntT::new(4, 0, status), 89, 109));
        let status = XrefEntStatus::InUse { file_ofs: 380 };
        ents.push(LocatedVal::new(XrefEntT::new(5, 0, status), 109, 129));
        let ssect = LocatedVal::new(
            XrefSubSectT {
                start: 0,
                count: 6,
                ents,
            },
            5,
            129,
        );
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
"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb).unwrap();

        // quick and dirty test.
        let BodyT { objs } = val.unwrap();
        assert_eq!(objs.len(), 5);
        for (idx, o) in objs.iter().enumerate() {
            assert_eq!(idx + 1, o.val().num());
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
endobj"
                .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb).unwrap();

        // quick and dirty test.
        let BodyT { objs } = val.unwrap();
        assert_eq!(objs.len(), 5);
        for (idx, o) in objs.iter().enumerate() {
            assert_eq!(idx + 1, o.val().num());
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
"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);

        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Size".as_bytes()));
        map.insert(
            LocatedVal::new(key, 12, 17).val().normalize(),
            Rc::new(LocatedVal::new(PDFObjT::Integer(IntegerT::new(8)), 18, 19)),
        );
        let key = NameT::new(Vec::from("Root".as_bytes()));
        map.insert(
            LocatedVal::new(key, 21, 26).val().normalize(),
            Rc::new(LocatedVal::new(
                PDFObjT::Reference(ReferenceT::new(1, 0)),
                27,
                32,
            )),
        );
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
