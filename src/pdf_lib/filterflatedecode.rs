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
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn filterflatedecode_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_1 = TypeCheck::new(
        tctx,
        "columns",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_0 = TypeCheck::new(
        tctx,
        "colors",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let choices_earlychange = ChoicePred(
        String::from("Invalid EarlyChange"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
        ],
    );
    let choices_bitspercomponent = ChoicePred(
        String::from("Invalid BitsPerComponent"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("4"))),
            PDFObjT::Name(NameT::new(Vec::from("8"))),
            PDFObjT::Name(NameT::new(Vec::from("16"))),
        ],
    );
    let choices_predictor = ChoicePred(
        String::from("Invalid Predictor"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("10"))),
            PDFObjT::Name(NameT::new(Vec::from("11"))),
            PDFObjT::Name(NameT::new(Vec::from("12"))),
            PDFObjT::Name(NameT::new(Vec::from("13"))),
            PDFObjT::Name(NameT::new(Vec::from("14"))),
            PDFObjT::Name(NameT::new(Vec::from("15"))),
        ],
    );
    let predictor_field = DictEntry {
        key: Vec::from("Predictor"),
        chk: TypeCheck::new_refined(
            tctx,
            "predictor",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_predictor),
        ),
        opt: DictKeySpec::Optional,
    };
    let colors_field = DictEntry {
        key: Vec::from("Colors"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
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
    let columns_field = DictEntry {
        key: Vec::from("Columns"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Optional,
    };
    let earlychange_field = DictEntry {
        key: Vec::from("EarlyChange"),
        chk: TypeCheck::new_refined(
            tctx,
            "earlychange",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_earlychange),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "filterflatedecode",
        Rc::new(PDFType::Dict(vec![
            predictor_field,
            colors_field,
            bitspercomponent_field,
            columns_field,
            earlychange_field,
        ])),
    )
}
