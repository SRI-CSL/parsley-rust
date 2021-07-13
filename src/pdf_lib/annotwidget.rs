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
use crate::pdf_lib::addactionwidgetannotation::addactionwidgetannotation_type;
use crate::pdf_lib::appearance::appearance_type;
use crate::pdf_lib::appearancecharacteristics::appearancecharacteristics_type;
use crate::pdf_lib::arrayof_4annotbordercharacteristics::arrayof_4annotbordercharacteristics_type;
use crate::pdf_lib::arrayof_4numberscolorannotation::arrayof_4numberscolorannotation_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::borderstyle::borderstyle_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::exdata3dmarkup::exdata3dmarkup_type;
use crate::pdf_lib::exdatamarkupgeo::exdatamarkupgeo_type;
use crate::pdf_lib::exdataprojection::exdataprojection_type;
use crate::pdf_lib::field::field_type;
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::optcontentgroup::optcontentgroup_type;
use crate::pdf_lib::optcontentmembership::optcontentmembership_type;
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn annotwidget_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_46 = exdataprojection_type(tctx);
    let assignment_45 = exdatamarkupgeo_type(tctx);
    let assignment_44 = exdata3dmarkup_type(tctx);
    let assignment_43 = field_type(tctx);
    let assignment_42 = borderstyle_type(tctx);
    let assignment_41 = addactionwidgetannotation_type(tctx);
    let assignment_40 = actionrichmediaexecute_type(tctx);
    let assignment_39 = actionecmascript_type(tctx);
    let assignment_38 = actiongoto3dview_type(tctx);
    let assignment_37 = actiontransition_type(tctx);
    let assignment_36 = actionrendition_type(tctx);
    let assignment_35 = actionsetocgstate_type(tctx);
    let assignment_34 = actionimportdata_type(tctx);
    let assignment_33 = actionresetform_type(tctx);
    let assignment_32 = actionsubmitform_type(tctx);
    let assignment_31 = actionnamed_type(tctx);
    let assignment_30 = actionhide_type(tctx);
    let assignment_29 = actionmovie_type(tctx);
    let assignment_28 = actionsound_type(tctx);
    let assignment_27 = actionuri_type(tctx);
    let assignment_26 = actionthread_type(tctx);
    let assignment_25 = actionlaunch_type(tctx);
    let assignment_24 = actiongotodp_type(tctx);
    let assignment_23 = actiongotoe_type(tctx);
    let assignment_22 = actiongotor_type(tctx);
    let assignment_21 = actiongoto_type(tctx);
    let assignment_20 = appearancecharacteristics_type(tctx);
    let assignment_19 = TypeCheck::new(
        tctx,
        "lang",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_18 = TypeCheck::new(tctx, "bm", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_17 = TypeCheck::new(tctx, "ca_0", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_16 = TypeCheck::new(tctx, "ca", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_15 = filespecification_type(tctx);
    let assignment_14 = arrayoffilespecifications_type(tctx);
    let assignment_13 = optcontentmembership_type(tctx);
    let assignment_12 = optcontentgroup_type(tctx);
    let assignment_integer_11 = TypeCheck::new(
        tctx,
        "structparent",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_10 = arrayof_4numberscolorannotation_type(tctx);
    let assignment_9 = arrayof_4annotbordercharacteristics_type(tctx);
    let assignment_8 = TypeCheck::new(tctx, "as", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_7 = appearance_type(tctx);
    let assignment_integer_6 =
        TypeCheck::new(tctx, "f", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_5 = TypeCheck::new(tctx, "m", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_date_4 = mk_date_typchk(tctx);
    let assignment_3 = TypeCheck::new(tctx, "nm", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_2 = pageobject_type(tctx);
    let assignment_1 = TypeCheck::new(
        tctx,
        "contents",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_rectangle_0 = mk_rectangle_typchk(tctx);
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_4]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_0]));
    let dis_13 = TypeCheck::new(
        tctx,
        "exdata",
        Rc::new(PDFType::Disjunct(vec![
            assignment_44,
            assignment_45,
            assignment_46,
        ])),
    );
    let dis_12 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![assignment_43])),
    );
    let dis_11 = TypeCheck::new(tctx, "bs", Rc::new(PDFType::Disjunct(vec![assignment_42])));
    let dis_10 = TypeCheck::new(tctx, "aa", Rc::new(PDFType::Disjunct(vec![assignment_41])));
    let dis_9 = TypeCheck::new(
        tctx,
        "a",
        Rc::new(PDFType::Disjunct(vec![
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
            assignment_40,
        ])),
    );
    let dis_8 = TypeCheck::new(tctx, "mk", Rc::new(PDFType::Disjunct(vec![assignment_20])));
    let dis_7 = TypeCheck::new(
        tctx,
        "af",
        Rc::new(PDFType::Disjunct(vec![assignment_14, assignment_15])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "oc",
        Rc::new(PDFType::Disjunct(vec![assignment_12, assignment_13])),
    );
    let dis_5 = TypeCheck::new(tctx, "c", Rc::new(PDFType::Disjunct(vec![assignment_10])));
    let dis_4 = TypeCheck::new(
        tctx,
        "border",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_3 = TypeCheck::new(tctx, "ap", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_2 = TypeCheck::new(tctx, "m", assignments_disjuncts_1);
    let dis_1 = TypeCheck::new(tctx, "p", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_0 = TypeCheck::new(tctx, "rect", assignments_disjuncts_0);
    let choices_h = ChoicePred(
        String::from("Invalid H"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("N"))),
            PDFObjT::Name(NameT::new(Vec::from("I"))),
            PDFObjT::Name(NameT::new(Vec::from("O"))),
            PDFObjT::Name(NameT::new(Vec::from("P"))),
            PDFObjT::Name(NameT::new(Vec::from("T"))),
        ],
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Widget")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Annot")))],
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
        opt: DictKeySpec::Required,
    };
    let rect_field = DictEntry {
        key: Vec::from("Rect"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let contents_field = DictEntry {
        key: Vec::from("Contents"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let nm_field = DictEntry {
        key: Vec::from("NM"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let m_field = DictEntry {
        key: Vec::from("M"),
        chk: TypeCheck::new(
            tctx,
            "m",
            Rc::new(PDFType::Disjunct(vec![dis_2, assignment_5])),
        ),
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: assignment_integer_6,

        opt: DictKeySpec::Optional,
    };
    let ap_field = DictEntry {
        key: Vec::from("AP"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let as_field = DictEntry {
        key: Vec::from("AS"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    let border_field = DictEntry {
        key: Vec::from("Border"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let structparent_field = DictEntry {
        key: Vec::from("StructParent"),
        chk: assignment_integer_11,

        opt: DictKeySpec::Optional,
    };
    let oc_field = DictEntry {
        key: Vec::from("OC"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let ca_field = DictEntry {
        key: Vec::from("ca"),
        chk: assignment_16,

        opt: DictKeySpec::Optional,
    };
    let ca_0_field = DictEntry {
        key: Vec::from("CA_0"),
        chk: assignment_17,

        opt: DictKeySpec::Optional,
    };
    let bm_field = DictEntry {
        key: Vec::from("BM"),
        chk: assignment_18,

        opt: DictKeySpec::Optional,
    };
    let lang_field = DictEntry {
        key: Vec::from("Lang"),
        chk: assignment_19,

        opt: DictKeySpec::Optional,
    };
    let h_field = DictEntry {
        key: Vec::from("H"),
        chk: TypeCheck::new_refined(
            tctx,
            "h",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_h),
        ),
        opt: DictKeySpec::Optional,
    };
    let mk_field = DictEntry {
        key: Vec::from("MK"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let aa_field = DictEntry {
        key: Vec::from("AA"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let bs_field = DictEntry {
        key: Vec::from("BS"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_12,
        opt: DictKeySpec::Optional,
    };
    let exdata_field = DictEntry {
        key: Vec::from("ExData"),
        chk: dis_13,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "annotwidget",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            rect_field,
            contents_field,
            p_field,
            nm_field,
            m_field,
            f_field,
            ap_field,
            as_field,
            border_field,
            c_field,
            structparent_field,
            oc_field,
            af_field,
            ca_field,
            ca_0_field,
            bm_field,
            lang_field,
            h_field,
            mk_field,
            a_field,
            aa_field,
            bs_field,
            parent_field,
            exdata_field,
        ])),
    )
}
