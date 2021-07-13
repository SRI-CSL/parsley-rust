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
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::embeddedfileparameter::embeddedfileparameter_type;
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
pub fn embeddedfilestream_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_20 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_11 = filespecification_type(tctx);
    let assignment_10 = filtercrypt_type(tctx);
    let assignment_19 = filtercrypt_type(tctx);
    let assignment_18 = filterdctdecode_type(tctx);
    let assignment_9 = filterdctdecode_type(tctx);
    let assignment_17 = filterjbig2decode_type(tctx);
    let assignment_8 = filterjbig2decode_type(tctx);
    let assignment_7 = filterccittfaxdecode_type(tctx);
    let assignment_16 = filterccittfaxdecode_type(tctx);
    let assignment_15 = filterflatedecode_type(tctx);
    let assignment_6 = filterflatedecode_type(tctx);
    let assignment_5 = filterlzwdecode_type(tctx);
    let assignment_14 = filterlzwdecode_type(tctx);
    let assignment_4 = arrayofdecodeparams_type(tctx);
    let assignment_13 = arrayofdecodeparams_type(tctx);
    let assignment_12 = arrayoffilternames_type(tctx);
    let assignment_3 = arrayoffilternames_type(tctx);
    let assignment_integer_2 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_1 = embeddedfileparameter_type(tctx);
    let assignment_0 = TypeCheck::new(
        tctx,
        "subtype",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_13,
            assignment_14,
            assignment_15,
            assignment_16,
            assignment_17,
            assignment_18,
            assignment_19,
        ])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_12])),
    );
    let dis_3 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_11])));
    let dis_2 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_4,
            assignment_5,
            assignment_6,
            assignment_7,
            assignment_8,
            assignment_9,
            assignment_10,
        ])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "params",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("EmbeddedFile")))],
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
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let params_field = DictEntry {
        key: Vec::from("Params"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_2,

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
        chk: assignment_integer_20,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "embeddedfilestream",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            params_field,
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
