// Basic PDF objects (simple and compound).

use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, Location, LocatedVal, ParseResult, ErrorKind};

use super::pdf_prim::{WhitespaceEOL, Comment};
use super::pdf_prim::{Boolean, Null, IntegerT, IntegerP, RealT, RealP, HexString, RawLiteralString};
use super::pdf_prim::{RawName, StreamContent};

// Object locations in the PDF file.  This will need to become
// hierarchical to handle nested object streams.

pub struct PDFLocation {
    start: usize,
    end:   usize
}

impl Location for PDFLocation {
    fn loc_start(&self) -> usize { self.start }
    fn loc_end(&self)   -> usize { self.end }
}

// PDF object parsing context.
// This keeps track of information collected during parsing.

pub struct PDFObjContext {
    defns: HashMap<(i64, i64), Rc<LocatedVal<PDFObjT>>>
}

impl PDFObjContext {
    pub fn new() -> PDFObjContext {
        PDFObjContext { defns: HashMap::new() }
    }
    pub fn register_obj(&mut self, p: &IndirectT, o: Rc<LocatedVal<PDFObjT>>) -> Option<Rc<LocatedVal<PDFObjT>>> {
        self.defns.insert((p.num(), p.gen()), o)
    }
    pub fn lookup_obj(&self, oid: (i64, i64)) -> Option<&LocatedVal<PDFObjT>> {
        match self.defns.get(&oid) {
            Some(v) => Some(&v),
            None    => None
        }
    }
}


// Array a { objs : [PDFObj] } := '[' ( o=PDFObj { a.objs.append(o) } )* ']'

#[derive(Debug, PartialEq)]
pub struct ArrayT {
    objs: Vec<LocatedVal<PDFObjT>>
}
impl ArrayT {
    pub fn new(objs: Vec<LocatedVal<PDFObjT>>) -> ArrayT {
        ArrayT { objs }
    }
    pub fn objs(&self) -> &[LocatedVal<PDFObjT>] {
        self.objs.as_slice()
    }
}

struct ArrayP<'a> {
    ctxt: &'a mut PDFObjContext
}

impl ArrayP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> ArrayP<'a> {
        ArrayP { ctxt }
    }
    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<LocatedVal<ArrayT>> {
        let start = buf.get_cursor();
        if let Err(_) = buf.exact("[".as_bytes()) {
            return Err(ErrorKind::GuardError("not at array object"))
        }
        let mut objs = Vec::new();
        let mut end = false;
        while !end {
            // Need more precise handling of whitespace to
            // differentiate between legal and illegal empty
            // whitespace.  This will be easier when auto-generated;
            // for now in the handwritten case, just be close enough.
            let mut ws = WhitespaceEOL::new(true);
            ws.parse(buf)?;
            if let Err(_) = buf.exact("]".as_bytes()) {
                let mut p = PDFObjP::new(&mut self.ctxt);
                let o = p.parse(buf)?;
                objs.push(o);
            } else {
                end = true;
            }
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ArrayT::new(objs), start, end))
    }
}

// Dict d { map : map<typeof(NameObj.val), PDFObj> } :=
//  { names : set<typeof(NameObj.val)> }
//  '<<' ( n=NameObj [ !names.contains(n.val) "Unique dictionary key" ] o=PDFObj { d.map[n.val] := o; names.add(n.val) } )* '>>' ;

#[derive(Debug, PartialEq)]
pub struct DictT {
    map: HashMap<<RawName as ParsleyParser>::T, LocatedVal<PDFObjT>>
}
impl DictT {
    pub fn new(map: HashMap<<RawName as ParsleyParser>::T, LocatedVal<PDFObjT>>) -> DictT {
        DictT { map }
    }
    pub fn map(&self) -> &HashMap<<RawName as ParsleyParser>::T, LocatedVal<PDFObjT>> {
        &self.map
    }
}

