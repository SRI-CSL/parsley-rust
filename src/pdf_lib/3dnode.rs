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
use crate::pdf_lib::arrayof3dtransmatrix::arrayof3dtransmatrix_type;
use crate::pdf_lib::richmediainstances::richmediainstances_type;
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::3drendermode::3drendermode_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DNode"))),
           ],
     );
pub fn 3dnode_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let n_field = DictEntry { 
       key: Vec::from("N"), 
       opt: DictKeySpec::Required,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       chk: choices_o(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       opt: DictKeySpec::Optional,
    }; 
    let m_field = DictEntry { 
       key: Vec::from("M"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         3_type,
         d_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         m_type,
         a_type,
         t_type,
         r_type,
         i_type,
         x_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let instance_field = DictEntry { 
       key: Vec::from("Instance"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         r_type,
         i_type,
         c_type,
         h_type,
         m_type,
         e_type,
         d_type,
         i_type,
         a_type,
         i_type,
         n_type,
         s_type,
         t_type,
         a_type,
         n_type,
         c_type,
         e_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let data_field = DictEntry { 
       key: Vec::from("Data"), 
       chk: Rc::new(PDFType::Disjunct(vec![
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
       opt: DictKeySpec::Optional,
    }; 
    let rm_field = DictEntry { 
       key: Vec::from("RM"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         3_type,
         d_type,
         r_type,
         e_type,
         n_type,
         d_type,
         e_type,
         r_type,
         m_type,
         o_type,
         d_type,
         e_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dnode",
    Rc::new(PDFType::Dict(vec![
      type_field,
      n_field,
      o_field,
      v_field,
      m_field,
      instance_field,
      data_field,
      rm_field,
   ]))
}