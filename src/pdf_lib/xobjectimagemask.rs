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
use crate::pdf_lib::arrayofdecodeparams::arrayofdecodeparams_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::arrayoffilternames::arrayoffilternames_type;
use crate::pdf_lib::arrayofimagealternates::arrayofimagealternates_type;
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::filterccittfaxdecode::filterccittfaxdecode_type;
use crate::pdf_lib::filtercrypt::filtercrypt_type;
use crate::pdf_lib::filterdctdecode::filterdctdecode_type;
use crate::pdf_lib::filterflatedecode::filterflatedecode_type;
use crate::pdf_lib::filterjbig2decode::filterjbig2decode_type;
use crate::pdf_lib::filterlzwdecode::filterlzwdecode_type;
use crate::pdf_lib::measuregeo::measuregeo_type;
use crate::pdf_lib::measurerl::measurerl_type;
use crate::pdf_lib::metadata::metadata_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::opiversion13::opiversion13_type;
use crate::pdf_lib::opiversion20::opiversion20_type;
use crate::pdf_lib::optcontentgroup::optcontentgroup_type;
use crate::pdf_lib::optcontentmembership::optcontentmembership_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::pointdata::pointdata_type;
use crate::pdf_lib::xobjectimagesoftmask::xobjectimagesoftmask_type;
use std::rc::Rc;
pub fn xobjectimagemask_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_40 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_39 = filtercrypt_type(tctx);
    let assignment_30 = filtercrypt_type(tctx);
    let assignment_38 = filterdctdecode_type(tctx);
    let assignment_29 = filterdctdecode_type(tctx);
    let assignment_37 = filterjbig2decode_type(tctx);
    let assignment_28 = filterjbig2decode_type(tctx);
    let assignment_36 = filterccittfaxdecode_type(tctx);
    let assignment_27 = filterccittfaxdecode_type(tctx);
    let assignment_26 = filterflatedecode_type(tctx);
    let assignment_35 = filterflatedecode_type(tctx);
    let assignment_34 = filterlzwdecode_type(tctx);
    let assignment_25 = filterlzwdecode_type(tctx);
    let assignment_33 = arrayofdecodeparams_type(tctx);
    let assignment_24 = arrayofdecodeparams_type(tctx);
    let assignment_23 = arrayoffilternames_type(tctx);
    let assignment_32 = arrayoffilternames_type(tctx);
    let assignment_integer_22 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_21 = pointdata_type(tctx);
    let assignment_20 = measuregeo_type(tctx);
    let assignment_19 = measurerl_type(tctx);
    let assignment_18 = filespecification_type(tctx);
    let assignment_31 = filespecification_type(tctx);
    let assignment_17 = arrayoffilespecifications_type(tctx);
    let assignment_16 = optcontentmembership_type(tctx);
    let assignment_15 = optcontentgroup_type(tctx);
    let assignment_14 = metadata_type(tctx);
    let assignment_13 = opiversion20_type(tctx);
    let assignment_12 = opiversion13_type(tctx);
    let assignment_11 = TypeCheck::new(tctx, "id", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_integer_10 = TypeCheck::new(
        tctx,
        "structparent",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_9 = TypeCheck::new(tctx, "name", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_integer_8 = TypeCheck::new(
        tctx,
        "smaskindata",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_7 = xobjectimagesoftmask_type(tctx);
    let assignment_6 = arrayofimagealternates_type(tctx);
    let assignment_bool_5 = TypeCheck::new(
        tctx,
        "interpolate",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_4 = arrayofnumbersgeneral_type(tctx);
    let assignment_bool_3 = TypeCheck::new(
        tctx,
        "imagemask",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_2 = TypeCheck::new(
        tctx,
        "intent",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_integer_1 = TypeCheck::new(
        tctx,
        "height",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_0 = TypeCheck::new(
        tctx,
        "width",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let dis_13 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_33,
            assignment_34,
            assignment_35,
            assignment_36,
            assignment_37,
            assignment_38,
            assignment_39,
        ])),
    );
    let dis_12 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_32])),
    );
    let dis_11 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_31])));
    let dis_10 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_24,
            assignment_25,
            assignment_26,
            assignment_27,
            assignment_28,
            assignment_29,
            assignment_30,
        ])),
    );
    let dis_9 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_23])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "ptdata",
        Rc::new(PDFType::Disjunct(vec![assignment_21])),
    );
    let dis_7 = TypeCheck::new(
        tctx,
        "measure",
        Rc::new(PDFType::Disjunct(vec![assignment_19, assignment_20])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "af",
        Rc::new(PDFType::Disjunct(vec![assignment_17, assignment_18])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "oc",
        Rc::new(PDFType::Disjunct(vec![assignment_15, assignment_16])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "metadata",
        Rc::new(PDFType::Disjunct(vec![assignment_14])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "opi",
        Rc::new(PDFType::Disjunct(vec![assignment_12, assignment_13])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "smask",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "alternates",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "decode",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let choices_bitspercomponent = ChoicePred(
        String::from("Invalid BitsPerComponent"),
        vec![PDFObjT::Name(NameT::new(Vec::from("1")))],
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Image")))],
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
    let width_field = DictEntry {
        key: Vec::from("Width"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Required,
    };
    let height_field = DictEntry {
        key: Vec::from("Height"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Required,
    };
    let bitspercomponent_field = DictEntry {
        key: Vec::from("BitsPerComponent"),
        chk: TypeCheck::new_refined(
            tctx,
            "bitspercomponent",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_bitspercomponent),
        ),
        opt: DictKeySpec::Optional,
    };
    let intent_field = DictEntry {
        key: Vec::from("Intent"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let imagemask_field = DictEntry {
        key: Vec::from("ImageMask"),
        chk: assignment_bool_3,

        opt: DictKeySpec::Required,
    };
    let decode_field = DictEntry {
        key: Vec::from("Decode"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let interpolate_field = DictEntry {
        key: Vec::from("Interpolate"),
        chk: assignment_bool_5,

        opt: DictKeySpec::Optional,
    };
    let alternates_field = DictEntry {
        key: Vec::from("Alternates"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let smask_field = DictEntry {
        key: Vec::from("SMask"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let smaskindata_field = DictEntry {
        key: Vec::from("SMaskInData"),
        chk: assignment_integer_8,

        opt: DictKeySpec::Optional,
    };
    let name_field = DictEntry {
        key: Vec::from("Name"),
        chk: assignment_9,

        opt: DictKeySpec::Optional,
    };
    let structparent_field = DictEntry {
        key: Vec::from("StructParent"),
        chk: assignment_integer_10,

        opt: DictKeySpec::Optional,
    };
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: assignment_11,

        opt: DictKeySpec::Optional,
    };
    let opi_field = DictEntry {
        key: Vec::from("OPI"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let metadata_field = DictEntry {
        key: Vec::from("Metadata"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let oc_field = DictEntry {
        key: Vec::from("OC"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let measure_field = DictEntry {
        key: Vec::from("Measure"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let ptdata_field = DictEntry {
        key: Vec::from("PtData"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_22,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_12,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_13,
        opt: DictKeySpec::Optional,
    };
    let dl_field = DictEntry {
        key: Vec::from("DL"),
        chk: assignment_integer_40,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "xobjectimagemask",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            width_field,
            height_field,
            bitspercomponent_field,
            intent_field,
            imagemask_field,
            decode_field,
            interpolate_field,
            alternates_field,
            smask_field,
            smaskindata_field,
            name_field,
            structparent_field,
            id_field,
            opi_field,
            metadata_field,
            oc_field,
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
        ])),
    )
}