pub struct DictP<'a> {
    ctxt: &'a mut PDFObjContext
}

impl DictP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> DictP<'a> {
        DictP { ctxt }
    }
    pub fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<DictT> {
        buf.exact("<<".as_bytes())?;
        let mut end   = false;
        let mut map   = HashMap::new();
        let mut names = HashSet::new();
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
                if names.contains(n.val()) {
                    // TODO: need extensible error reporting
                    return Err(ErrorKind::GuardError("non-unique dictionary key"))
                }

                // do not require whitespace between key/value pairs
                let mut ws = WhitespaceEOL::new(true);
                ws.parse(buf)?;

                let mut p = PDFObjP::new(&mut self.ctxt);
                let o = p.parse(buf)?;

                // Entries with 'null' values are treated as though
                // the entry does not exist.
                if let PDFObjT::Null(_) = o.val() {
                    // Drop the entry.
                } else {
                    // Note: reuse of n requires a clonable type
                    names.insert(n.val().clone());
                    map.insert(n, o);
                }
            } else {
                end = true;
            }
        }
        Ok(DictT::new(map))
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
    dict:   LocatedVal<DictT>,
    stream: LocatedVal<Vec<u8>>,
}
impl StreamT {
    pub fn new(dict: LocatedVal<DictT>, stream: LocatedVal<Vec<u8>>) -> StreamT {
        StreamT { dict, stream }
    }
    pub fn dict(&self)   -> &LocatedVal<DictT>   { &self.dict }
    pub fn stream(&self) -> &LocatedVal<Vec<u8>> { &self.stream }
}

#[derive(Debug, PartialEq)]
pub struct IndirectT {
    num: i64,
    gen: i64,
    obj: Rc<LocatedVal<PDFObjT>>
}
impl IndirectT {
    pub fn new(num: i64, gen: i64, obj: Rc<LocatedVal<PDFObjT>>) -> IndirectT {
        IndirectT { num, gen, obj }
    }
    pub fn num(&self) -> i64 { self.num }
    pub fn gen(&self) -> i64 { self.gen }
    pub fn obj(&self) -> &LocatedVal<PDFObjT> { &self.obj }
}

struct IndirectP<'a> {
    ctxt: &'a mut PDFObjContext
}

impl IndirectP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> IndirectP<'a> {
        IndirectP { ctxt }
    }
    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<IndirectT> {
        let mut int = IntegerP;
        let mut ws = WhitespaceEOL::new(true);

        let mut cursor = buf.get_cursor();
        let num = int.parse(buf)?;
        if !num.val().is_positive() {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid object id"))
        }
        ws.parse(buf)?;
        cursor = buf.get_cursor();
        let gen = int.parse(buf)?;
        if ! (gen.val().is_zero() || gen.val().is_positive()) {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid object generation"))
        }
        ws.parse(buf)?;
        if let Err(_) = buf.exact("obj".as_bytes()) {
            return Err(ErrorKind::GuardError("invalid object tag"))
        }
        ws.parse(buf)?;

        let mut p = PDFObjP::new(&mut self.ctxt);
        let o = p.parse(buf)?;

        // If we parsed a dictionary, check whether this could be a
        // stream object.
        let obj =
            // This is ugly code and should be cleaned up.
            if let PDFObjT::Dict(_) = o.val() {
                let mut ws = WhitespaceEOL::new(true); // allow empty whitespace
                ws.parse(buf)?;
                if buf.check_prefix("stream".as_bytes())? {
                    // This is indeed a stream object.
                    let dict_start = o.loc_start();
                    let dict_end   = o.loc_end();
                    if let PDFObjT::Dict(dict) = o.unwrap() {
                        let dict = LocatedVal::new(dict, dict_start, dict_end);
                        let mut s = StreamContent;
                        let stream = s.parse(buf)?;
                        let start = dict_start;
                        let end = stream.loc_end();
                        LocatedVal::new(PDFObjT::Stream(StreamT { dict, stream }), start, end)
                    } else {
                        panic!("can never happen")
                    }
                } else {
                    o
                }
            } else {
                o
            };

        ws.parse(buf)?;
        if let Err(_) = buf.exact("endobj".as_bytes()) {
            return Err(ErrorKind::GuardError("invalid endobject tag"))
        }

        let obj = Rc::new(obj);
        let ind = IndirectT::new(num.val().int_val(), gen.val().int_val(), Rc::clone(&obj));
        match self.ctxt.register_obj(&ind, Rc::clone(&obj)) {
            None    => Ok(ind),
            Some(_) => Err(ErrorKind::GuardError("non-unique object id"))
        }
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
    pub fn num(&self) -> i64 { self.num }
    pub fn gen(&self) -> i64 { self.gen }
}

