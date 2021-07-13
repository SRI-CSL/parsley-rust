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
use crate::pdf_lib::dest0::dest0_type;
use crate::pdf_lib::dest0struct::dest0struct_type;
use crate::pdf_lib::dest1::dest1_type;
use crate::pdf_lib::dest1struct::dest1struct_type;
use crate::pdf_lib::dest4::dest4_type;
use crate::pdf_lib::dest4struct::dest4struct_type;
use crate::pdf_lib::destxyz::destxyz_type;
use crate::pdf_lib::destxyzstruct::destxyzstruct_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn destdict_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = dest4struct_type(tctx);
    let assignment_6 = dest1struct_type(tctx);
    let assignment_5 = dest0struct_type(tctx);
    let assignment_4 = destxyzstruct_type(tctx);
    let assignment_3 = dest4_type(tctx);
    let assignment_2 = dest1_type(tctx);
    let assignment_1 = dest0_type(tctx);
    let assignment_0 = destxyz_type(tctx);
    let dis_1 = TypeCheck::new(
        tctx,
        "sd",
        Rc::new(PDFType::Disjunct(vec![
            assignment_4,
            assignment_5,
            assignment_6,
            assignment_7,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "d",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
        ])),
    );
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let sd_field = DictEntry {
        key: Vec::from("SD"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "destdict",
        Rc::new(PDFType::Dict(vec![d_field, sd_field])),
    )
}
