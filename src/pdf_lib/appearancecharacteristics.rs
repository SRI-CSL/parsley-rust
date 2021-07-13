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
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::iconfit::iconfit_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::xobjectformtype1::xobjectformtype1_type;
use crate::pdf_lib::xobjectimage::xobjectimage_type;
use std::rc::Rc;
pub fn appearancecharacteristics_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_12 = iconfit_type(tctx);
    let assignment_11 = xobjectimage_type(tctx);
    let assignment_7 = xobjectimage_type(tctx);
    let assignment_9 = xobjectimage_type(tctx);
    let assignment_10 = xobjectformtype1_type(tctx);
    let assignment_8 = xobjectformtype1_type(tctx);
    let assignment_6 = xobjectformtype1_type(tctx);
    let assignment_5 = TypeCheck::new(tctx, "ac", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_4 = TypeCheck::new(tctx, "rc", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_3 = TypeCheck::new(tctx, "ca", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_1 = arrayofnumbersgeneral_type(tctx);
    let assignment_2 = arrayofnumbersgeneral_type(tctx);
    let assignment_integer_0 =
        TypeCheck::new(tctx, "r", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let dis_5 = TypeCheck::new(tctx, "if", Rc::new(PDFType::Disjunct(vec![assignment_12])));
    let dis_4 = TypeCheck::new(
        tctx,
        "ix",
        Rc::new(PDFType::Disjunct(vec![assignment_10, assignment_11])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "ri",
        Rc::new(PDFType::Disjunct(vec![assignment_8, assignment_9])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "i",
        Rc::new(PDFType::Disjunct(vec![assignment_6, assignment_7])),
    );
    let dis_1 = TypeCheck::new(tctx, "bg", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_0 = TypeCheck::new(tctx, "bc", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let choices_tp = ChoicePred(
        String::from("Invalid TP"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
            PDFObjT::Name(NameT::new(Vec::from("3"))),
            PDFObjT::Name(NameT::new(Vec::from("4"))),
            PDFObjT::Name(NameT::new(Vec::from("5"))),
            PDFObjT::Name(NameT::new(Vec::from("6"))),
        ],
    );
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let bc_field = DictEntry {
        key: Vec::from("BC"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let bg_field = DictEntry {
        key: Vec::from("BG"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let ca_field = DictEntry {
        key: Vec::from("CA"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let rc_field = DictEntry {
        key: Vec::from("RC"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let ac_field = DictEntry {
        key: Vec::from("AC"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let i_field = DictEntry {
        key: Vec::from("I"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let ri_field = DictEntry {
        key: Vec::from("RI"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let ix_field = DictEntry {
        key: Vec::from("IX"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let if_field = DictEntry {
        key: Vec::from("IF"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let tp_field = DictEntry {
        key: Vec::from("TP"),
        chk: TypeCheck::new_refined(
            tctx,
            "tp",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_tp),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "appearancecharacteristics",
        Rc::new(PDFType::Dict(vec![
            r_field, bc_field, bg_field, ca_field, rc_field, ac_field, i_field, ri_field, ix_field,
            if_field, tp_field,
        ])),
    )
}
