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

// Basic PDF objects.

use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, Location, ParseBufferT, ParseResult, ParsleyParser,
};
use super::pdf_prim::{
    Boolean, Comment, HexString, IntegerP, IntegerT, NameP, NameT, Null, RawLiteralString, RealP,
    RealT, StreamContentP, StreamContentT, WhitespaceEOL,
};

// Object locations in the PDF file.  This will need to become
// hierarchical to handle nested object streams.

pub struct PDFLocation {
    start: usize,
    end:   usize,
}

impl Location for PDFLocation {
    fn loc_start(&self) -> usize { self.start }
    fn loc_end(&self) -> usize { self.end }
}

// PDF object parsing context.  This keeps track of information
// collected during parsing.

pub struct PDFObjContext {
    // Input file
    file:                     String,
    // Maps object identifiers to their objects.
    defns:                    BTreeMap<(usize, usize), Rc<LocatedVal<PDFObjT>>>,
    // whether the document is encrypted
    encrypted:                bool,
    // Tracks the recursion depth.
    max_depth:                usize,
    cur_depth:                usize,
    // customized strictness
    eol_after_stream_content: bool,
}

impl PDFObjContext {
    pub fn new(file: &str, max_depth: usize) -> PDFObjContext {
        PDFObjContext {
            file: String::from(file),
            defns: BTreeMap::new(),
            encrypted: false,
            max_depth,
            cur_depth: 0,
            eol_after_stream_content: false, // not strict
        }
    }
    pub fn file(&self) -> &str { &self.file }
    pub fn register_obj(&mut self, p: &LocatedVal<IndirectT>) -> Option<Rc<LocatedVal<PDFObjT>>> {
        self.defns
            .insert((p.val().num(), p.val().gen()), Rc::clone(p.val().obj()))
    }
    pub fn lookup_obj(&self, oid: (usize, usize)) -> Option<&Rc<LocatedVal<PDFObjT>>> {
        self.defns.get(&oid)
    }
    pub fn set_encrypted(&mut self) { self.encrypted = true; }
    pub fn is_encrypted(&self) -> bool { self.encrypted }
    pub fn enter_obj(&mut self) -> bool {
        if self.cur_depth == self.max_depth {
            false
        } else {
            self.cur_depth += 1;
            true
        }
    }
    pub fn leave_obj(&mut self) {
        assert!(self.cur_depth != 0);
        self.cur_depth -= 1;
    }
    pub fn depth(&self) -> usize { self.cur_depth }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayT {
    objs: Vec<Rc<LocatedVal<PDFObjT>>>,
}

impl ArrayT {
    pub fn new(objs: Vec<Rc<LocatedVal<PDFObjT>>>) -> ArrayT { ArrayT { objs } }
    pub fn objs(&self) -> &[Rc<LocatedVal<PDFObjT>>] { self.objs.as_slice() }
}

struct ArrayP<'a> {
    ctxt: &'a mut PDFObjContext,
}

impl ArrayP<'_> {
    pub fn new(ctxt: &mut PDFObjContext) -> ArrayP { ArrayP { ctxt } }
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<LocatedVal<ArrayT>> {
        let start = buf.get_cursor();
        if let Err(e) = buf.exact(b"[") {
            let msg = format!("not at array object: {}", e.val());
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            return Err(locate_value(err, start, end))
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
            if buf.exact(b"]").is_err() {
                let o = parse_pdf_obj(&mut self.ctxt, buf)?;
                objs.push(Rc::new(o));
            } else {
                end = true;
            }
        }
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ArrayT::new(objs), start, end))
    }
}

