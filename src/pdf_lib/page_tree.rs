use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::PDFObjT;
use crate::pdf_lib::pdf_type_check::{PDFType, Predicate, TypeCheck, TypeCheckError};
use std::rc::Rc;

struct PageTreePredicate;
impl Predicate for PageTreePredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        if let PDFObjT::Dict(ref s) = obj.val() {
            let mappings = s.map();
            match mappings.get(&Vec::from("Count")) {
                Some(a) => {
                    if let PDFObjT::Integer(ref _s) = a.val() {}
                    return Some(TypeCheckError::PredicateError(
                        "Integer expected".to_string(),
                    ))
                },
                None => {},
            }
            match mappings.get(&Vec::from("Kids")) {
                Some(a) => {
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
                },
                None => {},
            }
            match mappings.get(&Vec::from("Parent")) {
                Some(a) => {
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
                },
                None => {},
            }
            if mappings.contains_key(&Vec::from("Parent"))
                && mappings.contains_key(&Vec::from("Kids"))
                && mappings.contains_key(&Vec::from("Count"))
                && mappings.contains_key(&Vec::from("Parent"))
            {
            } else if mappings.contains_key(&Vec::from("Parent"))
                && mappings.contains_key(&Vec::from("Kids"))
                && mappings.contains_key(&Vec::from("Count"))
                && mappings.contains_key(&Vec::from("Parent"))
            {
            } else if mappings.contains_key(&Vec::from("Parent"))
                && mappings.contains_key(&Vec::from("Kids"))
                && mappings.contains_key(&Vec::from("Count"))
                && mappings.contains_key(&Vec::from("Parent"))
            {
            } else if mappings.contains_key(&Vec::from("Parent"))
                && mappings.contains_key(&Vec::from("Kids"))
                && mappings.contains_key(&Vec::from("Count"))
                && mappings.contains_key(&Vec::from("Parent"))
            {
            } else {
                return Some(TypeCheckError::PredicateError(
                    "Missing field or Forbidden field".to_string(),
                ))
            }
            None
        } else {
            // Not a dictionary
            return Some(TypeCheckError::PredicateError(
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
            return Some(TypeCheckError::PredicateError(
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
