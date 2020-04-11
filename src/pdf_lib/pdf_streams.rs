use std::rc::Rc;
use super::super::pcore::parsebuffer::{
    ParseBuffer, ParseBufferT, ParsleyParser, ParseResult, LocatedVal,
    ErrorKind, make_error, make_error_with_loc
};

use super::pdf_prim::{WhitespaceEOL, IntegerP};
use super::pdf_obj::{PDFObjContext, PDFObjP, DictT, IndirectT};

type ObjStreamMetadata = Vec<(usize, usize)>; // (object#, offset) pairs
type ObjStreamContent  = Vec<LocatedVal<IndirectT>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjStreamT {
    dict: Rc<LocatedVal<DictT>>,
    objs: ObjStreamContent
}

impl ObjStreamT {
    pub fn new(dict: Rc<LocatedVal<DictT>>, objs: ObjStreamContent) -> ObjStreamT {
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
    pub fn new<'a>(ctxt: &'a mut PDFObjContext, dict: Rc<LocatedVal<DictT>>) -> ObjStreamP<'a> {
        ObjStreamP { ctxt, dict }
    }

    fn get_dict_info(&self, start: usize) -> ParseResult<(usize, usize)> {
        let stream_type = self.dict.val().get_name(b"Type");
        if stream_type.is_none() {
            let msg = format!("No /Type in object stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(make_error(err, start, start))
        }
        let stream_type = stream_type.unwrap();
        if stream_type != b"ObjStm" {
            let t_str = match std::str::from_utf8(stream_type) {
                Ok(v)  => v.to_string(),
                Err(e) => format!("(error: cannot convert to UTF-8: {})", e)
            };
            let msg = format!("Invalid /Type in object stream dictionary: {}", t_str);
            let err = ErrorKind::GuardError(msg);
            return Err(make_error(err, start, start))
        }

        let num_objs = self.dict.val().get_usize(b"N");
        if num_objs.is_none() {
            let msg = format!("No /N in object stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(make_error(err, start, start))
        }
        let num_objs = num_objs.unwrap();

        let first = self.dict.val().get_usize(b"First");
        if first.is_none() {
            let msg = format!("No /First in object stream dictionary.");
            let err = ErrorKind::GuardError(msg);
            return Err(make_error(err, start, start))
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
                return Err(make_error_with_loc(err, &obj))
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
                return Err(make_error_with_loc(err, &ofs))
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
    fn parse_stream(&mut self, buf: &mut dyn ParseBufferT, meta: ObjStreamMetadata)
                        -> ParseResult<ObjStreamContent> {
        let mut ws = WhitespaceEOL::new(true);
        let mut objs = Vec::new();
        for (i, (onum, ofs)) in meta.iter().enumerate() {
            // Ensure we are not past the specified offset.
            if buf.get_cursor() > *ofs {
                let msg = format!("parsed past offset {} for object #{} with id {}",
                                  ofs, i, onum);
                let err = ErrorKind::GuardError(msg);
                return Err(make_error(err, *ofs, buf.get_cursor()))
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
        let (num_objs, first) = self.get_dict_info(start)?;
        // First parse the metadata.
        let mut md_view = match ParseBuffer::restrict_view(buf, 0, first) {
            Some(v) => v,
            None => {
                let msg = format!("Unable to parse object-stream metadata, /First may be invalid: {}",
                                  first);
                let err = ErrorKind::GuardError(msg);
                return Err(make_error(err, start, start))
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
                return Err(make_error(err, first, first))
            }
        };
        let objs = self.parse_stream(&mut objs_view, meta)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(ObjStreamT { dict: Rc::clone(&self.dict), objs },
                           start, end))
    }
}

#[cfg(test)]
mod test_object_stream {
    use std::rc::Rc;
    use std::collections::BTreeMap;
    use super::super::super::pcore::parsebuffer::{
        ParsleyParser, ParseBuffer, LocatedVal, ErrorKind, make_error
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
        let chk = osp.get_dict_info(0);
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
        assert_eq!(chk, make_error(ErrorKind::GuardError(String::from(err)), 0, 0));

        // insufficient metadata
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 1 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "not at number";
        assert_eq!(chk, make_error(ErrorKind::GuardError(String::from(err)), 0, 0));

        // invalid object id
        let mut buf = ParseBuffer::new(Vec::from("10 0 0 1 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "invalid object id: 0";
        assert_eq!(chk, make_error(ErrorKind::GuardError(String::from(err)), 0, 0));
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
        let objs = osp.parse_stream(&mut cbuf, md).unwrap();

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
