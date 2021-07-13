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
use crate::pdf_lib::arrayoffilternames::arrayoffilternames_type;
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
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn xobjectimagesoftmask_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_25 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_16 = filespecification_type(tctx);
    let assignment_15 = filtercrypt_type(tctx);
    let assignment_24 = filtercrypt_type(tctx);
    let assignment_23 = filterdctdecode_type(tctx);
    let assignment_14 = filterdctdecode_type(tctx);
    let assignment_22 = filterjbig2decode_type(tctx);
    let assignment_13 = filterjbig2decode_type(tctx);
    let assignment_12 = filterccittfaxdecode_type(tctx);
    let assignment_21 = filterccittfaxdecode_type(tctx);
    let assignment_11 = filterflatedecode_type(tctx);
    let assignment_20 = filterflatedecode_type(tctx);
    let assignment_10 = filterlzwdecode_type(tctx);
    let assignment_19 = filterlzwdecode_type(tctx);
    let assignment_18 = arrayofdecodeparams_type(tctx);
    let assignment_9 = arrayofdecodeparams_type(tctx);
    let assignment_17 = arrayoffilternames_type(tctx);
    let assignment_8 = arrayoffilternames_type(tctx);
    let assignment_integer_7 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_bool_5 = TypeCheck::new(
        tctx,
        "interpolate",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_4 = arrayofnumbersgeneral_type(tctx);
    let assignment_6 = arrayofnumbersgeneral_type(tctx);
    let assignment_bool_3 = TypeCheck::new(
        tctx,
        "imagemask",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_integer_2 = TypeCheck::new(
        tctx,
        "bitspercomponent",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
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
    let dis_6 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_18,
            assignment_19,
            assignment_20,
            assignment_21,
            assignment_22,
            assignment_23,
            assignment_24,
        ])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_17])),
    );
    let dis_4 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_16])));
    let dis_3 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_9,
            assignment_10,
            assignment_11,
            assignment_12,
            assignment_13,
            assignment_14,
            assignment_15,
        ])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "matte",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "decode",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let choices_colorspace = ChoicePred(
        String::from("Invalid ColorSpace"),
        vec![PDFObjT::Name(NameT::new(Vec::from("DeviceGray")))],
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
    let colorspace_field = DictEntry {
        key: Vec::from("ColorSpace"),
        chk: TypeCheck::new_refined(
            tctx,
            "colorspace",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_colorspace),
        ),
        opt: DictKeySpec::Required,
    };
    let bitspercomponent_field = DictEntry {
        key: Vec::from("BitsPerComponent"),
        chk: assignment_integer_2,

        opt: DictKeySpec::Required,
    };
    let imagemask_field = DictEntry {
        key: Vec::from("ImageMask"),
        chk: assignment_bool_3,

        opt: DictKeySpec::Optional,
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
    let matte_field = DictEntry {
        key: Vec::from("Matte"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_7,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let dl_field = DictEntry {
        key: Vec::from("DL"),
        chk: assignment_integer_25,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "xobjectimagesoftmask",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            width_field,
            height_field,
            colorspace_field,
            bitspercomponent_field,
            imagemask_field,
            decode_field,
            interpolate_field,
            matte_field,
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
