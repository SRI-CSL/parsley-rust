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
use crate::pdf_lib::cidfontdescriptormetrics::cidfontdescriptormetrics_type;
pub fn fddict_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let *_field = DictEntry { 
       key: Vec::from("*"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         c_type,
         i_type,
         d_type,
         f_type,
         o_type,
         n_type,
         t_type,
         d_type,
         e_type,
         s_type,
         c_type,
         r_type,
         i_type,
         p_type,
         t_type,
         o_type,
         r_type,
         m_type,
         e_type,
         t_type,
         r_type,
         i_type,
         c_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "fddict",
    Rc::new(PDFType::Dict(vec![
      *_field,
   ]))
}