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
use crate::pdf_lib::arrayofarraysrbgroups::arrayofarraysrbgroups_type;
use crate::pdf_lib::arrayofnamesgeneral::arrayofnamesgeneral_type;
use crate::pdf_lib::arrayofocusage::arrayofocusage_type;
use crate::pdf_lib::arrayofoptcontentgroups::arrayofoptcontentgroups_type;
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
pub fn optcontentconfig_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = arrayofarraysrbgroups_type(tctx);
    let assignment_5 = arrayofocusage_type(tctx);
    let assignment_4 = arrayofnamesgeneral_type(tctx);
    let assignment_2 = arrayofoptcontentgroups_type(tctx);
    let assignment_8 = arrayofoptcontentgroups_type(tctx);
    let assignment_3 = arrayofoptcontentgroups_type(tctx);
    let assignment_6 = arrayofoptcontentgroups_type(tctx);
    let assignment_1 = TypeCheck::new(
        tctx,
        "creator",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_0 = TypeCheck::new(
        tctx,
        "name",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "locked",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "rbgroups",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "order",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_3 = TypeCheck::new(tctx, "as", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_2 = TypeCheck::new(
        tctx,
        "intent",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_1 = TypeCheck::new(tctx, "off", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(tctx, "on", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let choices_listmode = ChoicePred(
        String::from("Invalid ListMode"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("AllPages"))),
            PDFObjT::Name(NameT::new(Vec::from("VisiblePages"))),
        ],
    );
    let choices_basestate = ChoicePred(
        String::from("Invalid BaseState"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("ON"))),
            PDFObjT::Name(NameT::new(Vec::from("OFF"))),
            PDFObjT::Name(NameT::new(Vec::from("Unchanged"))),
        ],
    );
    let name_field = DictEntry {
        key: Vec::from("Name"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let creator_field = DictEntry {
        key: Vec::from("Creator"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let basestate_field = DictEntry {
        key: Vec::from("BaseState"),
        chk: TypeCheck::new_refined(
            tctx,
            "basestate",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_basestate),
        ),
        opt: DictKeySpec::Optional,
    };
    let on_field = DictEntry {
        key: Vec::from("ON"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let off_field = DictEntry {
        key: Vec::from("OFF"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let intent_field = DictEntry {
        key: Vec::from("Intent"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let as_field = DictEntry {
        key: Vec::from("AS"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let order_field = DictEntry {
        key: Vec::from("Order"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let listmode_field = DictEntry {
        key: Vec::from("ListMode"),
        chk: TypeCheck::new_refined(
            tctx,
            "listmode",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_listmode),
        ),
        opt: DictKeySpec::Optional,
    };
    let rbgroups_field = DictEntry {
        key: Vec::from("RBGroups"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let locked_field = DictEntry {
        key: Vec::from("Locked"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "optcontentconfig",
        Rc::new(PDFType::Dict(vec![
            name_field,
            creator_field,
            basestate_field,
            on_field,
            off_field,
            intent_field,
            as_field,
            order_field,
            listmode_field,
            rbgroups_field,
            locked_field,
        ])),
    )
}
