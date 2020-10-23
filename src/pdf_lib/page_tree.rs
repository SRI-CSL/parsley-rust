use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::PDFObjT;
use crate::pdf_lib::page::page_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckError,
};
use std::rc::Rc;

fn mk_pages_check() -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Pages not present."),
        vec![PDFObjT::Name(NameT::new(Vec::from("Pages")))],
    );
    Rc::new(TypeCheck::new_refined(
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
        Rc::new(pred),
    ))
}

fn mk_count_typchk() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
        PDFPrimType::Integer,
    ))))
}

fn mk_indirect_typchk() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new_refined(
        Rc::new(PDFType::Array {
            elem: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
            size: None,
        }),
        Rc::new(ReferencePredicate),
    ))
}

pub fn root_page_tree() -> Rc<TypeCheck> {
    let pages = DictEntry {
        key: Vec::from("Type"),
        chk: mk_pages_check(), // this must be a NameT
        opt: DictKeySpec::Required,
    };
    let count = DictEntry {
        key: Vec::from("Count"),
        chk: mk_count_typchk(),
        opt: DictKeySpec::Required,
    };
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: Rc::new(TypeCheck::new_all(
            Rc::new(PDFType::Disjunct(vec![page_type(), non_root_page_tree()])),
            None,
            IndirectSpec::Required,
        )),
        opt: DictKeySpec::Required,
    };
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: mk_indirect_typchk(),
        opt: DictKeySpec::Forbidden,
    };
    return Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![
        pages, count, kids, parent,
    ]))))
}

pub fn non_root_page_tree() -> Rc<TypeCheck> {
    let pages = DictEntry {
        key: Vec::from("Type"),
        chk: mk_pages_check(), // this must be a NameT
        opt: DictKeySpec::Required,
    };
    let count = DictEntry {
        key: Vec::from("Count"),
        chk: mk_count_typchk(),
        opt: DictKeySpec::Required,
    };
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: Rc::new(TypeCheck::new_all(
            Rc::new(PDFType::Disjunct(vec![page_type(), non_root_page_tree()])),
            None,
            IndirectSpec::Required,
        )),
        opt: DictKeySpec::Required,
    };
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Disjunct(vec![
            non_root_page_tree(),
            root_page_tree(),
        ])))),
        opt: DictKeySpec::Required,
    };
    return Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![
        pages, count, kids, parent,
    ]))))
}

struct PageTreePredicate;
impl Predicate for PageTreePredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        if let PDFObjT::Dict(ref s) = obj.val() {
            let mappings = s.map();
            if let Some(a) = mappings.get(&Vec::from("Count")) {
                if let PDFObjT::Integer(ref _s) = a.val() {}
                return Some(TypeCheckError::PredicateError(
                    "Integer expected".to_string(),
                ))
            }
            if let Some(a) = mappings.get(&Vec::from("Kids")) {
                if let PDFObjT::Array(ref s) = a.val() {
                    for c in s.objs() {
                        if let PDFObjT::Reference(ref _s2) = c.val() {
                        } else {
                            return Some(TypeCheckError::PredicateError(
                                "Reference expected".to_string(),
                            ))
                        }
                    }
                } else {
                    return Some(TypeCheckError::PredicateError(
                        "Reference wasn't an Array".to_string(),
                    ))
                }
            }
            if let Some(a) = mappings.get(&Vec::from("Parent")) {
                if let PDFObjT::Array(ref s) = a.val() {
                    for c in s.objs() {
                        if let PDFObjT::Reference(ref _s2) = c.val() {
                        } else {
                            return Some(TypeCheckError::PredicateError(
                                "Reference expected".to_string(),
                            ))
                        }
                    }
                } else {
                    return Some(TypeCheckError::PredicateError(
                        "Reference wasn't an Array".to_string(),
                    ))
                }
            }
            if (mappings.contains_key(&Vec::from("Parent"))
                && mappings.contains_key(&Vec::from("Kids"))
                && mappings.contains_key(&Vec::from("Count"))
                && mappings.contains_key(&Vec::from("Parent")))
                || (mappings.contains_key(&Vec::from("Parent"))
                    && mappings.contains_key(&Vec::from("Kids"))
                    && mappings.contains_key(&Vec::from("Count"))
                    && mappings.contains_key(&Vec::from("Parent")))
                || (mappings.contains_key(&Vec::from("Parent"))
                    && mappings.contains_key(&Vec::from("Kids"))
                    && mappings.contains_key(&Vec::from("Count"))
                    && mappings.contains_key(&Vec::from("Parent")))
                || (mappings.contains_key(&Vec::from("Parent"))
                    && mappings.contains_key(&Vec::from("Kids"))
                    && mappings.contains_key(&Vec::from("Count"))
                    && mappings.contains_key(&Vec::from("Parent")))
            {
            } else {
                return Some(TypeCheckError::PredicateError(
                    "Missing field or Forbidden field".to_string(),
                ))
            }
            None
        } else {
            // Not a dictionary
            Some(TypeCheckError::PredicateError(
                "No Dictionary, no Page Tree".to_string(),
            ))
        }
    }
}

pub fn page_tree() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new_refined(
        Rc::new(PDFType::Any),
        Rc::new(PageTreePredicate),
    ))
}

struct ReferencePredicate;

impl Predicate for ReferencePredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        if let PDFObjT::Array(ref s) = obj.val() {
            for c in s.objs() {
                if let PDFObjT::Reference(ref _s2) = c.val() {
                } else {
                    return Some(TypeCheckError::PredicateError(
                        "Reference expected".to_string(),
                    ))
                }
            }
            None
        } else {
            Some(TypeCheckError::PredicateError(
                "Reference wasn't an Array".to_string(),
            ))
        }
    }
}

#[cfg(test)]

mod test_page_tree {
    use super::super::super::pcore::parsebuffer::ParseBuffer;
    use super::super::pdf_obj::{parse_pdf_obj, PDFObjContext};
    use super::super::pdf_type_check::{check_type, TypeCheckError};
    use super::page_tree;
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    // Page Tree Non-Root Node Tests
    #[test]
    fn test_non_root_page_tree() {
        let mut ctxt = mk_new_context();
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3
        // >>".as_bytes());
        let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = page_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Integer expected".to_string()
            ))
        );
    }
    #[test]
    fn test_non_root_page_tree_not_wrong() {
        let mut ctxt = mk_new_context();
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3
        // >>".as_bytes());
        let v = Vec::from(
            "<</Type /Pages /Parent [4 0 R] /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes(),
        );
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = page_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Integer expected".to_string()
            ))
        );
    }
    // Page Tree Non Root Node Tests End

    #[test]
    fn test_page_tree_not_wrong() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = page_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Integer expected".to_string()
            ))
        );
    }
    // Page Tree Root Node Tests End
}
