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
    mk_array_of_dict_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
pub fn arrayof_3numbers_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let 0_field = DictEntry { 
       key: Vec::from("0"), 
       opt: DictKeySpec::Required,
    }; 
    let 1_field = DictEntry { 
       key: Vec::from("1"), 
       opt: DictKeySpec::Required,
    }; 
    let 2_field = DictEntry { 
       key: Vec::from("2"), 
       opt: DictKeySpec::Required,
    }; 
    TypeCheck::new(
    tctx,
    "arrayof_3numbers",
    Rc::new(PDFType::Dict(vec![
      0_field,
      1_field,
      2_field,
   ]))
}