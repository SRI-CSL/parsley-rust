// Copyright (c) 2020 SRI International.
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

use std::collections::BTreeMap;
use std::rc::Rc;

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParseResult, ParsleyParser,
};
use super::super::pcore::transforms::{BufferTransformT, RestrictView, RestrictViewFrom};

use super::pdf_filters::{ASCII85Decode, ASCIIHexDecode, DCTDecode, FlateDecode};
use super::pdf_obj::{
    parse_pdf_obj, DictKey, DictT, Filter, IndirectT, PDFObjContext, PDFObjT, StreamT,
};
use super::pdf_prim::{IntegerP, StreamContentT, WhitespaceEOL};

type ObjStreamObjInfo = (usize, usize); // (object#, offset) pairs
type ObjStreamMetadata = Vec<ObjStreamObjInfo>;
type ObjStreamContent = Vec<LocatedVal<IndirectT>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjStreamT {
    dict: Rc<LocatedVal<DictT>>,
    objs: ObjStreamContent,
}

impl ObjStreamT {
    pub fn new(dict: Rc<LocatedVal<DictT>>, objs: ObjStreamContent) -> ObjStreamT {
        ObjStreamT { dict, objs }
    }
    pub fn objs(&self) -> &[LocatedVal<IndirectT>] { self.objs.as_slice() }
}

pub struct ObjStreamP<'a> {
    ctxt:   &'a mut PDFObjContext,
    stream: &'a StreamT,
}

impl ObjStreamP<'_> {
    pub fn new<'a>(ctxt: &'a mut PDFObjContext, stream: &'a StreamT) -> ObjStreamP<'a> {
        ObjStreamP { ctxt, stream }
    }

    fn get_dict_info(&self) -> ParseResult<(usize, usize)> {
        let dict = self.stream.dict();
        let stream_type = dict.val().get_name(b"Type");
        if stream_type.is_none() {
            let msg = "No valid /Type in object stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let stream_type = stream_type.unwrap();
        if stream_type != b"ObjStm" {
            let t_str = match std::str::from_utf8(stream_type) {
                Ok(v) => v.to_string(),
                Err(e) => format!("(error: cannot convert to UTF-8: {})", e),
            };
            let msg = format!("Invalid /Type in object stream dictionary: {}", t_str);
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }

        let num_objs = dict.val().get_usize(b"N");
        if num_objs.is_none() {
            let msg = "No valid /N in object stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let num_objs = num_objs.unwrap();

        let first = dict.val().get_usize(b"First");
        if first.is_none() {
            let msg = "No valid /First in object stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let first = first.unwrap();
        Ok((num_objs, first))
    }

    // The assumptions are that:
    // - the parsing view corresponds to the metadata section of the stream content,
    //   delimited by the /First offset
    // - /N provides the number of integer pairs expected in the metadata
    // - parsing is done until the expected metadata is consumed, or an
    //   end-of-buffer error; i.e. it is not illegal to have more data after the end
    //   of the metadata and before /First
    // [This assumes certain future fixes to the PDF spec.]
    fn parse_metadata(
        &mut self, buf: &mut dyn ParseBufferT, n: usize,
    ) -> ParseResult<ObjStreamMetadata> {
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
            if !obj.val().is_usize() {
                let msg = format!("invalid object id: {}", obj.val().int_val());
                let err = ErrorKind::GuardError(msg);
                buf.set_cursor_unsafe(cursor);
                return Err(obj.place(err))
            }
            let obj = obj.val().usize_val();

            ws.parse(buf)?;
            // get the offset.
            cursor = buf.get_cursor();
            let ofs = int.parse(buf)?;
            if !ofs.val().is_usize() {
                let msg = format!("invalid or unsupported offset: {}", ofs.val().int_val());
                let err = ErrorKind::GuardError(msg);
                buf.set_cursor_unsafe(cursor);
                return Err(ofs.place(err))
            }
            // ensure that the offset is ordered.
            let ofs_val = ofs.val().usize_val();
            if ofs_val <= last_ofs && !obj_ofs.is_empty() {
                let msg = format!(
                    "offset {} of object {} is not greater than previous offset {}",
                    ofs_val, obj, last_ofs
                );
                let err = ErrorKind::GuardError(msg);
                buf.set_cursor_unsafe(cursor);
                return Err(ofs.place(err))
            }

            last_ofs = ofs_val;
            obj_ofs.push((obj, ofs_val));
            if obj_ofs.len() == n {
                break
            }
        }
        Ok(obj_ofs)
    }

    // The assumptions are that:
    // - the parsing view corresponds to the content section of the stream content,
    //   beginning at the /First offset
    // - parsing is done until the expected metadata is consumed, or an
    //   end-of-buffer error; i.e. it is not illegal to have more data after the end
    //   of the last offset specified in the metadata
    // [This assumes certain future fixes to the PDF spec.]
    fn parse_stream(
        &mut self, buf: &mut dyn ParseBufferT, meta: &[ObjStreamObjInfo],
    ) -> ParseResult<ObjStreamContent> {
        let mut ws = WhitespaceEOL::new(true);
        let mut objs = Vec::new();
        for (i, (onum, ofs)) in meta.iter().enumerate() {
            // Ensure we are not past the specified offset.
            if buf.get_cursor() > *ofs {
                let msg = format!(
                    "parsed past offset {} for object #{} with id {}",
                    ofs, i, onum
                );
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, *ofs, buf.get_cursor()))
            }
            ws.parse(buf)?;

            let start = buf.get_cursor();
            let o = parse_pdf_obj(&mut self.ctxt, buf)?;
            let end = buf.get_cursor();
            let obj = Rc::new(o);
            let ind = IndirectT::new(*onum, 0, Rc::clone(&obj));
            let ind = LocatedVal::new(ind, start, end);
            // Register the object into the context so that it can be
            // looked up by its id.
            match self.ctxt.register_obj(&ind) {
                None => (),
                Some(old) => {
                    let loc = old.start();
                    let msg = format!(
                        "non-unique object id ({}, {}), first found near offset {}",
                        *onum, 0, loc
                    );
                    let err = ErrorKind::GuardError(msg);
                    let end = buf.get_cursor();
                    return Err(locate_value(err, start, end))
                },
            }
            objs.push(ind)
        }
        Ok(objs)
    }
}

