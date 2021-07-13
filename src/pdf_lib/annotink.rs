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
use crate::pdf_lib::annot3d::annot3d_type;
use crate::pdf_lib::annotcaret::annotcaret_type;
use crate::pdf_lib::annotcircle::annotcircle_type;
use crate::pdf_lib::annotfileattachment::annotfileattachment_type;
use crate::pdf_lib::annotfreetext::annotfreetext_type;
use crate::pdf_lib::annothighlight::annothighlight_type;
use crate::pdf_lib::annotline::annotline_type;
use crate::pdf_lib::annotlink::annotlink_type;
use crate::pdf_lib::annotmovie::annotmovie_type;
use crate::pdf_lib::annotpolygon::annotpolygon_type;
use crate::pdf_lib::annotpopup::annotpopup_type;
use crate::pdf_lib::annotprintermark::annotprintermark_type;
use crate::pdf_lib::annotprojection::annotprojection_type;
use crate::pdf_lib::annotredact::annotredact_type;
use crate::pdf_lib::annotrichmedia::annotrichmedia_type;
use crate::pdf_lib::annotscreen::annotscreen_type;
use crate::pdf_lib::annotsound::annotsound_type;
use crate::pdf_lib::annotsquare::annotsquare_type;
use crate::pdf_lib::annotsquiggly::annotsquiggly_type;
use crate::pdf_lib::annotstamp::annotstamp_type;
use crate::pdf_lib::annotstrikeout::annotstrikeout_type;
use crate::pdf_lib::annottext::annottext_type;
use crate::pdf_lib::annotunderline::annotunderline_type;
use crate::pdf_lib::annotwatermark::annotwatermark_type;
use crate::pdf_lib::annotwidget::annotwidget_type;
use crate::pdf_lib::appearance::appearance_type;
use crate::pdf_lib::arrayof_4annotbordercharacteristics::arrayof_4annotbordercharacteristics_type;
use crate::pdf_lib::arrayof_4numberscolorannotation::arrayof_4numberscolorannotation_type;
use crate::pdf_lib::arrayofarraysinklist::arrayofarraysinklist_type;
use crate::pdf_lib::arrayofarrayspaths::arrayofarrayspaths_type;
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
use crate::pdf_lib::stream::stream_type;
use std::rc::Rc;
pub fn annotink_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_57 = exdataprojection_type(tctx);
    let assignment_56 = exdatamarkupgeo_type(tctx);
    let assignment_55 = exdata3dmarkup_type(tctx);
    let assignment_54 = arrayofarrayspaths_type(tctx);
    let assignment_53 = borderstyle_type(tctx);
    let assignment_52 = arrayofarraysinklist_type(tctx);
    let assignment_51 = TypeCheck::new(tctx, "it", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_50 = TypeCheck::new(
        tctx,
        "subj",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_49 = annotrichmedia_type(tctx);
    let assignment_48 = annotprojection_type(tctx);
    let assignment_47 = annotredact_type(tctx);
    let assignment_46 = annot3d_type(tctx);
    let assignment_45 = annotwatermark_type(tctx);
    let assignment_44 = annotprintermark_type(tctx);
    let assignment_43 = annotwidget_type(tctx);
    let assignment_42 = annotscreen_type(tctx);
    let assignment_41 = annotmovie_type(tctx);
    let assignment_40 = annotsound_type(tctx);
    let assignment_39 = annotfileattachment_type(tctx);
    let assignment_37 = annotink_type(tctx);
    let assignment_36 = annotstamp_type(tctx);
    let assignment_35 = annotcaret_type(tctx);
    let assignment_34 = annotstrikeout_type(tctx);
    let assignment_33 = annotsquiggly_type(tctx);
    let assignment_32 = annotunderline_type(tctx);
    let assignment_31 = annothighlight_type(tctx);
    let assignment_30 = annotpolygon_type(tctx);
    let assignment_29 = annotcircle_type(tctx);
    let assignment_28 = annotsquare_type(tctx);
    let assignment_27 = annotline_type(tctx);
    let assignment_26 = annotfreetext_type(tctx);
    let assignment_25 = annotlink_type(tctx);
    let assignment_24 = annottext_type(tctx);
    let assignment_22 = stream_type(tctx);
    let assignment_38 = annotpopup_type(tctx);
    let assignment_21 = annotpopup_type(tctx);
    let assignment_20 = TypeCheck::new(tctx, "t", Rc::new(PDFType::PrimType(PDFPrimType::String)));
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
    let assignment_date_23 = mk_date_typchk(tctx);
    let assignment_date_4 = mk_date_typchk(tctx);
    let assignment_3 = TypeCheck::new(tctx, "nm", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_2 = pageobject_type(tctx);
    let assignment_1 = TypeCheck::new(
        tctx,
        "contents",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_rectangle_0 = mk_rectangle_typchk(tctx);
    let assignments_disjuncts_2 = Rc::new(PDFType::Disjunct(vec![assignment_date_23]));
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_4]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_0]));
    let dis_15 = TypeCheck::new(
        tctx,
        "exdata",
        Rc::new(PDFType::Disjunct(vec![
            assignment_55,
            assignment_56,
            assignment_57,
        ])),
    );
    let dis_14 = TypeCheck::new(
        tctx,
        "path",
        Rc::new(PDFType::Disjunct(vec![assignment_54])),
    );
    let dis_13 = TypeCheck::new(tctx, "bs", Rc::new(PDFType::Disjunct(vec![assignment_53])));
    let dis_12 = TypeCheck::new(
        tctx,
        "inklist",
        Rc::new(PDFType::Disjunct(vec![assignment_52])),
    );
    let dis_11 = TypeCheck::new(
        tctx,
        "irt",
        Rc::new(PDFType::Disjunct(vec![
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
            assignment_41,
            assignment_42,
            assignment_43,
            assignment_44,
            assignment_45,
            assignment_46,
            assignment_47,
            assignment_48,
            assignment_49,
        ])),
    );
    let dis_10 = TypeCheck::new(tctx, "creationdate", assignments_disjuncts_2);
    let dis_9 = TypeCheck::new(tctx, "rc", Rc::new(PDFType::Disjunct(vec![assignment_22])));
    let dis_8 = TypeCheck::new(
        tctx,
        "popup",
        Rc::new(PDFType::Disjunct(vec![assignment_21])),
    );
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
    let choices_rt = ChoicePred(
        String::from("Invalid RT"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("R"))),
            PDFObjT::Name(NameT::new(Vec::from("Group"))),
        ],
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Ink")))],
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
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: assignment_20,

        opt: DictKeySpec::Optional,
    };
    let popup_field = DictEntry {
        key: Vec::from("Popup"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let rc_field = DictEntry {
        key: Vec::from("RC"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let creationdate_field = DictEntry {
        key: Vec::from("CreationDate"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let irt_field = DictEntry {
        key: Vec::from("IRT"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    let subj_field = DictEntry {
        key: Vec::from("Subj"),
        chk: assignment_50,

        opt: DictKeySpec::Optional,
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
    let it_field = DictEntry {
        key: Vec::from("IT"),
        chk: assignment_51,

        opt: DictKeySpec::Optional,
    };
    let inklist_field = DictEntry {
        key: Vec::from("InkList"),
        chk: dis_12,
        opt: DictKeySpec::Required,
    };
    let bs_field = DictEntry {
        key: Vec::from("BS"),
        chk: dis_13,
        opt: DictKeySpec::Optional,
    };
    let path_field = DictEntry {
        key: Vec::from("Path"),
        chk: dis_14,
        opt: DictKeySpec::Optional,
    };
    let exdata_field = DictEntry {
        key: Vec::from("ExData"),
        chk: dis_15,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "annotink",
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
            t_field,
            popup_field,
            rc_field,
            creationdate_field,
            irt_field,
            subj_field,
            rt_field,
            it_field,
            inklist_field,
            bs_field,
            path_field,
            exdata_field,
        ])),
    )
}
