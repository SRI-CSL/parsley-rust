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
use crate::pdf_lib::3dview::3dview_type;
use crate::pdf_lib::3dview::3dview_type;
use crate::pdf_lib::_universaldictionary::_universaldictionary_type;
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::3danimationstyle::3danimationstyle_type;
use crate::pdf_lib::calrgbcolorspace::calrgbcolorspace_type;
use crate::pdf_lib::iccbasedcolorspace::iccbasedcolorspace_type;
use crate::pdf_lib::arrayofcompressionfilternames::arrayofcompressionfilternames_type;
use crate::pdf_lib::arrayofdecodeparams::arrayofdecodeparams_type;
use crate::pdf_lib::filterlzwdecode::filterlzwdecode_type;
use crate::pdf_lib::filterflatedecode::filterflatedecode_type;
use crate::pdf_lib::filtercrypt::filtercrypt_type;
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::arrayofcompressionfilternames::arrayofcompressionfilternames_type;
use crate::pdf_lib::arrayofdecodeparams::arrayofdecodeparams_type;
use crate::pdf_lib::filterlzwdecode::filterlzwdecode_type;
use crate::pdf_lib::filterflatedecode::filterflatedecode_type;
use crate::pdf_lib::filtercrypt::filtercrypt_type;
pub fn 3dstream_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
pub fn 3dstream_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let va_field = DictEntry { 
       key: Vec::from("VA"), 
       chk: TypeCheck::new(
          tctx,
          "va",
          Rc::new(PDFType::Disjunct(vec![
         3dview_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let dv_field = DictEntry { 
       key: Vec::from("DV"), 
       chk: TypeCheck::new(
          tctx,
          "dv",
          Rc::new(PDFType::Disjunct(vec![
         3dview_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let resources_field = DictEntry { 
       key: Vec::from("Resources"), 
       chk: TypeCheck::new(
          tctx,
          "resources",
          Rc::new(PDFType::Disjunct(vec![
         _universaldictionary_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let oninstantiate_field = DictEntry { 
       key: Vec::from("OnInstantiate"), 
       chk: TypeCheck::new(
          tctx,
          "oninstantiate",
          Rc::new(PDFType::Disjunct(vec![
         stream_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let an_field = DictEntry { 
       key: Vec::from("AN"), 
       chk: TypeCheck::new(
          tctx,
          "an",
          Rc::new(PDFType::Disjunct(vec![
         3danimationstyle_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let colorspace_field = DictEntry { 
       key: Vec::from("ColorSpace"), 
       chk: TypeCheck::new(
          tctx,
          "colorspace",
          Rc::new(PDFType::Disjunct(vec![
         calrgbcolorspace_type,
         iccbasedcolorspace_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let length_field = DictEntry { 
       key: Vec::from("Length"), 
       chk:        TypeCheck::new(
          tctx,
          "length",
          Rc::new(PDFType::PrimType(PDFPrimType::Integer))
       ),       opt: DictKeySpec::Required,
    }; 
    let filter_field = DictEntry { 
       key: Vec::from("Filter"), 
       chk: TypeCheck::new(
          tctx,
          "filter",
          Rc::new(PDFType::Disjunct(vec![
         arrayofcompressionfilternames_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let decodeparms_field = DictEntry { 
       key: Vec::from("DecodeParms"), 
       chk: TypeCheck::new(
          tctx,
          "decodeparms",
          Rc::new(PDFType::Disjunct(vec![
         arrayofdecodeparams_type,
         filterlzwdecode_type,
         filterflatedecode_type,
         filtercrypt_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let f_field = DictEntry { 
       key: Vec::from("F"), 
       chk: TypeCheck::new(
          tctx,
          "f",
          Rc::new(PDFType::Disjunct(vec![
         filespecification_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let ffilter_field = DictEntry { 
       key: Vec::from("FFilter"), 
       chk: TypeCheck::new(
          tctx,
          "ffilter",
          Rc::new(PDFType::Disjunct(vec![
         arrayofcompressionfilternames_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let fdecodeparms_field = DictEntry { 
       key: Vec::from("FDecodeParms"), 
       chk: TypeCheck::new(
          tctx,
          "fdecodeparms",
          Rc::new(PDFType::Disjunct(vec![
         arrayofdecodeparams_type,
         filterlzwdecode_type,
         filterflatedecode_type,
         filtercrypt_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let dl_field = DictEntry { 
       key: Vec::from("DL"), 
       chk:        TypeCheck::new(
          tctx,
          "dl",
          Rc::new(PDFType::PrimType(PDFPrimType::Integer))
       ),       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dstream",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      va_field,
      dv_field,
      resources_field,
      oninstantiate_field,
      an_field,
      colorspace_field,
      length_field,
      filter_field,
      decodeparms_field,
      f_field,
      ffilter_field,
      fdecodeparms_field,
      dl_field,
   ]))
}    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3D"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("U3D"))),
         PDFObjT::Name(NameT::new(Vec::from("PRC"))),
           ],
     );
