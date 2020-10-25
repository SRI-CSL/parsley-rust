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

    pub fn resources(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        let extgstate = DictEntry {
            key: Vec::from("ExtGState"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let colorspace = DictEntry {
            key: Vec::from("ColorSpace"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let pattern = DictEntry {
            key: Vec::from("Pattern"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let shading = DictEntry {
            key: Vec::from("Shading"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let xobject = DictEntry {
            key: Vec::from("XObject"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let font = DictEntry {
            key: Vec::from("Font"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let procset = DictEntry {
            key: Vec::from("ProcSet"),
            chk: mk_generic_array_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        let properties = DictEntry {
            key: Vec::from("Properties"),
            chk: mk_generic_dict_typchk(tctx),
            opt: DictKeySpec::Optional,
        };
        TypeCheck::new(
            tctx,
            "page",
            Rc::new(PDFType::Dict(vec![
                extgstate, colorspace, pattern, shading, xobject, font, procset, properties,
            ])),
        )
    }

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

    pub fn mk_date_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
        TypeCheck::new_refined(
            tctx,
            "date",
            Rc::new(PDFType::PrimType(PDFPrimType::String)),
            Rc::new(DateStringPredicate),
        )
    }
    struct DateStringPredicate;
    impl Predicate for DateStringPredicate {
        fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
            /*
             * PDF spec 7.9.4 defines the date format like:
             *  (D:YYYYMMDDHHmmSSOHH'mm)
             */
            if let PDFObjT::String(ref s) = obj.val() {
                // regex for Date
                let re = regex::Regex::new(r"^D:\d{4}(([0][1-9]|[1][0-2])(([0][1-9]|[1-2][0-9]|[3][0-1])(([0-1][0-9]|[2][0-3])(([0-5][0-9])(([0-5][0-9])([+\-Z](([0-1][0-9]'|[2][0-3]')([0-5][0-9])?)?)?)?)?)?)?)?$").unwrap();
                let date_string = std::str::from_utf8(s).unwrap_or("");
                if !re.is_match(date_string) {
                    return Some(TypeCheckError::PredicateError(
                        "Not a Date string.".to_string(),
                    ))
                }
                None
            } else {
                Some(TypeCheckError::PredicateError(
                    "Not an Date string.".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::pdf_type_check::{
        check_type, normalize_check, ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType,
        PDFType, Predicate, TypeCheck, TypeCheckContext, TypeCheckError,
    };
    use super::{mk_date_typchk};

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn test_date_string() {
        fn run_date_type_check(raw_pdf_date_string: &str) -> Option<TypeCheckError> {
            let mut ctxt = mk_new_context();
            let mut tctx = TypeCheckContext::new();
            let v = Vec::from(raw_pdf_date_string.as_bytes());
            let mut pb = ParseBuffer::new(v);
            let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
            let typ_chk = mk_date_typchk(&mut tctx);
            return check_type(&ctxt, &tctx, Rc::new(obj), typ_chk)
        }

        let correct_test_cases = [
            "(D:1992)",
            "(D:199212)",
            "(D:19921223)",
            "(D:1992122319)",
            "(D:199212231952)",
            "(D:19921223195200)",
            "(D:19921223195200-)",
            "(D:19921223195200-08')",
            "(D:19921223195200-08'00)",
        ];
        for d in correct_test_cases.iter() {
            assert_eq!(run_date_type_check(d), None);
        }

        let incorrect_test_cases = [
            "(D1992)",
            "(D:199213)",
            "(D:19921243)",
            "(D:1992122349)",
            "(D:199212231972)",
            "(D:19921223195280)",
            "(D:19921223195290-)",
            "(D:199212231952-)",
            "(D:19921223195200-58')",
            "(D:19921223195200-08)",
            "(D:19921223195200-08'0099)",
            "(D:19921223195200-08'60)",
        ];
        for d in incorrect_test_cases.iter() {
            assert!(run_date_type_check(d).is_some());
        }
    }
}
