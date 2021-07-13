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
use crate::pdf_lib::arrayof_2integers::arrayof_2integers_type;
use crate::pdf_lib::arrayofmultilangtext::arrayofmultilangtext_type;
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
pub fn floatingwindowparameters_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_3 = arrayofmultilangtext_type(tctx);
    let assignment_bool_2 =
        TypeCheck::new(tctx, "uc", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_bool_1 =
        TypeCheck::new(tctx, "t", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_0 = arrayof_2integers_type(tctx);
    let dis_1 = TypeCheck::new(tctx, "tt", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(tctx, "d", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_r = ChoicePred(
        String::from("Invalid R"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
        ],
    );
    let choices_o = ChoicePred(
        String::from("Invalid O"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
        ],
    );
    let choices_p = ChoicePred(
        String::from("Invalid P"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("3"))),
            PDFObjT::Name(NameT::new(Vec::from("4"))),
            PDFObjT::Name(NameT::new(Vec::from("5"))),
            PDFObjT::Name(NameT::new(Vec::from("6"))),
            PDFObjT::Name(NameT::new(Vec::from("7"))),
            PDFObjT::Name(NameT::new(Vec::from("8"))),
        ],
    );
    let choices_rt = ChoicePred(
        String::from("Invalid RT"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("3"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("FWParams")))],
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
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let rt_field = DictEntry {
        key: Vec::from("RT"),
        chk: TypeCheck::new_refined(
            tctx,
            "rt",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_rt),
        ),
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
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: TypeCheck::new_refined(
            tctx,
            "o",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_o),
        ),
        opt: DictKeySpec::Optional,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let uc_field = DictEntry {
        key: Vec::from("UC"),
        chk: assignment_bool_2,

        opt: DictKeySpec::Optional,
    };
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: TypeCheck::new_refined(
            tctx,
            "r",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_r),
        ),
        opt: DictKeySpec::Optional,
    };
    let tt_field = DictEntry {
        key: Vec::from("TT"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "floatingwindowparameters",
        Rc::new(PDFType::Dict(vec![
            type_field, d_field, rt_field, p_field, o_field, t_field, uc_field, r_field, tt_field,
        ])),
    )
}