struct ReferenceP;
impl ReferenceP {
    fn parse(self, buf: &mut ParseBuffer) -> ParseResult<ReferenceT> {
        let mut int = IntegerP;
        let mut ws = WhitespaceEOL::new(true);

        let mut cursor = buf.get_cursor();
        let num = int.parse(buf)?;
        if !num.val().is_positive() {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid ref-object id"))
        }
        ws.parse(buf)?;

        cursor = buf.get_cursor();
        let gen = int.parse(buf)?;
        if ! (gen.val().is_zero() || gen.val().is_positive()) {
            buf.set_cursor(cursor);
            return Err(ErrorKind::GuardError("invalid ref-object generation"))
        }
        ws.parse(buf)?;
        if let Err(_) = buf.exact("R".as_bytes()) {
            return Err(ErrorKind::GuardError("invalid reference tag"))
        }

        // TODO: update refs

        Ok(ReferenceT::new(num.val().int_val(), gen.val().int_val()))
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

pub struct PDFObjP<'a> {
    ctxt: &'a mut PDFObjContext
}

impl PDFObjP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext) -> PDFObjP<'a> {
        PDFObjP { ctxt }
    }
    // The top-level object parser, as an internal helper.
    fn parse_internal(&mut self, buf: &mut ParseBuffer) -> ParseResult<PDFObjT> {
        let c = buf.peek();
        match c {
            None      => Err(ErrorKind::EndOfBuffer),

            Some(116) | Some(102) => { // 't' | 'f'
                let mut bp = Boolean;
                let b = bp.parse(buf)?;
                Ok(PDFObjT::Boolean(b.unwrap()))
            },
            Some(110) => { // 'n'
                let mut np = Null;
                let n = np.parse(buf)?;
                Ok(PDFObjT::Null(n.unwrap()))
            },
            Some(40)  => { // '('
                let mut rp = RawLiteralString;
                let r = rp.parse(buf)?;
                Ok(PDFObjT::String(r.unwrap()))
            },
            Some(37)  => { // '%'
                let mut cp = Comment;
                let c = cp.parse(buf)?;
                Ok(PDFObjT::Comment(c.unwrap()))
            },
            Some(47)  => { // '/'
                let mut np = RawName;
                let n = np.parse(buf)?;
                Ok(PDFObjT::Name(n.unwrap()))
            },
            Some(91)  => { // '['
                let mut ap = ArrayP::new(&mut self.ctxt);
                let a = ap.parse(buf)?;
                Ok(PDFObjT::Array(a.unwrap()))
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
                        let mut dp = DictP::new(&mut self.ctxt);
                        let d = dp.parse(buf)?;
                        Ok(PDFObjT::Dict(d))
                    },
                    Some(_) | None => {
                        let mut hp = HexString;
                        let s = hp.parse(buf)?;
                        Ok(PDFObjT::String(s.unwrap()))
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
                if !r.val().is_integer() {
                    return Ok(PDFObjT::Real(r.unwrap()))
                }

                // We parsed the first integer.
                let n1 = IntegerT::new(r.val().numerator());
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

                    let mut p = IndirectP::new(&mut self.ctxt);
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

impl ParsleyParser for PDFObjP<'_> {
    type T = LocatedVal<PDFObjT>;

    // The top-level object parser.
    fn parse(&mut self, buf: &mut ParseBuffer) -> ParseResult<Self::T> {
        // First, consume possibly empty whitespace.
        // TODO: what about EOL?
        let mut ws = WhitespaceEOL::new(true);
        ws.parse(buf)?;

        let start = buf.get_cursor();
        let val   = self.parse_internal(buf)?;
        let end   = buf.get_cursor();
        Ok(LocatedVal::new(val, start, end))
    }
}

#[cfg(test)]
mod test_pdf_obj {
    use std::rc::Rc;
    use std::borrow::Borrow;
    use std::collections::{HashMap};
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser, LocatedVal, ErrorKind};
    use super::super::pdf_prim::{RealT};
    use super::{PDFObjContext, PDFObjP, PDFObjT, ReferenceT, ArrayT, DictT, IndirectT, StreamT};

    #[test]
    fn empty() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);

        let v = Vec::from("".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn comment() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);

        // Comments are essentially whitespace.
        let v = Vec::from("\r\n %PDF-1.0 \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 14);
    }

    #[test]
    fn reference() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);

