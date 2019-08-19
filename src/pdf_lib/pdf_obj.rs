// Basic PDF objects (simple and compound).

use std::collections::{HashSet, HashMap};
use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};

use super::pdf_prim::{WhitespaceEOL, Comment};
use super::pdf_prim::{Boolean, Null, Number, NumberT, HexString, RawLiteralString};
use super::pdf_prim::{RawName, StreamContent};

// Array a { val : [PDFObj] } := '[' ( o=PDFObj { a.val.append(o) } )* ']'

#[derive(Debug, PartialEq)]
pub struct ArrayT {
    val: Vec<PDFObjT>
}
impl ArrayT {
    pub fn new(val: Vec<PDFObjT>) -> ArrayT {
        ArrayT { val }
    }
}

pub struct ArrayP {
    val: Vec<PDFObjT>
}
impl ArrayP {
    pub fn new() -> ArrayP {
        ArrayP { val: Vec::new() }
    }
    pub fn parse(mut self, buf: &mut ParseBuffer) -> Result<ArrayT, ErrorKind> {
        buf.exact("[".as_bytes())?;
        let mut end = false;
        while !end {
            // Need more precise handling of whitespace to
            // differentiate between legal and illegal empty
            // whitespace.  This will be easier when auto-generated;
            // for now in the handwritten case, just be close enough.
            let mut ws = WhitespaceEOL::new(true);
            ws.parse(buf)?;
            end = buf.skip_prefix("]".as_bytes())?;
            if !end {
                let p = PDFObjP::new();
                let o = p.parse(buf)?;
                self.val.push(o);
            }
        }
        Ok(ArrayT::new(self.val))
    }
}

// Dict d { val : map<typeof(NameObj.val), PDFObj> } :=
//  { names : set<typeof(NameObj.val)> }
//  '<<' ( n=NameObj [ !names.contains(n.val) "Unique dictionary key" ] o=PDFObj { d.val[n.val] := o; names.add(n.val) } )* '>>' ;

#[derive(Debug, PartialEq)]
pub struct DictT {
    // TODO: use a better way to name the type
    val: HashMap<<RawName as ParsleyParser>::T, PDFObjT>
}
impl DictT {
    pub fn new(val: HashMap<<RawName as ParsleyParser>::T, PDFObjT>) -> DictT {
        DictT { val }
    }
}

pub struct DictP {
    val:   HashMap<<RawName as ParsleyParser>::T, PDFObjT>,
    names: HashSet<<RawName as ParsleyParser>::T>
}
impl DictP {
    pub fn new() -> DictP {
        DictP { val: HashMap::new(), names: HashSet::new() }
    }
    pub fn parse(mut self, buf: &mut ParseBuffer) -> Result<DictT, ErrorKind> {
        buf.exact("<<".as_bytes())?;
        let mut end = false;
        while !end {
            // Need more precise handling of whitespace to
            // differentiate between legal and illegal empty
            // whitespace.  This will be easier when auto-generated;
            // for now in the handwritten case, just be close enough.
            let mut ws = WhitespaceEOL::new(true);
            ws.parse(buf)?;

            end = buf.skip_prefix(">>".as_bytes())?;
            if !end {
                let mut p = RawName;
                let n = p.parse(buf)?;
                if self.names.contains(&n) {
                    // TODO: need extensible error reporting
                    return Err(ErrorKind::GuardError("non-unique dictionary key"))
                }

                // require whitespace
                let mut ws = WhitespaceEOL::new(false);
                ws.parse(buf)?;

                let p = PDFObjP::new();
                let o = p.parse(buf)?;
                // Note: reuse of n requires a clonable type
                self.names.insert(n.clone());
                self.val.insert(n, o);
            }
        }
        Ok(DictT::new(self.val))
    }
}
// type struct Stream {
//   dict: DictObj,
//   steam: StreamObj
// }
//
// Indirect i { id : int, gen : int, val : PDFObj } :=
//     // the constraints check that the matched NumberObj objects
//     // are appropriate integers.
//     n=NumberObj [ n.is_integer() && n.int_val() >= 0 ]
//     g=NumberObj [ g.is_integer() && g.int_val() >= 0 && !defs.has_key((n.int_val(), g.int_val()))]
//
//     ( 'obj' o=PDFObj 'endobj'
//
//       // the semantic action computes the values of the attributes
//       // of Indirect.  $location() is a predefined function that
//       // returns the current parsing cursor location, at the end of
//       // the last matching right-hand-side entity in the rule.
//         { i.val := o }
//     | 'obj' o=PDFObj [o ~~ DictObj] s=StreamObj 'endobj'
//         { i.val := Stream { dict: o.val, stream: s.val } }
//     )
//     // At this point, we need to ensure that all attributes are defined for all cases
//     { i.id  := n.int_val();
//       i.gen := g.int_val();
//       defs[(n.int_val(), g.int_val())] := (i, $location())
//     }


