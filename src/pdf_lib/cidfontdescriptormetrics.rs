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
         PDFObjT::Name(NameT::new(Vec::from("FontDescriptor"))),
           ],
     );
pub fn cidfontdescriptormetrics_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Required,
    }; 
    let fontname_field = DictEntry { 
       key: Vec::from("FontName"), 
       opt: DictKeySpec::Required,
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
    let charset_field = DictEntry { 
       key: Vec::from("CharSet"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "cidfontdescriptormetrics",
    Rc::new(PDFType::Dict(vec![
      type_field,
      fontname_field,
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
      charset_field,
   ]))
}