//                         012345678901234567890
        let v = Vec::from("\r\n 1 0 R \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Reference(ReferenceT::new(1, 0));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(val, 3, 8)));
        assert_eq!(pb.get_cursor(), 8);

//                         012345678901234567890
        let v = Vec::from("\r\n -1 0 R \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("invalid ref-object id")));
        assert_eq!(pb.get_cursor(), 3);

//                         012345678901234567890
        let v = Vec::from("\r\n 1 -1 R \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("invalid ref-object generation")));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn array() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);

//                         012345678901234567890
        let v = Vec::from("[ 1 0 R ] \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 2, 7));
        let val = PDFObjT::Array(ArrayT::new(objs));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(val, 0, 9)));
        assert_eq!(pb.get_cursor(), 9);

//                         012345678901234567890
        let v = Vec::from("[ 1 \r 0 \n R ] \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 2, 11));
        let val = PDFObjT::Array(ArrayT::new(objs));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(val, 0, 13)));
        assert_eq!(pb.get_cursor(), 13);

        let v = Vec::from("[ -1 0 R ] \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("invalid ref-object id")));
        assert_eq!(pb.get_cursor(), 2);
    }

    #[test]
    fn dict() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                   1         2         3         4         5         6
//                         0123456789012345678901234567890123456789012345678901234567890123
        let v = Vec::from("<< /Entry [ 1 0 R ] \r\n >>".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 12, 17));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 10, 19);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Entry".as_bytes()), 3, 9), entval);
        let val = PDFObjT::Dict(DictT::new(map));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(val, 0, 25)));
        assert_eq!(pb.get_cursor(), vlen);

        // version with minimal whitespace
//                                   1         2         3         4         5         6
//                         0123456789012345678901234567890123456789012345678901234567890123
        let v = Vec::from("<</Entry [1 0 R]>>".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 10, 15));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 9, 16);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Entry".as_bytes()), 2, 8), entval);
        let val = PDFObjT::Dict(DictT::new(map));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(val, 0, 18)));
        assert_eq!(pb.get_cursor(), vlen);

        let v = Vec::from("<< /Entry [ 1 0 R ] /Entry \r\n >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(p.parse(&mut pb), Err(ErrorKind::GuardError("non-unique dictionary key")));
    }

    #[test]
    fn dict_lookup() {
        let entval = LocatedVal::new(PDFObjT::Null(()), 9, 16);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Entry".as_bytes()), 2, 8), entval);
        let val = PDFObjT::Dict(DictT::new(map));
        if let PDFObjT::Dict(d) = val {
            // the same located key
            assert!(d.map().contains_key(&LocatedVal::new(Vec::from("Entry".as_bytes()), 2, 8)));
            // different located key with the same val
            assert!(d.map().contains_key(&LocatedVal::new(Vec::from("Entry".as_bytes()), 0, 0)));
            // the same key val but not located
            assert!(d.map().contains_key(&Vec::from("Entry".as_bytes())));
        }
    }

    #[test]
    fn indirect() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                   1         2         3         4         5         6
