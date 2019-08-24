// File structure of PDF

use std::io::Read; // for read_to_string()
use std::convert::TryFrom;
use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
use super::pdf_prim::{IntegerP, WhitespaceEOL, WhitespaceNoEOL, Comment};
use super::pdf_obj::{PDFObjP, PDFObjT, DictT, DictP};

// This doesn't yet fully specify a legal version string.
//
// Header h { version: typeof(CommentObj),
//            binary: option(typeof(CommentObj)) } :=
//
//    v=CommentObj b=(CommentObj)?
//    // '?' is a builtin that creates an option-type

#[derive(Debug, PartialEq)]
pub struct HeaderT {
    version: Vec<u8>,
    binary:  Option<Vec<u8>>
}
impl HeaderT {
    pub fn version(&self) -> &[u8] { self.version.as_slice() }
    pub fn binary(&self) -> &Option<Vec<u8>> { &self.binary }
}

pub struct HeaderP;
impl ParsleyParser for HeaderP {
    type T = HeaderT;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let mut c = Comment;

        let version = c.parse(buf)?;
        let binary = if let Ok(s) = c.parse(buf) { Some(s) } else { None };
        Ok(HeaderT { version, binary })
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

#[derive(Debug, PartialEq)]
pub struct XrefT {
    info: u64,
    gen:  u64
}
impl XrefT {
    pub fn info(&self) -> u64 { self.info }
    pub fn gen(&self)  -> u64 { self.gen }
}
#[derive(Debug, PartialEq)]
pub enum XrefEntT {
    Inuse(XrefT),
    Free(XrefT)
}

struct XrefEntP;
impl XrefEntP {
    fn parse(&self, buf: &mut ParseBuffer) -> Result<XrefEntT, ErrorKind> {
        // Due to borrow checking of the mutable buf, each extracted
        // segment has to be processed completely before the next
        // extraction.

        // The offset (inuse case) or next object (free case) is 10 digits.
        let mut inf = buf.extract(10)?;
        let mut infs = String::new();
        if let Err(_) = inf.read_to_string(&mut infs) {
            return Err(ErrorKind::GuardError("bad xref ofs"))
        }
        // check if all bytes read are digits
        if infs.matches(&mut |c: char| { c.is_ascii_digit() }).count() != 10 {
            return Err(ErrorKind::GuardError("bad format for xref ofs"))
        }
        let inf = u64::from_str_radix(&infs, 10);
        if let Err(_) = inf {
            return Err(ErrorKind::GuardError("bad xref ofs conversion"))
        }
        let inf = inf.unwrap();

        // Single space seperator.
        let _   = buf.exact(" ".as_bytes())?;

        // The generation number is 5 digits.
        let mut gen = buf.extract(5)?;
        let mut gens = String::new();
        if let Err(_) = gen.read_to_string(&mut gens) {
            return Err(ErrorKind::GuardError("bad xref gen"))
        }
        // check if all bytes read are digits
        if gens.matches(&mut |c: char| { c.is_ascii_digit() }).count() != 5 {
            return Err(ErrorKind::GuardError("bad format for xref gen"))
        }
        let gen = u64::from_str_radix(&gens, 10);
        if let Err(_) = gen {
            return Err(ErrorKind::GuardError("bad xref gen"))
        }
        let gen = gen.unwrap();

        // Single space seperator.
        let _   = buf.exact(" ".as_bytes())?;

        // Entry type is a single character.
        let flg = buf.extract(1)?;

        let inuse = match flg[0] {
            102 => false,  // 'f'
            110 => true,   // 'n'
            _   => return Err(ErrorKind::GuardError("bad xref code"))
        };

        // Xrefent-specific EOL.
        let eol = buf.extract(2)?;
        if eol != " \r".as_bytes() && eol != " \n".as_bytes() && eol != "\r\n".as_bytes() {
            return Err(ErrorKind::GuardError("bad eol gen"))
        }

        if inuse { Ok(XrefEntT::Inuse(XrefT {info: inf, gen })) }
        else     { Ok(XrefEntT::Free (XrefT {info: inf, gen })) }
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
//      x.count := c.val }
//
//   ( e=XrefEnt [ x.ents.len() < x.count() ] { x.ents.append(e) } )* ;

#[derive(Debug, PartialEq)]
pub struct XrefSubSectT {
    start: u64,
    count: u64,
    ents:  Vec<XrefEntT>
}
impl XrefSubSectT {
    pub fn start(&self) -> u64 { self.start }
    pub fn count(&self) -> u64 { self.count }
    pub fn ents(&self)  -> &[XrefEntT] { self.ents.as_slice() }
}

struct XrefSubSectP;
impl XrefSubSectP {
    fn parse(&self, buf: &mut ParseBuffer) -> Result<XrefSubSectT, ErrorKind> {
        // The spec is not clear whether there is any leading or
        // trailing whitespace on the first line of a subsection.
        // Assume there can be possibly empty leading whitespace for now.
        let mut ws = WhitespaceNoEOL::new(true);
        ws.parse(buf)?;

        let mut int = IntegerP;
        let start = u64::try_from(int.parse(buf)?.int_val());
        if let Err(_) = start {
            return Err(ErrorKind::GuardError("conversion error on xref-subsect start"))
        }
        let start = start.unwrap();

        let _ = buf.exact(" ".as_bytes())?;
        let count = u64::try_from(int.parse(buf)?.int_val());
        if let Err(_) = count {
            return Err(ErrorKind::GuardError("conversion error on xref-subsect count"))
        }
        let count = count.unwrap();

        // Again, no clarity on the type of EOL terminating this line.
        // Assume we need one for now.
        let mut ws = WhitespaceEOL::new(false);
        ws.parse(buf)?;

        // Now get the specified number of entries.
        let p = XrefEntP;
        let mut ents = Vec::new();
        for _ in 0 .. count {
            let ent = p.parse(buf)?;
            // TODO: constrain object 0 to always be free
            ents.push(ent);
        }

        Ok(XrefSubSectT{ start, count, ents })
    }
}

// XrefSect x { sects: [XrefSubSect] } :=
//
//    'xref' ( s=XrefSubSect { x.sects.append(s) } )* ;

#[derive(Debug, PartialEq)]
pub struct XrefSectT {
    sects: Vec<XrefSubSectT>
}
impl XrefSectT {
    pub fn sects(&self) -> &[XrefSubSectT] { self.sects.as_slice() }
}

pub struct XrefSectP;
impl ParsleyParser for XrefSectP {
    type T = XrefSectT;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        if let Err(_) = buf.exact("xref".as_bytes()) {
            return Err(ErrorKind::GuardError("not at xref"))
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
            if let Err(_) = sect { break }
            sects.push(sect.unwrap());
        }
        Ok(XrefSectT { sects })
    }
}

// Body b { objs = [PDFObj] } :=
//
//    ( o=PDFObj [ o ~~ IndirectObj ] { b.objs.append(o) } )* ;

#[derive(Debug, PartialEq)]
pub struct BodyT {
    objs: Vec<PDFObjT>
}
impl BodyT {
    pub fn objs(&self) -> &[PDFObjT] { self.objs.as_slice() }
}

pub struct BodyP;
impl ParsleyParser for BodyP {
    type T = BodyT;

