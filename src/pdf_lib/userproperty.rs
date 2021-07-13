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
use crate::pdf_lib::_universaldictionary::_universaldictionary_type;
use crate::pdf_lib::stream::stream_type;
pub fn userproperty_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let n_field = DictEntry { 
       key: Vec::from("N"), 
       opt: DictKeySpec::Required,
    }; 
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         ]_type,
         ;_type,
         [_type,
         __type,
         u_type,
         n_type,
         i_type,
         v_type,
         e_type,
         r_type,
         s_type,
         a_type,
         l_type,
         d_type,
         i_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         a_type,
         r_type,
         y_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
         ;_type,
         [_type,
         s_type,
         t_type,
         r_type,
         e_type,
         a_type,
         m_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let f_field = DictEntry { 
       key: Vec::from("F"), 
       opt: DictKeySpec::Optional,
    }; 
    let h_field = DictEntry { 
       key: Vec::from("H"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "userproperty",
    Rc::new(PDFType::Dict(vec![
      n_field,
      v_field,
      f_field,
      h_field,
   ]))
}