use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{DictKey, PDFObjContext, PDFObjT, ReferenceT};
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PDFPrimType {
    Bool,
    String,
    Name,
    Null,
    Integer,
    Real,
    Comment,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IndirectSpec {
    Required,
    Allowed, // the default
    Forbidden,
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
    Disjunct(Vec<Rc<TypeCheck>>),
}

/* Errors reported by the type-checker */
#[derive(Debug, PartialEq)]
pub enum TypeCheckError {
    RefNotFound(ReferenceT),
    ArraySizeMismatch(/* expected */ usize, /* found */ usize),
    MissingKey(DictKey),
    ForbiddenKey(DictKey),
    TypeMismatch(/* expected */ Rc<PDFType>, /* found */ PDFType),
    ValueMismatch(/* found */ Rc<LocatedVal<PDFObjT>>, String),
    PredicateError(String),
    UnknownTypeCheck(String), /* undefined named typecheck */
}

// trait wrapper around predicate function
pub trait Predicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError>;
}

// typecheck context containing the named checks
pub struct TypeCheckContext {
    map: BTreeMap<String, Rc<TypeCheckRep>>,
}

impl TypeCheckContext {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    pub fn register(&mut self, chk: &Rc<TypeCheckRep>) {
        // This intentionally does not catch duplicates.
        self.map.insert(String::from(chk.name()), Rc::clone(chk));
    }
    pub fn lookup(&self, name: &str) -> Option<Rc<TypeCheckRep>> {
        self.map.get(name).map(|tc| Rc::clone(&tc))
    }
}

impl Default for TypeCheckContext {
    fn default() -> Self { Self::new() }
}

// the type check representation
pub struct TypeCheckRep {
    name:     String,
    typ:      Rc<PDFType>,
    pred:     Option<Rc<dyn Predicate>>,
    indirect: IndirectSpec,
}

impl TypeCheckRep {
    // created via calls to TypeCheck::new*(), so there are no new
    // calls here.

    // FIXME: this should eventually go away, once we have more
    // efficient once-only normalization.  But for now, since this is
    // called during every check, we don't need to register/replace it
    // in the context.
    pub fn new_replace_typ(typ: PDFType, chk: &Rc<TypeCheckRep>) -> Rc<TypeCheckRep> {
        Rc::new(TypeCheckRep {
            name:     String::from(chk.name()),
            typ:      Rc::new(typ),
            pred:     chk.pred.as_ref().map(|p| Rc::clone(&p)),
            indirect: chk.indirect,
        })
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn typ(&self) -> &PDFType { self.typ.as_ref() }
    pub fn typ_rc(&self) -> &Rc<PDFType> { &self.typ }
    pub fn pred(&self) -> &Option<Rc<dyn Predicate>> { &self.pred }
    pub fn indirect(&self) -> IndirectSpec { self.indirect }

    // make an indirect-allowed version of the check.
    pub fn allow_indirect(&self) -> Rc<Self> {
        Rc::new(Self {
            name:     String::from(&self.name),
            typ:      Rc::clone(&self.typ),
            pred:     self.pred.as_ref().map(|p| Rc::clone(&p)),
            indirect: IndirectSpec::Allowed,
        })
    }
}

// a type check either is a full representation, or the name of a
// representation.
#[derive(Debug, PartialEq)]
pub enum TypeCheck {
    Rep(Rc<TypeCheckRep>),
    Named(String),
}

impl TypeCheck {
    // the most commonly used constructor
    pub fn new(tctx: &mut TypeCheckContext, name: &str, typ: Rc<PDFType>) -> Rc<Self> {
        let tc = Rc::new(TypeCheckRep {
            name: String::from(name),
            typ,
            pred: None,
            indirect: IndirectSpec::Allowed,
        });
        tctx.register(&tc);
        Rc::new(TypeCheck::Rep(tc))
    }

    // the constructor with a refinement predicate
    pub fn new_refined(
        tctx: &mut TypeCheckContext, name: &str, typ: Rc<PDFType>, pred: Rc<dyn Predicate>,
    ) -> Rc<Self> {
        let tc = Rc::new(TypeCheckRep {
            name: String::from(name),
            typ,
            pred: Some(pred),
            indirect: IndirectSpec::Allowed,
        });
        tctx.register(&tc);
        Rc::new(TypeCheck::Rep(tc))
    }

