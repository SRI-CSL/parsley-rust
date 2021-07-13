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
use crate::pdf_lib::richmediaanimation::richmediaanimation_type;
use crate::pdf_lib::richmediaconfiguration::richmediaconfiguration_type;
use crate::pdf_lib::richmediapresentation::richmediapresentation_type;
use crate::pdf_lib::threedview::threedview_type;
use crate::pdf_lib::threedviewaddentries::threedviewaddentries_type;
use std::rc::Rc;
pub fn richmediaactivation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = richmediapresentation_type(tctx);
    let assignment_3 = richmediaconfiguration_type(tctx);
    let assignment_2 = threedviewaddentries_type(tctx);
    let assignment_1 = threedview_type(tctx);
    let assignment_0 = richmediaanimation_type(tctx);
    let dis_3 = TypeCheck::new(
        tctx,
        "presentation",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "configuration",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "view",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "animation",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_condition = ChoicePred(
        String::from("Invalid Condition"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("XA"))),
            PDFObjT::Name(NameT::new(Vec::from("PO"))),
            PDFObjT::Name(NameT::new(Vec::from("PV"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("RichMediaActivation")))],
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
    let condition_field = DictEntry {
        key: Vec::from("Condition"),
        chk: TypeCheck::new_refined(
            tctx,
            "condition",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_condition),
        ),
        opt: DictKeySpec::Optional,
    };
    let animation_field = DictEntry {
        key: Vec::from("Animation"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let view_field = DictEntry {
        key: Vec::from("View"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let configuration_field = DictEntry {
        key: Vec::from("Configuration"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let presentation_field = DictEntry {
        key: Vec::from("Presentation"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "richmediaactivation",
        Rc::new(PDFType::Dict(vec![
            type_field,
            condition_field,
            animation_field,
            view_field,
            configuration_field,
            presentation_field,
        ])),
    )
}