/* Wrap keys to get legible displays when possible. */
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DictKey(Vec<u8>);
impl DictKey {
    pub fn new(v: Vec<u8>) -> DictKey { DictKey(v) }
}
impl Clone for DictKey {
    fn clone(&self) -> Self { DictKey(self.0.clone()) }
}
impl std::fmt::Debug for DictKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.0) {
            Ok(s) => f.write_str(s),
            Err(_) => f.debug_list().entries(self.0.iter()).finish(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DictT {
    map: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>>,
}

impl DictT {
    pub fn new(map: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>>) -> DictT { DictT { map } }
    pub fn map(&self) -> &BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>> { &self.map }
    pub fn map_mut(&mut self) -> &mut BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>> { &mut self.map }
    pub fn get(&self, k: &[u8]) -> Option<&Rc<LocatedVal<PDFObjT>>> {
        self.map.get(&DictKey::new(Vec::from(k)))
    }
    // conveniences:
    // get the usize value of a key
    pub fn get_usize(&self, k: &[u8]) -> Option<usize> {
        // TODO: the option return-type does not allow distinguishing
        // between a key that's not present, from one that is present
        // but with an invalid (non-usize) integer.
        self.get(k).and_then(|lobj| match lobj.val() {
            PDFObjT::Integer(i) => {
                if i.is_usize() {
                    Some(i.usize_val())
                } else {
                    None
                }
            },
            _ => None,
        })
    }
    // get the name value of a key
    pub fn get_name(&self, k: &[u8]) -> Option<&[u8]> {
        self.get(k).and_then(|lobj| match lobj.val() {
            PDFObjT::Name(n) => Some(n.val()),
            _ => None,
        })
    }
    pub fn get_name_obj(&self, k: &[u8]) -> Option<NameT> {
        self.get(k).and_then(|lobj| match lobj.val() {
            PDFObjT::Name(n) => Some(n.clone()),
            _ => None,
        })
    }
    // get the array value of a key
    pub fn get_array(&self, k: &[u8]) -> Option<&ArrayT> {
        self.get(k).and_then(|lobj| match lobj.val() {
            PDFObjT::Array(a) => Some(a),
            _ => None,
        })
    }
    // get the dictionary value of a key
    pub fn get_dict(&self, k: &[u8]) -> Option<&DictT> {
        self.get(k).and_then(|lobj| match lobj.val() {
            PDFObjT::Dict(d) => Some(d),
            _ => None,
        })
    }
}

pub struct DictP<'a> {
    ctxt: &'a mut PDFObjContext,
}

impl DictP<'_> {
    pub fn new(ctxt: &mut PDFObjContext) -> DictP { DictP { ctxt } }
    pub fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<DictT> {
        buf.exact(b"<<")?;
        let mut end = false;
        let mut map = BTreeMap::new();
        let mut names = HashSet::new();
        while !end {
            // Need more precise handling of whitespace to
            // differentiate between legal and illegal empty
            // whitespace.  This will be easier when auto-generated;
            // for now in the handwritten case, just be close enough.
            let mut ws = WhitespaceEOL::new(true); // allow empty whitespace for now
            ws.parse(buf)?;

            if buf.exact(b">>").is_err() {
                let mut p = NameP;
                let n = p.parse(buf)?;
                // Construct a normalized name usable as a key.
                let key = n.val().normalize();
                if names.contains(&key) {
                    let msg = format!("non-unique dictionary key: {}", n.val().as_string());
                    let err = ErrorKind::GuardError(msg);
                    return Err(n.place(err))
                }

                // do not require whitespace between key/value pairs
                let mut ws = WhitespaceEOL::new(true);
                ws.parse(buf)?;

                let o = parse_pdf_obj(&mut self.ctxt, buf)?;

                // Entries with 'null' values are treated as though
                // the entry does not exist.
                if let PDFObjT::Null(_) = o.val() {
                    // Drop the entry.
                } else {
                    names.insert(key.clone());
                    map.insert(DictKey(key), Rc::new(o));
                }
            } else {
                end = true;
            }
        }
        Ok(DictT::new(map))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StreamT {
    dict:   Rc<LocatedVal<DictT>>,
    stream: LocatedVal<StreamContentT>,
}

pub struct Filter<'a> {
    name:    NameT,
    options: Option<&'a DictT>,
}

impl Filter<'_> {
    fn new(name: NameT, options: Option<&DictT>) -> Filter { Filter { name, options } }
    pub fn name(&self) -> &NameT { &self.name }
    pub fn options(&self) -> &Option<&DictT> { &self.options }
}

impl StreamT {
    pub fn new(dict: Rc<LocatedVal<DictT>>, stream: LocatedVal<StreamContentT>) -> StreamT {
        StreamT { dict, stream }
    }
    pub fn dict(&self) -> &Rc<LocatedVal<DictT>> { &self.dict }
    pub fn stream(&self) -> &LocatedVal<StreamContentT> { &self.stream }

