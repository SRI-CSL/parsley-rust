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
use crate::pdf_lib::iccprofilestream::iccprofilestream_type;
use crate::pdf_lib::destoutputprofileref::destoutputprofileref_type;
use crate::pdf_lib::devicenmixinghints::devicenmixinghints_type;
use crate::pdf_lib::spectraldata::spectraldata_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("OutputIntent"))),
           ],
     );
pub fn outputintents_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let type_field = DictEntry { 
       key: Vec::from("Type"), 
       chk: choices_type(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let s_field = DictEntry { 
       key: Vec::from("S"), 
       opt: DictKeySpec::Required,
    }; 
    let outputcondition_field = DictEntry { 
       key: Vec::from("OutputCondition"), 
       opt: DictKeySpec::Optional,
    }; 
    let outputconditionidentifier_field = DictEntry { 
       key: Vec::from("OutputConditionIdentifier"), 
       opt: DictKeySpec::Required,
    }; 
    let registryname_field = DictEntry { 
       key: Vec::from("RegistryName"), 
       opt: DictKeySpec::Optional,
    }; 
    let info_field = DictEntry { 
       key: Vec::from("Info"), 
       opt: DictKeySpec::Optional,
    }; 
    let destoutputprofile_field = DictEntry { 
       key: Vec::from("DestOutputProfile"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         i_type,
         c_type,
         c_type,
         p_type,
         r_type,
         o_type,
         f_type,
         i_type,
         l_type,
         e_type,
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
    let destoutputprofileref_field = DictEntry { 
       key: Vec::from("DestOutputProfileRef"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         d_type,
         e_type,
         s_type,
         t_type,
         o_type,
         u_type,
         t_type,
         p_type,
         u_type,
         t_type,
         p_type,
         r_type,
         o_type,
         f_type,
         i_type,
         l_type,
         e_type,
         r_type,
         e_type,
         f_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let mixinghints_field = DictEntry { 
       key: Vec::from("MixingHints"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         d_type,
         e_type,
         v_type,
         i_type,
         c_type,
         e_type,
         n_type,
         m_type,
         i_type,
         x_type,
         i_type,
         n_type,
         g_type,
         h_type,
         i_type,
         n_type,
         t_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let spectraldata_field = DictEntry { 
       key: Vec::from("SpectralData"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         s_type,
         p_type,
         e_type,
         c_type,
         t_type,
         r_type,
         a_type,
         l_type,
         d_type,
         a_type,
         t_type,
         a_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "outputintents",
    Rc::new(PDFType::Dict(vec![
      type_field,
      s_field,
      outputcondition_field,
      outputconditionidentifier_field,
      registryname_field,
      info_field,
      destoutputprofile_field,
      destoutputprofileref_field,
      mixinghints_field,
      spectraldata_field,
   ]))
}