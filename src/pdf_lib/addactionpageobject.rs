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
use crate::pdf_lib::actionecmascript::actionecmascript_type;
use crate::pdf_lib::actiongoto::actiongoto_type;
use crate::pdf_lib::actiongoto3dview::actiongoto3dview_type;
use crate::pdf_lib::actiongotodp::actiongotodp_type;
use crate::pdf_lib::actiongotoe::actiongotoe_type;
use crate::pdf_lib::actiongotor::actiongotor_type;
use crate::pdf_lib::actionhide::actionhide_type;
use crate::pdf_lib::actionimportdata::actionimportdata_type;
use crate::pdf_lib::actionlaunch::actionlaunch_type;
use crate::pdf_lib::actionmovie::actionmovie_type;
use crate::pdf_lib::actionnamed::actionnamed_type;
use crate::pdf_lib::actionrendition::actionrendition_type;
use crate::pdf_lib::actionresetform::actionresetform_type;
use crate::pdf_lib::actionrichmediaexecute::actionrichmediaexecute_type;
use crate::pdf_lib::actionsetocgstate::actionsetocgstate_type;
use crate::pdf_lib::actionsound::actionsound_type;
use crate::pdf_lib::actionsubmitform::actionsubmitform_type;
use crate::pdf_lib::actionthread::actionthread_type;
use crate::pdf_lib::actiontransition::actiontransition_type;
use crate::pdf_lib::actionuri::actionuri_type;
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
pub fn addactionpageobject_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_39 = actionrichmediaexecute_type(tctx);
    let assignment_19 = actionrichmediaexecute_type(tctx);
    let assignment_38 = actionecmascript_type(tctx);
    let assignment_18 = actionecmascript_type(tctx);
    let assignment_37 = actiongoto3dview_type(tctx);
    let assignment_17 = actiongoto3dview_type(tctx);
    let assignment_36 = actiontransition_type(tctx);
    let assignment_16 = actiontransition_type(tctx);
    let assignment_15 = actionrendition_type(tctx);
    let assignment_35 = actionrendition_type(tctx);
    let assignment_34 = actionsetocgstate_type(tctx);
    let assignment_14 = actionsetocgstate_type(tctx);
    let assignment_33 = actionimportdata_type(tctx);
    let assignment_13 = actionimportdata_type(tctx);
    let assignment_32 = actionresetform_type(tctx);
    let assignment_12 = actionresetform_type(tctx);
    let assignment_11 = actionsubmitform_type(tctx);
    let assignment_31 = actionsubmitform_type(tctx);
    let assignment_10 = actionnamed_type(tctx);
    let assignment_30 = actionnamed_type(tctx);
    let assignment_29 = actionhide_type(tctx);
    let assignment_9 = actionhide_type(tctx);
    let assignment_28 = actionmovie_type(tctx);
    let assignment_8 = actionmovie_type(tctx);
    let assignment_7 = actionsound_type(tctx);
    let assignment_27 = actionsound_type(tctx);
    let assignment_26 = actionuri_type(tctx);
    let assignment_6 = actionuri_type(tctx);
    let assignment_5 = actionthread_type(tctx);
    let assignment_25 = actionthread_type(tctx);
    let assignment_4 = actionlaunch_type(tctx);
    let assignment_24 = actionlaunch_type(tctx);
    let assignment_23 = actiongotodp_type(tctx);
    let assignment_3 = actiongotodp_type(tctx);
    let assignment_2 = actiongotoe_type(tctx);
    let assignment_22 = actiongotoe_type(tctx);
    let assignment_1 = actiongotor_type(tctx);
    let assignment_21 = actiongotor_type(tctx);
    let assignment_0 = actiongoto_type(tctx);
    let assignment_20 = actiongoto_type(tctx);
    let dis_1 = TypeCheck::new(
        tctx,
        "c",
        Rc::new(PDFType::Disjunct(vec![
            assignment_20,
            assignment_21,
            assignment_22,
            assignment_23,
            assignment_24,
            assignment_25,
            assignment_26,
            assignment_27,
            assignment_28,
            assignment_29,
            assignment_30,
            assignment_31,
            assignment_32,
            assignment_33,
            assignment_34,
            assignment_35,
            assignment_36,
            assignment_37,
            assignment_38,
            assignment_39,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "o",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
            assignment_4,
            assignment_5,
            assignment_6,
            assignment_7,
            assignment_8,
            assignment_9,
            assignment_10,
            assignment_11,
            assignment_12,
            assignment_13,
            assignment_14,
            assignment_15,
            assignment_16,
            assignment_17,
            assignment_18,
            assignment_19,
        ])),
    );
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "addactionpageobject",
        Rc::new(PDFType::Dict(vec![o_field, c_field])),
    )
}
