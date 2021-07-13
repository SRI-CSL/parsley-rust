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
use crate::pdf_lib::addactioncatalog::addactioncatalog_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::arrayofoutputintents::arrayofoutputintents_type;
use crate::pdf_lib::arrayofrequirements::arrayofrequirements_type;
use crate::pdf_lib::arrayofthreads::arrayofthreads_type;
use crate::pdf_lib::collection::collection_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::dest0::dest0_type;
use crate::pdf_lib::dest1::dest1_type;
use crate::pdf_lib::dest4::dest4_type;
use crate::pdf_lib::destsmap::destsmap_type;
use crate::pdf_lib::destxyz::destxyz_type;
use crate::pdf_lib::dpartroot::dpartroot_type;
use crate::pdf_lib::dss::dss_type;
use crate::pdf_lib::extensions::extensions_type;
use crate::pdf_lib::interactiveform::interactiveform_type;
use crate::pdf_lib::legalattestation::legalattestation_type;
use crate::pdf_lib::markinfo::markinfo_type;
use crate::pdf_lib::metadata::metadata_type;
use crate::pdf_lib::name::name_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::optcontentproperties::optcontentproperties_type;
use crate::pdf_lib::outline::outline_type;
use crate::pdf_lib::pagelabel::pagelabel_type;
use crate::pdf_lib::pagepiece::pagepiece_type;
use crate::pdf_lib::pagetreenoderoot::pagetreenoderoot_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::permissions::permissions_type;
use crate::pdf_lib::structtreeroot::structtreeroot_type;
use crate::pdf_lib::uri::uri_type;
use crate::pdf_lib::viewerpreferences::viewerpreferences_type;
use crate::pdf_lib::webcaptureinfo::webcaptureinfo_type;
use std::rc::Rc;
pub fn catalog_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_50 = dpartroot_type(tctx);
    let assignment_49 = arrayoffilespecifications_type(tctx);
    let assignment_48 = dss_type(tctx);
    let assignment_bool_47 = TypeCheck::new(
        tctx,
        "needsrendering",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_46 = collection_type(tctx);
    let assignment_45 = arrayofrequirements_type(tctx);
    let assignment_44 = legalattestation_type(tctx);
    let assignment_43 = permissions_type(tctx);
    let assignment_42 = optcontentproperties_type(tctx);
    let assignment_41 = pagepiece_type(tctx);
    let assignment_40 = arrayofoutputintents_type(tctx);
    let assignment_39 = webcaptureinfo_type(tctx);
    let assignment_38 = TypeCheck::new(
        tctx,
        "lang",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_37 = markinfo_type(tctx);
    let assignment_36 = structtreeroot_type(tctx);
    let assignment_35 = metadata_type(tctx);
    let assignment_34 = interactiveform_type(tctx);
    let assignment_33 = uri_type(tctx);
    let assignment_32 = addactioncatalog_type(tctx);
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
    let assignment_7 = arrayofthreads_type(tctx);
    let assignment_6 = outline_type(tctx);
    let assignment_5 = viewerpreferences_type(tctx);
    let assignment_4 = destsmap_type(tctx);
    let assignment_3 = name_type(tctx);
    let assignment_2 = pagelabel_type(tctx);
    let assignment_1 = pagetreenoderoot_type(tctx);
    let assignment_0 = extensions_type(tctx);
    let dis_25 = TypeCheck::new(
        tctx,
        "dpartroot",
        Rc::new(PDFType::Disjunct(vec![assignment_50])),
    );
    let dis_24 = TypeCheck::new(tctx, "af", Rc::new(PDFType::Disjunct(vec![assignment_49])));
    let dis_23 = TypeCheck::new(tctx, "dss", Rc::new(PDFType::Disjunct(vec![assignment_48])));
    let dis_22 = TypeCheck::new(
        tctx,
        "collection",
        Rc::new(PDFType::Disjunct(vec![assignment_46])),
    );
    let dis_21 = TypeCheck::new(
        tctx,
        "requirements",
        Rc::new(PDFType::Disjunct(vec![assignment_45])),
    );
    let dis_20 = TypeCheck::new(
        tctx,
        "legal",
        Rc::new(PDFType::Disjunct(vec![assignment_44])),
    );
    let dis_19 = TypeCheck::new(
        tctx,
        "perms",
        Rc::new(PDFType::Disjunct(vec![assignment_43])),
    );
    let dis_18 = TypeCheck::new(
        tctx,
        "ocproperties",
        Rc::new(PDFType::Disjunct(vec![assignment_42])),
    );
    let dis_17 = TypeCheck::new(
        tctx,
        "pieceinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_41])),
    );
    let dis_16 = TypeCheck::new(
        tctx,
        "outputintents",
        Rc::new(PDFType::Disjunct(vec![assignment_40])),
    );
    let dis_15 = TypeCheck::new(
        tctx,
        "spiderinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_39])),
    );
    let dis_14 = TypeCheck::new(
        tctx,
        "markinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_37])),
    );
    let dis_13 = TypeCheck::new(
        tctx,
        "structtreeroot",
        Rc::new(PDFType::Disjunct(vec![assignment_36])),
    );
    let dis_12 = TypeCheck::new(
        tctx,
        "metadata",
        Rc::new(PDFType::Disjunct(vec![assignment_35])),
    );
    let dis_11 = TypeCheck::new(
        tctx,
        "acroform",
        Rc::new(PDFType::Disjunct(vec![assignment_34])),
    );
    let dis_10 = TypeCheck::new(tctx, "uri", Rc::new(PDFType::Disjunct(vec![assignment_33])));
    let dis_9 = TypeCheck::new(tctx, "aa", Rc::new(PDFType::Disjunct(vec![assignment_32])));
    let dis_8 = TypeCheck::new(
        tctx,
        "openaction",
        Rc::new(PDFType::Disjunct(vec![
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
    let dis_7 = TypeCheck::new(
        tctx,
        "threads",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "outlines",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "viewerpreferences",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "dests",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "names",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "pagelabels",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "pages",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "extensions",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_pagemode = ChoicePred(
        String::from("Invalid PageMode"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("UseNone"))),
            PDFObjT::Name(NameT::new(Vec::from("UseOutlines"))),
            PDFObjT::Name(NameT::new(Vec::from("UseThumbs"))),
            PDFObjT::Name(NameT::new(Vec::from("FullScreen"))),
            PDFObjT::Name(NameT::new(Vec::from("UseOC"))),
            PDFObjT::Name(NameT::new(Vec::from("UseAttachments"))),
        ],
    );
    let choices_pagelayout = ChoicePred(
        String::from("Invalid PageLayout"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("SinglePage"))),
            PDFObjT::Name(NameT::new(Vec::from("OneColumn"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoColumnLeft"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoColumnRight"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoPageLeft"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoPageRight"))),
        ],
    );
    let choices_version = ChoicePred(
        String::from("Invalid Version"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("1.0"))),
            PDFObjT::Name(NameT::new(Vec::from("1.1"))),
            PDFObjT::Name(NameT::new(Vec::from("1.2"))),
            PDFObjT::Name(NameT::new(Vec::from("1.3"))),
            PDFObjT::Name(NameT::new(Vec::from("1.4"))),
            PDFObjT::Name(NameT::new(Vec::from("1.5"))),
            PDFObjT::Name(NameT::new(Vec::from("1.6"))),
            PDFObjT::Name(NameT::new(Vec::from("1.7"))),
            PDFObjT::Name(NameT::new(Vec::from("2.0"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Catalog")))],
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
    let version_field = DictEntry {
        key: Vec::from("Version"),
        chk: TypeCheck::new_refined(
            tctx,
            "version",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_version),
        ),
        opt: DictKeySpec::Optional,
    };
    let extensions_field = DictEntry {
        key: Vec::from("Extensions"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let pages_field = DictEntry {
        key: Vec::from("Pages"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let pagelabels_field = DictEntry {
        key: Vec::from("PageLabels"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let names_field = DictEntry {
        key: Vec::from("Names"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let dests_field = DictEntry {
        key: Vec::from("Dests"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let viewerpreferences_field = DictEntry {
        key: Vec::from("ViewerPreferences"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let pagelayout_field = DictEntry {
        key: Vec::from("PageLayout"),
        chk: TypeCheck::new_refined(
            tctx,
            "pagelayout",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_pagelayout),
        ),
        opt: DictKeySpec::Optional,
    };
    let pagemode_field = DictEntry {
        key: Vec::from("PageMode"),
        chk: TypeCheck::new_refined(
            tctx,
            "pagemode",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_pagemode),
        ),
        opt: DictKeySpec::Optional,
    };
    let outlines_field = DictEntry {
        key: Vec::from("Outlines"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let threads_field = DictEntry {
        key: Vec::from("Threads"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let openaction_field = DictEntry {
        key: Vec::from("OpenAction"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let aa_field = DictEntry {
        key: Vec::from("AA"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let uri_field = DictEntry {
        key: Vec::from("URI"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let acroform_field = DictEntry {
        key: Vec::from("AcroForm"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    let metadata_field = DictEntry {
        key: Vec::from("Metadata"),
        chk: dis_12,
        opt: DictKeySpec::Optional,
    };
    let structtreeroot_field = DictEntry {
        key: Vec::from("StructTreeRoot"),
        chk: dis_13,
        opt: DictKeySpec::Optional,
    };
    let markinfo_field = DictEntry {
        key: Vec::from("MarkInfo"),
        chk: dis_14,
        opt: DictKeySpec::Optional,
    };
    let lang_field = DictEntry {
        key: Vec::from("Lang"),
        chk: assignment_38,

        opt: DictKeySpec::Optional,
    };
    let spiderinfo_field = DictEntry {
        key: Vec::from("SpiderInfo"),
        chk: dis_15,
        opt: DictKeySpec::Optional,
    };
    let outputintents_field = DictEntry {
        key: Vec::from("OutputIntents"),
        chk: dis_16,
        opt: DictKeySpec::Optional,
    };
    let pieceinfo_field = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: dis_17,
        opt: DictKeySpec::Optional,
    };
    let ocproperties_field = DictEntry {
        key: Vec::from("OCProperties"),
        chk: dis_18,
        opt: DictKeySpec::Optional,
    };
    let perms_field = DictEntry {
        key: Vec::from("Perms"),
        chk: dis_19,
        opt: DictKeySpec::Optional,
    };
    let legal_field = DictEntry {
        key: Vec::from("Legal"),
        chk: dis_20,
        opt: DictKeySpec::Optional,
    };
    let requirements_field = DictEntry {
        key: Vec::from("Requirements"),
        chk: dis_21,
        opt: DictKeySpec::Optional,
    };
    let collection_field = DictEntry {
        key: Vec::from("Collection"),
        chk: dis_22,
        opt: DictKeySpec::Optional,
    };
    let needsrendering_field = DictEntry {
        key: Vec::from("NeedsRendering"),
        chk: assignment_bool_47,

        opt: DictKeySpec::Optional,
    };
    let dss_field = DictEntry {
        key: Vec::from("DSS"),
        chk: dis_23,
        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_24,
        opt: DictKeySpec::Optional,
    };
    let dpartroot_field = DictEntry {
        key: Vec::from("DPartRoot"),
        chk: dis_25,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "catalog",
        Rc::new(PDFType::Dict(vec![
            type_field,
            version_field,
            extensions_field,
            pages_field,
            pagelabels_field,
            names_field,
            dests_field,
            viewerpreferences_field,
            pagelayout_field,
            pagemode_field,
            outlines_field,
            threads_field,
            openaction_field,
            aa_field,
            uri_field,
            acroform_field,
            metadata_field,
            structtreeroot_field,
            markinfo_field,
            lang_field,
            spiderinfo_field,
            outputintents_field,
            pieceinfo_field,
            ocproperties_field,
            perms_field,
            legal_field,
            requirements_field,
            collection_field,
            needsrendering_field,
            dss_field,
            af_field,
            dpartroot_field,
        ])),
    )
}
