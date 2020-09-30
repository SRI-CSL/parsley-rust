use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{PDFObjContext, PDFObjT, ReferenceT};
use std::collections::VecDeque;
use std::rc::Rc;

/* Basic type structure of PDF objects */

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PDFPrimType {
    Bool,
    String,
    Name,
    Null,
    Integer,
    Real,
    Comment,
    Indirect
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DictKeySpec {
    Required,
    Optional,
    Forbidden,
}

#[derive(Debug, PartialEq)]
pub struct DictEntry {
    pub key: Vec<u8>,
    pub chk: Rc<TypeCheck>,
    pub opt: DictKeySpec,
}

#[derive(Debug, PartialEq)]
pub enum PDFType {
    Any,
    PrimType(PDFPrimType),
    Array {
        elem: Rc<TypeCheck>,
        size: Option<usize>,
    },
    Dict(Vec<DictEntry>),
    Stream(Vec<DictEntry>),
}

/* Errors reported by the type-checker */
#[derive(Debug, PartialEq)]
pub enum TypeCheckError {
    RefNotFound(ReferenceT),
    ArraySizeMismatch(/* expected */ usize, /* found */ usize),
    MissingKey(Vec<u8>),
    ForbiddenKey(Vec<u8>),
    TypeMismatch(/* expected */ Rc<PDFType>, /* found */ PDFType),
    ValueMismatch(/* found */ Rc<LocatedVal<PDFObjT>>, String),
    PredicateError(String),
}

// trait wrapper around predicate function
pub trait Predicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError>;
}

pub struct TypeCheck {
    typ:  Rc<PDFType>,
    pred: Option<Box<dyn Predicate>>,
}

impl TypeCheck {
    // the most commonly used constructor
    pub fn new(typ: Rc<PDFType>) -> Self { Self { typ, pred: None } }

    // the constructor with a refinement predicate
    pub fn new_refined(typ: Rc<PDFType>, pred: Box<dyn Predicate>) -> Self {
        Self {
            typ,
            pred: Some(pred),
        }
    }

    pub fn typ(&self) -> &PDFType { self.typ.as_ref() }
    pub fn pred(&self) -> &Option<Box<dyn Predicate>> { &self.pred }
}

impl PartialEq for TypeCheck {
    fn eq(&self, other: &Self) -> bool { *self.typ == *other.typ }
}

impl std::fmt::Debug for TypeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeCheck").field("typ", &self.typ).finish()
    }
}

