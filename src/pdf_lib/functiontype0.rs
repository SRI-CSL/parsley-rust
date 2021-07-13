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
use crate::pdf_lib::arrayofintegersgeneral::arrayofintegersgeneral_type;
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
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
use std::rc::Rc;
pub fn functiontype0_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_17 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_11 = filespecification_type(tctx);
    let assignment_10 = filtercrypt_type(tctx);
    let assignment_16 = filtercrypt_type(tctx);
    let assignment_9 = filterflatedecode_type(tctx);
    let assignment_15 = filterflatedecode_type(tctx);
    let assignment_14 = filterlzwdecode_type(tctx);
    let assignment_8 = filterlzwdecode_type(tctx);
    let assignment_7 = arrayofdecodeparams_type(tctx);
    let assignment_13 = arrayofdecodeparams_type(tctx);
    let assignment_12 = arrayofcompressionfilternames_type(tctx);
    let assignment_6 = arrayofcompressionfilternames_type(tctx);
    let assignment_integer_5 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_2 = arrayofintegersgeneral_type(tctx);
    let assignment_1 = arrayofnumbersgeneral_type(tctx);
    let assignment_0 = arrayofnumbersgeneral_type(tctx);
    let assignment_4 = arrayofnumbersgeneral_type(tctx);
    let assignment_3 = arrayofnumbersgeneral_type(tctx);
    let dis_9 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_13,
            assignment_14,
            assignment_15,
            assignment_16,
        ])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_12])),
    );
    let dis_7 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_11])));
    let dis_6 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_7,
            assignment_8,
            assignment_9,
            assignment_10,
        ])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "decode",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "encode",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_2 = TypeCheck::new(tctx, "size", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_1 = TypeCheck::new(
        tctx,
        "range",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "domain",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_order = ChoicePred(
        String::from("Invalid Order"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("3"))),
        ],
    );
    let choices_bitspersample = ChoicePred(
        String::from("Invalid BitsPerSample"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("4"))),
            PDFObjT::Name(NameT::new(Vec::from("8"))),
            PDFObjT::Name(NameT::new(Vec::from("12"))),
            PDFObjT::Name(NameT::new(Vec::from("16"))),
            PDFObjT::Name(NameT::new(Vec::from("24"))),
            PDFObjT::Name(NameT::new(Vec::from("32"))),
        ],
    );
    let choices_functiontype = ChoicePred(
        String::from("Invalid FunctionType"),
        vec![PDFObjT::Name(NameT::new(Vec::from("0")))],
    );
    let functiontype_field = DictEntry {
        key: Vec::from("FunctionType"),
        chk: TypeCheck::new_refined(
            tctx,
            "functiontype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_functiontype),
        ),
        opt: DictKeySpec::Required,
    };
    let domain_field = DictEntry {
        key: Vec::from("Domain"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let range_field = DictEntry {
        key: Vec::from("Range"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let size_field = DictEntry {
        key: Vec::from("Size"),
        chk: dis_2,
        opt: DictKeySpec::Required,
    };
    let bitspersample_field = DictEntry {
        key: Vec::from("BitsPerSample"),
        chk: TypeCheck::new_refined(
            tctx,
            "bitspersample",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_bitspersample),
        ),
        opt: DictKeySpec::Required,
    };
    let order_field = DictEntry {
        key: Vec::from("Order"),
        chk: TypeCheck::new_refined(
            tctx,
            "order",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_order),
        ),
        opt: DictKeySpec::Optional,
    };
    let encode_field = DictEntry {
        key: Vec::from("Encode"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let decode_field = DictEntry {
        key: Vec::from("Decode"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_5,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let dl_field = DictEntry {
        key: Vec::from("DL"),
        chk: assignment_integer_17,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "functiontype0",
        Rc::new(PDFType::Dict(vec![
            functiontype_field,
            domain_field,
            range_field,
            size_field,
            bitspersample_field,
            order_field,
            encode_field,
            decode_field,
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