    // the constructor with an indirect specification
    pub fn new_indirect(
        tctx: &mut TypeCheckContext, name: &str, typ: Rc<PDFType>, indirect: IndirectSpec,
    ) -> Rc<Self> {
        let tc = Rc::new(TypeCheckRep {
            name: String::from(name),
            typ,
            pred: None,
            indirect,
        });
        tctx.register(&tc);
        Rc::new(TypeCheck::Rep(tc))
    }

    pub fn new_all(
        tctx: &mut TypeCheckContext, name: &str, typ: Rc<PDFType>, pred: Option<Rc<dyn Predicate>>,
        indirect: IndirectSpec,
    ) -> Rc<Self> {
        let tc = Rc::new(TypeCheckRep {
            name: String::from(name),
            typ,
            pred,
            indirect,
        });
        tctx.register(&tc);
        Rc::new(TypeCheck::Rep(tc))
    }

    // a type check named after another one, used for recursion
    pub fn new_named(name: &str) -> Rc<Self> { Rc::new(TypeCheck::Named(String::from(name))) }
}

impl PartialEq for TypeCheckRep {
    fn eq(&self, other: &Self) -> bool { *self.typ == *other.typ }
}

impl std::fmt::Debug for TypeCheckRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeCheck")
            .field("name", &self.name)
            .field("typ", &self.typ)
            .finish()
    }
}

