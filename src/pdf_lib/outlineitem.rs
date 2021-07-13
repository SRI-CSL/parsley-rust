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
use crate::pdf_lib::arrayof_3rgbnumbers::arrayof_3rgbnumbers_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::dest0::dest0_type;
use crate::pdf_lib::dest1::dest1_type;
use crate::pdf_lib::dest4::dest4_type;
use crate::pdf_lib::destxyz::destxyz_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::outline::outline_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::structelem::structelem_type;
use std::rc::Rc;
pub fn outlineitem_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_34 =
        TypeCheck::new(tctx, "f", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_33 = arrayof_3rgbnumbers_type(tctx);
    let assignment_32 = structelem_type(tctx);
    let assignment_31 = actionrichmediaexecute_type(tctx);
    let assignment_30 = actionecmascript_type(tctx);
    let assignment_29 = actiongoto3dview_type(tctx);
    let assignment_28 = actiontransition_type(tctx);
    let assignment_27 = actionrendition_type(tctx);
    let assignment_26 = actionsetocgstate_type(tctx);
    let assignment_25 = actionimportdata_type(tctx);
    let assignment_24 = actionresetform_type(tctx);
    let assignment_23 = actionsubmitform_type(tctx);
    let assignment_22 = actionnamed_type(tctx);
    let assignment_21 = actionhide_type(tctx);
    let assignment_20 = actionmovie_type(tctx);
    let assignment_19 = actionsound_type(tctx);
    let assignment_18 = actionuri_type(tctx);
    let assignment_17 = actionthread_type(tctx);
    let assignment_16 = actionlaunch_type(tctx);
    let assignment_15 = actiongotodp_type(tctx);
    let assignment_14 = actiongotoe_type(tctx);
    let assignment_13 = actiongotor_type(tctx);
    let assignment_12 = actiongoto_type(tctx);
    let assignment_11 = dest4_type(tctx);
    let assignment_10 = dest1_type(tctx);
    let assignment_9 = dest0_type(tctx);
    let assignment_8 = destxyz_type(tctx);
    let assignment_integer_7 = TypeCheck::new(
        tctx,
        "count",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_2 = outline_type(tctx);
    let assignment_5 = outlineitem_type(tctx);
    let assignment_1 = outlineitem_type(tctx);
    let assignment_4 = outlineitem_type(tctx);
    let assignment_3 = outlineitem_type(tctx);
    let assignment_6 = outlineitem_type(tctx);
    let assignment_0 = TypeCheck::new(
        tctx,
        "title",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let dis_8 = TypeCheck::new(tctx, "c", Rc::new(PDFType::Disjunct(vec![assignment_33])));
    let dis_7 = TypeCheck::new(tctx, "se", Rc::new(PDFType::Disjunct(vec![assignment_32])));
    let dis_6 = TypeCheck::new(
        tctx,
        "a",
        Rc::new(PDFType::Disjunct(vec![
            assignment_12,
            assignment_13,
            assignment_14,
            assignment_15,
            assignment_16,
            assignment_17,
            assignment_18,
            assignment_19,
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
        ])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "dest",
        Rc::new(PDFType::Disjunct(vec![
            assignment_8,
            assignment_9,
            assignment_10,
            assignment_11,
        ])),
    );
    let dis_4 = TypeCheck::new(tctx, "last", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_3 = TypeCheck::new(
        tctx,
        "first",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_2 = TypeCheck::new(tctx, "next", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_1 = TypeCheck::new(tctx, "prev", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let title_field = DictEntry {
        key: Vec::from("Title"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let prev_field = DictEntry {
        key: Vec::from("Prev"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let next_field = DictEntry {
        key: Vec::from("Next"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let first_field = DictEntry {
        key: Vec::from("First"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let last_field = DictEntry {
        key: Vec::from("Last"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let count_field = DictEntry {
        key: Vec::from("Count"),
        chk: assignment_integer_7,

        opt: DictKeySpec::Optional,
    };
    let dest_field = DictEntry {
        key: Vec::from("Dest"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let se_field = DictEntry {
        key: Vec::from("SE"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: assignment_integer_34,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "outlineitem",
        Rc::new(PDFType::Dict(vec![
            title_field,
            parent_field,
            prev_field,
            next_field,
            first_field,
            last_field,
            count_field,
            dest_field,
            a_field,
            se_field,
            c_field,
            f_field,
        ])),
    )
}