/* computes the top-most general type of an object without descending into it */
fn type_of(obj: &PDFObjT) -> PDFType {
    match obj {
        PDFObjT::Dict(_) => PDFType::Dict(Vec::new()),
        PDFObjT::Array(_) => PDFType::Array {
            elem: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
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

fn check_predicate(
    obj: &Rc<LocatedVal<PDFObjT>>, pred: &Option<Box<dyn Predicate>>,
) -> Option<TypeCheckError> {
    match pred {
        None => None,
        Some(pred) => pred.check(obj),
    }
}

/* checks a parsed PDF object against its expected type */
pub fn check_type(
    ctxt: &PDFObjContext, obj: Rc<LocatedVal<PDFObjT>>, chk: Rc<TypeCheck>,
) -> Option<TypeCheckError> {
    /* use a queue to avoid recursion-induced stack overflows */
    let mut q = VecDeque::new();
    q.push_back((Rc::clone(&obj), Rc::clone(&chk)));
    while q.len() > 0 {
        let next = q.pop_front();
        if next.is_none() {
            break
        }
        let (o, c) = next.unwrap();
        match (o.val(), c.typ()) {
            (_, PDFType::Any) => continue, // choices are not checked
            (PDFObjT::Reference(r), _) => {
                // lookup referenced object and add it to the queue
                match ctxt.lookup_obj(r.id()) {
                    Some(obj) => {
                        q.push_back((Rc::clone(obj), c));
                        continue
                    },
                    None => return Some(TypeCheckError::RefNotFound(*r)),
                }
            },
            (PDFObjT::Boolean(_), PDFType::PrimType(PDFPrimType::Bool)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
            (PDFObjT::String(_), PDFType::PrimType(PDFPrimType::String)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
            (PDFObjT::Name(_), PDFType::PrimType(PDFPrimType::Name)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
            (PDFObjT::Null(_), PDFType::PrimType(PDFPrimType::Null)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
            (PDFObjT::Integer(_), PDFType::PrimType(PDFPrimType::Integer)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
            (PDFObjT::Real(_), PDFType::PrimType(PDFPrimType::Real)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
            (PDFObjT::Comment(_), PDFType::PrimType(PDFPrimType::Comment)) => {
                let invalid = check_predicate(&o, c.pred());
                if let None = invalid {
                    continue
                } else {
                    return invalid
                }
            },
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
                if let PDFType::Any = elem.typ() {
                    let invalid = check_predicate(&o, c.pred());
                    if let None = invalid {
                        continue
                    } else {
                        return invalid
                    }
                }
                for e in ao.objs() {
                    q.push_back((Rc::clone(e), Rc::clone(elem)))
                }
            },
            (PDFObjT::Dict(d), PDFType::Dict(ents)) => {
                for ent in ents {
                    let val = d.get(&ent.key);
                    match (val, ent.opt, ent.chk.typ()) {
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
                        (Some(v), _, _) => q.push_back((Rc::clone(v), Rc::clone(&ent.chk))),
                    }
                }
            },
            (PDFObjT::Stream(s), PDFType::Stream(ents)) => {
                // Same code as above for now, copied in case we need to customize later.
                for ent in ents {
                    let val = s.dict().val().get(&ent.key);
                    match (val, ent.opt, ent.chk.typ()) {
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
                        (Some(v), _, _) => q.push_back((Rc::clone(v), Rc::clone(&ent.chk))),
                    }
                }
            },
            (obj, _) => {
                return Some(TypeCheckError::TypeMismatch(
                    Rc::clone(&c.typ),
                    type_of(obj),
                ))
            },
        }
    }
    return None
}

// One common predicate is to allow a value in a set.
pub struct ChoicePred(pub String, pub Vec<PDFObjT>);

impl Predicate for ChoicePred {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        let vec = &self.1;
        if vec.into_iter().any(|c| obj.val() == c) {
            None
        } else {
            Some(TypeCheckError::ValueMismatch(
                Rc::clone(obj),
                self.0.clone(),
            ))
        }
    }
}

#[cfg(test)]
mod test_pdf_types {
    use super::super::super::pcore::parsebuffer::{LocatedVal, ParseBuffer};
    use super::super::pdf_obj::{parse_pdf_obj, PDFObjContext, PDFObjT};
    use super::super::pdf_prim::NameT;
    use super::{
        check_type, ChoicePred, DictEntry, DictKeySpec, PDFPrimType, PDFType, Predicate, TypeCheck,
        TypeCheckError,
    };
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    fn mk_rectangle_typchk() -> Rc<TypeCheck> {
        Rc::new(TypeCheck::new(Rc::new(PDFType::Array {
            elem: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
                PDFPrimType::Integer,
            )))),
            size: Some(4),
        })))
    }

    #[test]
    fn test_rectangle() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("[1 2 3 4]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = mk_rectangle_typchk();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_dict() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let ent1 = DictEntry {
            key: Vec::from("Entry"),
            chk: mk_rectangle_typchk(),
            opt: DictKeySpec::Required,
        };
        let ent2 = DictEntry {
            key: Vec::from("Dummy1"),
            chk: mk_rectangle_typchk(),
            opt: DictKeySpec::Forbidden,
        };
        let ent3 = DictEntry {
            key: Vec::from("Dummy2"),
            chk: mk_rectangle_typchk(),
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent1, ent2, ent3])));
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);
    }

    #[test]
    fn test_dict_required() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let ent = DictEntry {
            key: Vec::from("Dummy"),
            chk: mk_rectangle_typchk(),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            Some(TypeCheckError::MissingKey(Vec::from("Dummy")))
        );
    }

    #[test]
    fn test_dict_forbidden() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let ent = DictEntry {
            key: Vec::from("Entry"),
            chk: mk_rectangle_typchk(),
            opt: DictKeySpec::Forbidden,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            Some(TypeCheckError::ForbiddenKey(Vec::from("Entry")))
        );
    }

    fn mk_pagemode_typchk() -> Rc<TypeCheck> {
        let pred = ChoicePred(
            String::from("Invalid PageMode"),
            vec![
                PDFObjT::Name(NameT::new(Vec::from("UseNone"))),
                PDFObjT::Name(NameT::new(Vec::from("UseOutlines"))),
            ],
        );
        Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Box::new(pred),
        ))
    }

    #[test]
    fn test_dict_allowed_value() {
        // valid value for required key
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /PageMode /UseNone >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: mk_pagemode_typchk(),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);

        // valid value for optional key
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /PageMode /UseNone >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: mk_pagemode_typchk(),
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);

        // optional key absent
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: mk_pagemode_typchk(),
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);

        // forbidden key present
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /PageMode /UseNone >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: mk_pagemode_typchk(),
            opt: DictKeySpec::Forbidden,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            Some(TypeCheckError::ForbiddenKey(Vec::from("PageMode")))
        );

        // invalid value for optional key
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /PageMode /Dummy >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: mk_pagemode_typchk(),
            opt: DictKeySpec::Optional,
        };
        let val = Rc::new(LocatedVal::new(
            PDFObjT::Name(NameT::new(Vec::from("Dummy"))),
            0,
            0,
        ));
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            Some(TypeCheckError::ValueMismatch(
                val,
                String::from("Invalid PageMode")
            ))
        );
    }

    struct AsciiStringPredicate;
    impl Predicate for AsciiStringPredicate {
        fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
            if let PDFObjT::String(ref s) = obj.val() {
                for c in s {
                    if *c >= 128 {
                        return Some(TypeCheckError::PredicateError(
                            "Not an ASCII string.".to_string(),
                        ))
                    }
                }
            }
            None
        }
    }

    fn mk_ascii_typchk() -> Rc<TypeCheck> {
        Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::PrimType(PDFPrimType::String)),
            Box::new(AsciiStringPredicate),
        ))
    }

    #[test]
    fn test_ascii_string() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("(ascii)".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let chk = mk_ascii_typchk();
        assert_eq!(check_type(&ctxt, Rc::new(obj), chk), None);

        //                     (                )
        let v: Vec<u8> = vec![40, 129, 255, 0, 41];
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let chk = mk_ascii_typchk();
        let err = TypeCheckError::PredicateError("Not an ASCII string.".to_string());
        assert_eq!(check_type(&ctxt, Rc::new(obj), chk), Some(err));
    }
}
