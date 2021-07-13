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
use crate::pdf_lib::mediaduration::mediaduration_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn mediaplayparametersbe_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = TypeCheck::new(tctx, "rc", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_bool_3 =
        TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_2 = mediaduration_type(tctx);
    let assignment_bool_1 =
        TypeCheck::new(tctx, "c", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_integer_0 =
        TypeCheck::new(tctx, "v", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let dis_0 = TypeCheck::new(tctx, "d", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let choices_f = ChoicePred(
        String::from("Invalid F"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("3"))),
            PDFObjT::Name(NameT::new(Vec::from("4"))),
            PDFObjT::Name(NameT::new(Vec::from("5"))),
        ],
    );
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: TypeCheck::new_refined(
            tctx,
            "f",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_f),
        ),
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: assignment_bool_3,

        opt: DictKeySpec::Optional,
    };
    let rc_field = DictEntry {
        key: Vec::from("RC"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "mediaplayparametersbe",
        Rc::new(PDFType::Dict(vec![
            v_field, c_field, f_field, d_field, a_field, rc_field,
        ])),
    )
}
