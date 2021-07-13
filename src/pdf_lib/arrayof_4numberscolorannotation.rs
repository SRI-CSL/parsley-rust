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
pub fn arrayof_4numberscolorannotation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_3 = TypeCheck::new(tctx, "3", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_2 = TypeCheck::new(tctx, "2", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_1 = TypeCheck::new(tctx, "1", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_0 = TypeCheck::new(tctx, "0", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let zero_field = DictEntry {
        key: Vec::from("0"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let one_field = DictEntry {
        key: Vec::from("1"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let two_field = DictEntry {
        key: Vec::from("2"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let three_field = DictEntry {
        key: Vec::from("3"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "arrayof_4numberscolorannotation",
        Rc::new(PDFType::Dict(vec![
            zero_field,
            one_field,
            two_field,
            three_field,
        ])),
    )
}
