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
use crate::pdf_lib::shadingtype1::shadingtype1_type;
use crate::pdf_lib::shadingtype2::shadingtype2_type;
use crate::pdf_lib::shadingtype3::shadingtype3_type;
use crate::pdf_lib::shadingtype4::shadingtype4_type;
use crate::pdf_lib::shadingtype5::shadingtype5_type;
use crate::pdf_lib::shadingtype6::shadingtype6_type;
use crate::pdf_lib::shadingtype7::shadingtype7_type;
use crate::pdf_lib::matrix::matrix_type;
use crate::pdf_lib::graphicsstateparameter::graphicsstateparameter_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Pattern"))),
           ],
     );
    let choices_patterntype = ChoicesPred(
        String::From("Invalid PatternType"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("2"))),
           ],
     );
pub fn patterntype2_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let patterntype_field = DictEntry { 
       key: Vec::from("PatternType"), 
       chk: choices_patterntype(tctx),
       opt: DictKeySpec::Required,
    }; 
    let shading_field = DictEntry { 
       key: Vec::from("Shading"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         ,_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         2_type,
         ,_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         3_type,
         ]_type,
         ;_type,
         [_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         4_type,
         ,_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         5_type,
         ,_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         6_type,
         ,_type,
         s_type,
         h_type,
         a_type,
         d_type,
         i_type,
         n_type,
         g_type,
         t_type,
         y_type,
         p_type,
         e_type,
         7_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let matrix_field = DictEntry { 
       key: Vec::from("Matrix"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
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
    let extgstate_field = DictEntry { 
       key: Vec::from("ExtGState"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         g_type,
         r_type,
         a_type,
         p_type,
         h_type,
         i_type,
         c_type,
         s_type,
         s_type,
         t_type,
         a_type,
         t_type,
         e_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         e_type,
         t_type,
         e_type,
         r_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "patterntype2",
    Rc::new(PDFType::Dict(vec![
      type_field,
      patterntype_field,
      shading_field,
      matrix_field,
      extgstate_field,
   ]))
}