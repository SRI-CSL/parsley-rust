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
use crate::pdf_lib::arrayofintegersgeneral::arrayofintegersgeneral_type;
use crate::pdf_lib::fontdescriptortruetype::fontdescriptortruetype_type;
use crate::pdf_lib::encoding::encoding_type;
use crate::pdf_lib::stream::stream_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Font"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("TrueType"))),
           ],
     );
pub fn fonttruetype_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Required,
    }; 
    let subtype_field = DictEntry { 
       key: Vec::from("Subtype"), 
       chk: choices_subtype(tctx),
       opt: DictKeySpec::Required,
    }; 
    let name_field = DictEntry { 
       key: Vec::from("Name"), 
       opt: DictKeySpec::Optional,
    }; 
    let basefont_field = DictEntry { 
       key: Vec::from("BaseFont"), 
       opt: DictKeySpec::Required,
    }; 
    let firstchar_field = DictEntry { 
       key: Vec::from("FirstChar"), 
       opt: DictKeySpec::Required,
    }; 
    let lastchar_field = DictEntry { 
       key: Vec::from("LastChar"), 
       opt: DictKeySpec::Required,
    }; 
    let widths_field = DictEntry { 
       key: Vec::from("Widths"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         i_type,
         n_type,
         t_type,
         e_type,
         g_type,
         e_type,
         r_type,
         s_type,
         g_type,
         e_type,
         n_type,
         e_type,
         r_type,
         a_type,
         l_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let fontdescriptor_field = DictEntry { 
       key: Vec::from("FontDescriptor"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         o_type,
         n_type,
         t_type,
         d_type,
         e_type,
         s_type,
         c_type,
         r_type,
         i_type,
         p_type,
         t_type,
         o_type,
         r_type,
         t_type,
         r_type,
         u_type,
         e_type,
         t_type,
         y_type,
         p_type,
         e_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let encoding_field = DictEntry { 
       key: Vec::from("Encoding"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         e_type,
         n_type,
         c_type,
         o_type,
         d_type,
         i_type,
         n_type,
         g_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let tounicode_field = DictEntry { 
       key: Vec::from("ToUnicode"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         s_type,
         t_type,
         r_type,
         e_type,
         a_type,
         m_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "fonttruetype",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      name_field,
      basefont_field,
      firstchar_field,
      lastchar_field,
      widths_field,
      fontdescriptor_field,
      encoding_field,
      tounicode_field,
   ]))
}