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
use crate::pdf_lib::appearance::appearance_type;
use crate::pdf_lib::arrayof_4annotbordercharacteristics::arrayof_4annotbordercharacteristics_type;
use crate::pdf_lib::arrayof_4numberscolorannotation::arrayof_4numberscolorannotation_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
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
use crate::pdf_lib::richmediacontent::richmediacontent_type;
use crate::pdf_lib::richmediasettings::richmediasettings_type;
use std::rc::Rc;
pub fn annotrichmedia_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_24 = exdataprojection_type(tctx);
    let assignment_23 = exdatamarkupgeo_type(tctx);
    let assignment_22 = exdata3dmarkup_type(tctx);
    let assignment_21 = richmediasettings_type(tctx);
    let assignment_20 = richmediacontent_type(tctx);
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
    let dis_10 = TypeCheck::new(
        tctx,
        "exdata",
        Rc::new(PDFType::Disjunct(vec![
            assignment_22,
            assignment_23,
            assignment_24,
        ])),
    );
    let dis_9 = TypeCheck::new(
        tctx,
        "richmediasettings",
        Rc::new(PDFType::Disjunct(vec![assignment_21])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "richmediacontent",
        Rc::new(PDFType::Disjunct(vec![assignment_20])),
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
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("RichMedia")))],
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
    let richmediacontent_field = DictEntry {
        key: Vec::from("RichMediaContent"),
        chk: dis_8,
        opt: DictKeySpec::Required,
    };
    let richmediasettings_field = DictEntry {
        key: Vec::from("RichMediaSettings"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let exdata_field = DictEntry {
        key: Vec::from("ExData"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "annotrichmedia",
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
            richmediacontent_field,
            richmediasettings_field,
            exdata_field,
        ])),
    )
}
