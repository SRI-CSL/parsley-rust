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
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("CryptFilter"))),
           ],
     );
    let choices_cfm = ChoicesPred(
        String::From("Invalid CFM"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("None"))),
         PDFObjT::Name(NameT::new(Vec::from("V2)"))),
         PDFObjT::Name(NameT::new(Vec::from("AESV2))"))),
         PDFObjT::Name(NameT::new(Vec::from("AESV3)"))),
           ],
     );
    let choices_authevent = ChoicesPred(
        String::From("Invalid AuthEvent"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("DocOpen"))),
         PDFObjT::Name(NameT::new(Vec::from("EFOpen"))),
           ],
     );
    let choices_length = ChoicesPred(
        String::From("Invalid Length"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("128) || fn:RequiredValue(@CFM==AESV3"))),
         PDFObjT::Name(NameT::new(Vec::from("256) || fn:Expr(@self>=40 && @self<=128 && @self mod 8==0)"))),
           ],
     );
pub fn cryptfilter_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let cfm_field = DictEntry { 
       key: Vec::from("CFM"), 
       chk: choices_cfm(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let authevent_field = DictEntry { 
       key: Vec::from("AuthEvent"), 
       chk: choices_authevent(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let length_field = DictEntry { 
       key: Vec::from("Length"), 
       chk: choices_length(tctx),
       opt: DictKeySpec::Required,
    }; 
    TypeCheck::new(
    tctx,
    "cryptfilter",
    Rc::new(PDFType::Dict(vec![
      type_field,
      cfm_field,
      authevent_field,
      length_field,
   ]))
}