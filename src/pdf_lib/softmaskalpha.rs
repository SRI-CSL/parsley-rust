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
use crate::pdf_lib::grouptransparency::grouptransparency_type;
use crate::pdf_lib::functiontype2::functiontype2_type;
use crate::pdf_lib::functiontype3::functiontype3_type;
use crate::pdf_lib::functiontype0::functiontype0_type;
use crate::pdf_lib::functiontype4::functiontype4_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Mask"))),
           ],
     );
    let choices_s = ChoicesPred(
        String::From("Invalid S"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Alpha"))),
           ],
     );
pub fn softmaskalpha_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let s_field = DictEntry { 
       key: Vec::from("S"), 
       chk: choices_s(tctx),
       opt: DictKeySpec::Required,
    }; 
    let g_field = DictEntry { 
       key: Vec::from("G"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         g_type,
         r_type,
         o_type,
         u_type,
         p_type,
         t_type,
         r_type,
         a_type,
         n_type,
         s_type,
         p_type,
         a_type,
         r_type,
         e_type,
         n_type,
         c_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let tr_field = DictEntry { 
       key: Vec::from("TR"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         u_type,
         n_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         t_type,
         y_type,
         p_type,
         e_type,
         2_type,
         ,_type,
         f_type,
         u_type,
         n_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         t_type,
         y_type,
         p_type,
         e_type,
         3_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
         ;_type,
         [_type,
         f_type,
         u_type,
         n_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         t_type,
         y_type,
         p_type,
         e_type,
         0_type,
         ,_type,
         f_type,
         u_type,
         n_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         t_type,
         y_type,
         p_type,
         e_type,
         4_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "softmaskalpha",
    Rc::new(PDFType::Dict(vec![
      type_field,
      s_field,
      g_field,
      tr_field,
   ]))
}