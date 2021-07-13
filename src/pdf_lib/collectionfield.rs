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
         PDFObjT::Name(NameT::new(Vec::from("CollectionField"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("S"))),
         PDFObjT::Name(NameT::new(Vec::from("D"))),
         PDFObjT::Name(NameT::new(Vec::from("N"))),
         PDFObjT::Name(NameT::new(Vec::from("F"))),
         PDFObjT::Name(NameT::new(Vec::from("Desc"))),
         PDFObjT::Name(NameT::new(Vec::from("ModDate"))),
         PDFObjT::Name(NameT::new(Vec::from("CreationDate"))),
         PDFObjT::Name(NameT::new(Vec::from("Size"))),
         PDFObjT::Name(NameT::new(Vec::from("CompressedSize)"))),
           ],
     );
pub fn collectionfield_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let n_field = DictEntry { 
       key: Vec::from("N"), 
       opt: DictKeySpec::Required,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       opt: DictKeySpec::Optional,
    }; 
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       opt: DictKeySpec::Optional,
    }; 
    let e_field = DictEntry { 
       key: Vec::from("E"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "collectionfield",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      n_field,
      o_field,
      v_field,
      e_field,
   ]))
}