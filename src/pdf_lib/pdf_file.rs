// File structure of PDF

use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
use super::pdf_obj::{PDFObjP, PDFObjT};

#[derive(Debug, PartialEq)]
pub struct HeaderT {
    version: Vec<u8>,
    binary:  Option<Vec<u8>>
}

#[derive(Debug, PartialEq)]
pub struct ObjXrefT {
    offset: u64,
    gen:    u64
}

#[derive(Debug, PartialEq)]
pub struct FreeXrefT {
    next: u64,
    gen:  u64
}

#[derive(Debug, PartialEq)]
pub enum XrefEntT {
    Obj(ObjXrefT),
    Free(FreeXrefT)
}

#[derive(Debug, PartialEq)]
pub struct XrefSubSectT {
    start_obj: u64,
    obj_cnt:   u64,
    ents:      Vec<XrefEntT>
}

#[derive(Debug, PartialEq)]
pub struct XrefSectT {
    sects: Vec<XrefSubSectT>
}

#[derive(Debug, PartialEq)]
pub struct XrefSectP {
    sects: Vec<XrefSectT>
}

impl XrefSectP {
    pub fn new() -> XrefSectP {
        XrefSectP { sects: Vec::new() }
    }
}

impl ParsleyParser for XrefSectP {
    type T = XrefSectT;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<XrefSectT, ErrorKind> {
        if let Err(_) = buf.exact("xref".as_bytes()) {
            return Err(ErrorKind::GuardError("not at xref"))
        }
        
        Err(ErrorKind::EndOfBuffer)
    }
}
