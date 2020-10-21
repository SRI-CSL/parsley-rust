pub mod structures {
    use super::super::super::pcore::parsebuffer::LocatedVal;
    use super::super::name_tree::name_tree;
    use super::super::pdf_obj::PDFObjT;
    use super::super::pdf_type_check::{
        ChoicePred, DictEntry, DictKeySpec, PDFPrimType, PDFType, Predicate, TypeCheck,
        TypeCheckError,
    };
    use crate::pdf_lib::pdf_prim::NameT;
    use std::rc::Rc;
    pub fn mk_name_check(name: String) -> Rc<TypeCheck> {
        let pred = ChoicePred(
            String::from("Catalog not present."),
            vec![PDFObjT::Name(NameT::new(Vec::from(name)))],
        );
        Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(pred),
        ))
    }
    pub fn mk_rectangle_typchk() -> Rc<TypeCheck> {
        Rc::new(TypeCheck::new(Rc::new(PDFType::Array {
            elem: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
                PDFPrimType::Integer,
            )))),
            size: Some(4),
        })))
    }

    pub fn name_dictionary() -> Rc<TypeCheck> {
        let dests = DictEntry {
            key: Vec::from("Dests"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let ap = DictEntry {
            key: Vec::from("AP"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let javascript = DictEntry {
            key: Vec::from("JavaScript"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let pages = DictEntry {
            key: Vec::from("Pages"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let templates = DictEntry {
            key: Vec::from("Templates"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let ids = DictEntry {
            key: Vec::from("IDS"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let urls = DictEntry {
            key: Vec::from("URLS"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let embedded_files = DictEntry {
            key: Vec::from("EmbeddedFiles"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let alternate_presentations = DictEntry {
            key: Vec::from("AlternatePresentations"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let renditions = DictEntry {
            key: Vec::from("Renditions"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let typ = Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![
            dests,
            ap,
            javascript,
            pages,
            templates,
            ids,
            urls,
            alternate_presentations,
            embedded_files,
            renditions,
        ]))));
        typ
    }
    struct SingleReferencePredicate;

    impl Predicate for SingleReferencePredicate {
        fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
            if let PDFObjT::Reference(ref _s2) = obj.val() {
            } else {
                return Some(TypeCheckError::PredicateError(
                    "Reference expected".to_string(),
                ))
            }
            None
        }
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
    pub fn mk_reference_typchk() -> Rc<TypeCheck> {
        Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::Array {
                elem: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
                size: None,
            }),
            Rc::new(ReferencePredicate),
        ))
    }
    pub fn mk_single_reference_typchk() -> Rc<TypeCheck> {
        Rc::new(TypeCheck::new_refined(
            Rc::new(PDFType::Any),
            Rc::new(ReferencePredicate),
        ))
    }
}