    // PDF documents will almost never be parsed in this linear
    // fashion, but will instead be parsed by seeking to offsets
    // specified in the xref table.  Nevertheless, this function can
    // be used with simple files for debugging purposes.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let mut op = PDFObjP;
        // The outermost objects should all be indirect objects.  In
        // the simplest case, we just terminate when we can't parse
        // any more objects.
        let mut objs = Vec::new();
        loop {
            let o = op.parse(buf);
            if let Err(_) = o { break }
            let o = o.unwrap();
            if let PDFObjT::Indirect(_) = o {
                objs.push(o)
            } else {
                return Err(ErrorKind::GuardError("non-indirect object in body"))
            }
        }
        Ok(BodyT{ objs })
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

pub struct TrailerP;
impl ParsleyParser for TrailerP {
    type T = TrailerT;

    // This assumes we are positioned at 'trailer'.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        if let Err(_) = buf.exact("trailer".as_bytes()) {
            return Err(ErrorKind::GuardError("not at trailer"))
        }
        let mut ws = WhitespaceEOL::new(false); // need to consume an EOL
        ws.parse(buf)?;

        let dp = DictP;
        let dict = dp.parse(buf);
        if let Err(_) = dict {
            return Err(ErrorKind::GuardError("error parsing trailer dictionary"))
        }
        let dict = dict.unwrap();
        Ok(TrailerT{ dict })
    }
}

#[derive(Debug, PartialEq)]
pub struct StartXrefT {
    offset: u64
}
impl StartXrefT {
    pub fn offset(&self) -> u64 { self.offset }
}

pub struct StartXrefP;
impl ParsleyParser for StartXrefP {
    type T = StartXrefT;

    // This assumes we are positioned at 'startxref', which is
    // typically found by scanning backwards from EOF.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        if let Err(_) = buf.exact("startxref".as_bytes()) {
            return Err(ErrorKind::GuardError("not at startxref"))
        }
        let mut ws = WhitespaceEOL::new(false); // need to consume an EOL
        ws.parse(buf)?;

        let mut int = IntegerP;
        let offset = u64::try_from(int.parse(buf)?.int_val());
        if let Err(_) = offset {
            return Err(ErrorKind::GuardError("conversion error on startxref"))
        }
        let offset = offset.unwrap();
        Ok(StartXrefT{ offset })
    }
}

#[cfg(test)]
mod test_pdf_file {
    use std::collections::{HashMap};
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
    use super::super::super::pdf_lib::pdf_obj::{PDFObjT, ReferenceT, DictT};
    use super::super::super::pdf_lib::pdf_prim::{IntegerT};
    use super::{HeaderT, HeaderP, BodyT, BodyP, StartXrefT, StartXrefP, TrailerT, TrailerP};
    use super::{XrefT, XrefEntT, XrefEntP, XrefSubSectT, XrefSubSectP, XrefSectT, XrefSectP};

