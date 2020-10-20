use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{PDFObjContext, PDFObjT};
use crate::pdf_lib::common_data_structures::structures::mk_reference_typchk;
use crate::pdf_lib::pdf_type_check::{
    DictEntry, DictKeySpec, PDFPrimType, PDFType, Predicate, TypeCheck, TypeCheckError,
};
use std::rc::Rc;

fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

fn mk_limits_typchk() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new(Rc::new(PDFType::Array {
        elem: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
            PDFPrimType::String,
        )))),
        size: Some(2),
    })))
}

struct NameTreePredicate;
impl Predicate for NameTreePredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        if let PDFObjT::Dict(ref s) = obj.val() {
            let mappings = s.map();
            match mappings.get(&Vec::from("Names")) {
                Some(a) => {
                    if let PDFObjT::Array(ref s) = a.val() {
                        if s.objs().len() % 2 == 0 {
                            for c in (0 .. s.objs().len()).step_by(2) {
                                if let PDFObjT::String(ref _s1) = s.objs()[c].val() {
                                    if let PDFObjT::Reference(ref _s2) = s.objs()[c + 1].val() {
                                    } else {
                                        return Some(TypeCheckError::PredicateError(
                                            "Reference not found in Name Tree".to_string(),
                                        ))
                                    }
                                } else {
                                    return Some(TypeCheckError::PredicateError(
                                        "String not found in name tree".to_string(),
                                    ))
                                }
                            }
                        } else {
                            return Some(TypeCheckError::PredicateError(
                                "Array found but not correct length in Name Tree".to_string(),
                            ))
                        }
                    } else {
                        return Some(TypeCheckError::PredicateError(
                            "Array not found in Name Tree".to_string(),
                        ))
                    }
                },
                None => {},
            }
            match mappings.get(&Vec::from("Limits")) {
                Some(a) => {
                    if let PDFObjT::Array(ref s) = a.val() {
                        println!("Limits {:?}", s);
                        for c in s.objs() {
                            if let PDFObjT::String(ref _s1) = c.val() {
                            } else {
                                return Some(TypeCheckError::PredicateError(
                                    "TypeMismatch: String expected".to_string(),
                                ))
                            }
                        }
                        if s.objs().len() != 2 {
                            return Some(TypeCheckError::PredicateError(
                                "Length Mismatch".to_string(),
                            ))
                        }
                    }
                },
                None => {},
            }
            match mappings.get(&Vec::from("Kids")) {
                Some(a) => {
                    if let PDFObjT::Array(ref s) = a.val() {
                        for c in s.objs() {
                            println!("{:?}", c);
                            if let PDFObjT::Reference(ref _s2) = c.val() {
                            } else {
                                return Some(TypeCheckError::PredicateError(
                                    "Reference expected".to_string(),
                                ))
                            }
                        }
                    } else {
                        println!("{:?}", a);
                        return Some(TypeCheckError::PredicateError(
                            "Reference wasn't an Array".to_string(),
                        ))
                    }
                },
                None => {},
            }

            if mappings.contains_key(&Vec::from("Names"))
                && mappings.contains_key(&Vec::from("Limits"))
                && !mappings.contains_key(&Vec::from("Kids"))
            {
                None
            } else if !mappings.contains_key(&Vec::from("Names"))
                && mappings.contains_key(&Vec::from("Limits"))
                && mappings.contains_key(&Vec::from("Kids"))
            {
                None
            } else if !mappings.contains_key(&Vec::from("Names"))
                && !mappings.contains_key(&Vec::from("Limits"))
                && mappings.contains_key(&Vec::from("Kids"))
            {
                None
            } else if mappings.contains_key(&Vec::from("Names"))
                && !mappings.contains_key(&Vec::from("Limits"))
                && !mappings.contains_key(&Vec::from("Kids"))
            {
                None
            } else {
                return Some(TypeCheckError::PredicateError(
                    "Missing field or Forbidden field".to_string(),
                ))
            }
        } else {
            return Some(TypeCheckError::PredicateError(
                "No Dictionary, no Name Tree".to_string(),
            ))
        }
    }
}
struct NamesPredicate;
impl Predicate for NamesPredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<TypeCheckError> {
        if let PDFObjT::Array(ref s) = obj.val() {
            if s.objs().len() % 2 == 0 {
                for c in (0 .. s.objs().len()).step_by(2) {
                    if let PDFObjT::String(ref _s1) = s.objs()[c].val() {
                        if let PDFObjT::Reference(ref _s2) = s.objs()[c + 1].val() {
                        } else {
                            return Some(TypeCheckError::PredicateError(
                                "Reference not found in Name Tree".to_string(),
                            ))
                        }
                    } else {
                        return Some(TypeCheckError::PredicateError(
                            "String not found in name tree".to_string(),
                        ))
                    }
                }
                None
            } else {
                return Some(TypeCheckError::PredicateError(
                    "Array found but not correct length in Name Tree".to_string(),
                ))
            }
        } else {
            return Some(TypeCheckError::PredicateError(
                "Array not found in Name Tree".to_string(),
            ))
        }
    }
}

