pub mod structures {
    use super::super::super::pcore::parsebuffer::LocatedVal;
    use super::super::name_tree::name_tree;
    use super::super::pdf_obj::PDFObjT;
    use super::super::pdf_type_check::{
        ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate,
        TypeCheck, TypeCheckContext, TypeCheckError,
    };
    use crate::pdf_lib::pdf_prim::NameT;
    use std::rc::Rc;

    // A generic dictionary, typically used for out-of-scope
    // dictionary values.
    pub fn mk_generic_dict_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new(tctx, "", Rc::new(PDFType::Dict(vec![])))
    }

    // A generic dictionary that is required to be an indirect reference,
    // typically used for out-of-scope dictionary values.
    pub fn mk_generic_indirect_dict_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new_indirect(
            tctx,
            "",
            Rc::new(PDFType::Dict(vec![])),
            IndirectSpec::Required,
        )
    }

    // A generic array, typically used for out-of-scope
    // array values.
    pub fn mk_generic_array_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let elem = TypeCheck::new(tctx, "", Rc::new(PDFType::Any));
        TypeCheck::new(tctx, "", Rc::new(PDFType::Array { elem, size: None }))
    }

    // A generic array that is required to be an indirect reference,
    // typically used for out-of-scope array values.
    pub fn mk_generic_indirect_array_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new_indirect(
            tctx,
            "",
            Rc::new(PDFType::Dict(vec![])),
            IndirectSpec::Required,
        )
    }

    pub fn mk_name_check(msg: String, name: String, tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let pred = ChoicePred(msg, vec![PDFObjT::Name(NameT::new(Vec::from(name)))]);
        TypeCheck::new_refined(
            tctx,
            "",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(pred),
        )
    }
    pub fn mk_rectangle_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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

    pub fn name_dictionary(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let dests = DictEntry {
            key: Vec::from("Dests"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let ap = DictEntry {
            key: Vec::from("AP"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let javascript = DictEntry {
            key: Vec::from("JavaScript"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let pages = DictEntry {
            key: Vec::from("Pages"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let templates = DictEntry {
            key: Vec::from("Templates"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let ids = DictEntry {
            key: Vec::from("IDS"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let urls = DictEntry {
            key: Vec::from("URLS"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let embedded_files = DictEntry {
            key: Vec::from("EmbeddedFiles"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let alternate_presentations = DictEntry {
            key: Vec::from("AlternatePresentations"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        let renditions = DictEntry {
            key: Vec::from("Renditions"),
            chk: name_tree(tctx), // this must be a NameT
            opt: DictKeySpec::Optional,
        };
        TypeCheck::new(
            tctx,
            "namedictionary",
            Rc::new(PDFType::Dict(vec![
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
            ])),
        )
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
                Some(TypeCheckError::PredicateError(
                    "Reference wasn't an Array".to_string(),
                ))
            }
        }
    }
    pub fn mk_reference_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let elem = TypeCheck::new(tctx, "", Rc::new(PDFType::Any));
        TypeCheck::new_refined(
            tctx,
            "reference",
            Rc::new(PDFType::Array { elem, size: None }),
            Rc::new(ReferencePredicate),
        )
    }
    pub fn mk_single_reference_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new_refined(
            tctx,
            "single",
            Rc::new(PDFType::Any),
            Rc::new(ReferencePredicate),
        )
    }
}
