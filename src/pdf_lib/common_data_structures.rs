pub mod structures {
    use super::super::super::pcore::parsebuffer::LocatedVal;
    use super::super::super::pcore::parsebuffer::ParseBuffer;
    use super::super::name_tree::name_tree;
    use super::super::pdf_obj::{parse_pdf_obj, PDFObjContext, PDFObjT};
    use super::super::pdf_type_check::{
        check_type, ChoicePred, DictEntry, DictKeySpec, PDFPrimType, PDFType, Predicate, TypeCheck,
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

    fn name_dictionary() -> TypeCheck {
        let Dests = DictEntry {
            key: Vec::from("Dests"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let AP = DictEntry {
            key: Vec::from("AP"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let JavaScript = DictEntry {
            key: Vec::from("JavaScript"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let Pages = DictEntry {
            key: Vec::from("Pages"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let Templates = DictEntry {
            key: Vec::from("Templates"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let IDS = DictEntry {
            key: Vec::from("IDS"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let URLS = DictEntry {
            key: Vec::from("URLS"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let EmbeddedFiles = DictEntry {
            key: Vec::from("EmbeddedFiles"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let AlternatePresentations = DictEntry {
            key: Vec::from("AlternatePresentations"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let Renditions = DictEntry {
            key: Vec::from("Renditions"),
            chk: name_tree(), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![
            Dests,
            AP,
            JavaScript,
            Pages,
            Templates,
            IDS,
            URLS,
            EmbeddedFiles,
            Renditions,
        ])));
        typ
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
}
