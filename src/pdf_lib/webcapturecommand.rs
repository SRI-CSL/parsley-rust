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
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::webcapturecommandsettings::webcapturecommandsettings_type;
use std::rc::Rc;
pub fn webcapturecommand_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_6 = webcapturecommandsettings_type(tctx);
    let assignment_5 = TypeCheck::new(tctx, "h", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_4 = TypeCheck::new(tctx, "ct", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_3 = stream_type(tctx);
    let assignment_integer_2 =
        TypeCheck::new(tctx, "f", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_integer_1 =
        TypeCheck::new(tctx, "l", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_0 = TypeCheck::new(tctx, "url", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let dis_1 = TypeCheck::new(tctx, "s", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_0 = TypeCheck::new(tctx, "p", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let url_field = DictEntry {
        key: Vec::from("URL"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let l_field = DictEntry {
        key: Vec::from("L"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: assignment_integer_2,

        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let ct_field = DictEntry {
        key: Vec::from("CT"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let h_field = DictEntry {
        key: Vec::from("H"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "webcapturecommand",
        Rc::new(PDFType::Dict(vec![
            url_field, l_field, f_field, p_field, ct_field, h_field, s_field,
        ])),
    )
}
