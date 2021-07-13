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
         PDFObjT::Name(NameT::new(Vec::from("NumberFormat"))),
           ],
     );
    let choices_f = ChoicesPred(
        String::From("Invalid F"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("D"))),
         PDFObjT::Name(NameT::new(Vec::from("F"))),
         PDFObjT::Name(NameT::new(Vec::from("R"))),
         PDFObjT::Name(NameT::new(Vec::from("T"))),
           ],
     );
    let choices_o = ChoicesPred(
        String::From("Invalid O"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("S"))),
         PDFObjT::Name(NameT::new(Vec::from("P"))),
           ],
     );
pub fn numberformat_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let u_field = DictEntry { 
       key: Vec::from("U"), 
       opt: DictKeySpec::Required,
    }; 
    let c_field = DictEntry { 
       key: Vec::from("C"), 
       opt: DictKeySpec::Required,
    }; 
    let f_field = DictEntry { 
       key: Vec::from("F"), 
       chk: choices_f(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let d_field = DictEntry { 
       key: Vec::from("D"), 
       opt: DictKeySpec::Optional,
    }; 
    let fd_field = DictEntry { 
       key: Vec::from("FD"), 
       opt: DictKeySpec::Optional,
    }; 
    let rt_field = DictEntry { 
       key: Vec::from("RT"), 
       opt: DictKeySpec::Optional,
    }; 
    let rd_field = DictEntry { 
       key: Vec::from("RD"), 
       opt: DictKeySpec::Optional,
    }; 
    let ps_field = DictEntry { 
       key: Vec::from("PS"), 
       opt: DictKeySpec::Optional,
    }; 
    let ss_field = DictEntry { 
       key: Vec::from("SS"), 
       opt: DictKeySpec::Optional,
    }; 
    let o_field = DictEntry { 
       key: Vec::from("O"), 
       chk: choices_o(tctx),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "numberformat",
    Rc::new(PDFType::Dict(vec![
      type_field,
      u_field,
      c_field,
      f_field,
      d_field,
      fd_field,
      rt_field,
      rd_field,
      ps_field,
      ss_field,
      o_field,
   ]))
}