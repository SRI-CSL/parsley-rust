use std::rc::Rc;
use super::super::pcore::parsebuffer::{
    ParseBuffer, ParseBufferT, ParsleyParser, ParseResult, LocatedVal,
    ErrorKind, locate_value
};

use super::pdf_prim::{WhitespaceEOL, IntegerP, NameT};
use super::pdf_obj::{
    PDFObjContext, PDFObjP, PDFObjT, DictT, IndirectT, StreamT
};

type ObjStreamMetadata = Vec<(usize, usize)>; // (object#, offset) pairs
type ObjStreamContent  = Vec<LocatedVal<IndirectT>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjStreamT {
    dict: Rc<LocatedVal<DictT>>,
    objs: ObjStreamContent
}

impl ObjStreamT {
    pub fn new(dict: Rc<LocatedVal<DictT>>, objs: ObjStreamContent)
               -> ObjStreamT {
        ObjStreamT { dict, objs }
    }
    pub fn objs(&self) -> &[LocatedVal<IndirectT>] {
        self.objs.as_slice()
    }
}

pub struct ObjStreamP<'a> {
    ctxt: &'a mut PDFObjContext,
    dict: Rc<LocatedVal<DictT>>
}


impl ObjStreamP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext, dict: Rc<LocatedVal<DictT>>)
                   -> ObjStreamP<'a> {
        ObjStreamP { ctxt, dict }
    }

    fn get_dict_info(&self) -> ParseResult<(usize, usize)> {
        let stream_type = self.dict.val().get_name(b"Type");
        if stream_type.is_none() {
            let msg = format!("No valid /Type in object stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, self.dict.start(), self.dict.end()))
        }
        let stream_type = stream_type.unwrap();
        if stream_type != b"ObjStm" {
            let t_str = match std::str::from_utf8(stream_type) {
                Ok(v)  => v.to_string(),
                Err(e) => format!("(error: cannot convert to UTF-8: {})", e)
            };
            let msg = format!("Invalid /Type in object stream dictionary: {}", t_str);
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, self.dict.start(), self.dict.end()))
        }

        let num_objs = self.dict.val().get_usize(b"N");
        if num_objs.is_none() {
            let msg = format!("No valid /N in object stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, self.dict.start(), self.dict.end()))
        }
        let num_objs = num_objs.unwrap();

        let first = self.dict.val().get_usize(b"First");
        if first.is_none() {
            let msg = format!("No valid /First in object stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, self.dict.start(), self.dict.end()))
        }
        let first = first.unwrap();
        Ok((num_objs, first))
    }

    // The assumptions are that:
    // - the parsing view corresponds to the metadata section of the stream content,
    //   delimited by the /First offset
    // - /N provides the number of integer pairs expected in the metadata
    // - parsing is done until the expected metadata is consumed, or
    //   an end-of-buffer error; i.e. it is not illegal to have more
    //   data after the end of the metadata and before /First
    // [This assumes certain future fixes to the PDF spec.]
    fn parse_metadata(&mut self, buf: &mut dyn ParseBufferT, n: usize)
                      -> ParseResult<ObjStreamMetadata> {
        let mut obj_ofs = Vec::new();
        let mut last_ofs = 0; // to ensure offset ordering.
        loop {
            // Collect the object#/offset pairs.
            let mut int = IntegerP;
            let mut ws = WhitespaceEOL::new(true);
            ws.parse(buf)?;

            // get the object#
            let mut cursor = buf.get_cursor();
            let obj = int.parse(buf)?;
            if !obj.val().is_positive() {
                let msg = format!("invalid object id: {}", obj.val().int_val());
                let err = ErrorKind::GuardError(msg);
                buf.set_cursor(cursor);
                return Err(obj.place(err))
            }
            let obj = obj.val().usize_val();

            ws.parse(buf)?;
            // get the offset.
            cursor = buf.get_cursor();
            let ofs = int.parse(buf)?;
            // ensure that the offset is ordered.
            let ofs_val = ofs.val().usize_val();
            if ofs_val <= last_ofs && obj_ofs.len() > 0 {
                let msg = format!("offset {} of object {} is not greater than previous offset {}",
                                  ofs_val, obj, last_ofs);
                let err = ErrorKind::GuardError(msg);
                buf.set_cursor(cursor);
                return Err(ofs.place(err))
            }

            last_ofs = ofs_val;
            obj_ofs.push((obj, ofs_val));
            if obj_ofs.len() == n { break }
        }
        Ok(obj_ofs)
    }

    // The assumptions are that:
    // - the parsing view corresponds to the content section of the stream content,
    //   beginning at the /First offset
    // - parsing is done until the expected metadata is consumed, or
    //   an end-of-buffer error; i.e. it is not illegal to have more
    //   data after the end of the last offset specified in the metadata
    // [This assumes certain future fixes to the PDF spec.]
    fn parse_stream(&mut self, buf: &mut dyn ParseBufferT, meta: &ObjStreamMetadata)
                    -> ParseResult<ObjStreamContent> {
        let mut ws = WhitespaceEOL::new(true);
        let mut objs = Vec::new();
        for (i, (onum, ofs)) in meta.iter().enumerate() {
            // Ensure we are not past the specified offset.
            if buf.get_cursor() > *ofs {
                let msg = format!("parsed past offset {} for object #{} with id {}",
                                  ofs, i, onum);
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, *ofs, buf.get_cursor()))
            }
            ws.parse(buf)?;

            let mut p = PDFObjP::new(&mut self.ctxt);
            let start = buf.get_cursor();
            let o = p.parse(buf)?;
            let obj = IndirectT::new(*onum, 0, Rc::new(o));
            let end = buf.get_cursor();
            objs.push(LocatedVal::new(obj, start, end))
        }
        Ok(objs)
    }
}

