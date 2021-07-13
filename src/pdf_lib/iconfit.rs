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
use crate::pdf_lib::arrayof_2numbers::arrayof_2numbers_type;
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
pub fn iconfit_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_bool_1 =
        TypeCheck::new(tctx, "fb", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_0 = arrayof_2numbers_type(tctx);
    let dis_0 = TypeCheck::new(tctx, "a", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("A"))),
            PDFObjT::Name(NameT::new(Vec::from("P"))),
        ],
    );
    let choices_sw = ChoicePred(
        String::from("Invalid SW"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("A"))),
            PDFObjT::Name(NameT::new(Vec::from("B"))),
            PDFObjT::Name(NameT::new(Vec::from("S"))),
            PDFObjT::Name(NameT::new(Vec::from("N"))),
        ],
    );
    let sw_field = DictEntry {
        key: Vec::from("SW"),
        chk: TypeCheck::new_refined(
            tctx,
            "sw",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_sw),
        ),
        opt: DictKeySpec::Optional,
    };
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: TypeCheck::new_refined(
            tctx,
            "s",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_s),
        ),
        opt: DictKeySpec::Optional,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let fb_field = DictEntry {
        key: Vec::from("FB"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "iconfit",
        Rc::new(PDFType::Dict(vec![sw_field, s_field, a_field, fb_field])),
    )
}
