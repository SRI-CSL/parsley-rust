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
    mk_rectangle_typchk, mk_date_typchk
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};use std::rc::Rc;
pub fn 3dlightingscheme_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
pub fn 3dlightingscheme_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    TypeCheck::new(
    tctx,
    "3dlightingscheme",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
   ]))
}    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DLightingScheme"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Artwork"))),
         PDFObjT::Name(NameT::new(Vec::from("None"))),
         PDFObjT::Name(NameT::new(Vec::from("White"))),
         PDFObjT::Name(NameT::new(Vec::from("Day"))),
         PDFObjT::Name(NameT::new(Vec::from("Night"))),
         PDFObjT::Name(NameT::new(Vec::from("Hard"))),
         PDFObjT::Name(NameT::new(Vec::from("Primary"))),
         PDFObjT::Name(NameT::new(Vec::from("Blue"))),
         PDFObjT::Name(NameT::new(Vec::from("Red"))),
         PDFObjT::Name(NameT::new(Vec::from("Cube"))),
         PDFObjT::Name(NameT::new(Vec::from("CAD"))),
         PDFObjT::Name(NameT::new(Vec::from("Headlamp"))),
           ],
     );
