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
use crate::pdf_lib::arrayofdashpatterns::arrayofdashpatterns_type;
pub fn arrayofgspdashpatterns_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let 0_field = DictEntry { 
       key: Vec::from("0"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         d_type,
         a_type,
         s_type,
         h_type,
         p_type,
         a_type,
         t_type,
         t_type,
         e_type,
         r_type,
         n_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let 1_field = DictEntry { 
       key: Vec::from("1"), 
       opt: DictKeySpec::Required,
    }; 
    TypeCheck::new(
    tctx,
    "arrayofgspdashpatterns",
    Rc::new(PDFType::Dict(vec![
      0_field,
      1_field,
   ]))
}