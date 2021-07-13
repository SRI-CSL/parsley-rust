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
use crate::pdf_lib::cryptfilter::cryptfilter_type;
    let choices_filter = ChoicesPred(
        String::From("Invalid Filter"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Standard"))),
           ],
     );
    let choices_v = ChoicesPred(
        String::From("Invalid V"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("0)"))),
         PDFObjT::Name(NameT::new(Vec::from("1)"))),
         PDFObjT::Name(NameT::new(Vec::from("2))"))),
         PDFObjT::Name(NameT::new(Vec::from("3))"))),
         PDFObjT::Name(NameT::new(Vec::from("4))"))),
         PDFObjT::Name(NameT::new(Vec::from("5)"))),
           ],
     );
    let choices_r = ChoicesPred(
        String::From("Invalid R"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("2))"))),
         PDFObjT::Name(NameT::new(Vec::from("3))"))),
         PDFObjT::Name(NameT::new(Vec::from("4))"))),
         PDFObjT::Name(NameT::new(Vec::from("5))"))),
         PDFObjT::Name(NameT::new(Vec::from("6))"))),
           ],
     );
    let choices_p = ChoicesPred(
        String::From("Invalid P"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("2)"))),
         PDFObjT::Name(NameT::new(Vec::from("3"))),
         PDFObjT::Name(NameT::new(Vec::from("4"))),
         PDFObjT::Name(NameT::new(Vec::from("5"))),
         PDFObjT::Name(NameT::new(Vec::from("6"))),
         PDFObjT::Name(NameT::new(Vec::from("8)"))),
         PDFObjT::Name(NameT::new(Vec::from("9"))),
         PDFObjT::Name(NameT::new(Vec::from("11"))),
         PDFObjT::Name(NameT::new(Vec::from("12"))),
         PDFObjT::Name(NameT::new(Vec::from("32)"))),
           ],
     );
pub fn encryptionstandard_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let filter_field = DictEntry { 
       key: Vec::from("Filter"), 
       chk: choices_filter(tctx),
       opt: DictKeySpec::Required,
    }; 
    let subfilter_field = DictEntry { 
       key: Vec::from("SubFilter"), 
       opt: DictKeySpec::Optional,
    }; 
    let v_field = DictEntry { 
       key: Vec::from("V"), 
       chk: choices_v(tctx),
       opt: DictKeySpec::Required,
    }; 
    let length_field = DictEntry { 
       key: Vec::from("Length"), 
       chk: choices_length(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let cf_field = DictEntry { 
       key: Vec::from("CF"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         c_type,
         r_type,
         y_type,
         p_type,
         t_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let stmf_field = DictEntry { 
       key: Vec::from("StmF"), 
       chk: choices_stmf(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let strf_field = DictEntry { 
       key: Vec::from("StrF"), 
       opt: DictKeySpec::Optional,
    }; 
    let eff_field = DictEntry { 
       key: Vec::from("EFF"), 
       opt: DictKeySpec::Optional,
    }; 
    let r_field = DictEntry { 
       key: Vec::from("R"), 
       chk: choices_r(tctx),
       opt: DictKeySpec::Required,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       opt: DictKeySpec::Required,
    }; 
    let u_field = DictEntry { 
       key: Vec::from("U"), 
       opt: DictKeySpec::Required,
    }; 
    let oe_field = DictEntry { 
       key: Vec::from("OE"), 
       opt: DictKeySpec::Optional,
    }; 
    let ue_field = DictEntry { 
       key: Vec::from("UE"), 
       opt: DictKeySpec::Optional,
    }; 
    let p_field = DictEntry { 
       key: Vec::from("P"), 
       chk: choices_p(tctx),
       opt: DictKeySpec::Required,
    }; 
    let perms_field = DictEntry { 
       key: Vec::from("Perms"), 
       opt: DictKeySpec::Optional,
    }; 
    let encryptmetadata_field = DictEntry { 
       key: Vec::from("EncryptMetadata"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "encryptionstandard",
    Rc::new(PDFType::Dict(vec![
      filter_field,
      subfilter_field,
      v_field,
      length_field,
      cf_field,
      stmf_field,
      strf_field,
      eff_field,
      r_field,
      o_field,
      u_field,
      oe_field,
      ue_field,
      p_field,
      perms_field,
      encryptmetadata_field,
   ]))
}