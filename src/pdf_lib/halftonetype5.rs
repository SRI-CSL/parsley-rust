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
use crate::pdf_lib::halftonetype1::halftonetype1_type;
use crate::pdf_lib::halftonetype6::halftonetype6_type;
use crate::pdf_lib::halftonetype10::halftonetype10_type;
use crate::pdf_lib::halftonetype16::halftonetype16_type;
use crate::pdf_lib::halftonetype1::halftonetype1_type;
use crate::pdf_lib::halftonetype6::halftonetype6_type;
use crate::pdf_lib::halftonetype10::halftonetype10_type;
use crate::pdf_lib::halftonetype16::halftonetype16_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Halftone"))),
           ],
     );
    let choices_halftonetype = ChoicesPred(
        String::From("Invalid HalftoneType"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("5"))),
           ],
     );
pub fn halftonetype5_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let halftonetype_field = DictEntry { 
       key: Vec::from("HalftoneType"), 
       chk: choices_halftonetype(tctx),
       opt: DictKeySpec::Required,
    }; 
    let halftonename_field = DictEntry { 
       key: Vec::from("HalftoneName"), 
       opt: DictKeySpec::Optional,
    }; 
    let *_field = DictEntry { 
       key: Vec::from("*"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         ]_type,
         ;_type,
         [_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         6_type,
         ,_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         0_type,
         ,_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         6_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let default_field = DictEntry { 
       key: Vec::from("Default"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         ]_type,
         ;_type,
         [_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         6_type,
         ,_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         0_type,
         ,_type,
         h_type,
         a_type,
         l_type,
         f_type,
         t_type,
         o_type,
         n_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         1_type,
         6_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    TypeCheck::new(
    tctx,
    "halftonetype5",
    Rc::new(PDFType::Dict(vec![
      type_field,
      halftonetype_field,
      halftonename_field,
      *_field,
      default_field,
   ]))
}