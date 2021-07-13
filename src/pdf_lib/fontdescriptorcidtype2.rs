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
use crate::pdf_lib::fontfile::fontfile_type;
use crate::pdf_lib::fontfile2cidtype2::fontfile2cidtype2_type;
use crate::pdf_lib::styledict::styledict_type;
use crate::pdf_lib::fddict::fddict_type;
use crate::pdf_lib::stream::stream_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("FontDescriptor"))),
           ],
     );
    let choices_fontstretch = ChoicesPred(
        String::From("Invalid FontStretch"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("UltraCondensed"))),
         PDFObjT::Name(NameT::new(Vec::from("ExtraCondensed"))),
         PDFObjT::Name(NameT::new(Vec::from("Condensed"))),
         PDFObjT::Name(NameT::new(Vec::from("SemiCondensed"))),
         PDFObjT::Name(NameT::new(Vec::from("Normal"))),
         PDFObjT::Name(NameT::new(Vec::from("SemiExpanded"))),
         PDFObjT::Name(NameT::new(Vec::from("Expanded"))),
         PDFObjT::Name(NameT::new(Vec::from("ExtraExpanded"))),
         PDFObjT::Name(NameT::new(Vec::from("UltraExpanded"))),
           ],
     );
    let choices_fontweight = ChoicesPred(
        String::From("Invalid FontWeight"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("100"))),
         PDFObjT::Name(NameT::new(Vec::from("200"))),
         PDFObjT::Name(NameT::new(Vec::from("300"))),
         PDFObjT::Name(NameT::new(Vec::from("400"))),
         PDFObjT::Name(NameT::new(Vec::from("500"))),
         PDFObjT::Name(NameT::new(Vec::from("600"))),
         PDFObjT::Name(NameT::new(Vec::from("700"))),
         PDFObjT::Name(NameT::new(Vec::from("800"))),
         PDFObjT::Name(NameT::new(Vec::from("900"))),
           ],
     );
pub fn fontdescriptorcidtype2_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Required,
    }; 
    let fontname_field = DictEntry { 
       key: Vec::from("FontName"), 
       opt: DictKeySpec::Required,
    }; 
    let fontfamily_field = DictEntry { 
       key: Vec::from("FontFamily"), 
       opt: DictKeySpec::Optional,
    }; 
    let fontstretch_field = DictEntry { 
       key: Vec::from("FontStretch"), 
       chk: choices_fontstretch(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let fontweight_field = DictEntry { 
       key: Vec::from("FontWeight"), 
       chk: choices_fontweight(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let flags_field = DictEntry { 
       key: Vec::from("Flags"), 
       opt: DictKeySpec::Required,
    }; 
    let fontbbox_field = DictEntry { 
       key: Vec::from("FontBBox"), 
       opt: DictKeySpec::Required,
    }; 
    let italicangle_field = DictEntry { 
       key: Vec::from("ItalicAngle"), 
       opt: DictKeySpec::Required,
    }; 
    let ascent_field = DictEntry { 
       key: Vec::from("Ascent"), 
       opt: DictKeySpec::Required,
    }; 
    let descent_field = DictEntry { 
       key: Vec::from("Descent"), 
       chk: choices_descent(tctx),
       opt: DictKeySpec::Required,
    }; 
    let leading_field = DictEntry { 
       key: Vec::from("Leading"), 
       opt: DictKeySpec::Optional,
    }; 
    let capheight_field = DictEntry { 
       key: Vec::from("CapHeight"), 
       opt: DictKeySpec::Optional,
    }; 
    let xheight_field = DictEntry { 
       key: Vec::from("XHeight"), 
       opt: DictKeySpec::Optional,
    }; 
    let stemv_field = DictEntry { 
       key: Vec::from("StemV"), 
       opt: DictKeySpec::Required,
    }; 
    let stemh_field = DictEntry { 
       key: Vec::from("StemH"), 
       opt: DictKeySpec::Optional,
    }; 
    let avgwidth_field = DictEntry { 
       key: Vec::from("AvgWidth"), 
       opt: DictKeySpec::Optional,
    }; 
    let maxwidth_field = DictEntry { 
       key: Vec::from("MaxWidth"), 
       opt: DictKeySpec::Optional,
    }; 
    let missingwidth_field = DictEntry { 
       key: Vec::from("MissingWidth"), 
       opt: DictKeySpec::Optional,
    }; 
    let fontfile_field = DictEntry { 
       key: Vec::from("FontFile"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         o_type,
         n_type,
         t_type,
         f_type,
         i_type,
         l_type,
         e_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let fontfile2_field = DictEntry { 
       key: Vec::from("FontFile2"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         o_type,
         n_type,
         t_type,
         f_type,
         i_type,
         l_type,
         e_type,
         2_type,
         c_type,
         i_type,
         d_type,
         t_type,
         y_type,
         p_type,
         e_type,
         2_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let charset_field = DictEntry { 
       key: Vec::from("CharSet"), 
       opt: DictKeySpec::Optional,
    }; 
    let style_field = DictEntry { 
       key: Vec::from("Style"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         s_type,
         t_type,
         y_type,
         l_type,
         e_type,
         d_type,
         i_type,
         c_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let lang_field = DictEntry { 
       key: Vec::from("Lang"), 
       opt: DictKeySpec::Optional,
    }; 
    let fd_field = DictEntry { 
       key: Vec::from("FD"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         d_type,
         d_type,
         i_type,
         c_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let cidset_field = DictEntry { 
       key: Vec::from("CIDSet"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         s_type,
         t_type,
         r_type,
         e_type,
         a_type,
         m_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "fontdescriptorcidtype2",
    Rc::new(PDFType::Dict(vec![
      type_field,
      fontname_field,
      fontfamily_field,
      fontstretch_field,
      fontweight_field,
      flags_field,
      fontbbox_field,
      italicangle_field,
      ascent_field,
      descent_field,
      leading_field,
      capheight_field,
      xheight_field,
      stemv_field,
      stemh_field,
      avgwidth_field,
      maxwidth_field,
      missingwidth_field,
      fontfile_field,
      fontfile2_field,
      charset_field,
      style_field,
      lang_field,
      fd_field,
      cidset_field,
   ]))
}