fn mk_names_check() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new_refined(
        Rc::new(PDFType::Array {
            elem: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
            size: None,
        }),
        Rc::new(NamesPredicate),
    ))
}

// Leaves: Limits required, Names required, Kids forbidden

// Permutations possible for root
// Root with names
// Root with kids

pub fn name_tree() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new_refined(
        Rc::new(PDFType::Any),
        Rc::new(NameTreePredicate),
    ))
}

fn root_names_type() -> TypeCheck {
    let names = DictEntry {
        key: Vec::from("Names"),
        chk: mk_names_check(), // this must be a NameT
        opt: DictKeySpec::Required,
    };
    let limits = DictEntry {
        key: Vec::from("Limits"),
        chk: mk_limits_typchk(),
        opt: DictKeySpec::Forbidden,
    };
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: mk_reference_typchk(),
        opt: DictKeySpec::Forbidden,
    };
    let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![names, limits, kids])));
    typ
}
fn root_kids_type() -> TypeCheck {
    let names = DictEntry {
        key: Vec::from("Names"),
        chk: mk_names_check(), // this must be a NameT
        opt: DictKeySpec::Forbidden,
    };
    let limits = DictEntry {
        key: Vec::from("Limits"),
        chk: mk_limits_typchk(),
        opt: DictKeySpec::Forbidden,
    };
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: mk_reference_typchk(),
        opt: DictKeySpec::Required,
    };
    let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![names, limits, kids])));
    typ
}
// Intermediate: kids and limits--required, names forbidden

fn intermediate_type() -> TypeCheck {
    let names = DictEntry {
        key: Vec::from("Names"),
        chk: mk_names_check(), // this must be a NameT
        opt: DictKeySpec::Forbidden,
    };
    let limits = DictEntry {
        key: Vec::from("Limits"),
        chk: mk_limits_typchk(),
        opt: DictKeySpec::Required,
    };
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: mk_reference_typchk(),
        opt: DictKeySpec::Required,
    };
    let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![names, limits, kids])));
    typ
}
fn leaves_type() -> TypeCheck {
    let names = DictEntry {
        key: Vec::from("Names"),
        chk: mk_names_check(), // this must be a NameT
        opt: DictKeySpec::Required,
    };
    let limits = DictEntry {
        key: Vec::from("Limits"),
        chk: mk_limits_typchk(),
        opt: DictKeySpec::Required,
    };
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: mk_reference_typchk(),
        opt: DictKeySpec::Forbidden,
    };
    let typ = TypeCheck::new(Rc::new(PDFType::Dict(vec![names, limits, kids])));
    typ
}

#[cfg(test)]
mod test_name_tree {
    use super::super::super::pcore::parsebuffer::ParseBuffer;
    use super::super::pdf_obj::parse_pdf_obj;
    use super::super::pdf_type_check::{check_type, PDFPrimType, PDFType, TypeCheckError};
    use super::{
        intermediate_type, leaves_type, mk_names_check, mk_new_context, name_tree, root_kids_type,
        root_names_type,
    };
    use std::rc::Rc;