#[derive(Debug, PartialEq)]
pub struct StreamT {
    dict: DictT,
    stream: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct IndirectT {
    num: i64,
    gen: i64,
    obj: Box<PDFObjT>
}
impl IndirectT {
    pub fn new(num: i64, gen: i64, obj: Box<PDFObjT>) -> IndirectT {
        IndirectT { num, gen, obj }
    }
}

pub struct IndirectP;
impl IndirectP {
    pub fn parse(self, buf: &mut ParseBuffer) -> Result<IndirectT, ErrorKind> {
        let mut int = Number;
        let mut ws = WhitespaceEOL::new(true);

        let num = int.parse(buf)?;
        if ! (num.is_integer() && num.is_positive()) {
            return Err(ErrorKind::GuardError("invalid object id"))
        }
        ws.parse(buf)?;
        let gen = int.parse(buf)?;
        if ! (gen.is_integer() && (gen.is_zero() || gen.is_positive())) {
            return Err(ErrorKind::GuardError("invalid object generation"))
        }
        ws.parse(buf)?;
        buf.skip_prefix("obj".as_bytes())?;
        ws.parse(buf)?;

        let p = PDFObjP::new();
        let o = p.parse(buf)?;

        // If we parsed a dictionary, check whether this could be a
        // stream object.
        let obj =
            if let PDFObjT::Dict(d) = o {
                let mut ws = WhitespaceEOL::new(false); // allow empty whitespace
                ws.parse(buf)?;
                if buf.check_prefix("stream".as_bytes())? {
                    let mut s = StreamContent;
                    let c = s.parse(buf)?;
                    PDFObjT::Stream(StreamT { dict: d, stream: c})
                } else {
                    PDFObjT::Dict(d)
                }
            } else {
                o
            };

        buf.skip_prefix("endobj".as_bytes())?;

        // TODO: update defs

        Ok(IndirectT::new(num.int_val(), gen.int_val(), Box::new(obj)))
    }
}

// Reference r { id : int, gen : int } :=
//
//     // the constraints check that the matched NumberObj objects
//     // are appropriate integers.
//     n=NumberObj [ n.is_integer() && n.int_val() > 0 ]
//     g=NumberObj [ g.is_integer() && g.int_val() > 0 ]
//
//     'R'
//
//     { r.id  := n.int_val();
//       r.gen := g.int_val();
//       refs[(n.int_val(), g.int_val())] := $location();
//     } ;

#[derive(Debug, PartialEq)]
pub struct ReferenceT {
    num: i64,
    gen: i64
}
impl ReferenceT {
    pub fn new(num: i64, gen: i64) -> ReferenceT {
        ReferenceT { num, gen }
    }
}

pub struct ReferenceP;
impl ReferenceP {
    pub fn parse(self, buf: &mut ParseBuffer) -> Result<ReferenceT, ErrorKind> {
        let mut int = Number;
        let mut ws = WhitespaceEOL::new(true);

        let num = int.parse(buf)?;
        if ! (num.is_integer() && num.is_positive()) {
            return Err(ErrorKind::GuardError("invalid object id"))
        }
        ws.parse(buf)?;
        let gen = int.parse(buf)?;
        if ! (gen.is_integer() && (gen.is_zero() || gen.is_positive())) {
            return Err(ErrorKind::GuardError("invalid object generation"))
        }
        ws.parse(buf)?;
        buf.skip_prefix("R".as_bytes())?;

        // TODO: update refs

        Ok(ReferenceT::new(num.int_val(), gen.int_val()))
    }
}


#[derive(Debug, PartialEq)]
pub enum PDFObjT {
    Array(ArrayT),
    Dict(DictT),
    Indirect(IndirectT),
    Reference(ReferenceT),
    Boolean(bool),
    String(Vec<u8>),
    Name(Vec<u8>),
    Stream(StreamT),
    Null(()),
    Comment(Vec<u8>),
    Number(NumberT)
}

pub struct PDFObjP;
impl PDFObjP {
    pub fn new() -> PDFObjP {
        PDFObjP {}
    }

