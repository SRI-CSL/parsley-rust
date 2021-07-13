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
use crate::pdf_lib::arrayof_2numbers::arrayof_2numbers_type;
use crate::pdf_lib::arrayofduration::arrayofduration_type;
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
pub fn movieactivation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = arrayof_2numbers_type(tctx);
    let assignment_6 = arrayof_2integers_type(tctx);
    let assignment_bool_5 = TypeCheck::new(
        tctx,
        "synchronous",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_4 = TypeCheck::new(
        tctx,
        "showcontrols",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_3 = TypeCheck::new(
        tctx,
        "volume",
        Rc::new(PDFType::PrimType(PDFPrimType::Real)),
    );
    let assignment_2 = TypeCheck::new(tctx, "rate", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_1 = arrayofduration_type(tctx);
    let assignment_0 = arrayofduration_type(tctx);
    let dis_3 = TypeCheck::new(
        tctx,
        "fwposition",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "fwscale",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "duration",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "start",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_mode = ChoicePred(
        String::from("Invalid Mode"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Once"))),
            PDFObjT::Name(NameT::new(Vec::from("Open"))),
            PDFObjT::Name(NameT::new(Vec::from("Repeat"))),
            PDFObjT::Name(NameT::new(Vec::from("Palindrome"))),
        ],
    );
    let start_field = DictEntry {
        key: Vec::from("Start"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let duration_field = DictEntry {
        key: Vec::from("Duration"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let rate_field = DictEntry {
        key: Vec::from("Rate"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let volume_field = DictEntry {
        key: Vec::from("Volume"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let showcontrols_field = DictEntry {
        key: Vec::from("ShowControls"),
        chk: assignment_bool_4,

        opt: DictKeySpec::Optional,
    };
    let mode_field = DictEntry {
        key: Vec::from("Mode"),
        chk: TypeCheck::new_refined(
            tctx,
            "mode",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_mode),
        ),
        opt: DictKeySpec::Optional,
    };
    let synchronous_field = DictEntry {
        key: Vec::from("Synchronous"),
        chk: assignment_bool_5,

        opt: DictKeySpec::Optional,
    };
    let fwscale_field = DictEntry {
        key: Vec::from("FWScale"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let fwposition_field = DictEntry {
        key: Vec::from("FWPosition"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "movieactivation",
        Rc::new(PDFType::Dict(vec![
            start_field,
            duration_field,
            rate_field,
            volume_field,
            showcontrols_field,
            mode_field,
            synchronous_field,
            fwscale_field,
            fwposition_field,
        ])),
    )
}
