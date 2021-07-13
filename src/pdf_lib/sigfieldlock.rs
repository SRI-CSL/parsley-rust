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
use super::pdf_obj::PDFObjT;
use crate::pdf_lib::arrayofstringstext::arrayofstringstext_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn sigfieldlock_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_0 = arrayofstringstext_type(tctx);
    let dis_0 = TypeCheck::new(
        tctx,
        "fields",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_p = ChoicePred(
        String::from("Invalid P"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("3"))),
        ],
    );
    let choices_action = ChoicePred(
        String::from("Invalid Action"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("All"))),
            PDFObjT::Name(NameT::new(Vec::from("Include"))),
            PDFObjT::Name(NameT::new(Vec::from("Exclude"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("SigFieldLock")))],
    );
    let type_field = DictEntry {
        key: Vec::from("Type"),
        chk: TypeCheck::new_refined(
            tctx,
            "type",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_type),
        ),
        opt: DictKeySpec::Optional,
    };
    let action_field = DictEntry {
        key: Vec::from("Action"),
        chk: TypeCheck::new_refined(
            tctx,
            "action",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_action),
        ),
        opt: DictKeySpec::Required,
    };
    let fields_field = DictEntry {
        key: Vec::from("Fields"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: TypeCheck::new_refined(
            tctx,
            "p",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_p),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "sigfieldlock",
        Rc::new(PDFType::Dict(vec![
            type_field,
            action_field,
            fields_field,
            p_field,
        ])),
    )
}
