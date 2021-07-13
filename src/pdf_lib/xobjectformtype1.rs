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
use crate::pdf_lib::arrayofcompressionfilternames::arrayofcompressionfilternames_type;
use crate::pdf_lib::arrayofdecodeparams::arrayofdecodeparams_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::arrayofintegersgeneral::arrayofintegersgeneral_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::filtercrypt::filtercrypt_type;
use crate::pdf_lib::filterflatedecode::filterflatedecode_type;
use crate::pdf_lib::filterlzwdecode::filterlzwdecode_type;
use crate::pdf_lib::group::group_type;
use crate::pdf_lib::matrix::matrix_type;
use crate::pdf_lib::measuregeo::measuregeo_type;
use crate::pdf_lib::measurerl::measurerl_type;
use crate::pdf_lib::metadata::metadata_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::opiversion13::opiversion13_type;
use crate::pdf_lib::opiversion20::opiversion20_type;
use crate::pdf_lib::optcontentgroup::optcontentgroup_type;
use crate::pdf_lib::optcontentmembership::optcontentmembership_type;
use crate::pdf_lib::pagepiece::pagepiece_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::pointdata::pointdata_type;
use crate::pdf_lib::reference::reference_type;
use crate::pdf_lib::resource::resource_type;
use std::rc::Rc;
pub fn xobjectformtype1_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_33 = arrayofintegersgeneral_type(tctx);
    let assignment_integer_32 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_25 = filtercrypt_type(tctx);
    let assignment_31 = filtercrypt_type(tctx);
    let assignment_30 = filterflatedecode_type(tctx);
    let assignment_24 = filterflatedecode_type(tctx);
    let assignment_23 = filterlzwdecode_type(tctx);
    let assignment_29 = filterlzwdecode_type(tctx);
    let assignment_22 = arrayofdecodeparams_type(tctx);
    let assignment_28 = arrayofdecodeparams_type(tctx);
    let assignment_21 = arrayofcompressionfilternames_type(tctx);
    let assignment_27 = arrayofcompressionfilternames_type(tctx);
    let assignment_integer_20 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_19 = pointdata_type(tctx);
    let assignment_18 = measuregeo_type(tctx);
    let assignment_17 = measurerl_type(tctx);
    let assignment_26 = filespecification_type(tctx);
    let assignment_16 = filespecification_type(tctx);
    let assignment_15 = arrayoffilespecifications_type(tctx);
    let assignment_14 = TypeCheck::new(tctx, "name", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_13 = optcontentmembership_type(tctx);
    let assignment_12 = optcontentgroup_type(tctx);
    let assignment_11 = opiversion20_type(tctx);
    let assignment_10 = opiversion13_type(tctx);
    let assignment_integer_9 = TypeCheck::new(
        tctx,
        "structparents",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_8 = TypeCheck::new(
        tctx,
        "structparent",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_date_7 = mk_date_typchk(tctx);
    let assignment_6 = pagepiece_type(tctx);
    let assignment_5 = metadata_type(tctx);
    let assignment_4 = reference_type(tctx);
    let assignment_3 = group_type(tctx);
    let assignment_2 = resource_type(tctx);
    let assignment_1 = matrix_type(tctx);
    let assignment_rectangle_0 = mk_rectangle_typchk(tctx);
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_7]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_0]));
    let dis_18 = TypeCheck::new(
        tctx,
        "xuid",
        Rc::new(PDFType::Disjunct(vec![assignment_33])),
    );
    let dis_17 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_28,
            assignment_29,
            assignment_30,
            assignment_31,
        ])),
    );
    let dis_16 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_27])),
    );
    let dis_15 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_26])));
    let dis_14 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_22,
            assignment_23,
            assignment_24,
            assignment_25,
        ])),
    );
    let dis_13 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_21])),
    );
    let dis_12 = TypeCheck::new(
        tctx,
        "ptdata",
        Rc::new(PDFType::Disjunct(vec![assignment_19])),
    );
    let dis_11 = TypeCheck::new(
        tctx,
        "measure",
        Rc::new(PDFType::Disjunct(vec![assignment_17, assignment_18])),
    );
    let dis_10 = TypeCheck::new(
        tctx,
        "af",
        Rc::new(PDFType::Disjunct(vec![assignment_15, assignment_16])),
    );
    let dis_9 = TypeCheck::new(
        tctx,
        "oc",
        Rc::new(PDFType::Disjunct(vec![assignment_12, assignment_13])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "opi",
        Rc::new(PDFType::Disjunct(vec![assignment_10, assignment_11])),
    );
    let dis_7 = TypeCheck::new(tctx, "lastmodified", assignments_disjuncts_1);
    let dis_6 = TypeCheck::new(
        tctx,
        "pieceinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "metadata",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_4 = TypeCheck::new(tctx, "ref", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_3 = TypeCheck::new(
        tctx,
        "group",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "resources",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "matrix",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(tctx, "bbox", assignments_disjuncts_0);
    let choices_formtype = ChoicePred(
        String::from("Invalid FormType"),
        vec![PDFObjT::Name(NameT::new(Vec::from("1")))],
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Form")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("XObject")))],
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
    let formtype_field = DictEntry {
        key: Vec::from("FormType"),
        chk: TypeCheck::new_refined(
            tctx,
            "formtype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_formtype),
        ),
        opt: DictKeySpec::Optional,
    };
    let bbox_field = DictEntry {
        key: Vec::from("BBox"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let matrix_field = DictEntry {
        key: Vec::from("Matrix"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let resources_field = DictEntry {
        key: Vec::from("Resources"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let group_field = DictEntry {
        key: Vec::from("Group"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let ref_field = DictEntry {
        key: Vec::from("Ref"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let metadata_field = DictEntry {
        key: Vec::from("Metadata"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let pieceinfo_field = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let lastmodified_field = DictEntry {
        key: Vec::from("LastModified"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let structparent_field = DictEntry {
        key: Vec::from("StructParent"),
        chk: assignment_integer_8,

        opt: DictKeySpec::Optional,
    };
    let structparents_field = DictEntry {
        key: Vec::from("StructParents"),
        chk: assignment_integer_9,

        opt: DictKeySpec::Optional,
    };
    let opi_field = DictEntry {
        key: Vec::from("OPI"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let oc_field = DictEntry {
        key: Vec::from("OC"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let name_field = DictEntry {
        key: Vec::from("Name"),
        chk: assignment_14,

        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let measure_field = DictEntry {
        key: Vec::from("Measure"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    let ptdata_field = DictEntry {
        key: Vec::from("PtData"),
        chk: dis_12,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_20,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_13,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_14,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_15,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_16,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_17,
        opt: DictKeySpec::Optional,
    };
    let dl_field = DictEntry {
        key: Vec::from("DL"),
        chk: assignment_integer_32,

        opt: DictKeySpec::Optional,
    };
    let xuid_field = DictEntry {
        key: Vec::from("XUID"),
        chk: dis_18,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "xobjectformtype1",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            formtype_field,
            bbox_field,
            matrix_field,
            resources_field,
            group_field,
            ref_field,
            metadata_field,
            pieceinfo_field,
            lastmodified_field,
            structparent_field,
            structparents_field,
            opi_field,
            oc_field,
            name_field,
            af_field,
            measure_field,
            ptdata_field,
            length_field,
            filter_field,
            decodeparms_field,
            f_field,
            ffilter_field,
            fdecodeparms_field,
            dl_field,
            xuid_field,
        ])),
    )
}