impl ParsleyParser for ObjStreamP<'_> {
    type T = LocatedVal<ObjStreamT>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let (num_objs, first) = self.get_dict_info()?;
        let filters = self.stream.filters()?;

        // Handle filters, using the approach used in XrefStreamP.
        let input = buf;
        let mut views: Vec<ParseBuffer> = Vec::new();
        // Selects the input for the iteration from the view stack.
        fn get_input<'a>(
            inp: &'a mut dyn ParseBufferT, views: &'a mut Vec<ParseBuffer>,
        ) -> &'a mut dyn ParseBufferT {
            if views.is_empty() {
                inp
            } else {
                let last = views.len() - 1;
                &mut views[last]
            }
        }
        if self.ctxt.is_encrypted() {
            let msg = "Encrypted streams are currently unsupported".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(self.stream.dict().place(err))
        }
        for filter in &filters {
            let f = filter.name().as_string();
            let mut decoder: Box<dyn BufferTransformT> = match f.as_str() {
                "FlateDecode" => Box::new(FlateDecode::new(filter.options())),
                "ASCII85Decode" => Box::new(ASCII85Decode::new(filter.options())),
                "ASCIIHexDecode" => Box::new(ASCIIHexDecode::new(filter.options())),
                "DCTDecode" => Box::new(DCTDecode::new(filter.options())),
                s => {
                    let msg = format!("Cannot handle filter {} in object stream", s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(self.stream.dict().place(err))
                },
            };
            let input = get_input(input, &mut views);
            let output = decoder.transform(input)?;
            views.push(output);
        }
        let buf = get_input(input, &mut views);

        // TODO: the error-chaining from transforms to parse-errors
        // needs to compose with the ? operator in a way that allows
        // good error-messages.

        // Create a view bounding the metadata, and parse it.
        let mut view = RestrictView::new(0, first);
        let mut md_buf = match view.transform(buf) {
            Ok(b) => b,
            Err(_) => {
                let msg = format!(
                    "Unable to parse object-stream metadata, /First may be invalid: {}",
                    first
                );
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, start, start))
            },
        };
        let meta = self.parse_metadata(&mut md_buf, num_objs)?;
        // Create a view for the content, and parse it using the metadata.
        let mut view = RestrictViewFrom::new(first);
        let mut objs_buf = match view.transform(buf) {
            Ok(v) => v,
            Err(_) => {
                let msg = format!(
                    "Unable to parse object-stream content, /First may be invalid: {}",
                    first
                );
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, first, first))
            },
        };
        let objs = self.parse_stream(&mut objs_buf, &meta)?;
        let end = buf.get_cursor();
        Ok(LocatedVal::new(
            ObjStreamT::new(Rc::clone(&self.stream.dict()), objs),
            start,
            end,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum XrefEntStatus {
    Free {
        next: usize,
    },
    InUse {
        file_ofs: usize,
    },
    InStream {
        stream_obj: usize,
        obj_index:  usize,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct XrefEntT {
    obj:    usize,
    gen:    usize,
    status: XrefEntStatus,
}

impl XrefEntT {
    pub fn new(obj: usize, gen: usize, status: XrefEntStatus) -> XrefEntT {
        XrefEntT { obj, gen, status }
    }
    pub fn obj(&self) -> usize { self.obj }
    pub fn gen(&self) -> usize { self.gen }
    pub fn in_use(&self) -> bool { !matches!(self.status, XrefEntStatus::Free { .. }) }
    pub fn status(&self) -> &XrefEntStatus { &self.status }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XrefStreamT {
    dict: Rc<LocatedVal<DictT>>,
    ents: Vec<LocatedVal<XrefEntT>>,
}

impl XrefStreamT {
    pub fn new(dict: Rc<LocatedVal<DictT>>, ents: Vec<LocatedVal<XrefEntT>>) -> XrefStreamT {
        XrefStreamT { dict, ents }
    }
    pub fn dict(&self) -> &DictT { self.dict.val() }
    pub fn ents(&self) -> &[LocatedVal<XrefEntT>] { self.ents.as_slice() }
}

pub struct XrefStreamP<'a> {
    encrypted: bool,
    stream:    &'a StreamT,
}

pub struct XrefStreamDictInfo<'a> {
    size:    usize,
    prev:    Option<usize>,
    index:   Option<Vec<(usize, usize)>>, // like pdf_file::XrefSubSectT but without entries
    widths:  [usize; 3],
    filters: Vec<Filter<'a>>,
}

impl XrefStreamDictInfo<'_> {
    pub fn size(&self) -> usize { self.size }
    pub fn prev(&self) -> Option<usize> { self.prev }
}

impl XrefStreamP<'_> {
    pub fn new(encrypted: bool, stream: &StreamT) -> XrefStreamP {
        XrefStreamP { encrypted, stream }
    }

    pub fn stream(&self) -> &StreamT { self.stream }

    fn get_dict_info(&self) -> ParseResult<XrefStreamDictInfo> {
        let dict = self.stream.dict();
        let stream_type = dict.val().get_name(b"Type");
        if stream_type.is_none() {
            let msg = "No /Type in xref stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let stream_type = stream_type.unwrap();
        if stream_type != b"XRef" {
            let t_str = match std::str::from_utf8(stream_type) {
                Ok(v) => v.to_string(),
                Err(e) => format!("(error: cannot convert to UTF-8: {})", e),
            };
            let msg = format!("Invalid /Type in xref stream dictionary: {}", t_str);
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }

        let size = dict.val().get_usize(b"Size");
        if size.is_none() {
            let msg = "No valid /Size in xref stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let size = size.unwrap();

        let prev = dict.val().get_usize(b"Prev");

        let idx = dict.val().get_array(b"Index");
        let index = if let Some(i) = idx {
            let mut index_ents = Vec::new();
            if i.objs().len() % 2 != 0 {
                let msg = format!(
                    "Invalid non-even length {} for /Index in xref stream dictionary.",
                    i.objs().len()
                );
                let err = ErrorKind::GuardError(msg);
                return Err(locate_value(err, dict.start(), dict.end()))
            }
            for (s, c) in i
                .objs()
                .iter()
                .step_by(2)
                .zip(i.objs().iter().skip(1).step_by(2))
            {
                if let (PDFObjT::Integer(s), PDFObjT::Integer(c)) = (s.val(), c.val()) {
                    if !s.is_usize() {
                        let msg = format!(
                            "Invalid or unsupported integer in xref stream /Index: {}",
                            s.int_val()
                        );
                        let err = ErrorKind::GuardError(msg);
                        return Err(locate_value(err, dict.start(), dict.end()))
                    }
                    if !c.is_usize() {
                        let msg = format!(
                            "Invalid or unsupported integer in xref stream /Index: {}",
                            c.int_val()
                        );
                        let err = ErrorKind::GuardError(msg);
                        return Err(locate_value(err, dict.start(), dict.end()))
                    }
                    index_ents.push((s.usize_val(), c.usize_val()))
                } else {
                    let msg = "Invalid non-integer entries in /Index in xref stream dictionary."
                        .to_string();
                    let err = ErrorKind::GuardError(msg);
                    return Err(locate_value(err, dict.start(), dict.end()))
                }
            }
            Some(index_ents)
        // TODO: ensure that subsections cannot overlap
        } else {
            None
        };

        let w = dict.val().get_array(b"W");
        if w.is_none() {
            let msg = "No valid /W in xref stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let w = w.unwrap();
        if w.objs().len() != 3 {
            let msg = "Invalid length for /W in xref stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let mut w_array = Vec::new();
        for o in w.objs() {
            if let PDFObjT::Integer(i) = o.val() {
                if !i.is_usize() {
                    let msg = format!(
                        "Invalid or unsupported integer in /W in xref stream dictionary: {}",
                        i.int_val()
                    );
                    let err = ErrorKind::GuardError(msg);
                    return Err(locate_value(err, dict.start(), dict.end()))
                }
                let sz = i.usize_val();
                // Implementation detail: do not handle integer widths larger than 4 bytes
                if sz > 4 {
                    let msg = format!(
                        "Cannot handle {}-byte integer in /W in xref stream dictionary.",
                        sz
                    );
                    let err = ErrorKind::GuardError(msg);
                    return Err(locate_value(err, dict.start(), dict.end()))
                }
                w_array.push(sz);
                continue
            }
            let msg = "Invalid length for /W in xref stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        if w_array[1] == 0 {
            let msg = "Invalid zero-width field #2 in /W in xref stream dictionary.".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(locate_value(err, dict.start(), dict.end()))
        }
        let widths = [w_array[0], w_array[1], w_array[2]];

        let filters = self.stream.filters()?;

        Ok(XrefStreamDictInfo {
            size,
            prev,
            index,
            widths,
            filters,
        })
    }

    fn parse_usize_with_width(
        &self, buf: &mut dyn ParseBufferT, width: usize,
    ) -> ParseResult<usize> {
        let mut val: usize = 0;
        for _ in 0 .. width {
            let peek = buf.peek();
            if peek.is_none() {
                let cur = buf.get_cursor();
                return Err(locate_value(ErrorKind::EndOfBuffer, cur, cur))
            }
            let peek = peek.unwrap();
            val = (val << 8) | usize::from(peek);
            buf.incr_cursor_unsafe();
        }
        Ok(val)
    }

    fn parse_stream(
        &self, buf: &mut dyn ParseBufferT, meta: &XrefStreamDictInfo,
    ) -> ParseResult<Vec<LocatedVal<XrefEntT>>> {
        let mut ents = Vec::new();
        let index = match meta.index {
            Some(ref i) => i.clone(),
            None => vec![(0, meta.size)],
        };
        for (start_obj, count) in index.iter() {
            for c in 0 .. *count {
                let start = buf.get_cursor();
                // Field #1.
                let width = meta.widths[0];
                let typ = if width == 0 {
                    1 // default: Type 1
                } else {
                    let f = self.parse_usize_with_width(buf, width)?;
                    if f > 2 {
                        let cur = buf.get_cursor();
                        let msg =
                            format!("Invalid type {} in field #1 of entry in xref stream.", f);
                        let err = ErrorKind::GuardError(msg);
                        return Err(locate_value(err, start, cur))
                    }
                    f
                };
                // Field #2
                let width = meta.widths[1];
                let field2 = self.parse_usize_with_width(buf, width)?;
                // Field #3
                let width = meta.widths[2];
                let field3 = if width > 0 {
                    self.parse_usize_with_width(buf, width)?
                } else {
                    0
                };

                // Construct the entry
                let (status, gen) = match typ {
                    0 => (XrefEntStatus::Free { next: field2 }, field3),
                    1 => (XrefEntStatus::InUse { file_ofs: field2 }, field3),
                    2 => (
                        XrefEntStatus::InStream {
                            stream_obj: field2,
                            obj_index:  field3,
                        },
                        0,
                    ),
                    // we checked for valid entry typ above, so this should never happen
                    _ => panic!("unhandled entry type in xref stream"),
                };
                let ent = XrefEntT::new(start_obj + c, gen, status);
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
        let input = buf;

        // This vector is just being used to hoist the view generated
        // by a loop iteration to use as input in the next iteration.
        // There might be a standard idiomatic way of doing this; for
        // now, this seems the simplest that avoids an unsafe block.
        let mut views: Vec<ParseBuffer> = Vec::new();

        // Selects the input for the iteration from the view stack.
        fn get_input<'a>(
            inp: &'a mut dyn ParseBufferT, views: &'a mut Vec<ParseBuffer>,
        ) -> &'a mut dyn ParseBufferT {
            if views.is_empty() {
                inp
            } else {
                let last = views.len() - 1;
                &mut views[last]
            }
        }
        if self.encrypted {
            let msg = "Encrypted streams are currently unsupported".to_string();
            let err = ErrorKind::GuardError(msg);
            return Err(self.stream.dict().place(err))
        }
        for filter in &meta.filters {
            let f = filter.name().as_string();
            let mut decoder: Box<dyn BufferTransformT> = match f.as_str() {
                "FlateDecode" => Box::new(FlateDecode::new(filter.options())),
                "ASCII85Decode" => Box::new(ASCII85Decode::new(filter.options())),
                "ASCIIHexDecode" => Box::new(ASCIIHexDecode::new(filter.options())),
                "DCTDecode" => Box::new(DCTDecode::new(filter.options())),
                s => {
                    let msg = format!("Cannot handle filter {} in xref stream", s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(self.stream.dict().place(err))
                },
            };
            let input = get_input(input, &mut views);
            let output = decoder.transform(input)?;
            views.push(output);
        }
        let input = get_input(input, &mut views);
        let ents = self.parse_stream(input, &meta)?;
        let xref = XrefStreamT::new(Rc::clone(&self.stream.dict()), ents);
        let end = input.get_cursor();
        Ok(LocatedVal::new(xref, start, end))
    }
}

// A function to construct a decoded stream object from a possible
// encoded one.  The new decoded stream content is given the same
// location as the old stream content.  This has a few implications:
// the size in the location information will be inaccurate, and hence
// it can't be used for creating views into a parsebuffer.  Instead,
// the content should be used directly.  Similarly, the new stream
// dictionary will have any filter-specific entries pruned from the
// old dictionary, but will retain the old location.

pub fn decode_stream(strm: &StreamT) -> ParseResult<StreamT> {
    let dict = strm.dict().val();
    let content = strm.stream().val().content();
    let filters = strm.filters()?;

    // Handle filter sequence.
    let mut input = ParseBuffer::new(Vec::from(content));
    let mut views: Vec<ParseBuffer> = Vec::new();
    // Selects the input for the iteration from the view stack.
    fn get_input<'a>(
        inp: &'a mut ParseBuffer, views: &'a mut Vec<ParseBuffer>,
    ) -> &'a mut dyn ParseBufferT {
        if views.is_empty() {
            inp
        } else {
            let last = views.len() - 1;
            &mut views[last]
        }
    }
    // Work through the filter sequence.
    for filter in &filters {
        let f = filter.name().as_string();
        let mut decoder: Box<dyn BufferTransformT> = match f.as_str() {
            "FlateDecode" => Box::new(FlateDecode::new(filter.options())),
            "ASCII85Decode" => Box::new(ASCII85Decode::new(filter.options())),
            "ASCIIHexDecode" => Box::new(ASCIIHexDecode::new(filter.options())),
            "DCTDecode" => Box::new(DCTDecode::new(filter.options())),
            s => {
                let msg = format!("Cannot handle filter {} in object stream", s);
                let err = ErrorKind::GuardError(msg);
                return Err(strm.dict().place(err))
            },
        };
        let input = get_input(&mut input, &mut views);
        let output = decoder.transform(input)?;
        views.push(output);
    }
    // Get the final decoded buffer.
    let buf = get_input(&mut input, &mut views);
    let content = Vec::from(buf.buf());
    let content = StreamContentT::new(0, content.len(), content);
    let content = strm.stream().place(content);

    // Create a pruned dictionary.
    let mut map = BTreeMap::new();
    for (key, val) in dict.map() {
        if key == &DictKey::new(Vec::from("Filter"))
            || key == &DictKey::new(Vec::from("DecodeParms"))
        {
            continue
        }
        map.insert(key.clone(), Rc::clone(val));
    }
    let dict = strm.dict().place(DictT::new(map));

    // Construct the new stream object.
    let s = StreamT::new(Rc::new(dict), content);
    Ok(s)
}

#[cfg(test)]
mod test_object_stream {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::rc::Rc;

    use super::{decode_stream, ObjStreamP, ObjStreamT, XrefStreamP};
    use crate::pcore::parsebuffer::{
        locate_value, ErrorKind, LocatedVal, ParseBuffer, ParsleyParser,
    };
    use crate::pcore::transforms::{BufferTransformT, RestrictView};
    use crate::pdf_lib::pdf_obj::{
        ArrayT, DictP, DictT, IndirectP, IndirectT, PDFObjContext, PDFObjT, StreamT,
    };
    use crate::pdf_lib::pdf_prim::StreamContentT;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    fn mk_dict_buf(n: usize, first: usize) -> Vec<u8> {
        let v = format!("<</Type /ObjStm /N {} /First {}>>", n, first);
        Vec::from(v.as_bytes())
    }
    fn mk_dict(n: usize, first: usize) -> Rc<LocatedVal<DictT>> {
        let v = mk_dict_buf(n, first);
        let len = v.len();
        let mut buf = ParseBuffer::new(v);
        let mut ctxt = mk_new_context();
        let mut dp = DictP::new(&mut ctxt);
        Rc::new(LocatedVal::new(dp.parse(&mut buf).unwrap(), 0, len))
    }
    fn mk_objstm(n: usize, first: usize) -> StreamT {
        let dict = mk_dict(n, first);
        let content = StreamContentT::new(0, 0, Vec::new());
        StreamT::new(dict, LocatedVal::new(content, 0, 0))
    }

    #[test]
    fn test_dict_info() {
        let s = mk_objstm(3, 10);
        let mut ctxt = mk_new_context();
        let osp = ObjStreamP::new(&mut ctxt, &s);
        let chk = osp.get_dict_info();
        assert_eq!(chk, Ok((3, 10)));
    }

    #[test]
    fn test_metadata() {
        let s = mk_objstm(3, 10);
        let mut ctxt = mk_new_context();
        let mut osp = ObjStreamP::new(&mut ctxt, &s);

        // valid
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 1 30 2".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).unwrap();
        assert_eq!(chk, vec![(10, 0), (20, 1), (30, 2)]);

        // non-increasing offsets
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 0 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "offset 0 of object 20 is not greater than previous offset 0";
        assert_eq!(
            chk,
            locate_value(ErrorKind::GuardError(String::from(err)), 0, 0)
        );

        // insufficient metadata
        let mut buf = ParseBuffer::new(Vec::from("10 0 20 1 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "not at number";
        assert_eq!(
            chk,
            locate_value(ErrorKind::GuardError(String::from(err)), 0, 0)
        );

        // invalid object id
        let mut buf = ParseBuffer::new(Vec::from("10 0 0 1 30".as_bytes()));
        let chk = osp.parse_metadata(&mut buf, 3).err().unwrap();
        let err = "not at number";
        assert_eq!(
            chk,
            locate_value(ErrorKind::GuardError(String::from(err)), 0, 0)
        );
    }

    #[test]
    fn test_content() {
        //        012345678901234
        let md = "10 0 20 5 30 8";
        let mut mbuf = ParseBuffer::new(Vec::from(md));
        //             01234567890
        let content = "<<>> [] ()";
        let mut cbuf = ParseBuffer::new(Vec::from(content));

        let s = mk_objstm(3, 14);
        let mut ctxt = mk_new_context();
        let mut osp = ObjStreamP::new(&mut ctxt, &s);

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

        let s = mk_objstm(3, 14);
        let mut ctxt = mk_new_context();
        let mut osp = ObjStreamP::new(&mut ctxt, &s);

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

        let d = mk_dict(3, 14);
        let os = ObjStreamT::new(Rc::clone(&d), exp);
        let os = LocatedVal::new(os, 0, 24);
        assert_eq!(val, os);
    }

    #[test]
    fn test_obj_stream() {
        // Extracted from qpdf/examples/qtest/bookmarks/issue-179.pdf
        // using:
        //  qpdf --compress-streams=n --decode-level=generalized issue-179.pdf > out.pdf
        let v = Vec::from("
2 0 obj
<< /Type /ObjStm /Length 6129 /N 73 /First 558 >>
stream
3 0 4 45 5 113 6 169 7 214 8 269 9 325 10 393 11 479 12 535 13 591 14 647 15 703 16 771 17 871 18 953 19 1025 20 1107 21 1168 22 1224 23 1292 24 1392 25 1474 26 1546 27 1628 28 1702 29 1761 30 1820 31 1901 32 1983 33 2065 34 2139 35 2239 36 2327 37 2383 38 2439 39 2495 40 2551 41 2610 42 2691 43 2773 44 2855 45 2929 46 3029 47 3101 48 3183 49 3265 50 3347 51 3435 52 3494 53 3575 54 3657 55 3739 56 3813 57 3913 58 3985 59 4067 60 4149 61 4231 62 4319 63 4375 64 4434 65 4508 66 4604 67 4701 68 4798 69 4887 70 5002 71 5089 72 5186 73 5283 74 5380 75 5469
<< /Count 1 /Kids [ 76 0 R ] /Type /Pages >>
<< /CreationDate (D:20180202210211) /Creator (Master PDF Editor) >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Title <9e> >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Title <feff017e010d> >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 10 0 R /Title <feff017e010d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Prev 9 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 16 0 R /Title <feff017e010d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Next 17 0 R /Prev 15 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 11 0 R /C [ 0 0 0 ] /F 0 /Next 18 0 R /Prev 16 0 R /Title <feff017e0107> >>
<< /A 12 0 R /C [ 0 0 0 ] /F 0 /Next 19 0 R /Prev 17 0 R /Title <9e> >>
<< /A 13 0 R /C [ 0 0 0 ] /F 0 /Next 20 0 R /Prev 18 0 R /Title <feff017e0111> >>
<< /A 14 0 R /C [ 0 0 0 ] /F 0 /Prev 19 0 R /Title <9e9d> >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 23 0 R /Title <feff017e010d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Next 24 0 R /Prev 22 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 11 0 R /C [ 0 0 0 ] /F 0 /Next 25 0 R /Prev 23 0 R /Title <feff017e0107> >>
<< /A 12 0 R /C [ 0 0 0 ] /F 0 /Next 26 0 R /Prev 24 0 R /Title <9e> >>
<< /A 13 0 R /C [ 0 0 0 ] /F 0 /Next 27 0 R /Prev 25 0 R /Title <feff017e0111> >>
<< /A 14 0 R /C [ 0 0 0 ] /F 0 /Next 28 0 R /Prev 26 0 R /Title <9e9d> >>
<< /A 21 0 R /C [ 0 0 0 ] /F 0 /Prev 27 0 R /Title <9d> >>
<< /A 12 0 R /C [ 0 0 0 ] /F 0 /Next 30 0 R /Title <9e> >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 31 0 R /Prev 29 0 R /Title <feff017e010d> >>
<< /A 13 0 R /C [ 0 0 0 ] /F 0 /Next 32 0 R /Prev 30 0 R /Title <feff017e0111> >>
<< /A 11 0 R /C [ 0 0 0 ] /F 0 /Next 33 0 R /Prev 31 0 R /Title <feff017e0107> >>
<< /A 14 0 R /C [ 0 0 0 ] /F 0 /Next 34 0 R /Prev 32 0 R /Title <9e9d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Next 35 0 R /Prev 33 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 21 0 R /C [ 0 0 0 ] /F 0 /Prev 34 0 R /Title (\\235 ajklyghvbnmxcseqwuioprtzdf) >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /A 12 0 R /C [ 0 0 0 ] /F 0 /Next 41 0 R /Title <9e> >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 42 0 R /Prev 40 0 R /Title <feff017e010d> >>
<< /A 13 0 R /C [ 0 0 0 ] /F 0 /Next 43 0 R /Prev 41 0 R /Title <feff017e0111> >>
<< /A 11 0 R /C [ 0 0 0 ] /F 0 /Next 44 0 R /Prev 42 0 R /Title <feff017e0107> >>
<< /A 14 0 R /C [ 0 0 0 ] /F 0 /Next 45 0 R /Prev 43 0 R /Title <9e9d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Next 46 0 R /Prev 44 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 36 0 R /C [ 0 0 0 ] /F 0 /Next 47 0 R /Prev 45 0 R /Title <9d> >>
<< /A 37 0 R /C [ 0 0 0 ] /F 0 /Next 48 0 R /Prev 46 0 R /Title <feff0161010d> >>
<< /A 39 0 R /C [ 0 0 0 ] /F 0 /Next 49 0 R /Prev 47 0 R /Title <feff01610107> >>
<< /A 38 0 R /C [ 0 0 0 ] /F 0 /Next 50 0 R /Prev 48 0 R /Title <feff01610111> >>
<< /A 21 0 R /C [ 0 0 0 ] /F 0 /Prev 49 0 R /Title (\\235 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 12 0 R /C [ 0 0 0 ] /F 0 /Next 52 0 R /Title <9e> >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 53 0 R /Prev 51 0 R /Title <feff017e010d> >>
<< /A 13 0 R /C [ 0 0 0 ] /F 0 /Next 54 0 R /Prev 52 0 R /Title <feff017e0111> >>
<< /A 11 0 R /C [ 0 0 0 ] /F 0 /Next 55 0 R /Prev 53 0 R /Title <feff017e0107> >>
<< /A 14 0 R /C [ 0 0 0 ] /F 0 /Next 56 0 R /Prev 54 0 R /Title <9e9d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Next 57 0 R /Prev 55 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 36 0 R /C [ 0 0 0 ] /F 0 /Next 58 0 R /Prev 56 0 R /Title <9d> >>
<< /A 37 0 R /C [ 0 0 0 ] /F 0 /Next 59 0 R /Prev 57 0 R /Title <feff0161010d> >>
<< /A 38 0 R /C [ 0 0 0 ] /F 0 /Next 60 0 R /Prev 58 0 R /Title <feff01610111> >>
<< /A 39 0 R /C [ 0 0 0 ] /F 0 /Next 61 0 R /Prev 59 0 R /Title <feff01610107> >>
<< /A 21 0 R /C [ 0 0 0 ] /F 0 /Prev 60 0 R /Title (\\235 ajklyghvbnmxcseqwuioprtzdf) >>
<< /D [ 76 0 R /XYZ 0 841 1 ] /S /GoTo /Type /Action >>
<< /Count 12 /First 64 0 R /Last 75 0 R /Type /Outlines >>
<< /A 12 0 R /C [ 0 0 0 ] /F 0 /Next 65 0 R /Parent 63 0 R /Title <9e> >>
<< /A 5 0 R /C [ 0 0 0 ] /F 0 /Next 66 0 R /Parent 63 0 R /Prev 64 0 R /Title <feff017e010d> >>
<< /A 13 0 R /C [ 0 0 0 ] /F 0 /Next 67 0 R /Parent 63 0 R /Prev 65 0 R /Title <feff017e0111> >>
<< /A 11 0 R /C [ 0 0 0 ] /F 0 /Next 68 0 R /Parent 63 0 R /Prev 66 0 R /Title <feff017e0107> >>
<< /A 14 0 R /C [ 0 0 0 ] /F 0 /Next 69 0 R /Parent 63 0 R /Prev 67 0 R /Title <9e9d> >>
<< /A 8 0 R /C [ 0 0 0 ] /F 0 /Next 70 0 R /Parent 63 0 R /Prev 68 0 R /Title (\\236 ajklyghvbnmxcseqwuioprtzdf) >>
<< /A 36 0 R /C [ 0 0 0 ] /F 0 /Next 71 0 R /Parent 63 0 R /Prev 69 0 R /Title <9d> >>
<< /A 37 0 R /C [ 0 0 0 ] /F 0 /Next 72 0 R /Parent 63 0 R /Prev 70 0 R /Title <feff0161010d> >>
<< /A 38 0 R /C [ 0 0 0 ] /F 0 /Next 73 0 R /Parent 63 0 R /Prev 71 0 R /Title <feff01610111> >>
<< /A 39 0 R /C [ 0 0 0 ] /F 0 /Next 74 0 R /Parent 63 0 R /Prev 72 0 R /Title <feff01610107> >>
<< /A 62 0 R /C [ 0 0 0 ] /F 0 /Next 75 0 R /Parent 63 0 R /Prev 73 0 R /Title <9d9e> >>
<< /A 21 0 R /C [ 0 0 0 ] /F 0 /Parent 63 0 R /Prev 74 0 R /Title (\\235 ajklyghvbnmxcseqwuioprtzdf) >>
endstream
endobj");
        let mut pb = ParseBuffer::new(v);
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        let io = p.parse(&mut pb).expect("unable to parse stream");
        let io = io.val();
        if let PDFObjT::Stream(ref s) = io.obj().val() {
            let content = s.stream().val();
            let mut vxf = RestrictView::new(content.start(), content.size());
            let mut stream_buf = vxf.transform(&pb).unwrap();
            let mut osp = ObjStreamP::new(&mut ctxt, s);
            let ost = osp.parse(&mut stream_buf);
            let ost = ost.unwrap();
            assert_eq!(ost.val().objs().len(), 73)
        } else {
            assert!(false);
        }
    }

    fn get_test_data(test_file: &str) -> Vec<u8> {
        let mut file = match File::open(test_file) {
            Err(why) => panic!("Cannot open {}: {}", test_file, why.to_string()),
            Ok(f) => f,
        };
        let mut v = Vec::new();
        match file.read_to_end(&mut v) {
            Err(why) => panic!("Cannot read {}: {}", test_file, why.to_string()),
            Ok(_) => (),
        };
        v
    }

    #[test]
    fn test_xref_stream() {
        let v = get_test_data("tests/test_files/filter_tests/xref_stm.obj");
        let mut pb = ParseBuffer::new(v);
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        let io = p.parse(&mut pb).expect("unable to parse stream");
        let io = io.val();
        if let PDFObjT::Stream(ref s) = io.obj().val() {
            let content = s.stream().val();
            let mut vxf = RestrictView::new(content.start(), content.size());
            let mut xref_buf = vxf.transform(&pb).unwrap();
            let mut xrsp = XrefStreamP::new(false, s);
            let xrt = xrsp.parse(&mut xref_buf).unwrap();
            let ents = xrt.val().ents();
            let size = s.dict().val().get_usize(b"Size").unwrap();
            assert_eq!(size, ents.len())
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_jpg_stream() {
        let v = get_test_data("tests/test_files/filter_tests/jpeg_stm.obj");
        let mut pb = ParseBuffer::new(v);
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        let io = p.parse(&mut pb).expect("unable to parse stream");
        let io = io.val();
        if let PDFObjT::Stream(ref s) = io.obj().val() {
            match decode_stream(s) {
                Ok(_) => (),
                Err(_) => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_xref_stream_flate() {
        let v = get_test_data("tests/test_files/filter_tests/xref_stm_flate.obj");
        let mut pb = ParseBuffer::new(v);
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        let io = p
            .parse(&mut pb)
            .expect("unable to parse tests/test_files/filter_tests/xref_stm_flate.obj");
        let io = io.val();
        if let PDFObjT::Stream(ref s) = io.obj().val() {
            let content = s.stream().val();
            let mut vxf = RestrictView::new(content.start(), content.size());
            let mut xref_buf = vxf.transform(&pb).unwrap();
            let mut xrsp = XrefStreamP::new(false, s);
            let xrt = xrsp.parse(&mut xref_buf).unwrap();
            let ents = xrt.val().ents();
            let size = s.dict().val().get_usize(b"Size").unwrap();
            assert_eq!(size, ents.len())
        } else {
            assert!(false);
        }
    }

    fn do_test_decode_stream(
        test_file: &str, ans_file: &str, has_decode_parms: bool, decompress: bool,
        chk_content: bool,
    ) {
        // parse the test data
        let v = get_test_data(test_file);
        let mut pb = ParseBuffer::new(v);
        let mut ctxt = mk_new_context();
        let mut p = IndirectP::new(&mut ctxt);
        let tio = p
            .parse(&mut pb)
            .expect(&format!("unable to parse {}", test_file));
        let tio = tio.val();

        // parse the answer data
        let v = get_test_data(ans_file);
        let mut pb = ParseBuffer::new(v);
        let mut ctxt = mk_new_context(); // don't use the old context
        let mut p = IndirectP::new(&mut ctxt);
        let aio = p
            .parse(&mut pb)
            .expect(&format!("unable to parse {}", ans_file));
        let aio = aio.val();

        if let PDFObjT::Stream(ref ts) = tio.obj().val() {
            assert!(ts.dict().val().get(b"Filter").is_some());
            if has_decode_parms {
                assert!(ts.dict().val().get(b"DecodeParms").is_some())
            };
            let old_size = ts.stream().val().size();
            let nts = decode_stream(ts).unwrap();
            let new_size = nts.stream().val().size();
            assert!(nts.dict().val().get(b"Filter").is_none());
            assert!(nts.dict().val().get(b"DecodeParms").is_none());
            if decompress {
                assert!(old_size <= new_size)
            } else {
                assert!(old_size >= new_size)
            }
            if let PDFObjT::Stream(ref s) = aio.obj().val() {
                // the answer file should not have any filters
                assert!(s.dict().val().get(b"Filter").is_none());
                assert!(s.dict().val().get(b"DecodeParms").is_none());
                if chk_content {
                    // the decoded buffer should match the answer data
                    let our_decode = nts.stream().val().content();
                    let their_decode = s.stream().val().content();
                    assert_eq!(our_decode, their_decode)
                }
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_decode_stream() {
        do_test_decode_stream(
            "tests/test_files/filter_tests/xref_stm_flate.obj",
            "tests/test_files/filter_tests/xref_stm.obj",
            true,  // has decode parms
            true,  // decompresses
            false, // FIXME: decoded content doesn't match
        );
        do_test_decode_stream(
            "tests/test_files/filter_tests/xobject_stm_ascii85.obj",
            "tests/test_files/filter_tests/xobject_stm.obj",
            false, // no decode parms
            false, // ascii decompression actually expands data
            true,  // check content
        );
    }
}