    pub fn filters(&self) -> ParseResult<Vec<Filter>> {
        let mut filters = Vec::new();
        // check for the single filter case
        let f = self.dict.val().get_name_obj(b"Filter");
        if let Some(name) = f {
            // There should be an optional single dictionary
            // value as filter param.
            match self.dict.val().get_dict(b"DecodeParms") {
                Some(d) => filters.push(Filter::new(name, Some(d))),
                None => {
                    // Ensure there is no array value.
                    if self.dict.val().get_array(b"DecodeParms").is_some() {
                        let msg = "Mismatched Filter and DecodeParms in stream".to_string();
                        let err = ErrorKind::GuardError(msg);
                        return Err(self.dict.place(err))
                    }
                    filters.push(Filter::new(name, None))
                },
            }
            return Ok(filters)
        }
        // check the array case
        let fa_opt = self.dict.val().get_array(b"Filter");
        if let Some(fa) = fa_opt {
            match self.dict.val().get_array(b"DecodeParms") {
                Some(da) => {
                    if da.objs().len() != fa.objs().len() {
                        let msg =
                            "Mismatched lengths for Filter and DecodeParms of stream".to_string();
                        let err = ErrorKind::GuardError(msg);
                        return Err(self.dict.place(err))
                    }
                    for (f, d) in fa.objs().iter().zip(da.objs().iter()) {
                        match (f.val(), d.val()) {
                            (PDFObjT::Name(name), PDFObjT::Null(_)) => {
                                filters.push(Filter::new(name.clone(), None))
                            },
                            (PDFObjT::Name(name), PDFObjT::Dict(ref d)) => {
                                filters.push(Filter::new(name.clone(), Some(d)))
                            },
                            (PDFObjT::Name(_), _) => {
                                let msg = "Invalid objects in DecodeParms of stream".to_string();
                                let err = ErrorKind::GuardError(msg);
                                return Err(self.dict.place(err))
                            },
                            (_, _) => {
                                let msg = "Invalid objects in Filter of stream".to_string();
                                let err = ErrorKind::GuardError(msg);
                                return Err(self.dict.place(err))
                            },
                        }
                    }
                },
                None => {
                    // Ensure that all are name objects.
                    for f in fa.objs() {
                        match f.val() {
                            PDFObjT::Name(name) => filters.push(Filter::new(name.clone(), None)),
                            _ => {
                                let msg = "Invalid objects in Filter of stream".to_string();
                                let err = ErrorKind::GuardError(msg);
                                return Err(self.dict.place(err))
                            },
                        }
                    }
                },
            }
        }
        Ok(filters)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ReferenceT {
    num: usize,
    gen: usize,
}

impl ReferenceT {
    pub fn new(num: usize, gen: usize) -> ReferenceT { ReferenceT { num, gen } }
    pub fn num(&self) -> usize { self.num }
    pub fn gen(&self) -> usize { self.gen }
    pub fn id(&self) -> (usize, usize) { (self.num, self.gen) }
}

struct ReferenceP;

impl ReferenceP {
    fn parse(self, buf: &mut dyn ParseBufferT) -> ParseResult<ReferenceT> {
        let mut int = IntegerP;
        let mut ws = WhitespaceEOL::new(true);

        let mut cursor = buf.get_cursor();
        let num = int.parse(buf)?;
        if !(num.val().is_zero() || num.val().is_usize()) {
            let msg = format!(
                "invalid or unsupported ref-object id: {}",
                num.val().int_val()
            );
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            buf.set_cursor(cursor);
            return Err(locate_value(err, cursor, end))
        }
        ws.parse(buf)?;

        cursor = buf.get_cursor();
        let gen = int.parse(buf)?;
        if !(gen.val().is_zero() || gen.val().is_usize()) {
            let msg = format!(
                "invalid or unsupported ref-object generation: {}",
                gen.val().int_val()
            );
            let err = ErrorKind::GuardError(msg);
            let end = buf.get_cursor();
            buf.set_cursor(cursor);
            return Err(locate_value(err, cursor, end))
        }
        ws.parse(buf)?;
        if let Err(e) = buf.exact(b"R") {
            let err = ErrorKind::GuardError("invalid reference tag".to_string());
            return Err(e.place(err))
        }

        Ok(ReferenceT::new(
            num.val().usize_val(),
            gen.val().usize_val(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PDFObjT {
    Array(ArrayT),
    Dict(DictT),
    Stream(StreamT),
    Reference(ReferenceT),
    Boolean(bool),
    String(Vec<u8>),
    Name(NameT),
    Null(()),
    Comment(Vec<u8>),
    Integer(IntegerT),
    Real(RealT),
}

// Private to ensure that the parse_pdf_obj wrapper is used.
struct PDFObjP<'a> {
    ctxt: &'a mut PDFObjContext,
}

impl PDFObjP<'_> {
    fn new(ctxt: &mut PDFObjContext) -> PDFObjP { PDFObjP { ctxt } }

    // The top-level basic object parser, as an internal helper.  Note
    // that this does not parse streams, even though they are 'basic'
    // objects according to the PDF spec, since they can only appear
    // within indirect/labelled objects, and hence do not nest.  This
    // parser deals with basic objects that can be found at any
    // nesting.
    fn parse_internal(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<PDFObjT> {
        let c = buf.peek();
        match c {
            None => {
                let start = buf.get_cursor();
                let err = ErrorKind::EndOfBuffer;
                Err(locate_value(err, start, start))
            },

            Some(116) | Some(102) => {
                // 't' | 'f'
                let mut bp = Boolean;
                let b = bp.parse(buf)?;
                Ok(PDFObjT::Boolean(b.unwrap()))
            },
            Some(110) => {
                // 'n'
                let mut np = Null;
                let n = np.parse(buf)?;
                Ok({
                    n.unwrap();
                    PDFObjT::Null(())
                })
            },
            Some(40) => {
                // '('
                let mut rp = RawLiteralString;
                let r = rp.parse(buf)?;
                Ok(PDFObjT::String(r.unwrap()))
            },
            Some(37) => {
                // '%'
                let mut cp = Comment;
                let c = cp.parse(buf)?;
                Ok(PDFObjT::Comment(c.unwrap()))
            },
            Some(47) => {
                // '/'
                let mut np = NameP;
                let n = np.parse(buf)?;
                Ok(PDFObjT::Name(n.unwrap()))
            },
            Some(91) => {
                // '['
                let mut ap = ArrayP::new(&mut self.ctxt);
                let a = ap.parse(buf)?;
                Ok(PDFObjT::Array(a.unwrap()))
            },
            Some(60) => {
                // '<'
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
                    },
                }
            },
            Some(b) => {
                if !b.is_ascii_digit()
                    && b != 45 // '-' to handle negative numbers
                    && b != 46
                // '.' to handle reals
                {
                    let start = buf.get_cursor();
                    let err = ErrorKind::GuardError("not at PDF object".to_string());
                    return Err(locate_value(err, start, start))
                }
                let cursor = buf.get_cursor();

                // We have to distinguish between an indirect
                // reference and a real number.  The first will always
                // have two integer numbers as a prefix.

                let mut real = RealP;
                let mut int = IntegerP;
                let mut ws = WhitespaceEOL::new(false); // no empty whitespace

                // Check if we are at a real.
                let r = real.parse(buf)?;
                if !r.val().is_integer() {
                    return Ok(PDFObjT::Real(r.unwrap()))
                }

                // We parsed the first integer.
                let n1 = IntegerT::new(r.val().numerator());
                let n1_end_cursor = buf.get_cursor();

                // Skip past non-empty whitespace.
                if ws.parse(buf).is_err() {
                    // We've already parsed a number, so set the
                    // cursor past that and return it.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }

                // Get the second integer.
                let n2 = int.parse(buf);
                if n2.is_err() {
                    // See above comment.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }

                // Skip past non-empty whitespace.
                if ws.parse(buf).is_err() {
                    // We've already parsed the first number, so set the
                    // cursor past that and return it.
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }

                // We have now seen two integers, so this could be an
                // indirect reference.
                let prefix = buf.check_prefix(b"R");
                if prefix.is_err() {
                    buf.set_cursor(n1_end_cursor);
                    return Ok(PDFObjT::Integer(n1))
                }
                if prefix.unwrap() {
                    // This looks like an indirect reference.  Rewind
                    // and call its parser (though we could optimize
                    // this case since we've actually already parsed
                    // it.).
                    buf.set_cursor(cursor);

                    let p = ReferenceP;
                    return Ok(PDFObjT::Reference(p.parse(buf)?))
                }

                // Fallback case: these were two integers after all;
                // rewind to the first.
                buf.set_cursor(n1_end_cursor);
                Ok(PDFObjT::Integer(n1))
            },
        }
    }
}

impl ParsleyParser for PDFObjP<'_> {
    type T = LocatedVal<PDFObjT>;

    // The top-level object parser.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        // First, consume possibly empty whitespace.
        // TODO: what about EOL?
        let mut ws = WhitespaceEOL::new(true);
        ws.parse(buf)?;

        let start = buf.get_cursor();
        let val = self.parse_internal(buf)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(val, start, end))
    }
}

// Wrapper to handle the recursion depth.
pub fn parse_pdf_obj(
    ctxt: &mut PDFObjContext, buf: &mut dyn ParseBufferT,
) -> ParseResult<LocatedVal<PDFObjT>> {
    if ctxt.enter_obj() {
        let mut p = PDFObjP::new(ctxt);
        let o = p.parse(buf);
        ctxt.leave_obj();
        o
    } else {
        let start = buf.get_cursor();
        let msg = format!("max recursion bound {} exceeded", ctxt.depth());
        let err = ErrorKind::GuardError(msg);
        Err(locate_value(err, start, start))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndirectT {
    num: usize,
    gen: usize,
    obj: Rc<LocatedVal<PDFObjT>>,
}

impl IndirectT {
    pub fn new(num: usize, gen: usize, obj: Rc<LocatedVal<PDFObjT>>) -> IndirectT {
        IndirectT { num, gen, obj }
    }
    pub fn num(&self) -> usize { self.num }
    pub fn gen(&self) -> usize { self.gen }
    pub fn obj(&self) -> &Rc<LocatedVal<PDFObjT>> { &self.obj }
}

pub struct IndirectP<'a> {
    ctxt: &'a mut PDFObjContext,
}

fn convert_stream_length(obj: &LocatedVal<PDFObjT>) -> ParseResult<usize> {
    match obj.val() {
        PDFObjT::Integer(i) => {
            if i.is_usize() {
                Ok(i.usize_val())
            } else {
                let err =
                    ErrorKind::GuardError("unsupported Length specified for stream".to_string());
                Err(obj.place(err))
            }
        },
        _ => {
            let err = ErrorKind::GuardError("invalid Length specified for stream".to_string());
            Err(obj.place(err))
        },
    }
}

impl IndirectP<'_> {
    pub fn new(ctxt: &mut PDFObjContext) -> IndirectP { IndirectP { ctxt } }
    fn parse_internal(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<LocatedVal<IndirectT>> {
        let mut int = IntegerP;
        let mut ws = WhitespaceEOL::new(true);

        let start = buf.get_cursor();
        let mut cursor = start;
        let num = int.parse(buf)?;
        if !num.val().is_usize() {
            let msg = format!("invalid or unsupported object id: {}", num.val().int_val());
            let err = ErrorKind::GuardError(msg);
            buf.set_cursor(cursor);
            return Err(num.place(err))
        }
        ws.parse(buf)?;
        cursor = buf.get_cursor();
        let gen = int.parse(buf)?;
        if !(gen.val().is_zero() || gen.val().is_usize()) {
            let msg = format!(
                "invalid or unsupported object generation: {}",
                gen.val().int_val()
            );
            let err = ErrorKind::GuardError(msg);
            buf.set_cursor(cursor);
            return Err(gen.place(err))
        }
        ws.parse(buf)?;
        if let Err(e) = buf.exact(b"obj") {
            let err = ErrorKind::GuardError("invalid object tag".to_string());
            return Err(e.place(err))
        }
        ws.parse(buf)?;

        let o = parse_pdf_obj(&mut self.ctxt, buf)?;

        // If we parsed a dictionary, check whether this could be a
        // stream object.
        let obj =
            // This is ugly code and should be cleaned up.
            if let PDFObjT::Dict(_) = o.val() {
                let mut ws = WhitespaceEOL::new(true); // allow empty whitespace
                ws.parse(buf)?;
                if buf.check_prefix(b"stream")? {
                    // This is indeed a stream object.
                    let dict_start = o.loc_start();
                    let dict_end = o.loc_end();
                    if let PDFObjT::Dict(dict) = o.unwrap() {
                        // In real files, the Length is either a
                        // number, or a reference to a number object.
                        // If we find a reference, the reference could
                        // aleady be in the context, or it might not
                        // yet have been parsed.
                        let length = match dict.get(b"Length") {
                            None => {
                                let err = ErrorKind::GuardError("no Length specified for stream"
                                                                .to_string());
                                return Err(LocatedVal::new(err, dict_start, dict_end))
                            },
                            Some(l) => match l.val() {
                                PDFObjT::Integer(_) =>
                                    convert_stream_length(l)?,
                                PDFObjT::Reference(r) =>
                                    match self.ctxt.lookup_obj(r.id()) {
                                        Some(o) => convert_stream_length(o)?,
                                        None => {
                                            let err = ErrorKind::InsufficientContext;
                                            return Err(l.place(err))
                                        }
                                    },
                                _ =>  {
                                    let err = ErrorKind::GuardError("invalid Length specified for stream"
                                                                    .to_string());
                                    return Err(LocatedVal::new(err, dict_start, dict_end))
                                },
                            }
                        };
                        let dict = LocatedVal::new(dict, dict_start, dict_end);
                        let mut s = StreamContentP::new(length, self.ctxt.eol_after_stream_content);
                        let stream = s.parse(buf)?;
                        let start = dict_start;
                        let end = stream.loc_end();
                        let obj = PDFObjT::Stream(StreamT { dict: Rc::new(dict), stream });
                        LocatedVal::new(obj, start, end)
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

        if let Err(e) = buf.exact(b"endobj") {
            let err = ErrorKind::GuardError("invalid endobject tag".to_string());
            return Err(e.place(err))
        }

        let end = buf.get_cursor();
        let obj = Rc::new(obj);
        let ind = IndirectT::new(
            num.val().usize_val(),
            gen.val().usize_val(),
            Rc::clone(&obj),
        );
        let ind = LocatedVal::new(ind, start, end);
        match self.ctxt.register_obj(&ind) {
            None => Ok(ind),
            Some(old) => {
                // Note that this location is inside any 'n g obj' prefix for the indirect
                // object.
                let loc = old.start();
                let msg = format!(
                    "non-unique object id ({}, {}), first found near offset {}",
                    num.val().int_val(),
                    gen.val().int_val(),
                    loc
                );
                let err = ErrorKind::GuardError(msg);
                let end = buf.get_cursor();
                Err(locate_value(err, start, end))
            },
        }
    }
}

impl ParsleyParser for IndirectP<'_> {
    type T = LocatedVal<IndirectT>;

    // The top-level indirect object parser.
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        // First, consume possibly empty whitespace.
        // TODO: what about EOL?
        let mut ws = WhitespaceEOL::new(true);
        ws.parse(buf)?;

        let val = self.parse_internal(buf)?;
        Ok(val)
    }
}

pub fn parse_pdf_indirect_obj(
    ctxt: &mut PDFObjContext, buf: &mut dyn ParseBufferT,
) -> ParseResult<LocatedVal<IndirectT>> {
    let mut p = IndirectP::new(ctxt);
    p.parse(buf)
}

#[cfg(test)]
mod test_pdf_obj {
    use super::super::super::pcore::parsebuffer::{
        locate_value, ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParsleyParser,
    };
    use super::super::pdf_prim::{IntegerT, NameT, RealT, StreamContentT};
    use super::{
        parse_pdf_obj, ArrayT, DictKey, DictT, IndirectP, IndirectT, PDFObjContext, PDFObjT,
        ReferenceT, StreamT,
    };
    use std::borrow::Borrow;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn empty() {
        let mut ctxt = mk_new_context();

        let v = Vec::from("".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::EndOfBuffer, 0, 0);
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn comment() {
        let mut ctxt = mk_new_context();

        // Comments are essentially whitespace.
        let v = Vec::from("\r\n %PDF-1.0 \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(ErrorKind::EndOfBuffer, 0, 14);
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 14);
    }

    #[test]
    #[rustfmt::skip]
    fn reference() {
        let mut ctxt = mk_new_context();

        //                  0 12345678 9 0
        let v = Vec::from("\r\n 1 0 R \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Reference(ReferenceT::new(1, 0));
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Ok(LocatedVal::new(val, 3, 8)));
        assert_eq!(pb.get_cursor(), 8);

        //                  0 123456789 0 1
        let v = Vec::from("\r\n -1 0 R \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("invalid or unsupported ref-object id: -1".to_string()),
            5,
            7,
        );
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 3);

        //                  0 123456789 0 1
        let v = Vec::from("\r\n 1 -1 R \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("invalid or unsupported ref-object generation: -1".to_string()),
            7,
            9,
        );
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 5);
    }

    #[test]
    fn numbers() {
        let mut ctxt = mk_new_context();

        let v = Vec::from("1\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Integer(IntegerT::new(1));
        assert_eq!(
            parse_pdf_obj(&mut ctxt, &mut pb),
            Ok(LocatedVal::new(val, 0, 1))
        );
        assert_eq!(pb.get_cursor(), 1);

        let v = Vec::from("-1\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Integer(IntegerT::new(-1));
        assert_eq!(
            parse_pdf_obj(&mut ctxt, &mut pb),
            Ok(LocatedVal::new(val, 0, 2))
        );
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("0.1\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Real(RealT::new(1, 10));
        assert_eq!(
            parse_pdf_obj(&mut ctxt, &mut pb),
            Ok(LocatedVal::new(val, 0, 3))
        );
        assert_eq!(pb.get_cursor(), 3);

        let v = Vec::from("-0.1\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Real(RealT::new(-1, 10));
        assert_eq!(
            parse_pdf_obj(&mut ctxt, &mut pb),
            Ok(LocatedVal::new(val, 0, 4))
        );
        assert_eq!(pb.get_cursor(), 4);

        let v = Vec::from(".1\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Real(RealT::new(1, 10));
        assert_eq!(
            parse_pdf_obj(&mut ctxt, &mut pb),
            Ok(LocatedVal::new(val, 0, 2))
        );
        assert_eq!(pb.get_cursor(), 2);

        let v = Vec::from("-.1\r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = PDFObjT::Real(RealT::new(-1, 10));
        assert_eq!(
            parse_pdf_obj(&mut ctxt, &mut pb),
            Ok(LocatedVal::new(val, 0, 3))
        );
        assert_eq!(pb.get_cursor(), 3);
    }

    #[test]
    #[rustfmt::skip]
    fn array() {
        let mut ctxt = mk_new_context();

        //                 0123456789 0 1
        let v = Vec::from("[ 1 0 R ] \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Reference(ReferenceT::new(1, 0)),
            2,
            7,
        )));
        let val = PDFObjT::Array(ArrayT::new(objs));
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Ok(LocatedVal::new(val, 0, 9)));
        assert_eq!(pb.get_cursor(), 9);

        //                 0123 4567 890123 4 5
        let v = Vec::from("[ 1 \r 0 \n R ] \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Reference(ReferenceT::new(1, 0)),
            2,
            11,
        )));
        let val = PDFObjT::Array(ArrayT::new(objs));
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Ok(LocatedVal::new(val, 0, 13)));
        assert_eq!(pb.get_cursor(), 13);

        let v = Vec::from("[ -1 0 R ] \r\n".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("invalid or unsupported ref-object id: -1".to_string()),
            2,
            4,
        );
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Err(e));
        assert_eq!(pb.get_cursor(), 2);
    }

    #[test]
    #[rustfmt::skip]
    fn array_recursion() {
        let mut ctxt = PDFObjContext::new(2);

        let v = Vec::from("[[]]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb);
        let inner = PDFObjT::Array(ArrayT::new(Vec::new()));
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(inner, 1, 3)));
        let outer = PDFObjT::Array(ArrayT::new(objs));
        assert_eq!(obj, Ok(LocatedVal::new(outer, 0, 4)));

        let v = Vec::from("[[[]]]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = parse_pdf_obj(&mut ctxt, &mut pb);
        let e = locate_value(
            ErrorKind::GuardError("max recursion bound 2 exceeded".to_string()),
            2,
            2,
        );
        assert_eq!(val, Err(e));
    }

    #[test]
    #[rustfmt::skip]
    fn dict() {
        let mut ctxt = mk_new_context();

        //                          1         2
        //                012345678901234567890 1 2345
        let v = Vec::from("<< /Entry [ 1 0 R ] \r\n >>".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Reference(ReferenceT::new(1, 0)),
            12,
            17,
        )));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 10, 19);
        let mut map = BTreeMap::new();
        map.insert(DictKey::new(Vec::from("Entry".as_bytes())), Rc::new(entval));
        let val = PDFObjT::Dict(DictT::new(map));
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Ok(LocatedVal::new(val, 0, 25)));
        assert_eq!(pb.get_cursor(), vlen);

