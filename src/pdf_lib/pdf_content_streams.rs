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

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser,
};
use super::pdf_obj::{ArrayP, ArrayT, DictP, DictT, ObjectId, PDFObjContext, PDFObjT};
use super::pdf_operator_types::{ArgType, OpType, OPERATORS};
use super::pdf_prim::{
    Comment, HexString, IntegerT, NameP, NameT, OperatorP, RawLiteralString, RealP, RealT,
    WhitespaceEOL,
};
use std::collections::BTreeMap;

// Individual objects in content stream, and their parser.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CSObjT {
    Op(String),
    Array(ArrayT),
    Dict(DictT),
    Boolean(bool),
    String(Vec<u8>),
    Name(NameT),
    Null(()),
    Comment(Vec<u8>),
    Integer(IntegerT),
    Real(RealT),
}

pub struct CSObjP<'a> {
    ctxt: &'a mut PDFObjContext,
}

impl CSObjP<'_> {
    pub fn new(ctxt: &mut PDFObjContext) -> CSObjP { CSObjP { ctxt } }

    fn parse_internal(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<CSObjT> {
        let c = buf.peek();
        match c {
            None => {
                let start = buf.get_cursor();
                let err = ErrorKind::EndOfBuffer;
                Err(locate_value(err, start, start))
            },

            Some(40) => {
                // '('
                let mut rp = RawLiteralString;
                let r = rp.parse(buf)?;
                Ok(CSObjT::String(r.unwrap()))
            },
            Some(37) => {
                // '%'
                let mut cp = Comment;
                let c = cp.parse(buf)?;
                Ok(CSObjT::Comment(c.unwrap()))
            },
            Some(47) => {
                // '/'
                let mut np = NameP;
                let n = np.parse(buf)?;
                Ok(CSObjT::Name(n.unwrap()))
            },
            Some(91) => {
                // '['
                let mut ap = ArrayP::new(&mut self.ctxt);
                let a = ap.parse(buf)?;
                Ok(CSObjT::Array(a.unwrap()))
            },
            Some(60) => {
                // '<'
                // We need to distinguish between a hex-string and a
                // dictionary object.  So peek ahead.
                let cursor = buf.get_cursor();
                buf.incr_cursor_unsafe();
                let next = buf.peek();
                buf.set_cursor_unsafe(cursor);

                match next {
                    Some(60) => {
                        let mut dp = DictP::new(&mut self.ctxt);
                        let d = dp.parse(buf)?;
                        Ok(CSObjT::Dict(d))
                    },
                    Some(_) | None => {
                        let mut hp = HexString;
                        let s = hp.parse(buf)?;
                        Ok(CSObjT::String(s.unwrap()))
                    },
                }
            },
            Some(b) if b.is_ascii_digit()
                || b == 45 // '-' to handle negative numbers
                || b == 46 // '.' to handle reals
                => {
                    // We will either have an integer or a real.
                    let mut real = RealP;

                    // Check if we are at a real.
                    let r = real.parse(buf)?;
                    if !r.val().is_integer() {
                        return Ok(CSObjT::Real(r.unwrap()))
                    }
                    // We parsed an integer.
                    let i = IntegerT::new(r.val().numerator());
                    Ok(CSObjT::Integer(i))

            },
            Some(_) => {
                // We have non-whitespace characters, which could
                // either be a PDF operator, or a boolean, or null.

                let mut opp = OperatorP;
                let op = opp.parse(buf)?;
                match op.val().name() {
                    "true" => Ok(CSObjT::Boolean(true)),
                    "false" => Ok(CSObjT::Boolean(false)),
                    "null" => Ok(CSObjT::Null(())),
                    _ => Ok(CSObjT::Op(op.val().name().to_string()))
                }

            }
        }
    }
}

impl ParsleyParser for CSObjP<'_> {
    type T = LocatedVal<CSObjT>;

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

// The text extractor needs to preserve word spacing for legible
// output, and hence extracted text will be represented by the
// following token.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextToken {
    Space,
    RawText(Vec<u8>),
}

// Raw text extractor for the content stream.  It does not handle any font
// processing.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParserState {
    Content,
    Text,
    Path,
    Clipping,
    InlineImage,
}

