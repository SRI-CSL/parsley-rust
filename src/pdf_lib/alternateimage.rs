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
use crate::pdf_lib::xobjectimage::xobjectimage_type;
use crate::pdf_lib::optcontentgroup::optcontentgroup_type;
use crate::pdf_lib::optcontentmembership::optcontentmembership_type;
pub fn alternateimage_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let image_field = DictEntry { 
       key: Vec::from("Image"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         x_type,
         o_type,
         b_type,
         j_type,
         e_type,
         c_type,
         t_type,
         i_type,
         m_type,
         a_type,
         g_type,
         e_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let defaultforprinting_field = DictEntry { 
       key: Vec::from("DefaultForPrinting"), 
       opt: DictKeySpec::Optional,
    }; 
    let oc_field = DictEntry { 
       key: Vec::from("OC"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         o_type,
         p_type,
         t_type,
         c_type,
         o_type,
         n_type,
         t_type,
         e_type,
         n_type,
         t_type,
         g_type,
         r_type,
         o_type,
         u_type,
         p_type,
         ,_type,
         o_type,
         p_type,
         t_type,
         c_type,
         o_type,
         n_type,
         t_type,
         e_type,
         n_type,
         t_type,
         m_type,
         e_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         h_type,
         i_type,
         p_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "alternateimage",
    Rc::new(PDFType::Dict(vec![
      image_field,
      defaultforprinting_field,
      oc_field,
   ]))
}