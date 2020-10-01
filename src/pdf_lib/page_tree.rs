use super::super::pcore::parsebuffer::{LocatedVal};
use super::pdf_obj::{PDFObjContext, PDFObjT};
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckError, Predicate, ChoicePred
};
use std::rc::Rc;

fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

fn mk_pages_check() -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Pages not present."),
        vec![
        PDFObjT::Name(NameT::new(Vec::from("Pages"))),
        ],
        );
    Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Box::new(pred),
            ))
}

fn mk_count_typchk() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
                    PDFPrimType::Integer,
                    ))))
}

struct ReferencePredicate;

impl Predicate for ReferencePredicate {

    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        if let PDFObjT::Array(ref s) = obj.val() {
            for c in s.objs() {
                if let PDFObjT::Reference(ref s2) = c.val() {
                }
                else {
                    return Some(TypeCheckError::PredicateError(
                            "Reference expected".to_string(),
                            ))
                }
            }
            None
        }
        else {
            return Some(TypeCheckError::PredicateError(
                    "Reference wasn't an Array".to_string(),
                    ))
        }
    }
}

fn mk_indirect_typchk() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::Array {
                elem: Rc::new(TypeCheck::new(Rc::new(PDFType::Any,
                                                    ))),
                                                    size: None,
            }),
            Box::new(ReferencePredicate),
            ))
}

#[cfg(test)]

mod test_page_tree {
    use super::super::super::pcore::parsebuffer::{ParseBuffer};
    use super::super::pdf_obj::{PDFObjContext, PDFObjT, parse_pdf_obj};
    use crate::pdf_lib::pdf_prim::NameT;
    use super::super::pdf_type_check::{
        check_type, DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckError, ChoicePred
    };
    use std::rc::Rc;
    use super::{mk_new_context, mk_pages_check, mk_count_typchk, mk_indirect_typchk};


    // Page Tree Non-Root Node Tests
    #[test]
    fn test_non_root_page_tree() {
        let mut ctxt = mk_new_context();
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
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
            chk: mk_indirect_typchk(), 
            opt: DictKeySpec::Required,
        };
        let parent = DictEntry {
            key: Vec::from("Parent"),
            chk: mk_indirect_typchk(),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![pages, count, kids, parent])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            Some(TypeCheckError::MissingKey([80, 97, 114, 101, 110, 116].to_vec()))
            );
    }
    #[test]
    fn test_non_root_page_tree_not_wrong() {
        let mut ctxt = mk_new_context();
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        let v = Vec::from("<</Type /Pages /Parent [4 0 R] /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
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
            chk: mk_indirect_typchk(), 
            opt: DictKeySpec::Required,
        };
        let parent = DictEntry {
            key: Vec::from("Parent"),
            chk: mk_indirect_typchk(),
            opt: DictKeySpec::Required,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![pages, count, kids, parent])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            None
            );
    }
    // Page Tree Non Root Node Tests End

    // Page Tree Root Node Tests
    #[test]
    fn test_page_tree() {
        let mut ctxt = mk_new_context();
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let pages = DictEntry {
            key: Vec::from("Type"),
            chk: mk_pages_check(), // this must be a NameT
            opt: DictKeySpec::Required,
        };
        let _kids = DictEntry {
            key: Vec::from("Kids"),
            chk: mk_indirect_typchk(), 
            opt: DictKeySpec::Required,
        };
        let count = DictEntry {
            key: Vec::from("Count"),
            chk: mk_count_typchk(),
            opt: DictKeySpec::Required,
        };
        //let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![pages, kids, count])));
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![pages, count])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            Some(TypeCheckError::MissingKey([84, 121, 112, 101].to_vec()))
            );
    }
    #[test]
    fn test_page_tree_not_wrong() {
        let mut ctxt = mk_new_context();
        let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
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
            chk: mk_indirect_typchk(), 
            opt: DictKeySpec::Required,
        };
        let _parent = DictEntry {
            key: Vec::from("Parent"),
            chk: mk_pages_check(), // this must be a NameT
            opt: DictKeySpec::Forbidden,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![pages, count, kids])));
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), Rc::new(typ)),
            None
            );
    }
    // Page Tree Root Node Tests End
}
