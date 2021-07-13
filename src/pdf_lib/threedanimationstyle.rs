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
pub fn threedanimationstyle_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = TypeCheck::new(tctx, "tm", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_integer_0 =
        TypeCheck::new(tctx, "pc", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("None"))),
            PDFObjT::Name(NameT::new(Vec::from("Linear"))),
            PDFObjT::Name(NameT::new(Vec::from("Oscillating"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("3DAnimationStyle")))],
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
    let subtype_field = DictEntry {
        key: Vec::from("Subtype"),
        chk: TypeCheck::new_refined(
            tctx,
            "subtype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_subtype),
        ),
        opt: DictKeySpec::Optional,
    };
    let pc_field = DictEntry {
        key: Vec::from("PC"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let tm_field = DictEntry {
        key: Vec::from("TM"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "threedanimationstyle",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            pc_field,
            tm_field,
        ])),
    )
}
