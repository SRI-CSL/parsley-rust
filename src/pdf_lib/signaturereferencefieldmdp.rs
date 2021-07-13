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
use crate::pdf_lib::fieldmdptransformparameters::fieldmdptransformparameters_type;
use crate::pdf_lib::_universalarray::_universalarray_type;
use crate::pdf_lib::_universaldictionary::_universaldictionary_type;
use crate::pdf_lib::stream::stream_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("SigRef"))),
           ],
     );
    let choices_transformmethod = ChoicesPred(
        String::From("Invalid TransformMethod"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("FieldMDP"))),
           ],
     );
    let choices_digestmethod = ChoicesPred(
        String::From("Invalid DigestMethod"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("MD5)"))),
         PDFObjT::Name(NameT::new(Vec::from("SHA1)"))),
         PDFObjT::Name(NameT::new(Vec::from("SHA256"))),
         PDFObjT::Name(NameT::new(Vec::from("SHA384"))),
         PDFObjT::Name(NameT::new(Vec::from("SHA512"))),
         PDFObjT::Name(NameT::new(Vec::from("RIPEMD160"))),
           ],
     );
pub fn signaturereferencefieldmdp_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let transformmethod_field = DictEntry { 
       key: Vec::from("TransformMethod"), 
       chk: choices_transformmethod(tctx),
       opt: DictKeySpec::Required,
    }; 
    let transformparams_field = DictEntry { 
       key: Vec::from("TransformParams"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         i_type,
         e_type,
         l_type,
         d_type,
         m_type,
         d_type,
         p_type,
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
         t_type,
         e_type,
         r_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let data_field = DictEntry { 
       key: Vec::from("Data"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         __type,
         u_type,
         n_type,
         i_type,
         v_type,
         e_type,
         r_type,
         s_type,
         a_type,
         l_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
         ;_type,
         [_type,
         __type,
         u_type,
         n_type,
         i_type,
         v_type,
         e_type,
         r_type,
         s_type,
         a_type,
         l_type,
         d_type,
         i_type,
         c_type,
         t_type,
         i_type,
         o_type,
         n_type,
         a_type,
         r_type,
         y_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
         ;_type,
         [_type,
         s_type,
         t_type,
         r_type,
         e_type,
         a_type,
         m_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let digestmethod_field = DictEntry { 
       key: Vec::from("DigestMethod"), 
       chk: choices_digestmethod(tctx),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "signaturereferencefieldmdp",
    Rc::new(PDFType::Dict(vec![
      type_field,
      transformmethod_field,
      transformparams_field,
      data_field,
      digestmethod_field,
   ]))
}