    // The top-level object parser.
    pub fn parse(self, buf: &mut ParseBuffer) -> Result<PDFObjT, ErrorKind> {
        // First, consume possibly empty whitespace.
        // TODO: what about EOL?
        let mut ws = WhitespaceEOL::new(true);
        ws.parse(buf)?;

        let c = buf.peek();
        match c {
            None      => Err(ErrorKind::EndOfBuffer),

            Some(116) | Some(102) => { // 't' | 'f'
                let mut b = Boolean;
                Ok(PDFObjT::Boolean(b.parse(buf)?))
            },
            Some(110) => { // 'n'
                let mut n = Null;
                Ok(PDFObjT::Null(n.parse(buf)?))
            },
            Some(40)  => { // '('
                let mut r = RawLiteralString;
                Ok(PDFObjT::String(r.parse(buf)?))
            },
            Some(37)  => { // '%'
                let mut c = Comment;
                Ok(PDFObjT::Comment(c.parse(buf)?))
            },
            Some(47)  => { // '/'
                let mut n = RawName;
                Ok(PDFObjT::Name(n.parse(buf)?))
            },
            Some(91)  => { // '['
                let p = ArrayP::new();
                Ok(PDFObjT::Array(p.parse(buf)?))
            },
            Some(60)  => { // '<'
                // We need to distinguish between a hex-string and a
                // dictionary object.  So peek ahead.
                let cursor = buf.get_cursor();
                buf.incr_cursor();
                let next = buf.peek();
                buf.set_cursor(cursor);

                match next {
                    Some(60) => {
                        let p = DictP::new();
                        Ok(PDFObjT::Dict(p.parse(buf)?))
                    },
                    Some(_) | None => {
                        let mut h = HexString;
                        Ok(PDFObjT::String(h.parse(buf)?))
                    }
                }
            },
            Some(b)   => {
                if !b.is_ascii_digit() {
                    return Err(ErrorKind::GuardError("not at PDF object"))
                }
                let cursor = buf.get_cursor();

                let mut n = Number;
                let mut ws = WhitespaceEOL::new(false); // no empty whitespace

                // We have to distinguish between an indirect object,
                // an indirect reference, and a number.  The first two
                // will always have two integer numbers as a prefix.

                // Get the first number.
                let n1 = n.parse(buf)?;
                if !n1.is_integer() { return Ok(PDFObjT::Number(n1)) }
                let n1_end_cursor = buf.get_cursor();

                // Skip past non-empty whitespace.
                if let Err(_) = ws.parse(buf) {
                    // We've already parsed a number, so set the
                    // cursor past that and return it.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Number(n1))
                }

                // Get the second number.
                let n2 = n.parse(buf);
                if let Err(_) = n2 {
                    // See above comment.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Number(n1))
                }
                let n2 = n2.unwrap();
                if !n2.is_integer() {
                    // See above comment.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Number(n1))
                }

                // Skip past non-empty whitespace.
                if let Err(_) = ws.parse(buf) {
                    // We've already parsed a number, so set the
                    // cursor past that and return it.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Number(n1))
                }

                // We have now seen two integers.
                let prefix = buf.check_prefix("obj".as_bytes());
                if let Err(_) = prefix {
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Number(n1))
                }
                if prefix.unwrap() {
                    // This looks like an indirect object.  Rewind and
                    // call its parser.
                    buf.set_cursor(cursor);

                    let p = IndirectP;
                    return Ok(PDFObjT::Indirect(p.parse(buf)?))
                }

                let prefix = buf.check_prefix("R".as_bytes());
                if let Err(_) = prefix {
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Number(n1))
                }
                if prefix.unwrap() {
                    // This looks like an indirect reference.  Rewind
                    // and call its parser (though we could optimize
                    // this case).
                    buf.set_cursor(cursor);

                    let p = ReferenceP;
                    return Ok(PDFObjT::Reference(p.parse(buf)?))
                }

                // Fallback case.
                buf.set_cursor(n1_end_cursor);
                return Ok(PDFObjT::Number(n1))
            }
        }
    }
}
