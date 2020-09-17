use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{PDFObjContext, PDFObjT, ReferenceT};
use std::collections::VecDeque;
use std::rc::Rc;

/* Basic type structure of PDF objects */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DictKeySpec {
    Required,
    Optional,
    Forbidden,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DictEntry {
    key: Vec<u8>,
    typ: Rc<PDFType>,
    opt: DictKeySpec,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PDFPrimType {
    Bool,
    String,
    Name,
    Null,
    Integer,
    Real,
    Comment,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PDFType {
    Any,
    PrimType(PDFPrimType),
    Array {
        elem: Rc<PDFType>,
        size: Option<usize>,
    },
    Dict(Vec<DictEntry>),
    Stream(Vec<DictEntry>),
}

/* Errors reported by the type-checker */
#[derive(Debug, PartialEq, Eq)]
pub enum TypeCheckError {
    RefNotFound(ReferenceT),
    ArraySizeMismatch(/* expected */ usize, /* found */ usize),
    MissingKey(Vec<u8>),
    ForbiddenKey(Vec<u8>),
    TypeMismatch(/* expected */ Rc<PDFType>, /* found */ PDFType),
}

/* computes the top-most general type of an object without descending into it */
fn type_of(obj: &PDFObjT) -> PDFType {
    match obj {
        PDFObjT::Dict(_) => PDFType::Dict(Vec::new()),
        PDFObjT::Array(_) => PDFType::Array {
            elem: Rc::new(PDFType::Any),
            size: None,
        },
        PDFObjT::Stream(_) => PDFType::Stream(Vec::new()),
        PDFObjT::Reference(_) => {
            assert!(false); // we should never get a raw reference
            PDFType::Any
        },
        PDFObjT::Boolean(_) => PDFType::PrimType(PDFPrimType::Bool),
        PDFObjT::String(_) => PDFType::PrimType(PDFPrimType::String),
        PDFObjT::Name(_) => PDFType::PrimType(PDFPrimType::Name),
        PDFObjT::Null(_) => PDFType::PrimType(PDFPrimType::Null),
        PDFObjT::Integer(_) => PDFType::PrimType(PDFPrimType::Integer),
        PDFObjT::Real(_) => PDFType::PrimType(PDFPrimType::Real),
        PDFObjT::Comment(_) => PDFType::PrimType(PDFPrimType::Comment),
    }
}

/* checks a parsed PDF object against its expected type */
pub fn check_type(
    ctxt: &PDFObjContext, obj: Rc<LocatedVal<PDFObjT>>, typ: Rc<PDFType>,
) -> Option<TypeCheckError> {
    /* use a queue to avoid recursion-induced stack overflows */
    let mut q = VecDeque::new();
    q.push_back((Rc::clone(&obj), Rc::clone(&typ)));
    while q.len() > 0 {
        let next = q.pop_front();
        if next.is_none() {
            break
        }
        let (o, t) = next.unwrap();
        match (o.val(), t.as_ref()) {
            (_, PDFType::Any) => continue,
            (PDFObjT::Reference(r), _) => {
                // lookup referenced object and add it to the queue
                match ctxt.lookup_obj(r.id()) {
                    Some(obj) => {
                        q.push_back((Rc::clone(obj), t));
                        continue
                    },
                    None => return Some(TypeCheckError::RefNotFound(*r)),
                }
            },
            (PDFObjT::Boolean(_), PDFType::PrimType(PDFPrimType::Bool)) => continue,
            (PDFObjT::String(_), PDFType::PrimType(PDFPrimType::String)) => continue,
            (PDFObjT::Name(_), PDFType::PrimType(PDFPrimType::Name)) => continue,
            (PDFObjT::Null(_), PDFType::PrimType(PDFPrimType::Null)) => continue,
            (PDFObjT::Integer(_), PDFType::PrimType(PDFPrimType::Integer)) => continue,
            (PDFObjT::Real(_), PDFType::PrimType(PDFPrimType::Real)) => continue,
            (PDFObjT::Comment(_), PDFType::PrimType(PDFPrimType::Comment)) => continue,
            (PDFObjT::Array(ao), PDFType::Array { elem, size }) => {
                match size {
                    Some(sz) => {
                        if ao.objs().len() != *sz {
                            return Some(TypeCheckError::ArraySizeMismatch(*sz, ao.objs().len()))
                        }
                    },
                    None => (),
                };
                /* optimize PDFType::Any */
                if let PDFType::Any = elem.as_ref() {
                    continue
                }
                for e in ao.objs() {
                    q.push_back((Rc::clone(e), Rc::clone(elem)))
                }
            },
            (PDFObjT::Dict(d), PDFType::Dict(ents)) => {
                for ent in ents {
                    let val = d.get(&ent.key);
                    match (val, ent.opt, ent.typ.as_ref()) {
                        (None, DictKeySpec::Optional, _) => continue,
                        (None, DictKeySpec::Forbidden, _) => continue,
                        (None, DictKeySpec::Required, _) => {
                            let k = ent.key.clone();
                            return Some(TypeCheckError::MissingKey(k))
                        },
                        (Some(_), DictKeySpec::Forbidden, _) => {
                            let k = ent.key.clone();
                            return Some(TypeCheckError::ForbiddenKey(k))
                        }
                        (Some(_), _, PDFType::Any) => continue,
                        (Some(v), _, _) => q.push_back((Rc::clone(v), Rc::clone(&ent.typ))),
                    }
                }
            },
            (PDFObjT::Stream(s), PDFType::Stream(ents)) => {
                // Same code as above for now, copied in case we need to customize later.
                for ent in ents {
                    let val = s.dict().val().get(&ent.key);
                    match (val, ent.opt, ent.typ.as_ref()) {
                        (None, DictKeySpec::Optional, _) => continue,
                        (None, DictKeySpec::Forbidden, _) => continue,
                        (None, DictKeySpec::Required, _) => {
                            let k = ent.key.clone();
                            return Some(TypeCheckError::MissingKey(k))
                        },
                        (Some(_), DictKeySpec::Forbidden, _) => {
                            let k = ent.key.clone();
                            return Some(TypeCheckError::ForbiddenKey(k))
                        },
                        (Some(_), _, PDFType::Any) => continue,
                        (Some(v), _, _) => q.push_back((Rc::clone(v), Rc::clone(&ent.typ))),
                    }
                }
            },
            (obj, _) => return Some(TypeCheckError::TypeMismatch(Rc::clone(&t), type_of(obj))),
        }
    }
    return None
}

#[cfg(test)]
mod test_pdf_types {
    use super::super::super::pcore::parsebuffer::{ParseBuffer, ParsleyParser};
    use super::super::pdf_obj::{PDFObjContext, PDFObjP};
    use super::{check_type, DictEntry, DictKeySpec, PDFPrimType, PDFType};
    use std::rc::Rc;

    fn mk_rectangle_type() -> PDFType {
        PDFType::Array {
            elem: Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
            size: Some(4),
        }
    }
    #[test]
    fn test_rectangle() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
        let v = Vec::from("[1 2 3 4]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = p.parse(&mut pb).unwrap();
        let typ = mk_rectangle_type();
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);
    }

    #[test]
    fn test_dict() {
        let mut ctxt = PDFObjContext::new();
        let mut p = PDFObjP::new(&mut ctxt);
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = p.parse(&mut pb).unwrap();

        let ent1 = DictEntry {
            key: Vec::from("Entry"),
            typ: Rc::new(mk_rectangle_type()),
            opt: DictKeySpec::Required,
        };
        let ent2 = DictEntry {
            key: Vec::from("Dummy1"),
            typ: Rc::new(mk_rectangle_type()),
            opt: DictKeySpec::Forbidden,
        };
        let ent3 = DictEntry {
            key: Vec::from("Dummy2"),
            typ: Rc::new(mk_rectangle_type()),
            opt: DictKeySpec::Optional,
        };
        let typ = PDFType::Dict(vec![ent1, ent2, ent3]);
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);
    }
}
