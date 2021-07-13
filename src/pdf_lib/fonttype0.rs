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
use crate::pdf_lib::cmap::cmap_type;
use crate::pdf_lib::arrayofdescendantfonts::arrayofdescendantfonts_type;
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
         PDFObjT::Name(NameT::new(Vec::from("Type0"))),
           ],
     );
pub fn fonttype0_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let basefont_field = DictEntry { 
       key: Vec::from("BaseFont"), 
       opt: DictKeySpec::Required,
    }; 
    let encoding_field = DictEntry { 
       key: Vec::from("Encoding"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         ]_type,
         ;_type,
         [_type,
         c_type,
         m_type,
         a_type,
         p_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let descendantfonts_field = DictEntry { 
       key: Vec::from("DescendantFonts"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         d_type,
         e_type,
         s_type,
         c_type,
         e_type,
         n_type,
         d_type,
         a_type,
         n_type,
         t_type,
         f_type,
         o_type,
         n_type,
         t_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
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
    "fonttype0",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      basefont_field,
      encoding_field,
      descendantfonts_field,
      tounicode_field,
   ]))
}