// Copyright (c) 2019-2021 SRI International.
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

use super::base::PositionNumber;
use crate::pcore::parsebuffer::{ErrorKind, LocatedVal, ParseBufferT, ParseResult, ParsleyParser};
use crate::pcore::prim_binary::{Endian, UInt16P, UInt32P, UInt8P};
use std::collections::BTreeMap;

const MAX_CHANNELS: usize = 65535;
const MAX_STACK: usize = 65535;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SubElemType {
    Calc,
    CurveSet,
    CLUT,
    EmissionCLUT,
    EmissionMatrix,
    EmissionObserver,
    ExtendedCLUT,
    InverseEmissionMatrix,
    JabToXYZ,
    Matrix,
    SparseMatrix,
    ReflectanceCLUT,
    ReflectanceObserver,
    TintArray,
    XYZToJab,
    Future,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SubElemKind {
    Calc,
    CurveSet,
    CLUT,
    Matrix,
    Tint,
    Elem,
    Exact(SubElemType),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SubElemInfo {
    typ:             SubElemType,
    input_channels:  usize,
    output_channels: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum StkResource {
    Stk(usize),
}
impl StkResource {
    pub fn no_stack() -> Self { StkResource::Stk(0) }
    pub fn stk(s: usize) -> Self { StkResource::Stk(s) }
    pub fn hgt(&self) -> usize {
        match &self {
            StkResource::Stk(h) => *h,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct OpResource {
    consume:      StkResource,
    produce:      StkResource,
    tmp_channels: Option<(usize, usize)>, // (start, count)
}
impl OpResource {
    pub fn no_resource() -> Self {
        Self {
            consume:      StkResource::no_stack(),
            produce:      StkResource::no_stack(),
            tmp_channels: None,
        }
    }
    pub fn new(consume: StkResource, produce: StkResource) -> Self {
        Self {
            consume,
            produce,
            tmp_channels: None,
        }
    }
    pub fn new_with_temps(
        consume: StkResource, produce: StkResource, tmps: (usize, usize),
    ) -> Self {
        Self {
            consume,
            produce,
            tmp_channels: Some(tmps),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum OpType {
    Normal,
    If(usize),
    IfElse(usize, usize),
    SelCases(Vec<usize>), // default is just treated as another case for simplicity
    Unknown,
}

impl OpType {
    pub fn streams(&self) -> Vec<usize> {
        match self {
            OpType::Normal | OpType::Unknown => vec![],
            OpType::If(i) => vec![*i],
            OpType::IfElse(i, e) => vec![*i, *e],
            OpType::SelCases(cs) => Vec::from(cs.as_slice()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Operation {
    typ:       OpType,
    resource:  OpResource,
    opstreams: Option<Vec<OpStream>>,
}
impl Operation {
    pub fn new(typ: OpType, resource: OpResource) -> Self {
        Self {
            typ,
            resource,
            opstreams: None,
        }
    }
    pub fn new_with_streams(typ: OpType, resource: OpResource, streams: Vec<OpStream>) -> Self {
        Self {
            typ,
            resource,
            opstreams: Some(streams),
        }
    }
    // This representation bunches together multiple associated
    // branching operations into a single Operation for implementation
    // simplicity of resource bound computation.  However, the count
    // of operations within a stream counts the individual components
    // separately; this function returns that count.
    pub fn num_ops(&self) -> usize {
        match &self.typ {
            OpType::Normal => 1,
            OpType::If(_) => 1,
            OpType::IfElse(_, _) => 2,
            OpType::SelCases(cs) =>
            /* sel + (cases + dflt) */
            {
                1 + cs.len()
            },
            OpType::Unknown => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OpStream {
    stream: Vec<Operation>,
}
impl OpStream {
    pub fn new(stream: Vec<Operation>) -> Self { Self { stream } }
}

pub struct OperationP {
    subelem_map:  BTreeMap<usize, SubElemInfo>,
    in_channels:  usize,
    out_channels: usize,
}

impl OperationP {
    pub fn new(
        subelem_map: BTreeMap<usize, SubElemInfo>, in_channels: usize, out_channels: usize,
    ) -> Self {
        Self {
            subelem_map,
            in_channels,
            out_channels,
        }
    }
}

impl ParsleyParser for OperationP {
    type T = LocatedVal<Operation>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        let mut sig = String::new();
        let mut p = UInt8P;
        for _ in 0 .. 4 {
            let r = p.parse(buf)?;
            sig += &(*r.val() as char).to_string();
        }
        let (typ, resource) = opinfo_from_sig(&self, start, &sig, buf)?;
        let op = match typ {
            OpType::Normal | OpType::Unknown => Operation::new(typ, resource),
            OpType::If(t) => {
                // collect the stream
                let mut p = OpStreamP::new(
                    self.subelem_map.clone(),
                    self.in_channels,
                    self.out_channels,
                    t,
                );
                let stream = p.parse(buf)?;
                let stream = stream.unwrap();
                Operation::new_with_streams(typ, resource, vec![stream])
            },
            OpType::IfElse(t, u) => {
                // collect the two streams
                let mut p = OpStreamP::new(
                    self.subelem_map.clone(),
                    self.in_channels,
                    self.out_channels,
                    t,
                );
                let ifstm = p.parse(buf)?;
                let ifstm = ifstm.unwrap();
                let mut p = OpStreamP::new(
                    self.subelem_map.clone(),
                    self.in_channels,
                    self.out_channels,
                    u,
                );
                let elsestm = p.parse(buf)?;
                let elsestm = elsestm.unwrap();
                Operation::new_with_streams(typ, resource, vec![ifstm, elsestm])
            },
            OpType::SelCases(ref cs) => {
                let mut streams = Vec::<OpStream>::new();
                for c in cs {
                    let mut p = OpStreamP::new(
                        self.subelem_map.clone(),
                        self.in_channels,
                        self.out_channels,
                        *c,
                    );
                    let s = p.parse(buf)?;
                    let s = s.unwrap();
                    streams.push(s)
                }
                Operation::new_with_streams(typ, resource, streams)
            },
        };
        Ok(LocatedVal::new(op, start, buf.get_cursor()))
    }
}

pub struct OpStreamP {
    subelem_map:  BTreeMap<usize, SubElemInfo>,
    in_channels:  usize,
    out_channels: usize,
    num_ops:      usize,
}

impl OpStreamP {
    pub fn new(
        subelem_map: BTreeMap<usize, SubElemInfo>, in_channels: usize, out_channels: usize,
        num_ops: usize,
    ) -> Self {
        Self {
            subelem_map,
            in_channels,
            out_channels,
            num_ops,
        }
    }
}

impl ParsleyParser for OpStreamP {
    type T = LocatedVal<OpStream>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut p = OperationP::new(
            self.subelem_map.clone(),
            self.in_channels,
            self.out_channels,
        );
        let mut stream = Vec::<Operation>::new();
        let mut num_ops = 0;
        while num_ops < self.num_ops {
            let op = p.parse(buf)?;
            let op = op.unwrap();
            num_ops += op.num_ops();
            stream.push(op)
        }
        if num_ops > self.num_ops {
            let msg = format!(
                "found {} operations in stream, expected {}",
                num_ops, self.num_ops
            );
            let err = ErrorKind::GuardError(msg);
            return Err(LocatedVal::new(err, start, buf.get_cursor()))
        }
        let stream = OpStream::new(stream);
        return Ok(LocatedVal::new(stream, start, buf.get_cursor()))
    }
}

// various helpers used from above parsers.

fn subelem_type_check(_se_kind: SubElemKind, _se_typ: SubElemType) -> bool {
    // type-check logic.  not interesting.
    true
}

fn opinfo_from_sig(
    op: &OperationP, start: usize, sig: &str, buf: &mut dyn ParseBufferT,
) -> ParseResult<(OpType, OpResource)> {
    match sig {
        // Table 87: FP constant operations
        "data" => {
            let mut p = UInt32P::new(Endian::Big);
            let _ = p.parse(buf)?;
            Ok((OpType::Normal, OpResource::no_resource()))
        },
        // Table 89: channel vector operations
        "in  " => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            if usize::from(s + t) >= op.in_channels {
                let msg = format!(
                    "Use of out-of-bound input channel {} (of {})",
                    s + t,
                    op.in_channels
                );
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::no_stack();
            let produce = StkResource::stk(usize::from(t + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "out " => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            if usize::from(s + t) >= op.out_channels {
                let msg = format!(
                    "Use of out-of-bound output channel {} (of {})",
                    s + t,
                    op.out_channels
                );
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(usize::from(t + 1));
            let produce = StkResource::no_stack();
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "tget" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            if usize::from(s + t) >= MAX_CHANNELS {
                let msg = format!(
                    "Use of out-of-bound temp channel {} (of {})",
                    s + t,
                    MAX_CHANNELS
                );
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::no_stack();
            let produce = StkResource::stk(usize::from(t + 1));
            let tmps = (usize::from(*s), usize::from(*t));
            Ok((
                OpType::Normal,
                OpResource::new_with_temps(consume, produce, tmps),
            ))
        },
        "tput" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            if usize::from(s + t) >= MAX_CHANNELS {
                let msg = format!(
                    "Use of out-of-bound temp channel {} (of {})",
                    s + t,
                    MAX_CHANNELS
                );
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(usize::from(t + 1));
            let produce = StkResource::no_stack();
            let tmps = (usize::from(*s), usize::from(*t));
            Ok((
                OpType::Normal,
                OpResource::new_with_temps(consume, produce, tmps),
            ))
        },
        "tsav" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            if usize::from(s + t) >= MAX_CHANNELS {
                let msg = format!(
                    "Use of out-of-bound temp channel {} (of {})",
                    s + t,
                    MAX_CHANNELS
                );
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(usize::from(t + 1));
            let produce = StkResource::stk(usize::from(t + 1));
            let tmps = (usize::from(*s), usize::from(*t));
            Ok((
                OpType::Normal,
                OpResource::new_with_temps(consume, produce, tmps),
            ))
        },
        // Table 91: environment variable operations
        "env " => {
            let mut p = UInt32P::new(Endian::Big);
            let _ = p.parse(buf)?;
            let consume = StkResource::no_stack();
            let produce = StkResource::stk(2);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // Table 94: sub-element operations
        "curv" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(SubElemKind::CurveSet, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for curv", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "mtx " => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(SubElemKind::Matrix, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for mtx", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "clut" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(SubElemKind::CLUT, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for clut", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "calc" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(SubElemKind::Calc, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for calc", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "tint" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(SubElemKind::Tint, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for tint", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "elem" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            // No type check for `elem` sub-elements.
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // undocumented ops.
        "fJab" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let k = SubElemKind::Exact(SubElemType::JabToXYZ);
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(k, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for fJab", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "tJab" => {
            let mut p = UInt32P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let k = SubElemKind::Exact(SubElemType::XYZToJab);
            let se = match op.subelem_map.get(&(*s as usize)) {
                None => {
                    let msg = format!("Invalid sub-elem {}", *s);
                    let err = ErrorKind::GuardError(msg);
                    return Err(LocatedVal::new(err, start, buf.get_cursor()))
                },
                Some(se) => se,
            };
            if !subelem_type_check(k, se.typ) {
                let msg = format!("Invalid sub-elem type {:?} for tJab", se.typ);
                let err = ErrorKind::GuardError(msg);
                return Err(LocatedVal::new(err, start, buf.get_cursor()))
            }
            let consume = StkResource::stk(se.input_channels);
            let produce = StkResource::stk(se.output_channels);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // Table 96: stack operations
        "copy" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            let consume = StkResource::stk(usize::from(s + 1));
            let produce = StkResource::stk(usize::from(s + 1) * usize::from(t + 2));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "rotl" | "rotr" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?;
            let consume = StkResource::stk(usize::from(s + 1));
            let produce = StkResource::stk(usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "posd" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            let consume = StkResource::stk(usize::from(s + 1));
            let produce = StkResource::stk(usize::from(s + 1 + t + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "flip" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(usize::from(s + 2));
            let produce = StkResource::stk(usize::from(s + 2));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "pop " => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(usize::from(s + 1));
            let produce = StkResource::no_stack();
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // Table 98: matrix operations
        "solv" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            let consume = StkResource::stk(usize::from(s + 1) * usize::from(t + 2));
            let produce = StkResource::stk(usize::from(t + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "tran" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let t = p.parse(buf)?;
            let t = t.val();
            let consume = StkResource::stk(usize::from(s + 1) * usize::from(t + 1));
            let produce = StkResource::stk(usize::from(s + 1) * usize::from(t + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // Table 100: variable length functional operations
        "sum " | "prod" | "min " | "max " | "and " | "or  " => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(usize::from(s + 1));
            let produce = StkResource::stk(1);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // Table 101: functional vector operation encoding
        "pi  " | "+INF" | "-INF" | "NaN " => {
            let mut p = UInt16P::new(Endian::Big);
            let _ = p.parse(buf)?;
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::no_stack();
            let produce = StkResource::stk(1);
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "add " | "sub " | "mul " | "div " | "mod " | "pow " => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(2 * usize::from(s + 1));
            let produce = StkResource::stk(usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "gama" | "sadd" | "ssub" | "smul" | "sdiv" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(usize::from(s + 2));
            let produce = StkResource::stk(usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "sq  " | "sqrt" | "cb  " | "cbrt" | "abs " | "neg " | "rond" | "flor" | "ceil" | "trnc"
        | "sign" | "exp " | "log " | "ln  " | "sin " | "cos " | "tan " | "asin" | "acos"
        | "atan" | "rnum" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(usize::from(s + 1));
            let produce = StkResource::stk(usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "atn2" | "lt  " | "le  " | "eq  " | "near" | "ge  " | "gt  " | "vmin" | "vmax" | "vand"
        | "vor " => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(2 * usize::from(s + 1));
            let produce = StkResource::stk(usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "ctop" | "ptoc" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(2 * usize::from(s + 1));
            let produce = StkResource::stk(2 * usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        "tLab" | "tXYZ" | "fLab" => {
            let mut p = UInt16P::new(Endian::Big);
            let s = p.parse(buf)?;
            let s = s.val();
            let _ = p.parse(buf)?; // should be zero
            let consume = StkResource::stk(3 * usize::from(s + 1));
            let produce = StkResource::stk(3 * usize::from(s + 1));
            Ok((OpType::Normal, OpResource::new(consume, produce)))
        },
        // Table 103/104: conditional if and if with else
        "if  " => {
            let mut p = UInt32P::new(Endian::Big);
            let t = p.parse(buf)?;
            let t = *t.val() as usize;
            // Check if this is followed by an 'else'.
            let cursor = buf.get_cursor();
            let mut sig = String::new();
            let mut p = UInt8P;
            for _ in 0 .. 4 {
                match p.parse(buf) {
                    Err(_) => break,
                    Ok(r) => sig += &(*r.val() as char).to_string(),
                }
            }
            let consume = StkResource::stk(1);
            let produce = StkResource::no_stack();
            if sig == "else" {
                let u = p.parse(buf)?;
                let u = *u.val() as usize;
                Ok((OpType::IfElse(t, u), OpResource::new(consume, produce)))
            } else {
                // rewind back to the start of the sig
                let ret = buf.set_cursor(cursor);
                assert!(ret.is_ok());
                Ok((OpType::If(t), OpResource::new(consume, produce)))
            }
        },
        // Table 105: selection 'sel' with 'dflt'
        "sel " => {
            let mut p32 = UInt32P::new(Endian::Big);
            let _ = p32.parse(buf)?; // should be zero

            let mut sig = String::new();
            let mut cases = Vec::<usize>::new();
            // collect cases
            let mut p8 = UInt8P;
            loop {
                let cursor = buf.get_cursor();
                sig.clear();
                for _ in 0 .. 4 {
                    match p8.parse(buf) {
                        Err(_) => break,
                        Ok(r) => sig += &(*r.val() as char).to_string(),
                    }
                }
                if sig == "case" {
                    let u = p32.parse(buf)?;
                    cases.push(*u.val() as usize);
                } else if sig == "dflt" {
                    let u = p32.parse(buf)?;
                    cases.push(*u.val() as usize);
                    break
                } else {
                    // rewind back to the start of the sig
                    let ret = buf.set_cursor(cursor);
                    assert!(ret.is_ok());
                    break
                }
            }
            let consume = StkResource::stk(1);
            let produce = StkResource::no_stack();
            Ok((OpType::SelCases(cases), OpResource::new(consume, produce)))
        },
        "else" | "case" | "dflt" => {
            // it is an error to see these here
            let msg = format!("unguarded branching operation '{}'", sig);
            let err = ErrorKind::GuardError(msg);
            return Err(LocatedVal::new(err, start, buf.get_cursor()))
        },
        _ => {
            // consume remaining bytes of entry
            let mut p = UInt32P::new(Endian::Big);
            let _ = p.parse(buf)?;
            Ok((OpType::Unknown, OpResource::no_resource()))
        },
    }
}

// resource bound estimation

#[derive(Debug, Clone, Copy)]
pub struct StkUsage {
    min: usize,
    max: usize,
}
impl StkUsage {
    fn new() -> Self { Self { min: 0, max: 0 } }
    fn bounds() -> Self {
        Self {
            min: usize::MAX,
            max: 0,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum StkBoundError {
    Overflow(usize),
    Underflow(usize),
}

// recursive function to accumulate resource bounds across a stream
pub fn compute_usage_bounds(os: &OpStream, bnd: StkUsage) -> Result<StkUsage, StkBoundError> {
    assert!(bnd.min <= bnd.max);
    let mut usg = bnd;
    for op in os.stream.iter() {
        let c = op.resource.consume.hgt();
        if c > usg.min {
            return Err(StkBoundError::Underflow(c - usg.min))
        }
        let p = op.resource.produce.hgt();
        // these are unsigneds, so be careful not to wrap.
        if p + usg.max >= MAX_STACK + c {
            return Err(StkBoundError::Overflow(p + usg.max - MAX_STACK - c))
        }
        if c < p {
            usg.min += p - c;
            usg.max += p - c;
        } else {
            usg.min -= c - p;
            usg.max -= c - p;
        }
        // handle any branching from this operation
        if let Some(branches) = &op.opstreams {
            let mut bnds = StkUsage::bounds();
            for os in branches.iter() {
                // todo: a version without this recursion
                let bu = compute_usage_bounds(os, usg)?;
                // accumulate the minimum min and the maximum max
                if bu.min < bnds.min {
                    bnds.min = bu.min
                }
                if bu.max > bnds.max {
                    bnds.max = bu.max
                }
            }
            // update usg for the next iteration
            usg = bnds;
        }
    }
    assert!(usg.min <= usg.max);
    Ok(usg)
}

// Resource validation logic: this takes the stream representing the
// main function and returns whether stack usage is always within
// bounds.  It currently ignores temp channel info, and assumes the
// minumum max for stack size.
// TODO: track location information for better diagnostics.
pub fn validate_stack_usage(os: &OpStream) -> Result<(), StkBoundError> {
    let usg = StkUsage::new();
    let _ = compute_usage_bounds(os, usg)?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CalcElemInfo {
    number_of_subelements:  u32,
    main_function_position: u32,
    main_function_size:     u32,
    subelement_positions:   Vec<PositionNumber>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GeneralElement {
    typ:             SubElemType,
    input_channels:  u16,
    output_channels: u16,
}
impl GeneralElement {
    pub fn new(typ: SubElemType, input_channels: u16, output_channels: u16) -> Self {
        Self {
            typ,
            input_channels,
            output_channels,
        }
    }
    pub fn typ(self) -> SubElemType { self.typ }
    pub fn input_channels(self) -> u16 { self.input_channels }
    pub fn output_channels(self) -> u16 { self.output_channels }
}
pub struct GeneralElementP;
impl ParsleyParser for GeneralElementP {
    type T = LocatedVal<GeneralElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut p8 = UInt8P;
        let mut p16 = UInt16P::new(Endian::Big);
        let mut p32 = UInt32P::new(Endian::Big);

        let start = buf.get_cursor();

        let mut sig = String::new();
        for _ in 0 .. 4 {
            let r = p8.parse(buf)?;
            sig += &(*r.val() as char).to_string();
        }
        let _ = p32.parse(buf)?; // reserved
        let input_channels = p16.parse(buf)?;
        let input_channels = input_channels.unwrap();
        let output_channels = p16.parse(buf)?;
        let output_channels = output_channels.unwrap();

        let typ = match sig.as_str() {
            "calc" => SubElemType::Calc,
            "cvst" | "sngf" | "curf" | "parf" | "samf" => SubElemType::CurveSet,
            "clut" => SubElemType::CLUT,
            "eclt" => SubElemType::EmissionCLUT,
            "emtx" => SubElemType::EmissionMatrix,
            "eobs" => SubElemType::EmissionObserver,
            "xclt" => SubElemType::ExtendedCLUT,
            "iemx" => SubElemType::InverseEmissionMatrix,
            "JtoX" => SubElemType::JabToXYZ,
            "matf" => SubElemType::Matrix,
            "smet" => SubElemType::SparseMatrix,
            "rclt" => SubElemType::ReflectanceCLUT,
            "robs" => SubElemType::ReflectanceObserver,
            "tint" => SubElemType::TintArray,
            "XtoJ" => SubElemType::XYZToJab,
            _ => SubElemType::Future,
        };

        let g = GeneralElement::new(typ, input_channels, output_channels);
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}
