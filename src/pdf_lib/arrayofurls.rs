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
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::embeddedfilestream::embeddedfilestream_type;
pub fn arrayofurls_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let *_field = DictEntry { 
       key: Vec::from("*"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         i_type,
         l_type,
         e_type,
         s_type,
         p_type,
         e_type,
         c_type,
         i_type,
         f_type,
         i_type,
         c_type,
         a_type,
         t_type,
         i_type,
         o_type,
         n_type,
         ]_type,
         ;_type,
         [_type,
         e_type,
         m_type,
         b_type,
         e_type,
         d_type,
         d_type,
         e_type,
         d_type,
         f_type,
         i_type,
         l_type,
         e_type,
         s_type,
         t_type,
         r_type,
         e_type,
         a_type,
         m_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    TypeCheck::new(
    tctx,
    "arrayofurls",
    Rc::new(PDFType::Dict(vec![
      *_field,
   ]))
}