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
use crate::pdf_lib::arrayofstringsbyte::arrayofstringsbyte_type;
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
         PDFObjT::Name(NameT::new(Vec::from("V2"))),
         PDFObjT::Name(NameT::new(Vec::from("AESV2"))),
         PDFObjT::Name(NameT::new(Vec::from("AESV3"))),
           ],
     );
    let choices_authevent = ChoicesPred(
        String::From("Invalid AuthEvent"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("DocOpen"))),
         PDFObjT::Name(NameT::new(Vec::from("EFOpen"))),
           ],
     );
pub fn cryptfilterpublickey_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let recipients_field = DictEntry { 
       key: Vec::from("Recipients"), 
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
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let encryptmetadata_field = DictEntry { 
       key: Vec::from("EncryptMetadata"), 
       opt: DictKeySpec::Optional,
    }; 
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
       opt: DictKeySpec::Required,
    }; 
    TypeCheck::new(
    tctx,
    "cryptfilterpublickey",
    Rc::new(PDFType::Dict(vec![
      recipients_field,
      encryptmetadata_field,
      type_field,
      cfm_field,
      authevent_field,
      length_field,
   ]))
}