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
use crate::pdf_lib::extensions::extensions_type;
use crate::pdf_lib::arrayofrequirementshandlers::arrayofrequirementshandlers_type;
use crate::pdf_lib::requirementshandler::requirementshandler_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Requirement"))),
           ],
     );
    let choices_s = ChoicesPred(
        String::From("Invalid S"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Geospatial3D"))),
           ],
     );
pub fn requirementsgeospatial3d_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         e_type,
         x_type,
         t_type,
         e_type,
         n_type,
         s_type,
         i_type,
         o_type,
         n_type,
         s_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let rh_field = DictEntry { 
       key: Vec::from("RH"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         r_type,
         e_type,
         q_type,
         u_type,
         i_type,
         r_type,
         e_type,
         m_type,
         e_type,
         n_type,
         t_type,
         s_type,
         h_type,
         a_type,
         n_type,
         d_type,
         l_type,
         e_type,
         r_type,
         s_type,
         ]_type,
         ;_type,
         [_type,
         r_type,
         e_type,
         q_type,
         u_type,
         i_type,
         r_type,
         e_type,
         m_type,
         e_type,
         n_type,
         t_type,
         s_type,
         h_type,
         a_type,
         n_type,
         d_type,
         l_type,
         e_type,
         r_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let penalty_field = DictEntry { 
       key: Vec::from("Penalty"), 
       chk: choices_penalty(tctx),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "requirementsgeospatial3d",
    Rc::new(PDFType::Dict(vec![
      type_field,
      s_field,
      v_field,
      rh_field,
      penalty_field,
   ]))
}