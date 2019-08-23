// Basic PDF objects (simple and compound).

use std::collections::{HashSet, HashMap};
use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};

use super::pdf_prim::{WhitespaceEOL, Comment};
use super::pdf_prim::{Boolean, Null, IntegerT, IntegerP, RealT, RealP, HexString, RawLiteralString};
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

struct ArrayP {
    val: Vec<PDFObjT>
}
impl ArrayP {
    fn new() -> ArrayP {
        ArrayP { val: Vec::new() }
    }
    fn parse(mut self, buf: &mut ParseBuffer) -> Result<ArrayT, ErrorKind> {
        if let Err(_) = buf.exact("[".as_bytes()) {
            return Err(ErrorKind::GuardError("not at array object"))
        }
        let mut end = false;
        while !end {
            // Need more precise handling of whitespace to
            // differentiate between legal and illegal empty
            // whitespace.  This will be easier when auto-generated;
            // for now in the handwritten case, just be close enough.
            let mut ws = WhitespaceEOL::new(true);
            ws.parse(buf)?;
            if let Err(_) = buf.exact("]".as_bytes()) {
                let mut p = PDFObjP;
                let o = p.parse(buf)?;
                self.val.push(o);
            } else {
                end = true;
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

struct DictP {
    val:   HashMap<<RawName as ParsleyParser>::T, PDFObjT>,
    names: HashSet<<RawName as ParsleyParser>::T>
}
impl DictP {
    fn new() -> DictP {
        DictP { val: HashMap::new(), names: HashSet::new() }
    }
    fn parse(mut self, buf: &mut ParseBuffer) -> Result<DictT, ErrorKind> {
        buf.exact("<<".as_bytes())?;
        let mut end = false;
        while !end {
            // Need more precise handling of whitespace to
            // differentiate between legal and illegal empty
            // whitespace.  This will be easier when auto-generated;
            // for now in the handwritten case, just be close enough.
            let mut ws = WhitespaceEOL::new(true); // allow empty whitespace for now
            ws.parse(buf)?;

            if let Err(_) = buf.exact(">>".as_bytes()) {
                let mut p = RawName;
                let n = p.parse(buf)?;
                if self.names.contains(&n) {
                    // TODO: need extensible error reporting
                    return Err(ErrorKind::GuardError("non-unique dictionary key"))
                }

                // require whitespace
                let mut ws = WhitespaceEOL::new(false);
                ws.parse(buf)?;

                let mut p = PDFObjP;
                let o = p.parse(buf)?;
                // Note: reuse of n requires a clonable type
                self.names.insert(n.clone());
                self.val.insert(n, o);
            } else {
                end = true;
            }
        }
        Ok(DictT::new(self.val))
    }
}
// type struct Stream {
//   dict: DictObj,
//   stream: StreamObj
// }
//
// Indirect i { id : int, gen : int, val : PDFObj } :=
//     // the constraints check that the matched IntegerT objects
//     // are appropriate integers.
//     n=IntegerObj [ n.val >= 0 ]
//     g=IntegerObj [ g.val >= 0 && !defs.has_key((n.val, g.val))]
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
//     { i.id  := n.val;
//       i.gen := g.val;
//       defs[(n.val, g.val)] := (i, $location())
//     }


#[derive(Debug, PartialEq)]
pub struct StreamT {
    dict:   DictT,
    stream: Vec<u8>,
}
impl StreamT {
    pub fn new(dict: DictT, stream: Vec<u8>) -> StreamT {
        StreamT { dict, stream }
    }
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

struct IndirectP;
impl IndirectP {
    fn parse(self, buf: &mut ParseBuffer) -> Result<IndirectT, ErrorKind> {
        let mut int = IntegerP;
        let mut ws = WhitespaceEOL::new(true);

        let mut cursor = buf.get_cursor();
        let num = int.parse(buf)?;
        if !num.is_positive() {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid object id"))
        }
        ws.parse(buf)?;
        cursor = buf.get_cursor();
        let gen = int.parse(buf)?;
        if ! (gen.is_zero() || gen.is_positive()) {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid object generation"))
        }
        ws.parse(buf)?;
        if let Err(_) = buf.exact("obj".as_bytes()) {
            return Err(ErrorKind::GuardError("invalid object tag"))
        }
        ws.parse(buf)?;

        let mut p = PDFObjP;
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

        ws.parse(buf)?;
        if let Err(_) = buf.exact("endobj".as_bytes()) {
            return Err(ErrorKind::GuardError("invalid endobject tag"))
        }

        // TODO: update defs

        Ok(IndirectT::new(num.int_val(), gen.int_val(), Box::new(obj)))
    }
}

// Reference r { num : int, gen : int } :=
//
//     // the constraints check that the matched IntegerObj objects
//     // are appropriate integers.
//     n=IntegerObj [ n.val > 0 ]
//     g=IntegerObj [ g.val > 0 ]
//
//     'R'
//
//     { r.num := n.val;
//       r.gen := g.val;
//       refs[(n.val, g.val)] := $location();
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

struct ReferenceP;
impl ReferenceP {
    fn parse(self, buf: &mut ParseBuffer) -> Result<ReferenceT, ErrorKind> {
        let mut int = IntegerP;
        let mut ws = WhitespaceEOL::new(true);

        let mut cursor = buf.get_cursor();
        let num = int.parse(buf)?;
        if !num.is_positive() {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid ref-object id"))
        }
        ws.parse(buf)?;

        cursor = buf.get_cursor();
        let gen = int.parse(buf)?;
        if ! (gen.is_zero() || gen.is_positive()) {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid ref-object generation"))
        }
        ws.parse(buf)?;
        if let Err(_) = buf.exact("R".as_bytes()) {
            return Err(ErrorKind::GuardError("invalid reference tag"))
        }

        // TODO: update refs

        Ok(ReferenceT::new(num.int_val(), gen.int_val()))
    }
}


#[derive(Debug, PartialEq)]
pub enum PDFObjT {
    Array(ArrayT),
    Dict(DictT),
    Stream(StreamT),
    Indirect(IndirectT),
    Reference(ReferenceT),
    Boolean(bool),
    String(Vec<u8>),
    Name(Vec<u8>),
    Null(()),
    Comment(Vec<u8>),
    Integer(IntegerT),
    Real(RealT)
}

pub struct PDFObjP;
impl ParsleyParser for PDFObjP {
    type T = PDFObjT;

    // The top-level object parser.
    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
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
                if !b.is_ascii_digit() && b != 45 { // '-' to handle negative numbers
                    return Err(ErrorKind::GuardError("not at PDF object"))
                }
                let cursor = buf.get_cursor();

                // We have to distinguish between an indirect object,
                // an indirect reference, and a real number.  The
                // first two will always have two integer numbers as a
                // prefix.

                let mut real = RealP;
                let mut int  = IntegerP;
                let mut ws   = WhitespaceEOL::new(false); // no empty whitespace

                // Check if we are at a real.
                let r = real.parse(buf)?;
                if !r.is_integer() {
                    return Ok(PDFObjT::Real(r))
                }

                // We parsed the first integer.
                let n1 = IntegerT::new(r.numerator());
                let n1_end_cursor = buf.get_cursor();

                // Skip past non-empty whitespace.
                if let Err(_) = ws.parse(buf) {
                    // We've already parsed a number, so set the
                    // cursor past that and return it.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }

                // Get the second integer.
                let n2 = int.parse(buf);
                if let Err(_) = n2 {
                    // See above comment.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }

                // Skip past non-empty whitespace.
                if let Err(_) = ws.parse(buf) {
                    // We've already parsed the first number, so set the
                    // cursor past that and return it.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }

                // We have now seen two integers.
                let prefix = buf.check_prefix("obj".as_bytes());
                if let Err(_) = prefix {
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
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
                    return Ok(PDFObjT::Integer(n1))
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
                return Ok(PDFObjT::Integer(n1))
            }
        }
    }
}

#[cfg(test)]
mod test_pdf_obj {
    use std::collections::{HashMap};
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};
    use super::{PDFObjP, PDFObjT, ReferenceT, ArrayT, DictT, IndirectT, StreamT};

    #[test]
    fn empty() {
        let mut p = PDFObjP;

        let v = Vec::from("");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn comment() {
        let mut p = PDFObjP;

        let v = Vec::from("\r\n %PDF-1.0 \r\n");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Ok(PDFObjT::Comment(Vec::from("%PDF-1.0 \r".as_bytes()))));
        assert_eq!(pb.get_cursor(), 14);
    }

    #[test]
    fn reference() {
        let mut p = PDFObjP;

        let v = Vec::from("\r\n 1 0 R \r\n");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Ok(PDFObjT::Reference(ReferenceT::new(1, 0))));
        assert_eq!(pb.get_cursor(), 8);

        let v = Vec::from("\r\n -1 0 R \r\n");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("invalid ref-object id")));
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("\r\n 1 -1 R \r\n");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("invalid ref-object generation")));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn array() {
        let mut p = PDFObjP;

        let v = Vec::from("[ 1 0 R ] \r\n");
        let mut pb = ParseBuffer::new(v);
        let mut aval = Vec::new();
        aval.push(PDFObjT::Reference(ReferenceT::new(1, 0)));
        assert_eq!(p.parse(&mut pb), Ok(PDFObjT::Array(ArrayT::new(aval))));
        assert_eq!(pb.get_cursor(), 9);

        let v = Vec::from("[ 1 \r 0 \n R ] \r\n");
        let mut pb = ParseBuffer::new(v);
        let mut aval = Vec::new();
        aval.push(PDFObjT::Reference(ReferenceT::new(1, 0)));
        assert_eq!(p.parse(&mut pb), Ok(PDFObjT::Array(ArrayT::new(aval))));
        assert_eq!(pb.get_cursor(), 13);

        let v = Vec::from("[ -1 0 R ] \r\n");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("invalid ref-object id")));
        assert_eq!(pb.get_cursor(), 2);
    }

    #[test]
    fn dict() {
        let mut p = PDFObjP;

        let v = Vec::from("<< /Entry [ 1 0 R ] \r\n >>");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut aval = Vec::new();
        aval.push(PDFObjT::Reference(ReferenceT::new(1, 0)));
        let entval = PDFObjT::Array(ArrayT::new(aval));
        let mut hm = HashMap::new();
        hm.insert(Vec::from("Entry".as_bytes()), entval);
        assert_eq!(p.parse(&mut pb), Ok(PDFObjT::Dict(DictT { val: hm })));
        assert_eq!(pb.get_cursor(), vlen);

        // version with minimal whitespace
        let v = Vec::from("<</Entry [1 0 R]>>");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut aval = Vec::new();
        aval.push(PDFObjT::Reference(ReferenceT::new(1, 0)));
        let entval = PDFObjT::Array(ArrayT::new(aval));
        let mut hm = HashMap::new();
        hm.insert(Vec::from("Entry".as_bytes()), entval);
        assert_eq!(p.parse(&mut pb), Ok(PDFObjT::Dict(DictT { val: hm })));
        assert_eq!(pb.get_cursor(), vlen);

        let v = Vec::from("<< /Entry [ 1 0 R ] /Entry \r\n >>");
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("non-unique dictionary key")));
    }

    #[test]
    fn indirect() {
        let mut p = PDFObjP;

        let v = Vec::from("1 0 obj << /Entry [ 1 0 R ] \r\n >> endobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut aval = Vec::new();
        aval.push(PDFObjT::Reference(ReferenceT::new(1, 0)));
        let entval = PDFObjT::Array(ArrayT::new(aval));
        let mut hm = HashMap::new();
        hm.insert(Vec::from("Entry".as_bytes()), entval);
        let dict = PDFObjT::Dict(DictT::new(hm));
        let obj = PDFObjT::Indirect(IndirectT::new(1, 0, Box::new(dict)));
        assert_eq!(p.parse(&mut pb), Ok(obj));
        assert_eq!(pb.get_cursor(), vlen);
    }

    #[test]
    fn stream() {
        let mut p = PDFObjP;

        let v = Vec::from("1 0 obj << /Entry [ 1 0 R ] >> stream\n junk \nendstream\nendobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut aval = Vec::new();
        aval.push(PDFObjT::Reference(ReferenceT::new(1, 0)));
        let entval = PDFObjT::Array(ArrayT::new(aval));
        let mut hm = HashMap::new();
        hm.insert(Vec::from("Entry".as_bytes()), entval);
        let dict = DictT::new(hm);
        let stream = PDFObjT::Stream(StreamT::new(dict, Vec::from(" junk ".as_bytes())));
        let obj = PDFObjT::Indirect(IndirectT::new(1, 0, Box::new(stream)));
        assert_eq!(p.parse(&mut pb), Ok(obj));
        assert_eq!(pb.get_cursor(), vlen);
    }
}
