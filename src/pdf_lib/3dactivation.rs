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
use crate::pdf_lib::richmediawindow::richmediawindow_type;
pub fn 3dactivation_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
pub fn 3dactivation_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let a_field = DictEntry { 
       key: Vec::from("A"), 
       chk: choices_a(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let ais_field = DictEntry { 
       key: Vec::from("AIS"), 
       chk: choices_ais(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let d_field = DictEntry { 
       key: Vec::from("D"), 
       chk: choices_d(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let dis_field = DictEntry { 
       key: Vec::from("DIS"), 
       chk: choices_dis(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let tb_field = DictEntry { 
       key: Vec::from("TB"), 
       chk:        TypeCheck::new(
          tctx,
          "tb",
       Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let np_field = DictEntry { 
       key: Vec::from("NP"), 
       chk:        TypeCheck::new(
          tctx,
          "np",
       Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let style_field = DictEntry { 
       key: Vec::from("Style"), 
       chk: choices_style(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let window_field = DictEntry { 
       key: Vec::from("Window"), 
       chk: TypeCheck::new(
          tctx,
          "window",
          Rc::new(PDFType::Disjunct(vec![
         richmediawindow_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let transparent_field = DictEntry { 
       key: Vec::from("Transparent"), 
       chk:        TypeCheck::new(
          tctx,
          "transparent",
       Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
       ),       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dactivation",
    Rc::new(PDFType::Dict(vec![
      a_field,
      ais_field,
      d_field,
      dis_field,
      tb_field,
      np_field,
      style_field,
      window_field,
      transparent_field,
   ]))
}    let choices_a = ChoicesPred(
        String::From("Invalid A"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("PO"))),
         PDFObjT::Name(NameT::new(Vec::from("PV"))),
         PDFObjT::Name(NameT::new(Vec::from("XA"))),
           ],
     );
    let choices_ais = ChoicesPred(
        String::From("Invalid AIS"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("I"))),
         PDFObjT::Name(NameT::new(Vec::from("L"))),
           ],
     );
    let choices_d = ChoicesPred(
        String::From("Invalid D"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("PC"))),
         PDFObjT::Name(NameT::new(Vec::from("PI"))),
         PDFObjT::Name(NameT::new(Vec::from("XD"))),
           ],
     );
    let choices_dis = ChoicesPred(
        String::From("Invalid DIS"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("U"))),
         PDFObjT::Name(NameT::new(Vec::from("I"))),
         PDFObjT::Name(NameT::new(Vec::from("L"))),
           ],
     );
    let choices_style = ChoicesPred(
        String::From("Invalid Style"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Embedded"))),
         PDFObjT::Name(NameT::new(Vec::from("Windowed"))),
           ],
     );
