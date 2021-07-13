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
use crate::pdf_lib::arrayof3dmeasure::arrayof3dmeasure_type;
use crate::pdf_lib::arrayof3dtransmatrix::arrayof3dtransmatrix_type;
use crate::pdf_lib::arrayofstringstext::arrayofstringstext_type;
use crate::pdf_lib::projection::projection_type;
use crate::pdf_lib::xobjectformtype1::xobjectformtype1_type;
use crate::pdf_lib::3dbackground::3dbackground_type;
use crate::pdf_lib::3drendermode::3drendermode_type;
use crate::pdf_lib::3dlightingscheme::3dlightingscheme_type;
use crate::pdf_lib::arrayof3dcrosssection::arrayof3dcrosssection_type;
use crate::pdf_lib::arrayof3dnode::arrayof3dnode_type;
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::arrayofviewparams::arrayofviewparams_type;
pub fn 3dviewaddentries_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
pub fn 3dviewaddentries_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let xn_field = DictEntry { 
       key: Vec::from("XN"), 
       chk:        TypeCheck::new(
          tctx,
          "xn",
       Rc::new(PDFType::PrimType(PDFPrimType::String)),
       ),       opt: DictKeySpec::Required,
    }; 
    let in_field = DictEntry { 
       key: Vec::from("IN"), 
       chk: choices_in(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let ms_field = DictEntry { 
       key: Vec::from("MS"), 
       chk:        TypeCheck::new(
          tctx,
          "ms",
       Rc::new(PDFType::PrimType(PDFPrimType::Name)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let ma_field = DictEntry { 
       key: Vec::from("MA"), 
       chk: TypeCheck::new(
          tctx,
          "ma",
          Rc::new(PDFType::Disjunct(vec![
         arrayof3dmeasure_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let c2w_field = DictEntry { 
       key: Vec::from("C2W"), 
       chk: TypeCheck::new(
          tctx,
          "c2w",
          Rc::new(PDFType::Disjunct(vec![
         arrayof3dtransmatrix_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let u3dpath_field = DictEntry { 
       key: Vec::from("U3DPath"), 
       chk: TypeCheck::new(
          tctx,
          "u3dpath",
          Rc::new(PDFType::Disjunct(vec![
         arrayofstringstext_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let co_field = DictEntry { 
       key: Vec::from("CO"), 
       chk:        TypeCheck::new(
          tctx,
          "co",
       Rc::new(PDFType::PrimType(PDFPrimType::Real)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let p_field = DictEntry { 
       key: Vec::from("P"), 
       chk: TypeCheck::new(
          tctx,
          "p",
          Rc::new(PDFType::Disjunct(vec![
         projection_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       chk: TypeCheck::new(
          tctx,
          "o",
          Rc::new(PDFType::Disjunct(vec![
         xobjectformtype1_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let bg_field = DictEntry { 
       key: Vec::from("BG"), 
       chk: TypeCheck::new(
          tctx,
          "bg",
          Rc::new(PDFType::Disjunct(vec![
         3dbackground_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let rm_field = DictEntry { 
       key: Vec::from("RM"), 
       chk: TypeCheck::new(
          tctx,
          "rm",
          Rc::new(PDFType::Disjunct(vec![
         3drendermode_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let ls_field = DictEntry { 
       key: Vec::from("LS"), 
       chk: TypeCheck::new(
          tctx,
          "ls",
          Rc::new(PDFType::Disjunct(vec![
         3dlightingscheme_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let sa_field = DictEntry { 
       key: Vec::from("SA"), 
       chk: TypeCheck::new(
          tctx,
          "sa",
          Rc::new(PDFType::Disjunct(vec![
         arrayof3dcrosssection_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let na_field = DictEntry { 
       key: Vec::from("NA"), 
       chk: TypeCheck::new(
          tctx,
          "na",
          Rc::new(PDFType::Disjunct(vec![
         arrayof3dnode_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let nr_field = DictEntry { 
       key: Vec::from("NR"), 
       chk:        TypeCheck::new(
          tctx,
          "nr",
       Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
       ),       opt: DictKeySpec::Optional,
    }; 
    let snapshot_field = DictEntry { 
       key: Vec::from("Snapshot"), 
       chk: TypeCheck::new(
          tctx,
          "snapshot",
          Rc::new(PDFType::Disjunct(vec![
         stream_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    let params_field = DictEntry { 
       key: Vec::from("Params"), 
       chk: TypeCheck::new(
          tctx,
          "params",
          Rc::new(PDFType::Disjunct(vec![
         arrayofviewparams_type,
        ]))),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "3dviewaddentries",
    Rc::new(PDFType::Dict(vec![
      type_field,
      xn_field,
      in_field,
      ms_field,
      ma_field,
      c2w_field,
      u3dpath_field,
      co_field,
      p_field,
      o_field,
      bg_field,
      rm_field,
      ls_field,
      sa_field,
      na_field,
      nr_field,
      snapshot_field,
      params_field,
   ]))
}    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("3DView"))),
           ],
     );
    let choices_in = ChoicesPred(
        String::From("Invalid IN"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("@XN"))),
           ],
     );
