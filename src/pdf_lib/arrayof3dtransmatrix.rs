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
pub fn arrayof3dtransmatrix_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_11 = TypeCheck::new(tctx, "11", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_10 = TypeCheck::new(tctx, "10", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_9 = TypeCheck::new(tctx, "9", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_8 = TypeCheck::new(tctx, "8", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_7 = TypeCheck::new(tctx, "7", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_6 = TypeCheck::new(tctx, "6", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_5 = TypeCheck::new(tctx, "5", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_4 = TypeCheck::new(tctx, "4", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_3 = TypeCheck::new(tctx, "3", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_2 = TypeCheck::new(tctx, "2", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_1 = TypeCheck::new(tctx, "1", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_0 = TypeCheck::new(tctx, "0", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let zero_field = DictEntry {
        key: Vec::from("0"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let one_field = DictEntry {
        key: Vec::from("1"),
        chk: assignment_1,

        opt: DictKeySpec::Required,
    };
    let two_field = DictEntry {
        key: Vec::from("2"),
        chk: assignment_2,

        opt: DictKeySpec::Required,
    };
    let three_field = DictEntry {
        key: Vec::from("3"),
        chk: assignment_3,

        opt: DictKeySpec::Required,
    };
    let four_field = DictEntry {
        key: Vec::from("4"),
        chk: assignment_4,

        opt: DictKeySpec::Required,
    };
    let five_field = DictEntry {
        key: Vec::from("5"),
        chk: assignment_5,

        opt: DictKeySpec::Required,
    };
    let six_field = DictEntry {
        key: Vec::from("6"),
        chk: assignment_6,

        opt: DictKeySpec::Required,
    };
    let seven_field = DictEntry {
        key: Vec::from("7"),
        chk: assignment_7,

        opt: DictKeySpec::Required,
    };
    let eight_field = DictEntry {
        key: Vec::from("8"),
        chk: assignment_8,

        opt: DictKeySpec::Required,
    };
    let nine_field = DictEntry {
        key: Vec::from("9"),
        chk: assignment_9,

        opt: DictKeySpec::Required,
    };
    let one0_field = DictEntry {
        key: Vec::from("10"),
        chk: assignment_10,

        opt: DictKeySpec::Required,
    };
    let one1_field = DictEntry {
        key: Vec::from("11"),
        chk: assignment_11,

        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "arrayof3dtransmatrix",
        Rc::new(PDFType::Dict(vec![
            zero_field,
            one_field,
            two_field,
            three_field,
            four_field,
            five_field,
            six_field,
            seven_field,
            eight_field,
            nine_field,
            one0_field,
            one1_field,
        ])),
    )
}
