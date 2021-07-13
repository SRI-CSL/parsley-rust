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
use crate::pdf_lib::arrayofdifferences::arrayofdifferences_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Encoding"))),
           ],
     );
    let choices_baseencoding = ChoicesPred(
        String::From("Invalid BaseEncoding"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("MacRomanEncoding"))),
         PDFObjT::Name(NameT::new(Vec::from("MacExpertEncoding"))),
         PDFObjT::Name(NameT::new(Vec::from("WinAnsiEncoding"))),
           ],
     );
pub fn encoding_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let baseencoding_field = DictEntry { 
       key: Vec::from("BaseEncoding"), 
       chk: choices_baseencoding(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let differences_field = DictEntry { 
       key: Vec::from("Differences"), 
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
         i_type,
         f_type,
         f_type,
         e_type,
         r_type,
         e_type,
         n_type,
         c_type,
         e_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "encoding",
    Rc::new(PDFType::Dict(vec![
      type_field,
      baseencoding_field,
      differences_field,
   ]))
}