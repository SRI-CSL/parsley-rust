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
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::filtercrypt::filtercrypt_type;
use crate::pdf_lib::filterflatedecode::filterflatedecode_type;
use crate::pdf_lib::filterlzwdecode::filterlzwdecode_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::universalarray::universalarray_type;
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use std::rc::Rc;
pub fn soundobject_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_19 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_13 = filespecification_type(tctx);
    let assignment_18 = filtercrypt_type(tctx);
    let assignment_12 = filtercrypt_type(tctx);
    let assignment_11 = filterflatedecode_type(tctx);
    let assignment_17 = filterflatedecode_type(tctx);
    let assignment_10 = filterlzwdecode_type(tctx);
    let assignment_16 = filterlzwdecode_type(tctx);
    let assignment_9 = arrayofdecodeparams_type(tctx);
    let assignment_15 = arrayofdecodeparams_type(tctx);
    let assignment_14 = arrayofcompressionfilternames_type(tctx);
    let assignment_8 = arrayofcompressionfilternames_type(tctx);
    let assignment_integer_7 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_6 = stream_type(tctx);
    let assignment_5 = universaldictionary_type(tctx);
    let assignment_4 = universalarray_type(tctx);
    let assignment_3 = TypeCheck::new(tctx, "co", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_integer_2 =
        TypeCheck::new(tctx, "b", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_integer_1 =
        TypeCheck::new(tctx, "c", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_0 = TypeCheck::new(tctx, "r", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let dis_5 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_15,
            assignment_16,
            assignment_17,
            assignment_18,
        ])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_14])),
    );
    let dis_3 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_13])));
    let dis_2 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_9,
            assignment_10,
            assignment_11,
            assignment_12,
        ])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "cp",
        Rc::new(PDFType::Disjunct(vec![
            assignment_4,
            assignment_5,
            assignment_6,
        ])),
    );
    let choices_e = ChoicePred(
        String::from("Invalid E"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Raw"))),
            PDFObjT::Name(NameT::new(Vec::from("Signed"))),
            PDFObjT::Name(NameT::new(Vec::from("muLaw"))),
            PDFObjT::Name(NameT::new(Vec::from("ALaw"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Sound")))],
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
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Optional,
    };
    let b_field = DictEntry {
        key: Vec::from("B"),
        chk: assignment_integer_2,

        opt: DictKeySpec::Optional,
    };
    let e_field = DictEntry {
        key: Vec::from("E"),
        chk: TypeCheck::new_refined(
            tctx,
            "e",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_e),
        ),
        opt: DictKeySpec::Optional,
    };
    let co_field = DictEntry {
        key: Vec::from("CO"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let cp_field = DictEntry {
        key: Vec::from("CP"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_7,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let dl_field = DictEntry {
        key: Vec::from("DL"),
        chk: assignment_integer_19,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "soundobject",
        Rc::new(PDFType::Dict(vec![
            type_field,
            r_field,
            c_field,
            b_field,
            e_field,
            co_field,
            cp_field,
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
