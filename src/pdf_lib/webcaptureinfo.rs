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
use crate::pdf_lib::arrayofwebcapturecommands::arrayofwebcapturecommands_type;
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
pub fn webcaptureinfo_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = arrayofwebcapturecommands_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "v", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let dis_0 = TypeCheck::new(tctx, "c", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "webcaptureinfo",
        Rc::new(PDFType::Dict(vec![v_field, c_field])),
    )
}