impl ParsleyParser for ObjStreamP<'_> {
    type T = LocatedVal<ObjStreamT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let (num_objs, first) = self.get_dict_info()?;
        // First parse the metadata.
        let mut md_view = match ParseBuffer::restrict_view(buf, 0, first) {
            Some(v) => v,
            None => {
                let msg = format!("Unable to parse object-stream metadata, /First may be invalid: {}",
                                  first);
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, start, start))
            }
        };
        let meta = self.parse_metadata(&mut md_view, num_objs)?;
        // Then the content.
        let mut objs_view = match ParseBuffer::restrict_view_from(buf, first) {
            Some(v) => v,
            None => {
                let msg = format!("Unable to parse object-stream content, /First may be invalid: {}",
                                  first);
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, first, first))
            }
        };
        let objs = self.parse_stream(&mut objs_view, &meta)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ObjStreamT::new(Rc::clone(&self.dict), objs), start, end))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntStatus {
    Free { next: usize },
    InUse { file_ofs: usize },
    InStream { stream_obj: usize, obj_index: usize }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XrefStreamEntT {
    obj: usize,
    gen: usize,
    status: EntStatus
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XrefStreamT {
    dict: Rc<LocatedVal<DictT>>,
    ents: Vec<LocatedVal<XrefStreamEntT>>
}

impl XrefStreamT {
    pub fn new(dict: Rc<LocatedVal<DictT>>, ents: Vec<LocatedVal<XrefStreamEntT>>)
               -> XrefStreamT {
        XrefStreamT { dict, ents }
    }
}

pub struct XrefStreamP<'a> {
    stream: &'a StreamT
}

struct XrefStreamDictInfo<'a> {
    size: usize,
    pub prev: usize,
    index: Option<Vec<(usize, usize)>>,  // like pdf_file::XrefSubSectT but without entries
    widths: [usize; 3],
    filters: Vec<(NameT, Option<&'a DictT>)>
}

impl XrefStreamP<'_> {
    pub fn new(stream: &StreamT) -> XrefStreamP {
        XrefStreamP { stream }
    }

    fn get_dict_info(&self) -> ParseResult<XrefStreamDictInfo> {
        let dict = self.stream.dict();
        let stream_type = dict.val().get_name(b"Type");
        if stream_type.is_none() {
            let msg = format!("No /Type in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let stream_type = stream_type.unwrap();
        if stream_type != b"XRef" {
            let t_str = match std::str::from_utf8(stream_type) {
                Ok(v)  => v.to_string(),
                Err(e) => format!("(error: cannot convert to UTF-8: {})", e)
            };
            let msg = format!("Invalid /Type in xref stream dictionary: {}", t_str);
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }

        let size = dict.val().get_usize(b"Size");
        if size.is_none() {
            let msg = format!("No valid /Size in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let size = size.unwrap();

        let prev = dict.val().get_usize(b"Prev");
        if prev.is_none() {
            let msg = format!("No valid /Prev in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let prev = prev.unwrap();

        let idx = dict.val().get_array(b"Index");
        let mut index = Vec::new();
        if let Some(i) = idx {
            if i.objs().len() % 2 != 0 {
                let msg = format!("Invalid non-even length {} for /Index in xref stream dictionary.",
                                  i.objs().len());
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, dict.start(), dict.end()))
            }
            for (s, c) in i.objs().iter().step_by(2).zip(i.objs().iter().skip(1).step_by(2)) {
                if let (PDFObjT::Integer(s), PDFObjT::Integer(c)) = (s.val(), c.val()) {
                    index.push((s.usize_val(), c.usize_val()))
                } else {
                    let msg = format!("Invalid non-integer entries in /Index in xref stream dictionary.");
                    let err = ErrorKind::GuardError(msg);
                    return Err(locate_value(err, dict.start(), dict.end()))
                }
            }
            // TODO: ensure that subsections cannot overlap
        }

        let w = dict.val().get_array(b"W");
        if w.is_none() {
            let msg = format!("No valid /W in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let w = w.unwrap();
        if w.objs().len() != 3 {
            let msg = format!("Invalid length for /W in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let mut w_array = Vec::new();
        for o in w.objs() {
            if let PDFObjT::Integer(i) = o.val() {
                let sz = i.usize_val();
                // Implementation detail: do not handle integer widths larger than 4 bytes
                if sz > 4 {
                    let msg = format!("Cannot handle {}-byte integer in /W in xref stream dictionary.",
                                      sz);
                    let err = ErrorKind::GuardError(msg);
                    return Err(locate_value(err, dict.start(), dict.end()))
                }
                w_array.push(sz);
                continue;
            }
            let msg = format!("Invalid length for /W in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        if w_array[1] == 0 {
            let msg = format!("Invalid zero-width field #2 in /W in xref stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let widths = [w_array[0], w_array[1], w_array[2]];

        let filters = self.stream.filters()?;

        Ok(XrefStreamDictInfo { size, prev, index: Some(index), widths, filters })
    }

    fn parse_usize_with_width(&self, buf: &mut dyn ParseBufferT, width: usize)
                              -> ParseResult<usize> {
        let mut val: usize = 0;
        for _ in 0 .. width {
            let peek = buf.peek();
            if peek.is_none() {
                let cur = buf.get_cursor();
                return Err(locate_value(ErrorKind::EndOfBuffer, cur, cur))
            }
            let peek = peek.unwrap();
            val = (val << 8) | usize::from(peek);
            buf.incr_cursor();
        }
        Ok(val)
    }

    fn parse_stream(&self, buf: &mut dyn ParseBufferT, meta: &XrefStreamDictInfo)
                    -> ParseResult<Vec<LocatedVal<XrefStreamEntT>>> {
        let mut ents = Vec::new();
        let index = match meta.index {
            Some(ref i) => i.clone(),
            None        => vec![(0, meta.size)]
        };
        for (start_obj, count) in index.iter() {
            for c in 0 .. *count {
                let start = buf.get_cursor();
                // Field #1.
                let width = meta.widths[0];
                let typ =
                    if width == 0 {
                        1 // default: Type 1
                    } else {
                        let f = self.parse_usize_with_width(buf, width)?;
                        if f > 2 {
                            let cur = buf.get_cursor();
                            let msg = format!("Invalid type {} in field #1 of entry in xref stream.",
                                              f);
                            let err = ErrorKind::GuardError(msg);
                            return Err(locate_value(err, start, cur))
                        }
                        f
                    };
                // Field #2
                let width = meta.widths[1];
                let field2 = self.parse_usize_with_width(buf, width)?;
                // Field #3
                let width = meta.widths[1];
                let field3 =
                    if width > 0 {
                        self.parse_usize_with_width(buf, width)?
                    } else {
                        if typ == 2 {
                            // There is no default value for Type 2 entries.
                            let cur = buf.get_cursor();
                            let msg = format!("No Type 2 default for field #3 in xref stream.");
                            let err = ErrorKind::GuardError(msg);
                            return Err(locate_value(err, start, cur))
                        }
                        0
                    };

                // Construct the entry
                let (status, gen) =
                    match typ {
                        0 => (EntStatus::Free { next: field2 }, field3),
                        1 => (EntStatus::InUse { file_ofs: field2 }, field3),
                        2 => (EntStatus::InStream { stream_obj: field2, obj_index: field3 }, 0),
                        // we checked for valid entry typ above, so this should never happen
                        _ => panic!("unhandled entry type in xref stream")
                    };
                let ent = XrefStreamEntT { obj: start_obj + c, gen, status };
                let end = buf.get_cursor();
                ents.push(LocatedVal::new(ent, start, end));
            }
        }
        Ok(ents)
    }
}

impl ParsleyParser for XrefStreamP<'_> {
    type T = LocatedVal<XrefStreamT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let meta = self.get_dict_info()?;

        // TODO: handle filters
        if meta.filters.len() > 0 {
            let f = meta.filters[0].0.as_string();
            let msg = format!("Cannot handle filter {} in xref stream", f);
            let err = ErrorKind::GuardError(msg);
            return Err(self.stream.dict().place(err))
        }

        let ents = self.parse_stream(buf, &meta)?;
        let xref = XrefStreamT::new(Rc::clone(&self.stream.dict()), ents);
        let end = buf.get_cursor();
        Ok(LocatedVal::new(xref, start, end))
    }
}


#[cfg(test)]
mod test_object_stream {
    use std::rc::Rc;
    use std::collections::BTreeMap;
    use super::super::super::pcore::parsebuffer::{
        ParsleyParser, ParseBuffer, LocatedVal, ErrorKind, locate_value
    };
    use super::super::pdf_obj::{PDFObjContext, DictP, DictT, IndirectT, ArrayT, PDFObjT};
    use super::{ObjStreamP, ObjStreamT};

    fn make_dict_buf(n: usize, first: usize) -> Vec<u8> {
        let v = format!("<</Type /ObjStm /N {} /First {}>>", n, first);
        Vec::from(v.as_bytes())
    }
    fn make_dict(n: usize, first: usize) -> Rc<LocatedVal<DictT>> {
        let v = make_dict_buf(n, first);
        let len = v.len();
        let mut buf = ParseBuffer::new(v);
        let mut ctxt = PDFObjContext::new();
        let mut dp = DictP::new(&mut ctxt);
        Rc::new(LocatedVal::new(dp.parse(&mut buf).unwrap(), 0, len))
    }

    #[test]
    fn test_dict_info() {
        let d = make_dict(3, 10);
        let mut ctxt = PDFObjContext::new();
        let osp = ObjStreamP::new(&mut ctxt, d);
        let chk = osp.get_dict_info();
        assert_eq!(chk, Ok((3, 10)));
    }

    #[test]
    fn test_metadata() {
        let d = make_dict(3, 10);
        let mut ctxt = PDFObjContext::new();
        let mut osp = ObjStreamP::new(&mut ctxt, d);

        // valid
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 1 30 2".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).unwrap();
        assert_eq!(chk, vec![(10, 0), (20, 1), (30, 2)]);

        // non-increasing offsets
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 0 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "offset 0 of object 20 is not greater than previous offset 0";
        assert_eq!(chk, locate_value(ErrorKind::GuardError(String::from(err)), 0, 0));

        // insufficient metadata
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 1 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "not at number";
        assert_eq!(chk, locate_value(ErrorKind::GuardError(String::from(err)), 0, 0));

        // invalid object id
        let mut buf = ParseBuffer::new(Vec::from("10 0 0 1 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "invalid object id: 0";
        assert_eq!(chk, locate_value(ErrorKind::GuardError(String::from(err)), 0, 0));
    }

    #[test]
    fn test_content() {
        //        012345678901234
        let md = "10 0 20 5 30 8";
        let mut mbuf = ParseBuffer::new(Vec::from(md));
        //             01234567890
        let content = "<<>> [] ()";
        let mut cbuf = ParseBuffer::new(Vec::from(content));

        let d = make_dict(3, 14);
        let mut ctxt = PDFObjContext::new();
        let mut osp = ObjStreamP::new(&mut ctxt, d);

        let md = osp.parse_metadata(&mut mbuf, 3).unwrap();
        assert_eq!(md, vec![(10, 0), (20, 5), (30, 8)]);
        let objs = osp.parse_stream(&mut cbuf, &md).unwrap();

        let mut exp = Vec::new();
        let o = LocatedVal::new(PDFObjT::Dict(DictT::new(BTreeMap::new())), 0, 4);
        let o = IndirectT::new(10, 0, Rc::new(o));
        exp.push(LocatedVal::new(o, 0, 4));
        let o = LocatedVal::new(PDFObjT::Array(ArrayT::new(vec![])), 5, 7);
        let o = IndirectT::new(20, 0, Rc::new(o));
        exp.push(LocatedVal::new(o, 5, 7));
        let o = LocatedVal::new(PDFObjT::String(vec![]), 8, 10);
        let o = IndirectT::new(30, 0, Rc::new(o));
        exp.push(LocatedVal::new(o, 8, 10));
        assert_eq!(objs, exp);
    }

    #[test]
    fn test_stream() {
        //        012345678901234
        let md = "10 0 20 5 30 8";
        //             01234567890
        let content = "<<>> [] ()";
        let mut buf = Vec::from(md);
        buf.append(&mut Vec::from(content));
        let mut buf = ParseBuffer::new(buf);

        let d = make_dict(3, 14);
        let mut ctxt = PDFObjContext::new();
        let mut osp = ObjStreamP::new(&mut ctxt, Rc::clone(&d));

        let val = osp.parse(&mut buf).unwrap();

        let mut exp = Vec::new();
        let o = LocatedVal::new(PDFObjT::Dict(DictT::new(BTreeMap::new())), 0, 4);
        let o = IndirectT::new(10, 0, Rc::new(o));
        exp.push(LocatedVal::new(o, 0, 4));
        let o = LocatedVal::new(PDFObjT::Array(ArrayT::new(vec![])), 5, 7);
        let o = IndirectT::new(20, 0, Rc::new(o));
        exp.push(LocatedVal::new(o, 5, 7));
        let o = LocatedVal::new(PDFObjT::String(vec![]), 8, 10);
        let o = IndirectT::new(30, 0, Rc::new(o));
        exp.push(LocatedVal::new(o, 8, 10));
        let os = ObjStreamT::new(Rc::clone(&d), exp);
        let os = LocatedVal::new(os, 0, 24);
        assert_eq!(val, os);
    }
}
