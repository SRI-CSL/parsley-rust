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
use crate::pdf_lib::arrayof_3numbers::arrayof_3numbers_type;
use crate::pdf_lib::arrayof_3numbers::arrayof_3numbers_type;
use crate::pdf_lib::arrayof_2integers::arrayof_2integers_type;
use crate::pdf_lib::arrayof_3rgbnumbers::arrayof_3rgbnumbers_type;
use crate::pdf_lib::annotprojection::annotprojection_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DMeasure"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DC"))),
           ],
     );
pub fn 3dmeasure3dc_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let subtype_field = DictEntry { 
       key: Vec::from("Subtype"), 
       chk: choices_subtype(tctx),
       opt: DictKeySpec::Required,
    }; 
    let trl_field = DictEntry { 
       key: Vec::from("TRL"), 
       opt: DictKeySpec::Optional,
    }; 
    let a1_field = DictEntry { 
       key: Vec::from("A1"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         3_type,
         n_type,
         u_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let n1_field = DictEntry { 
       key: Vec::from("N1"), 
       opt: DictKeySpec::Optional,
    }; 
    let tp_field = DictEntry { 
       key: Vec::from("TP"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         3_type,
         n_type,
         u_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let tb_field = DictEntry { 
       key: Vec::from("TB"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         2_type,
         i_type,
         n_type,
         t_type,
         e_type,
         g_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let ts_field = DictEntry { 
       key: Vec::from("TS"), 
       opt: DictKeySpec::Optional,
    }; 
    let c_field = DictEntry { 
       key: Vec::from("C"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         3_type,
         r_type,
         g_type,
         b_type,
         n_type,
         u_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let ut_field = DictEntry { 
       key: Vec::from("UT"), 
       opt: DictKeySpec::Optional,
    }; 
    let s_field = DictEntry { 
       key: Vec::from("S"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         n_type,
         n_type,
         o_type,
         t_type,
         p_type,
         r_type,
         o_type,
         j_type,
         e_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dmeasure3dc",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      trl_field,
      a1_field,
      n1_field,
      tp_field,
      tb_field,
      ts_field,
      c_field,
      ut_field,
      s_field,
   ]))
}