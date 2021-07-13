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
use crate::pdf_lib::appearancesubdict::appearancesubdict_type;
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
use crate::pdf_lib::xobjectformps::xobjectformps_type;
use crate::pdf_lib::xobjectformpspassthrough::xobjectformpspassthrough_type;
use crate::pdf_lib::xobjectformtype1::xobjectformtype1_type;
use std::rc::Rc;
pub fn appearance_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_11 = xobjectformpspassthrough_type(tctx);
    let assignment_7 = xobjectformpspassthrough_type(tctx);
    let assignment_3 = xobjectformpspassthrough_type(tctx);
    let assignment_10 = xobjectformps_type(tctx);
    let assignment_2 = xobjectformps_type(tctx);
    let assignment_6 = xobjectformps_type(tctx);
    let assignment_1 = xobjectformtype1_type(tctx);
    let assignment_5 = xobjectformtype1_type(tctx);
    let assignment_9 = xobjectformtype1_type(tctx);
    let assignment_0 = appearancesubdict_type(tctx);
    let assignment_4 = appearancesubdict_type(tctx);
    let assignment_8 = appearancesubdict_type(tctx);
    let dis_2 = TypeCheck::new(
        tctx,
        "d",
        Rc::new(PDFType::Disjunct(vec![
            assignment_8,
            assignment_9,
            assignment_10,
            assignment_11,
        ])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "r",
        Rc::new(PDFType::Disjunct(vec![
            assignment_4,
            assignment_5,
            assignment_6,
            assignment_7,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "n",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
        ])),
    );
    let n_field = DictEntry {
        key: Vec::from("N"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "appearance",
        Rc::new(PDFType::Dict(vec![n_field, r_field, d_field])),
    )
}
