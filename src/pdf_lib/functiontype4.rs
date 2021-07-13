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
pub fn functiontype4_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_14 =
        TypeCheck::new(tctx, "dl", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_8 = filespecification_type(tctx);
    let assignment_7 = filtercrypt_type(tctx);
    let assignment_13 = filtercrypt_type(tctx);
    let assignment_12 = filterflatedecode_type(tctx);
    let assignment_6 = filterflatedecode_type(tctx);
    let assignment_11 = filterlzwdecode_type(tctx);
    let assignment_5 = filterlzwdecode_type(tctx);
    let assignment_10 = arrayofdecodeparams_type(tctx);
    let assignment_4 = arrayofdecodeparams_type(tctx);
    let assignment_9 = arrayofcompressionfilternames_type(tctx);
    let assignment_3 = arrayofcompressionfilternames_type(tctx);
    let assignment_integer_2 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_1 = arrayofnumbersgeneral_type(tctx);
    let assignment_0 = arrayofnumbersgeneral_type(tctx);
    let dis_6 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_10,
            assignment_11,
            assignment_12,
            assignment_13,
        ])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_4 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_8])));
    let dis_3 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_4,
            assignment_5,
            assignment_6,
            assignment_7,
        ])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
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
    let choices_functiontype = ChoicePred(
        String::from("Invalid FunctionType"),
        vec![PDFObjT::Name(NameT::new(Vec::from("4")))],
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
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_2,

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
        chk: assignment_integer_14,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "functiontype4",
        Rc::new(PDFType::Dict(vec![
            functiontype_field,
            domain_field,
            range_field,
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
