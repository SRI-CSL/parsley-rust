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
use crate::pdf_lib::arrayofnamesgeneral::arrayofnamesgeneral_type;
use crate::pdf_lib::arrayofurls::arrayofurls_type;
pub fn destoutputprofileref_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let checksum_field = DictEntry { 
       key: Vec::from("CheckSum"), 
       opt: DictKeySpec::Optional,
    }; 
    let coloranttable_field = DictEntry { 
       key: Vec::from("ColorantTable"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         n_type,
         a_type,
         m_type,
         e_type,
         s_type,
         g_type,
         e_type,
         n_type,
         e_type,
         r_type,
         a_type,
         l_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let iccversion_field = DictEntry { 
       key: Vec::from("ICCVersion"), 
       opt: DictKeySpec::Optional,
    }; 
    let profilecs_field = DictEntry { 
       key: Vec::from("ProfileCS"), 
       opt: DictKeySpec::Optional,
    }; 
    let profilename_field = DictEntry { 
       key: Vec::from("ProfileName"), 
       opt: DictKeySpec::Optional,
    }; 
    let urls_field = DictEntry { 
       key: Vec::from("URLs"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         u_type,
         r_type,
         l_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "destoutputprofileref",
    Rc::new(PDFType::Dict(vec![
      checksum_field,
      coloranttable_field,
      iccversion_field,
      profilecs_field,
      profilename_field,
      urls_field,
   ]))
}