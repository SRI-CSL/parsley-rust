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
use crate::pdf_lib::arrayofstringstext::arrayofstringstext_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("TransformParams"))),
           ],
     );
    let choices_action = ChoicesPred(
        String::From("Invalid Action"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("All"))),
         PDFObjT::Name(NameT::new(Vec::from("Include"))),
         PDFObjT::Name(NameT::new(Vec::from("Exclude"))),
           ],
     );
    let choices_v = ChoicesPred(
        String::From("Invalid V"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("1.2"))),
           ],
     );
pub fn fieldmdptransformparameters_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let action_field = DictEntry { 
       key: Vec::from("Action"), 
       chk: choices_action(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let fields_field = DictEntry { 
       key: Vec::from("Fields"), 
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
         t_type,
         e_type,
         x_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       chk: choices_v(tctx),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "fieldmdptransformparameters",
    Rc::new(PDFType::Dict(vec![
      type_field,
      action_field,
      fields_field,
      v_field,
   ]))
}