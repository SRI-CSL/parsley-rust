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
use crate::pdf_lib::arrayof_4colourspaceentries::arrayof_4colourspaceentries_type;
use crate::pdf_lib::arrayof_4colourspaceentries::arrayof_4colourspaceentries_type;
pub fn 3drendermode_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
pub fn 3drendermode_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let ac_field = DictEntry { 
       key: Vec::from("AC"), 
       chk: TypeCheck::new(
          tctx,
          "ac",
          Rc::new(PDFType::Disjunct(vec![
         arrayof_4colourspaceentries_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let fc_field = DictEntry { 
       key: Vec::from("FC"), 
       chk: TypeCheck::new(
          tctx,
          "fc",
          Rc::new(PDFType::Disjunct(vec![
         arrayof_4colourspaceentries_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       chk:        TypeCheck::new(
          tctx,
          "o",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let cv_field = DictEntry { 
       key: Vec::from("CV"), 
       chk:        TypeCheck::new(
          tctx,
          "cv",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3drendermode",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      ac_field,
      fc_field,
      o_field,
      cv_field,
   ]))
}    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DRenderMode"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Solid"))),
         PDFObjT::Name(NameT::new(Vec::from("SolidWireframe"))),
         PDFObjT::Name(NameT::new(Vec::from("Transparent"))),
         PDFObjT::Name(NameT::new(Vec::from("TransparentWireframe"))),
         PDFObjT::Name(NameT::new(Vec::from("BoundingBox"))),
         PDFObjT::Name(NameT::new(Vec::from("TransparentBoundingBox"))),
         PDFObjT::Name(NameT::new(Vec::from("TransparentBoundingBoxOutline"))),
         PDFObjT::Name(NameT::new(Vec::from("Wireframe"))),
         PDFObjT::Name(NameT::new(Vec::from("ShadedWireframe"))),
         PDFObjT::Name(NameT::new(Vec::from("HiddenWireframe"))),
         PDFObjT::Name(NameT::new(Vec::from("Vertices"))),
         PDFObjT::Name(NameT::new(Vec::from("ShadedVertices"))),
         PDFObjT::Name(NameT::new(Vec::from("Illustration"))),
         PDFObjT::Name(NameT::new(Vec::from("SolidOutline"))),
         PDFObjT::Name(NameT::new(Vec::from("ShadedIllustration"))),
           ],
     );
