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
pub fn richmediacommand_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_3 = TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_integer_2 =
        TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_bool_1 =
        TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_0 = TypeCheck::new(tctx, "c", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("RichMediaCommand")))],
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
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: TypeCheck::new(
            tctx,
            "a",
            Rc::new(PDFType::Disjunct(vec![
                assignment_bool_1,
                assignment_integer_2,
                assignment_3,
                assignment_4,
            ])),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "richmediacommand",
        Rc::new(PDFType::Dict(vec![type_field, c_field, a_field])),
    )
}
