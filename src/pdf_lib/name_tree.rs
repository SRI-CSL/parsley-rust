// Copyright (c) 2019-2021 SRI International.
// All rights reserved.
//
//    This file is part of the Parsley parser.
//
//    Parsley is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    Parsley is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{DictKey, PDFObjT};
use crate::pdf_lib::pdf_type_check::{
    PDFType, Predicate, TypeCheck, TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;

struct NameTreePredicate;
impl Predicate for NameTreePredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<LocatedVal<TypeCheckError>> {
        if let PDFObjT::Dict(ref s) = obj.val() {
            let mappings = s.map();
            if let Some(a) = mappings.get(&DictKey::new(Vec::from("Names"))) {
                if let PDFObjT::Array(ref s) = a.val() {
                    if s.objs().len() % 2 == 0 {
                        for c in (0 .. s.objs().len()).step_by(2) {
                            if let PDFObjT::String(ref _s1) = s.objs()[c].val() {
                                if let PDFObjT::Reference(ref _s2) = s.objs()[c + 1].val() {
                                } else {
                                    return Some(obj.place(TypeCheckError::PredicateError(
                                        "Reference not found in Name Tree".to_string(),
                                    )))
                                }
                            } else {
                                return Some(obj.place(TypeCheckError::PredicateError(
                                    "String not found in name tree".to_string(),
                                )))
                            }
                        }
                    } else {
                        return Some(obj.place(TypeCheckError::PredicateError(
                            "Array found but not correct length in Name Tree".to_string(),
                        )))
                    }
                } else {
                    return Some(obj.place(TypeCheckError::PredicateError(
                        "Array not found in Name Tree".to_string(),
                    )))
                }
            }
            if let Some(a) = mappings.get(&DictKey::new(Vec::from("Limits"))) {
                if let PDFObjT::Array(ref s) = a.val() {
                    for c in s.objs() {
                        if let PDFObjT::String(ref _s1) = c.val() {
                        } else {
                            return Some(obj.place(TypeCheckError::PredicateError(
                                "TypeMismatch: String expected".to_string(),
                            )))
                        }
                    }
                    if s.objs().len() != 2 {
                        return Some(obj.place(TypeCheckError::PredicateError(
                            "Length Mismatch".to_string(),
                        )))
                    }
                }
            }
            if let Some(a) = mappings.get(&DictKey::new(Vec::from("Kids"))) {
                if let PDFObjT::Array(ref s) = a.val() {
                    for c in s.objs() {
                        if let PDFObjT::Reference(ref _s2) = c.val() {
                        } else {
                            return Some(obj.place(TypeCheckError::PredicateError(
                                "Reference expected".to_string(),
                            )))
                        }
                    }
                } else {
                    return Some(obj.place(TypeCheckError::PredicateError(
                        "Reference wasn't an Array".to_string(),
                    )))
                }
            }

            if (mappings.contains_key(&DictKey::new(Vec::from("Names")))
                && mappings.contains_key(&DictKey::new(Vec::from("Limits")))
                && !mappings.contains_key(&DictKey::new(Vec::from("Kids"))))
                || (!mappings.contains_key(&DictKey::new(Vec::from("Names")))
                    && mappings.contains_key(&DictKey::new(Vec::from("Limits")))
                    && mappings.contains_key(&DictKey::new(Vec::from("Kids"))))
                || (!mappings.contains_key(&DictKey::new(Vec::from("Names")))
                    && !mappings.contains_key(&DictKey::new(Vec::from("Limits")))
                    && mappings.contains_key(&DictKey::new(Vec::from("Kids"))))
                || (mappings.contains_key(&DictKey::new(Vec::from("Names")))
                    && !mappings.contains_key(&DictKey::new(Vec::from("Limits")))
                    && !mappings.contains_key(&DictKey::new(Vec::from("Kids"))))
            {
                None
            } else {
                Some(obj.place(TypeCheckError::PredicateError(
                    "Missing field or Forbidden field".to_string(),
                )))
            }
        } else {
            Some(obj.place(TypeCheckError::PredicateError(
                "No Dictionary, no Name Tree".to_string(),
            )))
        }
    }
}
struct NamesPredicate;
impl Predicate for NamesPredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<LocatedVal<TypeCheckError>> {
        if let PDFObjT::Array(ref s) = obj.val() {
            if s.objs().len() % 2 == 0 {
                for c in (0 .. s.objs().len()).step_by(2) {
                    if let PDFObjT::String(ref _s1) = s.objs()[c].val() {
                        if let PDFObjT::Reference(ref _s2) = s.objs()[c + 1].val() {
                        } else {
                            return Some(obj.place(TypeCheckError::PredicateError(
                                "Reference not found in Name Tree".to_string(),
                            )))
                        }
                    } else {
                        return Some(obj.place(TypeCheckError::PredicateError(
                            "String not found in name tree".to_string(),
                        )))
                    }
                }
                None
            } else {
                Some(obj.place(TypeCheckError::PredicateError(
                    "Array found but not correct length in Name Tree".to_string(),
                )))
            }
        } else {
            Some(obj.place(TypeCheckError::PredicateError(
                "Array not found in Name Tree".to_string(),
            )))
        }
    }
}

// Leaves: Limits required, Names required, Kids forbidden

// Permutations possible for root
// Root with names
// Root with kids

pub fn name_tree(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    TypeCheck::new_refined(
        tctx,
        "nametree",
        Rc::new(PDFType::Any),
        Rc::new(NameTreePredicate),
    )
}

#[cfg(test)]
mod test_name_tree {
    use super::super::super::pcore::parsebuffer::ParseBuffer;
    use super::super::pdf_obj::{parse_pdf_obj, PDFObjContext};
    use super::super::pdf_type_check::{check_type, TypeCheckContext, TypeCheckError};
    use super::name_tree;
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn test_root_names_false_name_tree() {
        let mut tctx = TypeCheckContext::new();
        let mut ctxt = mk_new_context();

        let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
    #[test]
    fn test_root_names_true_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_root_kids_false_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

        let v = Vec::from("<</Type /Pages /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let typ = name_tree(&mut tctx);
        let err = obj.place(TypeCheckError::PredicateError(
            "Missing field or Forbidden field".to_string(),
        ));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), Some(err));
    }
    #[test]
    fn test_root_kids_forbidden_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_root_kids_true_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_intermediate_true_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_intermediate_false_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        let err = obj.place(TypeCheckError::PredicateError(
            "TypeMismatch: String expected".to_string(),
        ));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), Some(err));
    }

    #[test]
    fn test_forbidden_false_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        let err = obj.place(TypeCheckError::PredicateError(
            "Missing field or Forbidden field".to_string(),
        ));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), Some(err));
    }

    #[test]
    fn test_leaves_true_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_leaves_false_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        let err = obj.place(TypeCheckError::PredicateError(
            "TypeMismatch: String expected".to_string(),
        ));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), Some(err));
    }

    #[test]
    fn test_leaves_forbidden_false_name_tree() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();

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
        let typ = name_tree(&mut tctx);
        let err = obj.place(TypeCheckError::PredicateError(
            "Missing field or Forbidden field".to_string(),
        ));
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), Some(err));
    }
}
