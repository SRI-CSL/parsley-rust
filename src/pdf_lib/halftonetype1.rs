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
use crate::pdf_lib::arrayoffunctions::arrayoffunctions_type;
use crate::pdf_lib::functiontype2::functiontype2_type;
use crate::pdf_lib::functiontype3::functiontype3_type;
use crate::pdf_lib::functiontype0::functiontype0_type;
use crate::pdf_lib::functiontype4::functiontype4_type;
use crate::pdf_lib::functiontype2::functiontype2_type;
use crate::pdf_lib::functiontype3::functiontype3_type;
use crate::pdf_lib::functiontype0::functiontype0_type;
use crate::pdf_lib::functiontype4::functiontype4_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Halftone"))),
           ],
     );
    let choices_halftonetype = ChoicesPred(
        String::From("Invalid HalftoneType"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("1"))),
           ],
     );
pub fn halftonetype1_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let frequency_field = DictEntry { 
       key: Vec::from("Frequency"), 
       opt: DictKeySpec::Required,
    }; 
    let angle_field = DictEntry { 
       key: Vec::from("Angle"), 
       opt: DictKeySpec::Required,
    }; 
    let spotfunction_field = DictEntry { 
       key: Vec::from("SpotFunction"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         f_type,
         u_type,
         n_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         s_type,
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
       opt: DictKeySpec::Required,
    }; 
    let accuratescreens_field = DictEntry { 
       key: Vec::from("AccurateScreens"), 
       opt: DictKeySpec::Optional,
    }; 
    let transferfunction_field = DictEntry { 
       key: Vec::from("TransferFunction"), 
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
    "halftonetype1",
    Rc::new(PDFType::Dict(vec![
      type_field,
      halftonetype_field,
      halftonename_field,
      frequency_field,
      angle_field,
      spotfunction_field,
      accuratescreens_field,
      transferfunction_field,
   ]))
}