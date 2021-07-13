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
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Type1C"))),
         PDFObjT::Name(NameT::new(Vec::from("CIDFontType0C"))),
         PDFObjT::Name(NameT::new(Vec::from("OpenType"))),
           ],
     );
pub fn fontfile_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let length1_field = DictEntry { 
       key: Vec::from("Length1"), 
       opt: DictKeySpec::Optional,
    }; 
    let length2_field = DictEntry { 
       key: Vec::from("Length2"), 
       opt: DictKeySpec::Optional,
    }; 
    let length3_field = DictEntry { 
       key: Vec::from("Length3"), 
       opt: DictKeySpec::Optional,
    }; 
    let subtype_field = DictEntry { 
       key: Vec::from("Subtype"), 
       chk: choices_subtype(tctx),
       opt: DictKeySpec::Optional,
    }; 
    let length_field = DictEntry { 
       key: Vec::from("Length"), 
       opt: DictKeySpec::Required,
    }; 
    let filter_field = DictEntry { 
       key: Vec::from("Filter"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         c_type,
         o_type,
         m_type,
         p_type,
         r_type,
         e_type,
         s_type,
         s_type,
         i_type,
         o_type,
         n_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         n_type,
         a_type,
         m_type,
         e_type,
         s_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let decodeparms_field = DictEntry { 
       key: Vec::from("DecodeParms"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         d_type,
         e_type,
         c_type,
         o_type,
         d_type,
         e_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         s_type,
         ]_type,
         ;_type,
         [_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         l_type,
         z_type,
         w_type,
         d_type,
         e_type,
         c_type,
         o_type,
         d_type,
         e_type,
         ,_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         f_type,
         l_type,
         a_type,
         t_type,
         e_type,
         d_type,
         e_type,
         c_type,
         o_type,
         d_type,
         e_type,
         ,_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         c_type,
         r_type,
         y_type,
         p_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let f_field = DictEntry { 
       key: Vec::from("F"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         f_type,
         i_type,
         l_type,
         e_type,
         s_type,
         p_type,
         e_type,
         c_type,
         i_type,
         f_type,
         i_type,
         c_type,
         a_type,
         t_type,
         i_type,
         o_type,
         n_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let ffilter_field = DictEntry { 
       key: Vec::from("FFilter"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         c_type,
         o_type,
         m_type,
         p_type,
         r_type,
         e_type,
         s_type,
         s_type,
         i_type,
         o_type,
         n_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         n_type,
         a_type,
         m_type,
         e_type,
         s_type,
         ]_type,
         ;_type,
         [_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let fdecodeparms_field = DictEntry { 
       key: Vec::from("FDecodeParms"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         d_type,
         e_type,
         c_type,
         o_type,
         d_type,
         e_type,
         p_type,
         a_type,
         r_type,
         a_type,
         m_type,
         s_type,
         ]_type,
         ;_type,
         [_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         l_type,
         z_type,
         w_type,
         d_type,
         e_type,
         c_type,
         o_type,
         d_type,
         e_type,
         ,_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         f_type,
         l_type,
         a_type,
         t_type,
         e_type,
         d_type,
         e_type,
         c_type,
         o_type,
         d_type,
         e_type,
         ,_type,
         f_type,
         i_type,
         l_type,
         t_type,
         e_type,
         r_type,
         c_type,
         r_type,
         y_type,
         p_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let dl_field = DictEntry { 
       key: Vec::from("DL"), 
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "fontfile",
    Rc::new(PDFType::Dict(vec![
      length1_field,
      length2_field,
      length3_field,
      subtype_field,
      length_field,
      filter_field,
      decodeparms_field,
      f_field,
      ffilter_field,
      fdecodeparms_field,
      dl_field,
   ]))
}