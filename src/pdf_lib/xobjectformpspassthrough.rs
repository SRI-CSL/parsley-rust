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
use crate::pdf_lib::matrix::matrix_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::stream::stream_type;
use std::rc::Rc;
pub fn xobjectformpspassthrough_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_9 = filespecification_type(tctx);
    let assignment_14 = filtercrypt_type(tctx);
    let assignment_8 = filtercrypt_type(tctx);
    let assignment_7 = filterflatedecode_type(tctx);
    let assignment_13 = filterflatedecode_type(tctx);
    let assignment_12 = filterlzwdecode_type(tctx);
    let assignment_6 = filterlzwdecode_type(tctx);
    let assignment_11 = arrayofdecodeparams_type(tctx);
    let assignment_5 = arrayofdecodeparams_type(tctx);
    let assignment_10 = arrayofcompressionfilternames_type(tctx);
    let assignment_4 = arrayofcompressionfilternames_type(tctx);
    let assignment_integer_3 = TypeCheck::new(
        tctx,
        "length",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_2 = matrix_type(tctx);
    let assignment_1 = stream_type(tctx);
    let assignment_0 = stream_type(tctx);
    let dis_7 = TypeCheck::new(
        tctx,
        "fdecodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_11,
            assignment_12,
            assignment_13,
            assignment_14,
        ])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "ffilter",
        Rc::new(PDFType::Disjunct(vec![assignment_10])),
    );
    let dis_5 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_9])));
    let dis_4 = TypeCheck::new(
        tctx,
        "decodeparms",
        Rc::new(PDFType::Disjunct(vec![
            assignment_5,
            assignment_6,
            assignment_7,
            assignment_8,
        ])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "matrix",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(tctx, "ps", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let dis_0 = TypeCheck::new(
        tctx,
        "level1",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_bbox = ChoicePred(
        String::from("Invalid BBox"),
        vec![PDFObjT::Name(NameT::new(Vec::from("0 0 0 0")))],
    );
    let choices_formtype = ChoicePred(
        String::from("Invalid FormType"),
        vec![PDFObjT::Name(NameT::new(Vec::from("1")))],
    );
    let choices_subtype2 = ChoicePred(
        String::from("Invalid Subtype2"),
        vec![PDFObjT::Name(NameT::new(Vec::from("PS")))],
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("PS")))],
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
    let subtype2_field = DictEntry {
        key: Vec::from("Subtype2"),
        chk: TypeCheck::new_refined(
            tctx,
            "subtype2",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_subtype2),
        ),
        opt: DictKeySpec::Required,
    };
    let level1_field = DictEntry {
        key: Vec::from("Level1"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let ps_field = DictEntry {
        key: Vec::from("PS"),
        chk: dis_1,
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
        chk: TypeCheck::new_refined(
            tctx,
            "bbox",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_bbox),
        ),
        opt: DictKeySpec::Required,
    };
    let matrix_field = DictEntry {
        key: Vec::from("Matrix"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let length_field = DictEntry {
        key: Vec::from("Length"),
        chk: assignment_integer_3,

        opt: DictKeySpec::Required,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let decodeparms_field = DictEntry {
        key: Vec::from("DecodeParms"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let ffilter_field = DictEntry {
        key: Vec::from("FFilter"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let fdecodeparms_field = DictEntry {
        key: Vec::from("FDecodeParms"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "xobjectformpspassthrough",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            subtype2_field,
            level1_field,
            ps_field,
            formtype_field,
            bbox_field,
            matrix_field,
            length_field,
            filter_field,
            decodeparms_field,
            f_field,
            ffilter_field,
            fdecodeparms_field,
        ])),
    )
}
