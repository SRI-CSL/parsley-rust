// File structure of PDF

use std::io::Read; // for read_to_string()
use std::convert::TryFrom;
use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
use super::pdf_prim::{IntegerP, WhitespaceEOL, WhitespaceNoEOL};
use super::pdf_obj::{PDFObjP, PDFObjT};

#[derive(Debug, PartialEq)]
pub struct HeaderT {
    version: Vec<u8>,
    binary:  Option<Vec<u8>>
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

#[derive(Debug, PartialEq)]
pub struct XrefSubSectT {
    start: u64,
    cnt:   u64,
    ents:  Vec<XrefEntT>
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
        let cnt = u64::try_from(int.parse(buf)?.int_val());
        if let Err(_) = cnt {
            return Err(ErrorKind::GuardError("conversion error on xref-subsect count"))
        }
        let cnt = cnt.unwrap();

        // Again, no clarity on the type of EOL terminating this line.
        // Assume we need one for now.
        let mut ws = WhitespaceEOL::new(false);
        ws.parse(buf)?;

        // Now get the entries.
        let p = XrefEntP;
        let mut ents = Vec::new();
        for _i in 0 .. cnt {
            let ent = p.parse(buf)?;
            ents.push(ent);
        }

        Ok(XrefSubSectT{ start, cnt, ents })
    }
}

#[derive(Debug, PartialEq)]
pub struct XrefSectT {
    sects: Vec<XrefSubSectT>
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

#[derive(Debug, PartialEq)]
pub struct BodyT {
    objs: Vec<PDFObjT>
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


#[cfg(test)]
mod test_pdf_file {
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
    use super::{XrefT, XrefEntT, XrefEntP, XrefSubSectT, XrefSubSectP, XrefSectT, XrefSectP, BodyT, BodyP};
    use super::super::super::pdf_lib::pdf_obj::{PDFObjT};

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
        let s = XrefSubSectT { start: 0, cnt: 1, ents: vec![xref] };
        assert_eq!(val, Ok(s));

        // leading and trailing space on leading line
        let v = Vec::from(" 0 1 \r\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let s = XrefSubSectT { start: 0, cnt: 1, ents: vec![xref] };
        assert_eq!(val, Ok(s));
    }

    #[test]
    fn test_xref_sect() {
        let mut p = XrefSectP;
        let v = Vec::from("xref\n0 1\n1234567890 12345 f\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let ssect = XrefSubSectT { start: 0, cnt: 1, ents: vec![xref] };
        let s = XrefSectT { sects: vec![ssect] };
        assert_eq!(val, Ok(s));

        // different leading eol, and terminate with trailer
        let v = Vec::from("xref\r\n0 1\n1234567890 12345 f\r\ntrailer".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let xref = XrefEntT::Free(XrefT{ info: 1234567890, gen: 12345});
        let ssect = XrefSubSectT { start: 0, cnt: 1, ents: vec![xref] };
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
        let ssect = XrefSubSectT { start: 0, cnt: 6, ents };
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

    // FIXME: we are not handling embedded comments properly
    #[test]
    #[ignore]
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
}
