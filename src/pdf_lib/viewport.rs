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
use crate::pdf_lib::measurerl::measurerl_type;
use crate::pdf_lib::measuregeo::measuregeo_type;
use crate::pdf_lib::pointdata::pointdata_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Viewport"))),
           ],
     );
pub fn viewport_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let bbox_field = DictEntry { 
       key: Vec::from("BBox"), 
       opt: DictKeySpec::Required,
    }; 
    let name_field = DictEntry { 
       key: Vec::from("Name"), 
       opt: DictKeySpec::Optional,
    }; 
    let measure_field = DictEntry { 
       key: Vec::from("Measure"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         m_type,
         e_type,
         a_type,
         s_type,
         u_type,
         r_type,
         e_type,
         r_type,
         l_type,
         ,_type,
         m_type,
         e_type,
         a_type,
         s_type,
         u_type,
         r_type,
         e_type,
         g_type,
         e_type,
         o_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let ptdata_field = DictEntry { 
       key: Vec::from("PtData"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         p_type,
         o_type,
         i_type,
         n_type,
         t_type,
         d_type,
         a_type,
         t_type,
         a_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "viewport",
    Rc::new(PDFType::Dict(vec![
      type_field,
      bbox_field,
      name_field,
      measure_field,
      ptdata_field,
   ]))
}