//                         0123456789012345678901234567890123456789012345678901234567890123
        let v = Vec::from("1 0 obj << /Entry [ 1 0 R ] \r\n >> endobj".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 20, 25));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 18, 27);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Entry".as_bytes()), 11, 17), entval);
        let dict = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 8, 33));
        let obj = PDFObjT::Indirect(IndirectT::new(1, 0, Rc::clone(&dict)));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(obj, 0, 40)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(ctxt.lookup_obj((1,0)), Some(dict.borrow()));
    }

    #[test]
    fn stream() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                   1         2         3         4         5         6
//                         0123456789012345678901234567890123456789012345678901234567890123
        let v = Vec::from("1 0 obj << /Entry [ 1 0 R ] >> stream\n junk \nendstream\nendobj".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Reference(ReferenceT::new(1, 0)), 20, 25));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 18, 27);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Entry".as_bytes()), 11, 17), entval);
        let dict = LocatedVal::new(DictT::new(map), 8, 30);
        let content = LocatedVal::new(Vec::from(" junk ".as_bytes()), 31, 54);
        let stream = Rc::new(LocatedVal::new(PDFObjT::Stream(StreamT::new(dict, content)), 8, 54));
        let obj = PDFObjT::Indirect(IndirectT::new(1, 0, Rc::clone(&stream)));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(obj, 0, 61)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(ctxt.lookup_obj((1,0)), Some(stream.borrow()));
    }

    #[test]
    fn test_obj_no_embedded_comment() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                   1         2         3         4         5         6
//                         0123456789012345678901234567890123456789012345678901234567890123
        let v = Vec::from("1 0 obj  \n<<  /Type /Catalog\n  /Pages 2 0 R\n>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Type".as_bytes()), 14, 19),
                   LocatedVal::new(PDFObjT::Name(Vec::from("Catalog".as_bytes())), 20, 28));
        map.insert(LocatedVal::new(Vec::from("Pages".as_bytes()), 31, 37),
                   LocatedVal::new(PDFObjT::Reference(ReferenceT::new(2, 0)), 38, 43));
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 10, 46));
        let o = PDFObjT::Indirect(IndirectT::new(1, 0, Rc::clone(&d)));
        assert_eq!(val, Ok(LocatedVal::new(o, 0, 53)));
        assert_eq!(ctxt.lookup_obj((1,0)), Some(d.borrow()));
    }

    #[test]
    fn test_dict_null_value() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                    1         2          3         4           5         6
//                         012345678 9012345678901234567 89012345678901 234 5678901234567890123
        let v = Vec::from("1 0 obj  \n<<  /Type /Catalog\n  /Pages null\n>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Type".as_bytes()), 14, 19),
                   LocatedVal::new(PDFObjT::Name(Vec::from("Catalog".as_bytes())), 20, 28));
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 10, 45));
        let o = PDFObjT::Indirect(IndirectT::new(1, 0, Rc::clone(&d)));
        assert_eq!(val, Ok(LocatedVal::new(o, 0, 52)));
        assert_eq!(ctxt.lookup_obj((1,0)), Some(d.borrow()));
    }

    #[test]
    fn test_obj_embedded_comment() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                   1         2         3         4         5         6         7