    #[test]
    fn test_names() {
        let mut ctxt = mk_new_context();

        let typ = mk_names_check();

        let v = Vec::from(
            "[(Xenon) 129 0 R
        (Ytterbium) 130 0 R
        (Yttrium) 131 0 R
        (Zinc) 132 0 R
        (Zirconium) 133 0 R
        ]"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }
    #[test]
    fn test_names_false() {
        let mut ctxt = mk_new_context();

        let typ = mk_names_check();

        let v = Vec::from(
            "[(Xenon) 129
        (Ytterbium) 130 0 R
        (Yttrium) 131 0 R
        (Zinc) 132 0 R
        (Zirconium) 133 0 R
        ]"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Reference not found in Name Tree".to_string()
            ))
        );
    }

    #[test]
    fn test_root_names_false_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }
    #[test]
    fn test_root_names_true_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<< /Names [(Actinium) 25 0 R
        (Aluminum) 26 0 R
        (Americium) 27 0 R
        (Antimony) 28 0 R
        (Argon) 29 0 R
        (Arsenic) 30 0 R
        (Astatine) 31 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_root_kids_false_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from("<</Type /Pages /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Missing field or Forbidden field".to_string()
            ))
        );
    }
    #[test]
    fn test_root_kids_forbidden_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<< /Names [(Actinium) 25 0 R
        (Aluminum) 26 0 R
        (Americium) 27 0 R
        (Antimony) 28 0 R
        (Argon) 29 0 R
        (Arsenic) 30 0 R
        (Astatine) 31 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_root_kids_true_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Kids [2 0 R
        3 0 R
        4 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_intermediate_true_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Limits [(Hafnium) (Protactinium)]
        /Kids [12 0 R
        13 0 R
        14 0 R
        15 0 R
        16 0 R
        17 0 R
        18 0 R
        19 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = name_tree();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_intermediate_false_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Limits [(Hafnium) 1]
        /Kids [12 0 R
        13 0 R
        14 0 R
        15 0 R
        16 0 R
        17 0 R
        18 0 R
        19 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = name_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "TypeMismatch: String expected".to_string()
            ))
        );
    }

    #[test]
    fn test_forbidden_false_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Limits [(Hafnium) (Aluminum)]
        /Kids [12 0 R
        13 0 R
        14 0 R
        15 0 R
        16 0 R
        17 0 R
        18 0 R
        19 0 R
        ]
        /Names [(Actinium) 25 0 R
        (Aluminum) 26 0 R
        (Americium) 27 0 R
        (Antimony) 28 0 R
        (Argon) 29 0 R
        (Arsenic) 30 0 R
        (Astatine) 31 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = name_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Missing field or Forbidden field".to_string()
            ))
        );
    }

    #[test]
    fn test_leaves_true_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Limits [(Actinium) (Astatine)]
        /Names [(Actinium) 25 0 R
        (Aluminum) 26 0 R
        (Americium) 27 0 R
        (Antimony) 28 0 R
        (Argon) 29 0 R
        (Arsenic) 30 0 R
        (Astatine) 31 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = name_tree();
        assert_eq!(check_type(&ctxt, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_leaves_false_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Limits [(Xenon) 1]
        /Names [(Xenon) 129 0 R
        (Ytterbium) 130 0 R
        (Yttrium) 131 0 R
        (Zinc) 132 0 R
        (Zirconium) 133 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = name_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "TypeMismatch: String expected".to_string()
            ))
        );
    }

    #[test]
    fn test_leaves_forbidden_false_name_tree() {
        let mut ctxt = mk_new_context();

        let v = Vec::from(
            "<</Limits [(Hafnium) (Aluminum)]
        /Kids [12 0 R
        13 0 R
        14 0 R
        15 0 R
        16 0 R
        17 0 R
        18 0 R
        19 0 R
        ]
        /Names [(Actinium) 25 0 R
        (Aluminum) 26 0 R
        (Americium) 27 0 R
        (Antimony) 28 0 R
        (Argon) 29 0 R
        (Arsenic) 30 0 R
        (Astatine) 31 0 R
        ]
        >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = name_tree();
        assert_eq!(
            check_type(&ctxt, Rc::new(obj), typ),
            Some(TypeCheckError::PredicateError(
                "Missing field or Forbidden field".to_string()
            ))
        );
    }
}