        // version with minimal whitespace
        //                           1
        //                 0123456789012345678
        let v = Vec::from("<</Entry [1 0 R]>>".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Reference(ReferenceT::new(1, 0)),
            10,
            15,
        )));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 9, 16);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Entry".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(entval),
        );
        let val = PDFObjT::Dict(DictT::new(map));
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Ok(LocatedVal::new(val, 0, 18)));
        assert_eq!(pb.get_cursor(), vlen);

        //                           1         2         3
        //                 0123456789012345678901234567890123
        let v = Vec::from("<< /Entry [ 1 0 R ] /Entry \r\n >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let e = locate_value(
            ErrorKind::GuardError("non-unique dictionary key: Entry".to_string()),
            20,
            26,
        );
        assert_eq!(parse_pdf_obj(&mut ctxt, &mut pb), Err(e));
    }

    #[test]
    #[rustfmt::skip]
    fn dict_recursion() {
        let mut ctxt = PDFObjContext::new(2);

        //                 01234567890123456
        let v = Vec::from("<</Inner << >>>>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb);
        let inner = PDFObjT::Dict(DictT::new(BTreeMap::new()));
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Inner".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(LocatedVal::new(inner, 9, 14)),
        );
        let outer = PDFObjT::Dict(DictT::new(map));
        assert_eq!(obj, Ok(LocatedVal::new(outer, 0, 16)));

        //                 0123456789012345678901234
        let v = Vec::from("<</Mid <</Inner <<>>>>>>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = parse_pdf_obj(&mut ctxt, &mut pb);
        let e = locate_value(
            ErrorKind::GuardError("max recursion bound 2 exceeded".to_string()),
            16,
            16,
        );
        assert_eq!(val, Err(e));
    }

    #[test]
    fn dict_lookup() {
        let entval = LocatedVal::new(PDFObjT::Null(()), 9, 16);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Entry".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(entval),
        );
        let val = PDFObjT::Dict(DictT::new(map));
        if let PDFObjT::Dict(d) = val {
            // the same located key
            //assert!(d.map().contains_key(&LocatedVal::new(Vec::from("Entry".as_bytes()),
            // 2, 8))); different located key with the same val
            //assert!(d.map().contains_key(&LocatedVal::new(Vec::from("Entry".as_bytes()),
            // 0, 0))); the same key val but not located
            assert!(d
                .map()
                .contains_key(&DictKey::new(Vec::from("Entry".as_bytes()))));
        }
    }

    #[test]
    fn dict_eq() {
        // Ensure dict equality is independent of ordering of map insertions.

        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Entry1".as_bytes()));
        let entval = LocatedVal::new(PDFObjT::Null(()), 9, 16);
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(entval),
        );
        let key = NameT::new(Vec::from("Entry2".as_bytes()));
        let entval = LocatedVal::new(PDFObjT::Boolean(true), 9, 16);
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(entval),
        );
        let l = PDFObjT::Dict(DictT::new(map));

        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Entry2".as_bytes()));
        let entval = LocatedVal::new(PDFObjT::Boolean(true), 9, 16);
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(entval),
        );
        let key = NameT::new(Vec::from("Entry1".as_bytes()));
        let entval = LocatedVal::new(PDFObjT::Null(()), 9, 16);
        map.insert(
            DictKey::new(LocatedVal::new(key, 2, 8).val().normalize()),
            Rc::new(entval),
        );
        let r = PDFObjT::Dict(DictT::new(map));

        assert_eq!(l, r);
    }

    #[test]
    #[rustfmt::skip]
    fn indirect() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                           1         2         3         4
        //                 012345678901234567890123456789012345678901
        let v = Vec::from("1 0 obj << /Entry [ 1 0 R ] \r\n >> endobj".as_bytes());
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Reference(ReferenceT::new(1, 0)),
            20,
            25,
        )));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 18, 27);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Entry".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 11, 17).val().normalize()),
            Rc::new(entval),
        );
        let dict = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 8, 33));
        let obj = IndirectT::new(1, 0, Rc::clone(&dict));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(obj, 0, 40)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(ctxt.lookup_obj((1, 0)), Some(dict.borrow()));
    }

    #[test]
    #[rustfmt::skip]
    fn stream() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //             1         2         3         4          5          6
        //   0123456789012345678901234567890123456789012345 6789012 3456789012 3
        let v = Vec::from(
            "1 0 obj << /Entry [ 1 0 R ] /Length 6>> stream\n junk \nendstream\nendobj".as_bytes(),
        );
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Reference(ReferenceT::new(1, 0)),
            20,
            25,
        )));
        let entval = LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 18, 27);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Entry".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 11, 17).val().normalize()),
            Rc::new(entval),
        );
        let key = NameT::new(Vec::from("Length".as_bytes()));
        let entval = LocatedVal::new(PDFObjT::Integer(IntegerT::new(6)), 36, 37);
        map.insert(
            DictKey::new(LocatedVal::new(key, 11, 17).val().normalize()),
            Rc::new(entval),
        );
        let dict = Rc::new(LocatedVal::new(DictT::new(map), 8, 39));
        let content = LocatedVal::new(StreamContentT::new(47, 6, Vec::from(" junk ".as_bytes())), 40, 63);
        let stream = Rc::new(LocatedVal::new(
            PDFObjT::Stream(StreamT::new(dict, content)),
            8,
            63,
        ));
        let obj = IndirectT::new(1, 0, Rc::clone(&stream));
        assert_eq!(p.parse(&mut pb), Ok(LocatedVal::new(obj, 0, 70)));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(ctxt.lookup_obj((1, 0)), Some(stream.borrow()));
    }

    #[test]
    #[rustfmt::skip]
    fn test_obj_no_embedded_comment() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                           1         2         3         4         5
        //                 0123456789012345678901234567890123456789012345678901234567
        let v = Vec::from("1 0 obj  \n<<  /Type /Catalog\n  /Pages 2 0 R\n>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Type".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 14, 19).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Name(NameT::new(Vec::from("Catalog".as_bytes()))),
                20,
                28,
            )),
        );
        let key = NameT::new(Vec::from("Pages".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 31, 37).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Reference(ReferenceT::new(2, 0)),
                38,
                43,
            )),
        );
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 10, 46));
        let o = IndirectT::new(1, 0, Rc::clone(&d));
        assert_eq!(val, Ok(LocatedVal::new(o, 0, 53)));
        assert_eq!(ctxt.lookup_obj((1, 0)), Some(d.borrow()));
    }

    #[test]
    #[rustfmt::skip]
    fn test_dict_null_value() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                            1         2          3         4           5
        //                 012345678 9012345678901234567 89012345678901 234 5678901
        let v = Vec::from("1 0 obj  \n<<  /Type /Catalog\n  /Pages null\n>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Type".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 14, 19).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Name(NameT::new(Vec::from("Catalog".as_bytes()))),
                20,
                28,
            )),
        );
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 10, 45));
        let o = IndirectT::new(1, 0, Rc::clone(&d));
        assert_eq!(val, Ok(LocatedVal::new(o, 0, 52)));
        assert_eq!(ctxt.lookup_obj((1, 0)), Some(d.borrow()));
    }

    #[test]
    #[rustfmt::skip]
    fn test_obj_embedded_comment() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //             1         2          3         4          5           6         7
        //   0123456789012345678901 2345678901234567890 123456789012345 678 901234567890
        let v = Vec::from(
            "1 0 obj  % entry point\n<<  /Type /Catalog\n  /Pages 2 0 R\n>>\nendobj".as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("Type".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 27, 32).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Name(NameT::new(Vec::from("Catalog".as_bytes()))),
                33,
                41,
            )),
        );
        let key = NameT::new(Vec::from("Pages".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 44, 50).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Reference(ReferenceT::new(2, 0)),
                51,
                56,
            )),
        );
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 23, 59));
        let o = IndirectT::new(1, 0, Rc::clone(&d));
        assert_eq!(val, Ok(LocatedVal::new(o, 0, 66)));
        assert_eq!(ctxt.lookup_obj((1, 0)), Some(d.borrow()));
    }

    #[test]
    #[rustfmt::skip]
    fn test_nested_indirect() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                            1         2         3         4
        //                 012345678 90123456789012345678901234567890 1234567
        let v = Vec::from("1 0 obj  \n<<  /Type 1 0 obj true endobj>>\nendobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let e = locate_value(
            ErrorKind::GuardError("not at name object".to_string()),
            22,
            22,
        );
        assert_eq!(val, Err(e));
    }

    #[test]
    #[rustfmt::skip]
    // Tests extracted from Peter Wyatt's webinar slides.
    fn test_pdf_expert_dict() {
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                            1         2         3          4
        //                 01234567 890123456789012345678901234 56789012
        let v = Vec::from("10 0 obj\n[/<><</[]>>()[[]]-.1/+0]%]\nendobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut objs = Vec::new();
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Name(NameT::new(Vec::from(""))),
            10,
            11,
        )));
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::String(Vec::from("")),
            11,
            13,
        )));
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from(""));
        map.insert(
            DictKey::new(LocatedVal::new(key, 15, 16).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Array(ArrayT::new(Vec::new())),
                16,
                18,
            )),
        );
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Dict(DictT::new(map)),
            13,
            20,
        )));
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::String(Vec::from("")),
            20,
            22,
        )));
        let ea = Rc::new(LocatedVal::new(
            PDFObjT::Array(ArrayT::new(Vec::new())),
            23,
            25,
        ));
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Array(ArrayT::new(vec![ea])),
            22,
            26,
        )));
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Real(RealT::new(-1, 10)),
            26,
            29,
        )));
        objs.push(Rc::new(LocatedVal::new(
            PDFObjT::Name(NameT::new(Vec::from("+0"))),
            29,
            32,
        )));
        let a = Rc::new(LocatedVal::new(PDFObjT::Array(ArrayT::new(objs)), 9, 33));
        let o = IndirectT::new(10, 0, Rc::clone(&a));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(val, Ok(LocatedVal::new(o, 0, vlen)));
        assert_eq!(ctxt.lookup_obj((10, 0)), Some(a.borrow()));

        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                 012345678901234567890
        let v = Vec::from("10 0 obj<<//>>endobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 10, 11).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::Name(NameT::new(Vec::from("".as_bytes()))),
                11,
                12,
            )),
        );
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 8, 14));
        let o = IndirectT::new(10, 0, Rc::clone(&d));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(val, Ok(LocatedVal::new(o, 0, vlen)));
        assert_eq!(ctxt.lookup_obj((10, 0)), Some(d.borrow()));

        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        //                 0123456789012345678901
        let v = Vec::from("11 0 obj<</<>>>endobj");
        let vlen = v.len();
        let mut pb = ParseBuffer::new(v);
        let val = p.parse(&mut pb);
        let mut map = BTreeMap::new();
        let key = NameT::new(Vec::from("".as_bytes()));
        map.insert(
            DictKey::new(LocatedVal::new(key, 10, 11).val().normalize()),
            Rc::new(LocatedVal::new(
                PDFObjT::String(Vec::from("".as_bytes())),
                11,
                13,
            )),
        );
        let d = Rc::new(LocatedVal::new(PDFObjT::Dict(DictT::new(map)), 8, 15));
        let o = IndirectT::new(11, 0, Rc::clone(&d));
        assert_eq!(pb.get_cursor(), vlen);
        assert_eq!(val, Ok(LocatedVal::new(o, 0, vlen)));
        assert_eq!(ctxt.lookup_obj((11, 0)), Some(d.borrow()));

        // TODO: handle empty values
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        let v = Vec::from("12 0 obj/ endobj");
        let mut pb = ParseBuffer::new(v);
        let _val = p.parse(&mut pb);
    }
}
