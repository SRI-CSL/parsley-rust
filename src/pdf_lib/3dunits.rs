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
pub fn 3dunits_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
pub fn 3dunits_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let tsm_field = DictEntry { 
       key: Vec::from("TSm"), 
       chk:        TypeCheck::new(
          tctx,
          "tsm",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let tsn_field = DictEntry { 
       key: Vec::from("TSn"), 
       chk:        TypeCheck::new(
          tctx,
          "tsn",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let tu_field = DictEntry { 
       key: Vec::from("TU"), 
       chk:        TypeCheck::new(
          tctx,
          "tu",
       Rc::new(PDFType::PrimType(PDFPrimType::String)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let usm_field = DictEntry { 
       key: Vec::from("USm"), 
       chk:        TypeCheck::new(
          tctx,
          "usm",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let usn_field = DictEntry { 
       key: Vec::from("USn"), 
       chk:        TypeCheck::new(
          tctx,
          "usn",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let uu_field = DictEntry { 
       key: Vec::from("UU"), 
       chk:        TypeCheck::new(
          tctx,
          "uu",
       Rc::new(PDFType::PrimType(PDFPrimType::String)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let dsm_field = DictEntry { 
       key: Vec::from("DSm"), 
       chk:        TypeCheck::new(
          tctx,
          "dsm",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let dsn_field = DictEntry { 
       key: Vec::from("DSn"), 
       chk:        TypeCheck::new(
          tctx,
          "dsn",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let du_field = DictEntry { 
       key: Vec::from("DU"), 
       chk:        TypeCheck::new(
          tctx,
          "du",
       Rc::new(PDFType::PrimType(PDFPrimType::String)),
       ),       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dunits",
    Rc::new(PDFType::Dict(vec![
      tsm_field,
      tsn_field,
      tu_field,
      usm_field,
      usn_field,
      uu_field,
      dsm_field,
      dsn_field,
      du_field,
   ]))
}