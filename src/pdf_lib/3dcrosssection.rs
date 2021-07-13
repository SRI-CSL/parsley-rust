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
use crate::pdf_lib::arrayof_3centerofrotationnumbers::arrayof_3centerofrotationnumbers_type;
use crate::pdf_lib::arrayof_3orientationnumbers::arrayof_3orientationnumbers_type;
use crate::pdf_lib::arrayof_4colourspaceentries::arrayof_4colourspaceentries_type;
use crate::pdf_lib::arrayof_4colourspaceentries::arrayof_4colourspaceentries_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DCrossSection"))),
           ],
     );
pub fn 3dcrosssection_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let c_field = DictEntry { 
       key: Vec::from("C"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         3_type,
         c_type,
         e_type,
         n_type,
         t_type,
         e_type,
         r_type,
         o_type,
         f_type,
         r_type,
         o_type,
         t_type,
         a_type,
         t_type,
         i_type,
         o_type,
         n_type,
         n_type,
         u_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         3_type,
         o_type,
         r_type,
         i_type,
         e_type,
         n_type,
         t_type,
         a_type,
         t_type,
         i_type,
         o_type,
         n_type,
         n_type,
         u_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let po_field = DictEntry { 
       key: Vec::from("PO"), 
       opt: DictKeySpec::Optional,
    }; 
    let pc_field = DictEntry { 
       key: Vec::from("PC"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         4_type,
         c_type,
         o_type,
         l_type,
         o_type,
         u_type,
         r_type,
         s_type,
         p_type,
         a_type,
         c_type,
         e_type,
         e_type,
         n_type,
         t_type,
         r_type,
         i_type,
         e_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let iv_field = DictEntry { 
       key: Vec::from("IV"), 
       opt: DictKeySpec::Optional,
    }; 
    let ic_field = DictEntry { 
       key: Vec::from("IC"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         4_type,
         c_type,
         o_type,
         l_type,
         o_type,
         u_type,
         r_type,
         s_type,
         p_type,
         a_type,
         c_type,
         e_type,
         e_type,
         n_type,
         t_type,
         r_type,
         i_type,
         e_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let st_field = DictEntry { 
       key: Vec::from("ST"), 
       opt: DictKeySpec::Optional,
    }; 
    let sc_field = DictEntry { 
       key: Vec::from("SC"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dcrosssection",
    Rc::new(PDFType::Dict(vec![
      type_field,
      c_field,
      o_field,
      po_field,
      pc_field,
      iv_field,
      ic_field,
      st_field,
      sc_field,
   ]))
}