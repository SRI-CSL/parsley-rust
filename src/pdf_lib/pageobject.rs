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
use crate::pdf_lib::addactionpageobject::addactionpageobject_type;
use crate::pdf_lib::arrayofannots::arrayofannots_type;
use crate::pdf_lib::arrayofbeads::arrayofbeads_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::arrayofoutputintents::arrayofoutputintents_type;
use crate::pdf_lib::arrayofstreamsgeneral::arrayofstreamsgeneral_type;
use crate::pdf_lib::arrayofviewports::arrayofviewports_type;
use crate::pdf_lib::boxcolorinfo::boxcolorinfo_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::dpart::dpart_type;
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::group::group_type;
use crate::pdf_lib::metadata::metadata_type;
use crate::pdf_lib::navnode::navnode_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pagepiece::pagepiece_type;
use crate::pdf_lib::pagetreenode::pagetreenode_type;
use crate::pdf_lib::pagetreenoderoot::pagetreenoderoot_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::resource::resource_type;
use crate::pdf_lib::separation::separation_type;
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::thumbnail::thumbnail_type;
use crate::pdf_lib::transition::transition_type;
use std::rc::Rc;
pub fn pageobject_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_bool_34 =
        TypeCheck::new(tctx, "hid", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_33 = dpart_type(tctx);
    let assignment_32 = arrayofoutputintents_type(tctx);
    let assignment_31 = filespecification_type(tctx);
    let assignment_30 = arrayoffilespecifications_type(tctx);
    let assignment_29 = arrayofviewports_type(tctx);
    let assignment_28 = TypeCheck::new(
        tctx,
        "userunit",
        Rc::new(PDFType::PrimType(PDFPrimType::Real)),
    );
    let assignment_27 = navnode_type(tctx);
    let assignment_26 = TypeCheck::new(
        tctx,
        "templateinstantiated",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_25 = separation_type(tctx);
    let assignment_24 = TypeCheck::new(tctx, "pz", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_23 = TypeCheck::new(tctx, "id", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_integer_22 = TypeCheck::new(
        tctx,
        "structparents",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_21 = pagepiece_type(tctx);
    let assignment_20 = metadata_type(tctx);
    let assignment_19 = addactionpageobject_type(tctx);
    let assignment_18 = arrayofannots_type(tctx);
    let assignment_17 = transition_type(tctx);
    let assignment_16 = TypeCheck::new(tctx, "dur", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_15 = arrayofbeads_type(tctx);
    let assignment_14 = thumbnail_type(tctx);
    let assignment_13 = group_type(tctx);
    let assignment_integer_12 = TypeCheck::new(
        tctx,
        "rotate",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_11 = stream_type(tctx);
    let assignment_10 = arrayofstreamsgeneral_type(tctx);
    let assignment_9 = boxcolorinfo_type(tctx);
    let assignment_rectangle_8 = mk_rectangle_typchk(tctx);
    let assignment_rectangle_5 = mk_rectangle_typchk(tctx);
    let assignment_rectangle_6 = mk_rectangle_typchk(tctx);
    let assignment_rectangle_7 = mk_rectangle_typchk(tctx);
    let assignment_rectangle_4 = mk_rectangle_typchk(tctx);
    let assignment_3 = resource_type(tctx);
    let assignment_date_2 = mk_date_typchk(tctx);
    let assignment_1 = pagetreenoderoot_type(tctx);
    let assignment_0 = pagetreenode_type(tctx);
    let assignments_disjuncts_5 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_8]));
    let assignments_disjuncts_4 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_7]));
    let assignments_disjuncts_3 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_6]));
    let assignments_disjuncts_2 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_5]));
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_4]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_2]));
    let dis_23 = TypeCheck::new(
        tctx,
        "dpart",
        Rc::new(PDFType::Disjunct(vec![assignment_33])),
    );
    let dis_22 = TypeCheck::new(
        tctx,
        "outputintents",
        Rc::new(PDFType::Disjunct(vec![assignment_32])),
    );
    let dis_21 = TypeCheck::new(
        tctx,
        "af",
        Rc::new(PDFType::Disjunct(vec![assignment_30, assignment_31])),
    );
    let dis_20 = TypeCheck::new(tctx, "vp", Rc::new(PDFType::Disjunct(vec![assignment_29])));
    let dis_19 = TypeCheck::new(
        tctx,
        "pressteps",
        Rc::new(PDFType::Disjunct(vec![assignment_27])),
    );
    let dis_18 = TypeCheck::new(
        tctx,
        "separationinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_25])),
    );
    let dis_17 = TypeCheck::new(
        tctx,
        "pieceinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_21])),
    );
    let dis_16 = TypeCheck::new(
        tctx,
        "metadata",
        Rc::new(PDFType::Disjunct(vec![assignment_20])),
    );
    let dis_15 = TypeCheck::new(tctx, "aa", Rc::new(PDFType::Disjunct(vec![assignment_19])));
    let dis_14 = TypeCheck::new(
        tctx,
        "annots",
        Rc::new(PDFType::Disjunct(vec![assignment_18])),
    );
    let dis_13 = TypeCheck::new(
        tctx,
        "trans",
        Rc::new(PDFType::Disjunct(vec![assignment_17])),
    );
    let dis_12 = TypeCheck::new(tctx, "b", Rc::new(PDFType::Disjunct(vec![assignment_15])));
    let dis_11 = TypeCheck::new(
        tctx,
        "thumb",
        Rc::new(PDFType::Disjunct(vec![assignment_14])),
    );
    let dis_10 = TypeCheck::new(
        tctx,
        "group",
        Rc::new(PDFType::Disjunct(vec![assignment_13])),
    );
    let dis_9 = TypeCheck::new(
        tctx,
        "contents",
        Rc::new(PDFType::Disjunct(vec![assignment_10, assignment_11])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "boxcolorinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_7 = TypeCheck::new(tctx, "artbox", assignments_disjuncts_5);
    let dis_6 = TypeCheck::new(tctx, "trimbox", assignments_disjuncts_4);
    let dis_5 = TypeCheck::new(tctx, "bleedbox", assignments_disjuncts_3);
    let dis_4 = TypeCheck::new(tctx, "cropbox", assignments_disjuncts_2);
    let dis_3 = TypeCheck::new(tctx, "mediabox", assignments_disjuncts_1);
    let dis_2 = TypeCheck::new(
        tctx,
        "resources",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_1 = TypeCheck::new(tctx, "lastmodified", assignments_disjuncts_0);
    let dis_0 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![assignment_0, assignment_1])),
    );
    let choices_tabs = ChoicePred(
        String::from("Invalid Tabs"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("R"))),
            PDFObjT::Name(NameT::new(Vec::from("C"))),
            PDFObjT::Name(NameT::new(Vec::from("S"))),
            PDFObjT::Name(NameT::new(Vec::from("A"))),
            PDFObjT::Name(NameT::new(Vec::from("W"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Page"))),
            PDFObjT::Name(NameT::new(Vec::from("Template"))),
        ],
    );
    let type_field = DictEntry {
        key: Vec::from("Type"),
        chk: TypeCheck::new_refined(
            tctx,
            "type",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_type),
        ),
        opt: DictKeySpec::Required,
    };
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let lastmodified_field = DictEntry {
        key: Vec::from("LastModified"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let resources_field = DictEntry {
        key: Vec::from("Resources"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let mediabox_field = DictEntry {
        key: Vec::from("MediaBox"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let cropbox_field = DictEntry {
        key: Vec::from("CropBox"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let bleedbox_field = DictEntry {
        key: Vec::from("BleedBox"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let trimbox_field = DictEntry {
        key: Vec::from("TrimBox"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let artbox_field = DictEntry {
        key: Vec::from("ArtBox"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let boxcolorinfo_field = DictEntry {
        key: Vec::from("BoxColorInfo"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let contents_field = DictEntry {
        key: Vec::from("Contents"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let rotate_field = DictEntry {
        key: Vec::from("Rotate"),
        chk: assignment_integer_12,

        opt: DictKeySpec::Optional,
    };
    let group_field = DictEntry {
        key: Vec::from("Group"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let thumb_field = DictEntry {
        key: Vec::from("Thumb"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    let b_field = DictEntry {
        key: Vec::from("B"),
        chk: dis_12,
        opt: DictKeySpec::Optional,
    };
    let dur_field = DictEntry {
        key: Vec::from("Dur"),
        chk: assignment_16,

        opt: DictKeySpec::Optional,
    };
    let trans_field = DictEntry {
        key: Vec::from("Trans"),
        chk: dis_13,
        opt: DictKeySpec::Optional,
    };
    let annots_field = DictEntry {
        key: Vec::from("Annots"),
        chk: dis_14,
        opt: DictKeySpec::Optional,
    };
    let aa_field = DictEntry {
        key: Vec::from("AA"),
        chk: dis_15,
        opt: DictKeySpec::Optional,
    };
    let metadata_field = DictEntry {
        key: Vec::from("Metadata"),
        chk: dis_16,
        opt: DictKeySpec::Optional,
    };
    let pieceinfo_field = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: dis_17,
        opt: DictKeySpec::Optional,
    };
    let structparents_field = DictEntry {
        key: Vec::from("StructParents"),
        chk: assignment_integer_22,

        opt: DictKeySpec::Optional,
    };
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: assignment_23,

        opt: DictKeySpec::Optional,
    };
    let pz_field = DictEntry {
        key: Vec::from("PZ"),
        chk: assignment_24,

        opt: DictKeySpec::Optional,
    };
    let separationinfo_field = DictEntry {
        key: Vec::from("SeparationInfo"),
        chk: dis_18,
        opt: DictKeySpec::Optional,
    };
    let tabs_field = DictEntry {
        key: Vec::from("Tabs"),
        chk: TypeCheck::new_refined(
            tctx,
            "tabs",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_tabs),
        ),
        opt: DictKeySpec::Optional,
    };
    let templateinstantiated_field = DictEntry {
        key: Vec::from("TemplateInstantiated"),
        chk: assignment_26,

        opt: DictKeySpec::Optional,
    };
    let pressteps_field = DictEntry {
        key: Vec::from("PresSteps"),
        chk: dis_19,
        opt: DictKeySpec::Optional,
    };
    let userunit_field = DictEntry {
        key: Vec::from("UserUnit"),
        chk: assignment_28,

        opt: DictKeySpec::Optional,
    };
    let vp_field = DictEntry {
        key: Vec::from("VP"),
        chk: dis_20,
        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_21,
        opt: DictKeySpec::Optional,
    };
    let outputintents_field = DictEntry {
        key: Vec::from("OutputIntents"),
        chk: dis_22,
        opt: DictKeySpec::Optional,
    };
    let dpart_field = DictEntry {
        key: Vec::from("DPart"),
        chk: dis_23,
        opt: DictKeySpec::Optional,
    };
    let hid_field = DictEntry {
        key: Vec::from("Hid"),
        chk: assignment_bool_34,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "pageobject",
        Rc::new(PDFType::Dict(vec![
            type_field,
            parent_field,
            lastmodified_field,
            resources_field,
            mediabox_field,
            cropbox_field,
            bleedbox_field,
            trimbox_field,
            artbox_field,
            boxcolorinfo_field,
            contents_field,
            rotate_field,
            group_field,
            thumb_field,
            b_field,
            dur_field,
            trans_field,
            annots_field,
            aa_field,
            metadata_field,
            pieceinfo_field,
            structparents_field,
            id_field,
            pz_field,
            separationinfo_field,
            tabs_field,
            templateinstantiated_field,
            pressteps_field,
            userunit_field,
            vp_field,
            af_field,
            outputintents_field,
            dpart_field,
            hid_field,
        ])),
    )
}