/* computes the top-most general type of an object without descending into it */
fn type_of(obj: &PDFObjT) -> PDFType {
    // create a dummy context.
    let mut tctx = TypeCheckContext::new();
    match obj {
        PDFObjT::Dict(_) => PDFType::Dict(Vec::new()),
        PDFObjT::Array(_) => PDFType::Array {
            elem: TypeCheck::new(&mut tctx, "", Rc::new(PDFType::Any)),
            size: None,
        },
        PDFObjT::Stream(_) => PDFType::Stream(Vec::new()),
        PDFObjT::Reference(_) => {
            unreachable!(); // we should never get a raw reference
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
    obj: &Rc<LocatedVal<PDFObjT>>, pred: &Option<Rc<dyn Predicate>>,
) -> Option<TypeCheckError> {
    match pred {
        None => None,
        Some(pred) => pred.check(obj),
    }
}

/* The type check does a depth-first check, maintaining a state
 * represented by a stack of sets of pending checks.  Each set of
 * checks on the stack contains an index into the first element of the
 * set pointing at the next check from that element to be processed.
 * This index applies to Disjunct entries; for other entries, it
 * should be zero.  This index is used to control backtracking.
 */

/* A pending check */
type PendingCheck = (Rc<LocatedVal<PDFObjT>>, Rc<TypeCheck>);
/* An entry on the stack */
type PendingSet = VecDeque<PendingCheck>;
/* The state */
type State = VecDeque<(PendingSet, usize)>;

/* The result of picking the next check given the current state. */
type GetResult = Result<Option<PendingCheck>, ()>;

/* This picks the next check from the given state, given the current
 * check error if any. */
fn get_next_check(state: &mut State, check_error: &Option<TypeCheckError>) -> GetResult {
    loop {
        if let Some((pending, next_idx)) = state.get_mut(0) {
            if let Some((obj, tc)) = pending.pop_front() {
                match tc.as_ref() {
                    TypeCheck::Rep(chk) => {
                        match chk.as_ref().typ() {
                            // an in-progress disjunct
                            PDFType::Disjunct(set) if *next_idx > 0 => {
                                // if there is no match error, this
                                // disjunct checked successfully, so
                                // we can take the next check after
                                // resetting the index.
                                if check_error.is_none() {
                                    *next_idx = 0;
                                    continue
                                } else {
                                    // if there is an error, but there
                                    // are remaining cases to try in
                                    // the disjunct, we adjust the
                                    // index and return the next case.
                                    if *next_idx < set.len() {
                                        let c = Rc::clone(&set[*next_idx]);
                                        *next_idx += 1;
                                        pending.push_front((Rc::clone(&obj), tc));
                                        return Ok(Some((obj, c)))
                                    } else {
                                        // if there is an error but
                                        // there is no remaining case
                                        // in the disjunct, we need to
                                        // unwind to the previous
                                        // disjunct if any.
                                        if unwind(state) {
                                            continue
                                        } else {
                                            return Err(())
                                        }
                                    }
                                }
                            },
                            _ if check_error.is_some() => {
                                // unwind to the previous disjunct
                                if unwind(state) {
                                    continue
                                } else {
                                    return Err(())
                                }
                            },
                            // an unprocessed disjunct
                            PDFType::Disjunct(set) => {
                                if set.is_empty() {
                                    // No options to try: this is a
                                    // check specification error.
                                    unreachable!()
                                } else {
                                    // Take the first option, and mark
                                    // this disjunct in progress.
                                    let c = Rc::clone(&set[0]);
                                    *next_idx = 1;
                                    pending.push_front((Rc::clone(&obj), tc));
                                    return Ok(Some((obj, c)))
                                }
                            },
                            // a normal singular check
                            _ => return Ok(Some((obj, tc))),
                        }
                    },
                    TypeCheck::Named(_) if check_error.is_some() => {
                        // unwind to the previous disjunct
                        if unwind(state) {
                            continue
                        } else {
                            return Err(())
                        }
                    },
                    TypeCheck::Named(_) => return Ok(Some((obj, tc))),
                }
            } else {
                // we finished the last check in the set at the top.
                if check_error.is_some() {
                    if unwind(state) {
                        continue
                    } else {
                        return Err(())
                    }
                } else {
                    state.pop_front();
                    continue
                }
            }
        } else {
            // no more pending checks.
            if check_error.is_some() {
                return Err(())
            } else {
                return Ok(None)
            }
        }
    }
}

/* Unwinds a state upto an in-progress disjunct, if any; returns
 * whether it was successful. */
fn unwind(state: &mut State) -> bool {
    loop {
        if let Some((pending, next_idx)) = state.get_mut(0) {
            if let Some((_, tc)) = pending.front() {
                match tc.as_ref() {
                    TypeCheck::Rep(chk) => match chk.typ() {
                        // found an in-progress disjunct
                        PDFType::Disjunct(_) if *next_idx > 0 => return true,
                        _ => {
                            // discard this pending set
                            state.pop_front();
                            continue
                        },
                    },
                    TypeCheck::Named(_) => {
                        // discard this pending set
                        state.pop_front();
                        continue
                    },
                }
            } else {
                // no more in the top level set.
                state.pop_front();
                continue
            }
        } else {
            // no more pending checks
            return false
        }
    }
}

/* returns the last retrieved check back to the state, which must
 * non-empty (i.e. it should not have been modified since the
 * retrieval.) */
fn return_check(state: &mut State, chk: PendingCheck) {
    if let Some((pending, _)) = state.get_mut(0) {
        pending.push_front(chk);
    } else {
        unreachable!()
    }
}

/* adds a new set of checks to the state */
fn push_checks(state: &mut State, chks: Vec<PendingCheck>) {
    let mut set = VecDeque::new();
    for c in chks {
        set.push_back(c)
    }
    state.push_front((set, 0))
}

/* removes directly nested disjuncts */
pub(super) fn normalize_check(typ: &Rc<TypeCheckRep>) -> Rc<TypeCheckRep> {
    match typ.typ() {
        PDFType::Any | PDFType::PrimType(_) => Rc::clone(typ),
        PDFType::Array { elem, size } => TypeCheckRep::new_replace_typ(
            PDFType::Array {
                elem: match elem.as_ref() {
                    TypeCheck::Rep(r) => Rc::new(TypeCheck::Rep(normalize_check(r))),
                    _ => Rc::clone(elem),
                },
                size: *size,
            },
            typ,
        ),
        PDFType::Dict(ents) => {
            let mut v = Vec::new();
            for e in ents {
                v.push(DictEntry {
                    key: e.key.clone(),
                    chk: match e.chk.as_ref() {
                        TypeCheck::Rep(r) => Rc::new(TypeCheck::Rep(normalize_check(r))),
                        _ => Rc::clone(&e.chk),
                    },
                    opt: e.opt,
                })
            }
            TypeCheckRep::new_replace_typ(PDFType::Dict(v), typ)
        },
        PDFType::Stream(ents) => {
            let mut v = Vec::new();
            for e in ents {
                v.push(DictEntry {
                    key: e.key.clone(),
                    chk: match e.chk.as_ref() {
                        TypeCheck::Rep(r) => Rc::new(TypeCheck::Rep(normalize_check(r))),
                        _ => Rc::clone(&e.chk),
                    },
                    opt: e.opt,
                })
            }
            TypeCheckRep::new_replace_typ(PDFType::Stream(v), typ)
        },
        PDFType::Disjunct(opts) => {
            let mut v = Vec::new();
            for o in opts {
                match o.as_ref() {
                    TypeCheck::Rep(r) => {
                        let flat = normalize_check(r);
                        if let PDFType::Disjunct(nested) = flat.typ() {
                            for n in nested {
                                v.push(Rc::clone(n))
                            }
                        } else {
                            v.push(Rc::new(TypeCheck::Rep(flat)))
                        }
                    },
                    TypeCheck::Named(_) => v.push(Rc::clone(o)),
                }
            }
            TypeCheckRep::new_replace_typ(PDFType::Disjunct(v), typ)
        },
    }
}

fn resolve(
    tctx: &TypeCheckContext, chk: Rc<TypeCheck>,
) -> Result<Rc<TypeCheckRep>, TypeCheckError> {
    match chk.as_ref() {
        TypeCheck::Rep(r) => Ok(Rc::clone(r)),
        TypeCheck::Named(n) => match tctx.lookup(n) {
            None => Err(TypeCheckError::UnknownTypeCheck(format!(
                "Unknown typecheck {}",
                n
            ))),
            Some(rep) => Ok(Rc::clone(&rep)),
        },
    }
}

/* checks a parsed PDF object against its expected type */
pub fn check_type(
    ctxt: &PDFObjContext, tctx: &TypeCheckContext, obj: Rc<LocatedVal<PDFObjT>>, chk: Rc<TypeCheck>,
) -> Option<TypeCheckError> {
    // Resolve the check if needed.
    let rep = match resolve(tctx, chk) {
        Ok(rep) => rep,
        Err(err) => return Some(err),
    };

    /* normalize the given type */
    let chk = Rc::new(TypeCheck::Rep(normalize_check(&rep)));

    /* state initialization */
    let mut first = VecDeque::new();
    first.push_back((Rc::clone(&obj), Rc::clone(&chk)));
    let mut state = VecDeque::new();
    state.push_back((first, 0));

    let mut result = None;

    /* work loop */
    loop {
        let next: GetResult = get_next_check(&mut state, &result);
        if next.is_err() {
            // there can only be an error in getting the next check if
            // we already have an error.
            assert!(result.is_some());
            return result
        }

        let next = next.unwrap();
        if next.is_none() {
            // if there are no more checks left, all checks must have
            // passed.
            assert!(result.is_none());
            return result
        }
        let (o, tc) = next.unwrap();

        let c = match resolve(tctx, tc) {
            Ok(rep) => rep,
            Err(err) => return Some(err),
        };

        // reset for the next check.
        result = None;

        match (o.val(), c.typ(), c.indirect()) {
            // Indirects are best handled first.
            (PDFObjT::Reference(refnc), _, IndirectSpec::Allowed)
            | (PDFObjT::Reference(refnc), _, IndirectSpec::Required) => {
                // lookup referenced object and add it to the queue
                match ctxt.lookup_obj(refnc.id()) {
                    Some(obj) => {
                        // Remove any Required indirect from the check.
                        let chk = Rc::new(TypeCheck::Rep(c.allow_indirect()));
                        return_check(&mut state, (Rc::clone(obj), chk));
                    },
                    None => return Some(TypeCheckError::RefNotFound(*refnc)),
                }
            },
            (PDFObjT::Reference(_), _, IndirectSpec::Forbidden) => {
                result = Some(TypeCheckError::ValueMismatch(
                    Rc::clone(&o),
                    String::from("An indirect reference was forbidden"),
                ))
            },
            (_, _, IndirectSpec::Required) => {
                result = Some(TypeCheckError::ValueMismatch(
                    Rc::clone(&o),
                    String::from("An indirect reference was required"),
                ))
            },

            (_, PDFType::Disjunct(_), _) => {
                // We should not get Disjuncts directly.
                return Some(TypeCheckError::PredicateError(
                    "Unsupported disjunct type, most likely unnormalized.".to_string(),
                ))
            },

            (_, PDFType::Any, _) => result = check_predicate(&o, c.pred()),

            (PDFObjT::Boolean(_), PDFType::PrimType(PDFPrimType::Bool), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::String(_), PDFType::PrimType(PDFPrimType::String), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::Name(_), PDFType::PrimType(PDFPrimType::Name), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::Null(_), PDFType::PrimType(PDFPrimType::Null), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::Integer(_), PDFType::PrimType(PDFPrimType::Integer), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::Real(_), PDFType::PrimType(PDFPrimType::Real), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::Comment(_), PDFType::PrimType(PDFPrimType::Comment), _) => {
                result = check_predicate(&o, c.pred())
            },
            (PDFObjT::Array(ao), PDFType::Array { elem, size }, _) => {
                match size {
                    Some(sz) => {
                        if ao.objs().len() != *sz {
                            result = Some(TypeCheckError::ArraySizeMismatch(*sz, ao.objs().len()));
                            continue
                        }
                    },
                    None => (),
                };
                /* optimize PDFType::Any */
                let elem_rep = match resolve(tctx, Rc::clone(elem)) {
                    Ok(rep) => rep,
                    Err(err) => return Some(err),
                };
                if let PDFType::Any = elem_rep.typ() {
                    result = check_predicate(&o, c.pred());
                    continue
                }
                /* non-Any case */
                let mut chks = Vec::new();
                for e in ao.objs() {
                    chks.push((Rc::clone(e), Rc::clone(elem)))
                }
                push_checks(&mut state, chks);
            },
            (PDFObjT::Dict(dict), PDFType::Dict(ents), _) => {
                let mut chks = Vec::new();
                for ent in ents {
                    let val = dict.get(&ent.key);
                    let chk = match resolve(tctx, Rc::clone(&ent.chk)) {
                        Ok(rep) => rep,
                        Err(err) => return Some(err),
                    };
                    match (val, ent.opt, chk.typ()) {
                        (None, DictKeySpec::Optional, _) => continue,
                        (None, DictKeySpec::Forbidden, _) => continue,
                        (None, DictKeySpec::Required, _) => {
                            let key = DictKey::new(ent.key.clone());
                            result = Some(TypeCheckError::MissingKey(key))
                        },
                        (Some(_), DictKeySpec::Forbidden, _) => {
                            let key = DictKey::new(ent.key.clone());
                            result = Some(TypeCheckError::ForbiddenKey(key))
                        },
                        (Some(_), _, PDFType::Any) => continue,
                        (Some(v), _, _) => chks.push((Rc::clone(v), Rc::clone(&ent.chk))),
                    }
                }
                if result.is_none() {
                    push_checks(&mut state, chks)
                }
            },
            (PDFObjT::Stream(s), PDFType::Stream(ents), _) => {
                // Same code as above for now, copied in case we need to customize later.
                let mut chks = Vec::new();
                for ent in ents {
                    let val = s.dict().val().get(&ent.key);
                    let chk = match resolve(tctx, Rc::clone(&ent.chk)) {
                        Ok(rep) => rep,
                        Err(err) => return Some(err),
                    };
                    match (val, ent.opt, chk.typ()) {
                        (None, DictKeySpec::Optional, _) => continue,
                        (None, DictKeySpec::Forbidden, _) => continue,
                        (None, DictKeySpec::Required, _) => {
                            let key = DictKey::new(ent.key.clone());
                            result = Some(TypeCheckError::MissingKey(key));
                        },
                        (Some(_), DictKeySpec::Forbidden, _) => {
                            let key = DictKey::new(ent.key.clone());
                            result = Some(TypeCheckError::ForbiddenKey(key))
                        },
                        (Some(_), _, PDFType::Any) => continue,
                        (Some(v), _, _) => chks.push((Rc::clone(v), Rc::clone(&ent.chk))),
                    }
                }
                if result.is_none() {
                    push_checks(&mut state, chks)
                }
            },
            (obj, _, _) => {
                result = Some(TypeCheckError::TypeMismatch(
                    Rc::clone(&c.typ),
                    type_of(obj),
                ))
            },
        }
    }
}

// One common predicate is to allow a value in a set.
pub struct ChoicePred(pub String, pub Vec<PDFObjT>);

impl Predicate for ChoicePred {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        let vec = &self.1;
        if vec.iter().any(|c| obj.val() == c) {
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
    use super::{
        check_type, normalize_check, ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType,
        PDFType, Predicate, TypeCheck, TypeCheckContext, TypeCheckError,
    };
    use crate::pcore::parsebuffer::{LocatedVal, ParseBuffer};
    use crate::pdf_lib::pdf_obj::{parse_pdf_obj, DictKey, IndirectT, PDFObjContext, PDFObjT};
    use crate::pdf_lib::pdf_prim::{IntegerT, NameT};
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    fn mk_rectangle_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let elem = TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
        TypeCheck::new(
            tctx,
            "rectangle",
            Rc::new(PDFType::Array {
                elem,
                size: Some(4),
            }),
        )
    }

    pub fn mk_date_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new(
            tctx,
            "date",
            Rc::new(PDFType::PrimType(PDFPrimType::String)),
        )
    }

    #[test]
    fn test_rectangle() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("[1 2 3 4]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let typ = mk_rectangle_typchk(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_reference() {
        let mut ctxt = mk_new_context();
        let int = PDFObjT::Integer(IntegerT::new(10));
        let int = LocatedVal::new(int, 0, 1);
        let obj = IndirectT::new(2, 0, Rc::new(int)); // indirect ref: 2 0 R
        let obj = LocatedVal::new(obj, 0, 1);
        ctxt.register_obj(&obj);

        // parse the object directly
        let v = Vec::from("10".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let typ = TypeCheck::new(
            &mut tctx,
            "integer",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);

        // parse a reference pointing to that object
        let v = Vec::from("2 0 R".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = TypeCheck::new(
            &mut tctx,
            "integer",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);

        // require a referenced object
        let v = Vec::from("2 0 R".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = TypeCheck::new_all(
            &mut tctx,
            "integer-required",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
            None,
            IndirectSpec::Required,
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);

        // check forbidden error
        let v = Vec::from("2 0 R".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let obj = Rc::new(obj);
        let typ = TypeCheck::new_all(
            &mut tctx,
            "integer-forbidden",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
            None,
            IndirectSpec::Forbidden,
        );
        let err = TypeCheckError::ValueMismatch(
            Rc::clone(&obj),
            String::from("An indirect reference was forbidden"),
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), typ), Some(err));

        // check required error.
        let v = Vec::from("10".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let obj = Rc::new(obj);
        let typ = TypeCheck::new_all(
            &mut tctx,
            "integer-required",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
            None,
            IndirectSpec::Required,
        );
        let err = TypeCheckError::ValueMismatch(
            Rc::clone(&obj),
            String::from("An indirect reference was required"),
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), typ), Some(err));
    }

    #[test]
    fn test_dict() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let rect = mk_rectangle_typchk(&mut tctx);
        let ent1 = DictEntry {
            key: Vec::from("Entry"),
            chk: Rc::clone(&rect),
            opt: DictKeySpec::Required,
        };
        let ent2 = DictEntry {
            key: Vec::from("Dummy1"),
            chk: Rc::clone(&rect),
            opt: DictKeySpec::Forbidden,
        };
        let ent3 = DictEntry {
            key: Vec::from("Dummy2"),
            chk: Rc::clone(&rect),
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(
            &mut tctx,
            "dict",
            Rc::new(PDFType::Dict(vec![ent1, ent2, ent3])),
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_dict_required() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let ent = DictEntry {
            key: Vec::from("Dummy"),
            chk: mk_rectangle_typchk(&mut tctx),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), typ),
            Some(TypeCheckError::MissingKey(DictKey::new(Vec::from("Dummy"))))
        );
    }

    #[test]
    fn test_dict_forbidden() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /Entry [ 1 1 4 5 ] >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let ent = DictEntry {
            key: Vec::from("Entry"),
            chk: mk_rectangle_typchk(&mut tctx),
            opt: DictKeySpec::Forbidden,
        };
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), typ),
            Some(TypeCheckError::ForbiddenKey(DictKey::new(Vec::from(
                "Entry"
            ))))
        );
    }

    fn mk_pagemode_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let pred = ChoicePred(
            String::from("Invalid PageMode"),
            vec![
                PDFObjT::Name(NameT::new(Vec::from("UseNone"))),
                PDFObjT::Name(NameT::new(Vec::from("UseOutlines"))),
            ],
        );
        TypeCheck::new_refined(
            tctx,
            "pagemode",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(pred),
        )
    }

    #[test]
    fn test_dict_allowed_value() {
        // valid value for required key
        let mut ctxt = mk_new_context();
        let v = Vec::from("<< /PageMode /UseNone >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let pagemode = mk_pagemode_typchk(&mut tctx);
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: Rc::clone(&pagemode),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);

        // valid value for optional key
        let v = Vec::from("<< /PageMode /UseNone >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: Rc::clone(&pagemode),
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);

        // optional key absent
        let v = Vec::from("<< >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: Rc::clone(&pagemode),
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);

        // forbidden key present
        let v = Vec::from("<< /PageMode /UseNone >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: Rc::clone(&pagemode),
            opt: DictKeySpec::Forbidden,
        };
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), typ),
            Some(TypeCheckError::ForbiddenKey(DictKey::new(Vec::from(
                "PageMode"
            ))))
        );

        // invalid value for optional key
        let v = Vec::from("<< /PageMode /Dummy >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let ent = DictEntry {
            key: Vec::from("PageMode"),
            chk: Rc::clone(&pagemode),
            opt: DictKeySpec::Optional,
        };
        let val = Rc::new(LocatedVal::new(
            PDFObjT::Name(NameT::new(Vec::from("Dummy"))),
            0,
            0,
        ));
        let typ = TypeCheck::new(&mut tctx, "dict", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), typ),
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
                None
            } else {
                return Some(TypeCheckError::PredicateError(
                    "Not an ASCII string.".to_string(),
                ))
            }
        }
    }

    fn mk_ascii_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new_refined(
            tctx,
            "ascii",
            Rc::new(PDFType::PrimType(PDFPrimType::String)),
            Rc::new(AsciiStringPredicate),
        )
    }

    #[test]
    fn test_ascii_string() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("(ascii)".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let mut tctx = TypeCheckContext::new();
        let chk = mk_ascii_typchk(&mut tctx);
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), Rc::clone(&chk)),
            None
        );

        //                     (                )
        let v: Vec<u8> = vec![40, 129, 255, 0, 41];
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let err = TypeCheckError::PredicateError("Not an ASCII string.".to_string());
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), Rc::clone(&chk)),
            Some(err)
        );
    }

    #[test]
    fn test_ascii_pred() {
        let pred = AsciiStringPredicate;

        let v = Vec::from("(ascii)".as_bytes());
        let obj = LocatedVal::new(PDFObjT::String(v), 0, 0);
        assert_eq!(pred.check(&Rc::new(obj)), None);

        let obj = LocatedVal::new(PDFObjT::Null(()), 0, 0);
        let err = TypeCheckError::PredicateError("Not an ASCII string.".to_string());
        assert_eq!(pred.check(&Rc::new(obj)), Some(err));
    }

    #[test]
    fn test_any() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("(ascii)".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let chk = TypeCheck::new_refined(
            &mut tctx,
            "ascii",
            Rc::new(PDFType::Any),
            Rc::new(AsciiStringPredicate),
        );
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), Rc::clone(&chk)),
            None
        );

        let v = Vec::from("10".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let err = TypeCheckError::PredicateError("Not an ASCII string.".to_string());
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), Rc::clone(&chk)),
            Some(err)
        );
    }

    struct OrTestPredicate;
    impl Predicate for OrTestPredicate {
        fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
            if let PDFObjT::String(ref s) = obj.val() {
                for c in s {
                    if *c >= 128 {
                        return Some(TypeCheckError::PredicateError(
                            "Not an ASCII string.".to_string(),
                        ))
                    }
                }
                return None
            }
            if let PDFObjT::Integer(_) = obj.val() {
                None
            } else {
                Some(TypeCheckError::PredicateError(
                    "Not an ASCII string or an integer.".to_string(),
                ))
            }
        }
    }
    #[test]
    fn test_or_type() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("(ascii)".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let mut tctx = TypeCheckContext::new();
        let chk = TypeCheck::new_refined(
            &mut tctx,
            "or",
            Rc::new(PDFType::Any),
            Rc::new(OrTestPredicate),
        );
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), Rc::clone(&chk)),
            None
        );

        let v = Vec::from("10".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::new(obj), Rc::clone(&chk)),
            None
        );
    }

    #[test]
    fn test_normalize() {
        let mut tctx = TypeCheckContext::new();
        let t = TypeCheck::new(&mut tctx, "any", Rc::new(PDFType::Any));
        let opts = vec![Rc::clone(&t), Rc::clone(&t)];
        let d1 = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        let opts = vec![Rc::clone(&t), Rc::clone(&t)];
        let d2 = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));

        let opts = vec![Rc::clone(&d1), Rc::clone(&d2)];
        let nd1 = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        let opts = vec![Rc::clone(&d1), Rc::clone(&d2)];
        let nd2 = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));

        let opts = vec![Rc::clone(&nd1), Rc::clone(&nd2)];
        let d = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));

        // check that normalization flattens the nested disjuncts into
        // a flat set.
        if let TypeCheck::Rep(d) = d.as_ref() {
            let nd = normalize_check(d);
            if let PDFType::Disjunct(opts) = nd.typ() {
                assert_eq!(opts.len(), 8);
                for o in opts {
                    if let TypeCheck::Rep(r) = o.as_ref() {
                        assert_eq!(*r.typ(), PDFType::Any)
                    } else {
                        unreachable!()
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_disjunct() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("[1 2 3 4]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = Rc::new(parse_pdf_obj(&mut ctxt, &mut pb).unwrap());

        let mut tctx = TypeCheckContext::new();
        let rect = mk_rectangle_typchk(&mut tctx);
        let int = TypeCheck::new(
            &mut tctx,
            "int",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        );
        let date = mk_date_typchk(&mut tctx);

        let opts = vec![Rc::clone(&rect), Rc::clone(&int), Rc::clone(&date)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), chk), None);

        let opts = vec![Rc::clone(&int), Rc::clone(&rect), Rc::clone(&date)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), chk), None);

        let opts = vec![Rc::clone(&date), Rc::clone(&int), Rc::clone(&rect)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), chk), None);

        let v = Vec::from("<</Key [1 2 3 4]>>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = Rc::new(parse_pdf_obj(&mut ctxt, &mut pb).unwrap());

        let opts = vec![Rc::clone(&rect), Rc::clone(&int), Rc::clone(&date)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        let ent = DictEntry {
            key: Vec::from("Key"),
            chk: Rc::clone(&chk),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Dict(vec![ent])));
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), typ), None);
    }

    #[test]
    fn test_disjunct_fail() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("[1 2 3 4]".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = Rc::new(parse_pdf_obj(&mut ctxt, &mut pb).unwrap());

        let mut tctx = TypeCheckContext::new();
        let int = TypeCheck::new(
            &mut tctx,
            "int",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        );
        let date = mk_date_typchk(&mut tctx);

        let opts = vec![Rc::clone(&date), Rc::clone(&int)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        // should return the error for the last disjunct, i.e. int
        let err = TypeCheckError::TypeMismatch(
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
            PDFType::Array {
                elem: TypeCheck::new(&mut tctx, "", Rc::new(PDFType::Any)),
                size: None,
            },
        );
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), chk), Some(err));
    }

    #[test]
    fn test_unwind() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<</Key [1 2 3 4]>>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = Rc::new(parse_pdf_obj(&mut ctxt, &mut pb).unwrap());

        let mut tctx = TypeCheckContext::new();
        let int = TypeCheck::new(
            &mut tctx,
            "int",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        );
        let date = mk_date_typchk(&mut tctx);
        let rect = mk_rectangle_typchk(&mut tctx);

        // inner unwind of value match to failure
        let opts = vec![Rc::clone(&int), Rc::clone(&date)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        let ent = DictEntry {
            key: Vec::from("Key"),
            chk: Rc::clone(&chk),
            opt: DictKeySpec::Required,
        };
        let typ1 = TypeCheck::new(&mut tctx, "typ1", Rc::new(PDFType::Dict(vec![ent])));

        // unwind up a level
        let ent = DictEntry {
            key: Vec::from("Key"),
            chk: Rc::clone(&date),
            opt: DictKeySpec::Required,
        };
        let typ2 = TypeCheck::new(&mut tctx, "typ2", Rc::new(PDFType::Dict(vec![ent])));

        // inner unwind of value match to success
        let opts = vec![Rc::clone(&int), Rc::clone(&date), Rc::clone(&rect)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        let ent = DictEntry {
            key: Vec::from("Key"),
            chk: Rc::clone(&chk),
            opt: DictKeySpec::Required,
        };
        let typ3 = TypeCheck::new(&mut tctx, "typ3", Rc::new(PDFType::Dict(vec![ent])));

        // bound the unwinds to within a single successful top-level match
        let opts = vec![typ1, typ2, typ3];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        assert_eq!(check_type(&ctxt, &tctx, Rc::clone(&obj), chk), None);
    }

    #[test]
    fn test_recursive() {
        // A type (rect | dict) that refers to itself for recursion:
        // it is a dict, where the value of /Key could either be a
        // rectangle, or another (rect | dict).
        let mut tctx = TypeCheckContext::new();
        let rect = mk_rectangle_typchk(&mut tctx);
        // refer by name to the type that will be created later
        let named = TypeCheck::new_named("rect | dict");
        let opts = vec![Rc::clone(&rect), Rc::clone(&named)];
        let chk = TypeCheck::new(&mut tctx, "opt", Rc::new(PDFType::Disjunct(opts)));
        let ent = DictEntry {
            key: Vec::from("Key"),
            chk: Rc::clone(&chk),
            opt: DictKeySpec::Required,
        };
        // actually create the concrete type with the specified name
        let typ = TypeCheck::new(&mut tctx, "rect | dict", Rc::new(PDFType::Dict(vec![ent])));

        let mut ctxt = mk_new_context();
        // non-recursive case: the value is a rectangle
        let v = Vec::from("<</Key [1 2 3 4]>>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = Rc::new(parse_pdf_obj(&mut ctxt, &mut pb).unwrap());
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::clone(&obj), Rc::clone(&typ)),
            None
        );

        // recursive case: the value is another (rect | dict)
        let v = Vec::from("<</Key <</Key [1 2 3 4]>>>>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = Rc::new(parse_pdf_obj(&mut ctxt, &mut pb).unwrap());
        assert_eq!(
            check_type(&ctxt, &tctx, Rc::clone(&obj), Rc::clone(&typ)),
            None
        );
    }
}
