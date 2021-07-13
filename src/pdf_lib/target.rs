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
pub fn target_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_5 = target_type(tctx);
    let assignment_4 = TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_integer_3 =
        TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_2 = TypeCheck::new(tctx, "p", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_integer_1 =
        TypeCheck::new(tctx, "p", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_0 = TypeCheck::new(tctx, "n", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let dis_0 = TypeCheck::new(tctx, "t", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let choices_r = ChoicePred(
        String::from("Invalid R"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("P"))),
            PDFObjT::Name(NameT::new(Vec::from("C"))),
        ],
    );
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: TypeCheck::new_refined(
            tctx,
            "r",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_r),
        ),
        opt: DictKeySpec::Required,
    };
    let n_field = DictEntry {
        key: Vec::from("N"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: TypeCheck::new(
            tctx,
            "p",
            Rc::new(PDFType::Disjunct(vec![assignment_integer_1, assignment_2])),
        ),
        opt: DictKeySpec::Optional,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: TypeCheck::new(
            tctx,
            "a",
            Rc::new(PDFType::Disjunct(vec![assignment_integer_3, assignment_4])),
        ),
        opt: DictKeySpec::Optional,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "target",
        Rc::new(PDFType::Dict(vec![
            r_field, n_field, p_field, a_field, t_field,
        ])),
    )
}
