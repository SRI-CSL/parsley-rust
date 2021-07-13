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
pub fn threedunits_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_8 = TypeCheck::new(tctx, "du", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_7 = TypeCheck::new(tctx, "dsn", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_6 = TypeCheck::new(tctx, "dsm", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_5 = TypeCheck::new(tctx, "uu", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_4 = TypeCheck::new(tctx, "usn", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_3 = TypeCheck::new(tctx, "usm", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_2 = TypeCheck::new(tctx, "tu", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_1 = TypeCheck::new(tctx, "tsn", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_0 = TypeCheck::new(tctx, "tsm", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let tsm_field = DictEntry {
        key: Vec::from("TSm"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let tsn_field = DictEntry {
        key: Vec::from("TSn"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let tu_field = DictEntry {
        key: Vec::from("TU"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let usm_field = DictEntry {
        key: Vec::from("USm"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let usn_field = DictEntry {
        key: Vec::from("USn"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let uu_field = DictEntry {
        key: Vec::from("UU"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let dsm_field = DictEntry {
        key: Vec::from("DSm"),
        chk: assignment_6,

        opt: DictKeySpec::Optional,
    };
    let dsn_field = DictEntry {
        key: Vec::from("DSn"),
        chk: assignment_7,

        opt: DictKeySpec::Optional,
    };
    let du_field = DictEntry {
        key: Vec::from("DU"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "threedunits",
        Rc::new(PDFType::Dict(vec![
            tsm_field, tsn_field, tu_field, usm_field, usn_field, uu_field, dsm_field, dsn_field,
            du_field,
        ])),
    )
}
