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
use crate::pdf_lib::arrayofversions::arrayofversions_type;
use crate::pdf_lib::arrayofversions::arrayofversions_type;
use crate::pdf_lib::arrayofstringsbyte::arrayofstringsbyte_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("SoftwareIdentifier"))),
           ],
     );
pub fn softwareidentifier_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let u_field = DictEntry { 
       key: Vec::from("U"), 
       opt: DictKeySpec::Required,
    }; 
    let l_field = DictEntry { 
       key: Vec::from("L"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         v_type,
         e_type,
         r_type,
         s_type,
         i_type,
         o_type,
         n_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let li_field = DictEntry { 
       key: Vec::from("LI"), 
       opt: DictKeySpec::Optional,
    }; 
    let h_field = DictEntry { 
       key: Vec::from("H"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         v_type,
         e_type,
         r_type,
         s_type,
         i_type,
         o_type,
         n_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let hi_field = DictEntry { 
       key: Vec::from("HI"), 
       opt: DictKeySpec::Optional,
    }; 
    let os_field = DictEntry { 
       key: Vec::from("OS"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         s_type,
         t_type,
         r_type,
         i_type,
         n_type,
         g_type,
         s_type,
         b_type,
         y_type,
         t_type,
         e_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "softwareidentifier",
    Rc::new(PDFType::Dict(vec![
      type_field,
      u_field,
      l_field,
      li_field,
      h_field,
      hi_field,
      os_field,
   ]))
}