//                         01234567890123456789012345678901234567890123456789012345678901234567890
        let v = Vec::from("1 0 obj  % entry point\n<<  /Type /Catalog\n  /Pages 2 0 R\n>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("Type".as_bytes()), 27, 32),
                   LocatedVal::new(PDFObjT::Name(Vec::from("Catalog".as_bytes())), 33, 41));
        map.insert(LocatedVal::new(Vec::from("Pages".as_bytes()), 44, 50),
                   LocatedVal::new(PDFObjT::Reference(ReferenceT::new(2, 0)), 51, 56));
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 23, 59));
        let o = PDFObjT::Indirect(IndirectT::new(1, 0, Rc::clone(&d)));
        assert_eq!(val, Ok(LocatedVal::new(o, 0, 66)));
        assert_eq!(ctxt.lookup_obj((1,0)), Some(d.borrow()));
    }

    #[test]
    fn test_obj_nonunique() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                   1         2         3         4         5         6
//                         0123456789012345678901234567890123456789012345678901234567890123
        let v = Vec::from("1 0 obj  \n<<  /Type 1 0 obj true endobj>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        assert_eq!(val, Err(ErrorKind::GuardError("non-unique object id")));
    }

    #[test]
    // Tests extracted from Peter Wyatt's webinar slides.
    fn test_pdf_expert_dict() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                                    1         2         3          4
//                         01234567 890123456789012345678901234 56789012
        let v = Vec::from("10 0 obj\n[/<><</[]>>()[[]]-.1/+0]%]\nendobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut objs = Vec::new();
        objs.push(LocatedVal::new(PDFObjT::Name(Vec::from("")), 10, 11));
        objs.push(LocatedVal::new(PDFObjT::String(Vec::from("")), 11, 13));
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from(""), 15, 16),
                   LocatedVal::new(PDFObjT::Array(ArrayT::new(Vec::new())), 16, 18));
        objs.push(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 13, 20));
        objs.push(LocatedVal::new(PDFObjT::String(Vec::from("")), 20, 22));
        let ea = LocatedVal::new(PDFObjT::Array(ArrayT::new(Vec::new())), 23, 25);
        objs.push(LocatedVal::new(PDFObjT::Array(ArrayT::new(vec![ea])), 22, 26));
        objs.push(LocatedVal::new(PDFObjT::Real(RealT::new(-1, 10)), 26, 29));
        objs.push(LocatedVal::new(PDFObjT::Name(Vec::from("+0")), 29, 32));
        let a = Rc::new(LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 9, 33));
        let o = PDFObjT::Indirect(IndirectT::new(10, 0, Rc::clone(&a)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(val, Ok(LocatedVal::new(o, 0, vlen)));
        assert_eq!(ctxt.lookup_obj((10,0)), Some(a.borrow()));

        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                         012345678901234567890
        let v = Vec::from("10 0 obj<<//>>endobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("".as_bytes()), 10, 11),
                   LocatedVal::new(PDFObjT::Name(Vec::from("".as_bytes())), 11, 12));
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 8, 14));
        let o = PDFObjT::Indirect(IndirectT::new(10, 0, Rc::clone(&d)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(val, Ok(LocatedVal::new(o, 0, vlen)));
        assert_eq!(ctxt.lookup_obj((10,0)), Some(d.borrow()));

        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
//                         0123456789012345678901
        let v = Vec::from("11 0 obj<</<>>>endobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = HashMap::new();
        map.insert(LocatedVal::new(Vec::from("".as_bytes()), 10, 11),
                   LocatedVal::new(PDFObjT::String(Vec::from("".as_bytes())), 11, 13));
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 8, 15));
        let o = PDFObjT::Indirect(IndirectT::new(11, 0, Rc::clone(&d)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(val, Ok(LocatedVal::new(o, 0, vlen)));
        assert_eq!(ctxt.lookup_obj((11,0)), Some(d.borrow()));

        // TODO: handle empty values
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
        let v = Vec::from("12 0 obj/ endobj");
        let mut pb = ParseBuffer::new(v);
        let _val = p.parse(&mut pb);
    }
}
