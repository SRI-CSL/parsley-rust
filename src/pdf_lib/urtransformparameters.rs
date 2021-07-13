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
use crate::pdf_lib::urtransformparamdocumentarray::urtransformparamdocumentarray_type;
use crate::pdf_lib::urtransformparamannotsarray::urtransformparamannotsarray_type;
use crate::pdf_lib::urtransformparamformarray::urtransformparamformarray_type;
use crate::pdf_lib::urtransformparamsignaturearray::urtransformparamsignaturearray_type;
use crate::pdf_lib::urtransformparamefarray::urtransformparamefarray_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("TransformParams"))),
           ],
     );
pub fn urtransformparameters_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let document_field = DictEntry { 
       key: Vec::from("Document"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         u_type,
         r_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         f_type,
         o_type,
         r_type,
         m_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         d_type,
         o_type,
         c_type,
         u_type,
         m_type,
         e_type,
         n_type,
         t_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let msg_field = DictEntry { 
       key: Vec::from("Msg"), 
       opt: DictKeySpec::Optional,
    }; 
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       opt: DictKeySpec::Optional,
    }; 
    let annots_field = DictEntry { 
       key: Vec::from("Annots"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         u_type,
         r_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         f_type,
         o_type,
         r_type,
         m_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         a_type,
         n_type,
         n_type,
         o_type,
         t_type,
         s_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let form_field = DictEntry { 
       key: Vec::from("Form"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         u_type,
         r_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         f_type,
         o_type,
         r_type,
         m_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         f_type,
         o_type,
         r_type,
         m_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let signature_field = DictEntry { 
       key: Vec::from("Signature"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         u_type,
         r_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         f_type,
         o_type,
         r_type,
         m_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         s_type,
         i_type,
         g_type,
         n_type,
         a_type,
         t_type,
         u_type,
         r_type,
         e_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let ef_field = DictEntry { 
       key: Vec::from("EF"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         u_type,
         r_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         f_type,
         o_type,
         r_type,
         m_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         e_type,
         f_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let p_field = DictEntry { 
       key: Vec::from("P"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "urtransformparameters",
    Rc::new(PDFType::Dict(vec![
      type_field,
      document_field,
      msg_field,
      v_field,
      annots_field,
      form_field,
      signature_field,
      ef_field,
      p_field,
   ]))
}