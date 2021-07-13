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
use crate::pdf_lib::arrayofnumberformats::arrayofnumberformats_type;
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
pub fn measurerl_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_8 = TypeCheck::new(tctx, "cyx", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_7 = arrayof_2numbers_type(tctx);
    let assignment_5 = arrayofnumberformats_type(tctx);
    let assignment_1 = arrayofnumberformats_type(tctx);
    let assignment_2 = arrayofnumberformats_type(tctx);
    let assignment_4 = arrayofnumberformats_type(tctx);
    let assignment_3 = arrayofnumberformats_type(tctx);
    let assignment_6 = arrayofnumberformats_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "r", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let dis_6 = TypeCheck::new(tctx, "o", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_5 = TypeCheck::new(tctx, "s", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_4 = TypeCheck::new(tctx, "t", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_3 = TypeCheck::new(tctx, "a", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_2 = TypeCheck::new(tctx, "d", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_1 = TypeCheck::new(tctx, "y", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_0 = TypeCheck::new(tctx, "x", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("RL")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Measure")))],
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
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let x_field = DictEntry {
        key: Vec::from("X"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let y_field = DictEntry {
        key: Vec::from("Y"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_2,
        opt: DictKeySpec::Required,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: dis_3,
        opt: DictKeySpec::Required,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let cyx_field = DictEntry {
        key: Vec::from("CYX"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "measurerl",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            r_field,
            x_field,
            y_field,
            d_field,
            a_field,
            t_field,
            s_field,
            o_field,
            cyx_field,
        ])),
    )
}