    #[test]
    fn test_header() {
        let mut p = HeaderP;

        let v = Vec::from("%PDF-1.0 \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let hdr = HeaderT { version: Vec::from("PDF-1.0 \r".as_bytes()), binary: None };
        assert_eq!(val, Ok(hdr));
        assert_eq!(pb.get_cursor(), 11);

        let v = Vec::from("%PDF-1.0 \r\n%binary_bytes\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let hdr = HeaderT { version: Vec::from("PDF-1.0 \r".as_bytes()),
                            binary: Some(Vec::from("binary_bytes".as_bytes())) };
        assert_eq!(val, Ok(hdr));
        assert_eq!(pb.get_cursor(), 25);
    }

    #[test]
    fn test_xref_ent() {
        let p = XrefEntP;

        let v = Vec::from("1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefT{ info: 1234567890, gen: 12345};
        assert_eq!(ent, Ok(XrefEntT::Free(xref)));

        let v = Vec::from("1234567890 12345 n \n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefT{ info: 1234567890, gen: 12345};
        assert_eq!(ent, Ok(XrefEntT::Inuse(xref)));

        let v = Vec::from("1234567890 12345 f \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        let xref = XrefT{ info: 1234567890, gen: 12345};
        assert_eq!(ent, Ok(XrefEntT::Free(xref)));

        // bad eol
        let v = Vec::from("1234567890 12345 f  \r".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let ent = p.parse(&mut pb);
        assert_eq!(ent, Err(ErrorKind::GuardError("bad eol gen")));
    }

    #[test]
    fn test_xref_subsect() {
        let p = XrefSubSectP;
        let v = Vec::from("0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let s = XrefSubSectT { start: 0, count: 1, ents: vec![xref] };
        assert_eq!(val, Ok(s));

        // leading and trailing space on leading line
        let v = Vec::from(" 0 1 \r\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let s = XrefSubSectT { start: 0, count: 1, ents: vec![xref] };
        assert_eq!(val, Ok(s));
    }

    #[test]
    fn test_xref_sect() {
        let mut p = XrefSectP;
        let v = Vec::from("xref\n0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let ssect = XrefSubSectT { start: 0, count: 1, ents: vec![xref] };
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(s));

        // different leading eol, and terminate with trailer
        let v = Vec::from("xref\r\n0 1\n1234567890 12345 f\r\ntrailer".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let ssect = XrefSubSectT { start: 0, count: 1, ents: vec![xref] };
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(s));

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
".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);

        let mut ents = Vec::new();
        ents.push(XrefEntT::Free( XrefT{ info: 0,   gen: 65535}));
        ents.push(XrefEntT::Inuse(XrefT{ info: 10,  gen: 0}));
        ents.push(XrefEntT::Inuse(XrefT{ info: 79,  gen: 0}));
        ents.push(XrefEntT::Inuse(XrefT{ info: 173, gen: 0}));
        ents.push(XrefEntT::Inuse(XrefT{ info: 301, gen: 0}));
        ents.push(XrefEntT::Inuse(XrefT{ info: 380, gen: 0}));
        let ssect = XrefSubSectT { start: 0, count: 6, ents };
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(s));
    }

    #[test]
    fn test_body_without_comments() {
        let mut p = BodyP;
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
        let val = p.parse(&mut pb);

        // quick and dirty test.
        if let Ok(BodyT { objs }) = val {
            assert_eq!(objs.len(), 5);
            for o in objs {
                if let PDFObjT::Indirect(_) = o {
                } else {
                    assert!(false)
                }
            }
        }
    }

    #[test]
    fn test_body_with_comments() {
        let mut p = BodyP;
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
        let val = p.parse(&mut pb);

        // quick and dirty test.
        if let Ok(BodyT { objs }) = val {
            assert_eq!(objs.len(), 5);
            for o in objs {
                if let PDFObjT::Indirect(_) = o {
                } else {
                    assert!(false)
                }
            }
        }
    }

    #[test]
    fn test_trailer() {
        let mut p = TrailerP;

        let v = Vec::from(
"trailer
<<
 /Size 8
 /Root 1 0 R
>>
".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);

        let mut map = HashMap::new();
        map.insert(Vec::from("Size".as_bytes()), PDFObjT::Integer(IntegerT::new(8)));
        map.insert(Vec::from("Root".as_bytes()), PDFObjT::Reference(ReferenceT::new(1, 0)));
        let dict = DictT::new(map);
        assert_eq!(val, Ok(TrailerT{ dict }));
    }

    #[test]
    fn test_startxref() {
        let mut p = StartXrefP;

        let v = Vec::from("startxref \n642\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        assert_eq!(val, Ok(StartXrefT { offset: 642 }));

        let v = Vec::from("startxref %absurd comment \n642\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        assert_eq!(val, Ok(StartXrefT { offset: 642 }));
    }
}
