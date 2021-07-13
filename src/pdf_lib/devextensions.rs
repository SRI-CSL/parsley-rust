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
         PDFObjT::Name(NameT::new(Vec::from("DeveloperExtensions"))),
           ],
     );
    let choices_baseversion = ChoicesPred(
        String::From("Invalid BaseVersion"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("1.0"))),
         PDFObjT::Name(NameT::new(Vec::from("1.1"))),
         PDFObjT::Name(NameT::new(Vec::from("1.2"))),
         PDFObjT::Name(NameT::new(Vec::from("1.3"))),
         PDFObjT::Name(NameT::new(Vec::from("1.4"))),
         PDFObjT::Name(NameT::new(Vec::from("1.5"))),
         PDFObjT::Name(NameT::new(Vec::from("1.6"))),
         PDFObjT::Name(NameT::new(Vec::from("1.7"))),
         PDFObjT::Name(NameT::new(Vec::from("2.0"))),
           ],
     );
pub fn devextensions_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let baseversion_field = DictEntry { 
       key: Vec::from("BaseVersion"), 
       chk: choices_baseversion(tctx),
       opt: DictKeySpec::Required,
    }; 
    let extensionlevel_field = DictEntry { 
       key: Vec::from("ExtensionLevel"), 
       opt: DictKeySpec::Required,
    }; 
    let url_field = DictEntry { 
       key: Vec::from("URL"), 
       opt: DictKeySpec::Optional,
    }; 
    let extensionrevision_field = DictEntry { 
       key: Vec::from("ExtensionRevision"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "devextensions",
    Rc::new(PDFType::Dict(vec![
      type_field,
      baseversion_field,
      extensionlevel_field,
      url_field,
      extensionrevision_field,
   ]))
}