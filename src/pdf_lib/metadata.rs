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
use std::rc::Rc;
pub fn metadata_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_12 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_6 = filespecification_type(tctx);
    let assignment_11 = filtercrypt_type(tctx);
    let assignment_5 = filtercrypt_type(tctx);
    let assignment_10 = filterflatedecode_type(tctx);
    let assignment_4 = filterflatedecode_type(tctx);
    let assignment_9 = filterlzwdecode_type(tctx);
    let assignment_3 = filterlzwdecode_type(tctx);
    let assignment_2 = arrayofdecodeparams_type(tctx);
    let assignment_8 = arrayofdecodeparams_type(tctx);
    let assignment_1 = arrayofcompressionfilternames_type(tctx);
    let assignment_7 = arrayofcompressionfilternames_type(tctx);
    let assignment_integer_0 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_8,
            assignment_9,
            assignment_10,
            assignment_11,
        ])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_2 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_1 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_2,
            assignment_3,
            assignment_4,
            assignment_5,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("XML")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Metadata")))],
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
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let dl_field = DictEntry {
        key: Vec::from("DL"),
        chk: assignment_integer_12,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "metadata",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
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