pub struct TextExtractor<'a> {
    state:          ParserState,
    nested_compats: usize,
    ctxt:           &'a mut PDFObjContext,
    opinfo:         BTreeMap<String, &'static (OpType, &'static [ArgType])>,
    objectid:       ObjectId,
}

impl<'a> TextExtractor<'a> {
    pub fn new(ctxt: &'a mut PDFObjContext, objectid: &ObjectId) -> Self {
        let mut opinfo = BTreeMap::new();
        for (o, i) in OPERATORS.iter() {
            opinfo.insert(o.to_string(), i);
        }
        Self {
            state: ParserState::Content,
            nested_compats: 0,
            ctxt,
            opinfo,
            objectid: *objectid,
        }
    }

    // This parses the entire buffer of the content stream, and
    // returns the text objects collected in order.  Unlike the PDF
    // spec, it assumes the content stream ends outside an object
    // boundary, instead of outside a lexical token as in the spec.
    // If multiple content streams need to be parsed, they should be
    // concatenated first (with inserted whitespace at their join
    // points) before calling this parser.

    pub fn parse_internal(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Vec<TextToken>> {
        // the collected text objects
        let mut texts: Vec<TextToken> = Vec::new();

        // the internal parsers
        let mut op = CSObjP::new(self.ctxt);
        let mut ws = WhitespaceEOL::new(true);

        // collected arguments for an operator
        let mut args = Vec::new();
        loop {
            // First, consume possibly empty whitespace.
            ws.parse(buf)?;

            let start = buf.get_cursor();
            let o = op.parse(buf)?;
            let op_name = match o.val() {
                CSObjT::Comment(_) => continue, // drop comments
                CSObjT::Op(n) => n,
                _ => {
                    args.push(o);
                    continue
                },
            };

            let (op_type, op_args) = match self.opinfo.get(op_name) {
                None => {
                    // Unknown operators can be ignored if we are in a compat section.
                    if self.nested_compats > 0 {
                        args.clear();
                        continue
                    }
                    let msg = format!("unknown PDF operator: {:?}", op_name);
                    let err = ErrorKind::GuardError(msg);
                    let end = buf.get_cursor();
                    return Err(LocatedVal::new(err, start, end))
                },
                Some(info) => info,
            };
            // check operator context validity and compute next state
            // corresponding to Figure 9 in the spec.
            let next_state = match (&self.state, op_type, op_name.as_str()) {
                (
                    ParserState::Content,
                    OpType::GeneralGraphics
                        | OpType::SpecialGraphics
                        | OpType::Color
                        | OpType::TextState
                        | OpType::MarkedContent
                        | OpType::Compat
                    // immediate
                        | OpType::Shading
                        | OpType::XObject,
                    _
                ) => ParserState::Content,
                (ParserState::Content, OpType::TextObject, "BT") =>
                    ParserState::Text,
                (ParserState::Content, OpType::PathConstruction, "m" | "re") =>
                    ParserState::Path,
                (ParserState::Content, OpType::InlineImage, "BI") =>
                    ParserState::InlineImage,

                (ParserState::Path, OpType::PathConstruction, _) =>
                    ParserState::Path,
                (ParserState::Path, OpType::PathPainting, _) =>
                    ParserState::Content,
                (ParserState::Path, OpType::PathClipping, _) =>
                    ParserState::Clipping,

                (ParserState::Clipping, OpType::PathPainting, _) =>
                    ParserState::Content,

                (ParserState::InlineImage, OpType::InlineImage, "ID") =>
                    ParserState::InlineImage,
                (ParserState::InlineImage, OpType::InlineImage, "EI") =>
                    ParserState::Content,

                (
                    ParserState::Text,
                    OpType::GeneralGraphics
                        | OpType::Color
                        | OpType::TextState
                        | OpType::TextShow
                        | OpType::TextPositioning
                        | OpType::MarkedContent
                        | OpType::Compat,
                    _
                ) => ParserState::Text,
                (ParserState::Text, OpType::TextObject, "ET") =>
                    ParserState::Content,
                _ => {
                    // incompatible operator for this state.
                    let msg = format!("unexpected {:?} operator {:?} in state {:?}",
                                      op_type, op_name, &self.state);
                    let err = ErrorKind::GuardError(msg);
                    let end = buf.get_cursor();
                    return Err(LocatedVal::new(err, start, end))
                }
            };

            // handle the appropriate operators.  for text extraction,
            // mainly TextShow operators need to be handled.  the
            // others will affect line-breaking and spacing and may
            // need to also be handled.

            // some basic sanity checking.  TODO: actually check argument types.
            match (op_type, op_name.as_str()) {
                (OpType::TextShow, "Tj" | "'" | "\"") => {
                    if args.len() != op_args.len() {
                        let msg = format!(
                            "content stream {:?}: unexpected number {} of args for {}, {} expected",
                            self.objectid,
                            args.len(),
                            op_name,
                            op_args.len()
                        );
                        let err = ErrorKind::GuardError(msg);
                        let end = buf.get_cursor();
                        return Err(LocatedVal::new(err, start, end))
                    }
                    for a in args.iter() {
                        match a.val() {
                            CSObjT::String(v) => {
                                // We haven't type-checked the args,
                                // so we are relying on the fact that
                                // there is a single text argument to
                                // these operators.
                                if op_name.as_str() != "Tj" {
                                    texts.push(TextToken::Space);
                                }
                                texts.push(TextToken::RawText(v.clone()))
                            },
                            CSObjT::Integer(_) | CSObjT::Real(_) if op_name.as_str() == "\"" => (),
                            _ => {
                                let msg = format!(
                                    "content stream {:?}: unexpected arg type {:?} for {}",
                                    self.objectid,
                                    a.val(),
                                    op_name
                                );
                                let err = ErrorKind::GuardError(msg);
                                let end = buf.get_cursor();
                                return Err(LocatedVal::new(err, start, end))
                            },
                        }
                    }
                },
                (OpType::TextShow, "TJ") => {
                    match args.pop() {
                        Some(o) => match o.val() {
                            CSObjT::Array(array) => {
                                for o in array.objs() {
                                    match o.val() {
                                        PDFObjT::String(v) => {
                                            texts.push(TextToken::RawText(v.clone()))
                                        },

                                        // TODO: handle the case when
                                        // the "TJ" adjustments are
                                        // actually quite large,
                                        // effectively equivalent to
                                        // word-spacing.
                                        PDFObjT::Integer(_) | PDFObjT::Real(_) => (),

                                        _ => {
                                            let msg = format!(
                                                "content stream {:?}: unexpected array element type {:?} for {}",
                                                self.objectid,
                                                o.val(),
                                                op_name
                                            );
                                            let err = ErrorKind::GuardError(msg);
                                            let end = buf.get_cursor();
                                            return Err(LocatedVal::new(err, start, end))
                                        },
                                    }
                                }
                            },
                            _ => {
                                let msg = format!(
                                    "content stream {:?}: unexpected arg type {:?} for {}",
                                    self.objectid,
                                    o.val(),
                                    op_name
                                );
                                let err = ErrorKind::GuardError(msg);
                                let end = buf.get_cursor();
                                return Err(LocatedVal::new(err, start, end))
                            },
                        },
                        None => (), // empty array should be valid
                    }
                },
                (OpType::TextPositioning, "Td" | "TD" | "T*") => texts.push(TextToken::Space),
                // Introduce a space at the start and end of a text object.
                (OpType::TextObject, _) => texts.push(TextToken::Space),
                (OpType::Compat, "BX") => self.nested_compats += 1,
                (OpType::Compat, "EX") => {
                    if self.nested_compats > 0 {
                        self.nested_compats -= 1
                    } else {
                        // TODO: valid-with-warnings
                    }
                },
                _ => (),
            }
            args.clear();

            // clear whitespace
            ws.parse(buf)?;
            if buf.remaining() == 0 {
                break
            }

            // loop with new state
            self.state = next_state;
        }

        Ok(texts)
    }
}

impl ParsleyParser for TextExtractor<'_> {
    type T = LocatedVal<Vec<TextToken>>;

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
