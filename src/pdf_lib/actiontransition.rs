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
use crate::pdf_lib::actionuri::actionuri_type;
use crate::pdf_lib::arrayofactions::arrayofactions_type;
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
use crate::pdf_lib::transition::transition_type;
use std::rc::Rc;
pub fn actiontransition_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_21 = transition_type(tctx);
    let assignment_20 = actionrichmediaexecute_type(tctx);
    let assignment_19 = actionecmascript_type(tctx);
    let assignment_18 = actiongoto3dview_type(tctx);
    let assignment_17 = actiontransition_type(tctx);
    let assignment_16 = actionrendition_type(tctx);
    let assignment_15 = actionsetocgstate_type(tctx);
    let assignment_14 = actionimportdata_type(tctx);
    let assignment_13 = actionresetform_type(tctx);
    let assignment_12 = actionsubmitform_type(tctx);
    let assignment_11 = actionnamed_type(tctx);
    let assignment_10 = actionhide_type(tctx);
    let assignment_9 = actionmovie_type(tctx);
    let assignment_8 = actionsound_type(tctx);
    let assignment_7 = actionuri_type(tctx);
    let assignment_6 = actionthread_type(tctx);
    let assignment_5 = actionlaunch_type(tctx);
    let assignment_4 = actiongotodp_type(tctx);
    let assignment_3 = actiongotoe_type(tctx);
    let assignment_2 = actiongotor_type(tctx);
    let assignment_1 = actiongoto_type(tctx);
    let assignment_0 = arrayofactions_type(tctx);
    let dis_1 = TypeCheck::new(
        tctx,
        "trans",
        Rc::new(PDFType::Disjunct(vec![assignment_21])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "next",
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
            assignment_20,
        ])),
    );
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Trans")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Action")))],
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
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: TypeCheck::new_refined(
            tctx,
            "s",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_s),
        ),
        opt: DictKeySpec::Required,
    };
    let next_field = DictEntry {
        key: Vec::from("Next"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let trans_field = DictEntry {
        key: Vec::from("Trans"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "actiontransition",
        Rc::new(PDFType::Dict(vec![
            type_field,
            s_field,
            next_field,
            trans_field,
        ])),
    )
}
