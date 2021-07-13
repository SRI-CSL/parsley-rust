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
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::appearancetrapnet::appearancetrapnet_type;
use crate::pdf_lib::arrayof_4annotbordercharacteristics::arrayof_4annotbordercharacteristics_type;
use crate::pdf_lib::arrayof_4numberscolorannotation::arrayof_4numberscolorannotation_type;
use crate::pdf_lib::optcontentgroup::optcontentgroup_type;
use crate::pdf_lib::optcontentmembership::optcontentmembership_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::_universalarray::_universalarray_type;
use crate::pdf_lib::arrayofannotstates::arrayofannotstates_type;
use crate::pdf_lib::arrayoffonts::arrayoffonts_type;
    let choices_type = ChoicesPred(
        String::From("Invalid Type"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("Annot"))),
           ],
     );
    let choices_subtype = ChoicesPred(
        String::From("Invalid Subtype"), 
        vec![ 
         PDFObjT::Name(NameT::new(Vec::from("TrapNet"))),
           ],
     );
pub fn annottrapnetwork_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
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
    let rect_field = DictEntry { 
       key: Vec::from("Rect"), 
       opt: DictKeySpec::Required,
    }; 
    let contents_field = DictEntry { 
       key: Vec::from("Contents"), 
       opt: DictKeySpec::Optional,
    }; 
    let p_field = DictEntry { 
       key: Vec::from("P"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         p_type,
         a_type,
         g_type,
         e_type,
         o_type,
         b_type,
         j_type,
         e_type,
         c_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let nm_field = DictEntry { 
       key: Vec::from("NM"), 
       opt: DictKeySpec::Optional,
    }; 
    let m_field = DictEntry { 
       key: Vec::from("M"), 
       opt: DictKeySpec::Optional,
    }; 
    let f_field = DictEntry { 
       key: Vec::from("F"), 
       opt: DictKeySpec::Required,
    }; 
    let ap_field = DictEntry { 
       key: Vec::from("AP"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         p_type,
         p_type,
         e_type,
         a_type,
         r_type,
         a_type,
         n_type,
         c_type,
         e_type,
         t_type,
         r_type,
         a_type,
         p_type,
         n_type,
         e_type,
         t_type,
         ]_type,
]),
       opt: DictKeySpec::Required,
    }; 
    let as_field = DictEntry { 
       key: Vec::from("AS"), 
       opt: DictKeySpec::Required,
    }; 
    let border_field = DictEntry { 
       key: Vec::from("Border"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         4_type,
         a_type,
         n_type,
         n_type,
         o_type,
         t_type,
         b_type,
         o_type,
         r_type,
         d_type,
         e_type,
         r_type,
         c_type,
         h_type,
         a_type,
         r_type,
         a_type,
         c_type,
         t_type,
         e_type,
         r_type,
         i_type,
         s_type,
         t_type,
         i_type,
         c_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let c_field = DictEntry { 
       key: Vec::from("C"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         __type,
         4_type,
         n_type,
         u_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         c_type,
         o_type,
         l_type,
         o_type,
         r_type,
         a_type,
         n_type,
         n_type,
         o_type,
         t_type,
         a_type,
         t_type,
         i_type,
         o_type,
         n_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let structparent_field = DictEntry { 
       key: Vec::from("StructParent"), 
       opt: DictKeySpec::Optional,
    }; 
    let oc_field = DictEntry { 
       key: Vec::from("OC"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         o_type,
         p_type,
         t_type,
         c_type,
         o_type,
         n_type,
         t_type,
         e_type,
         n_type,
         t_type,
         g_type,
         r_type,
         o_type,
         u_type,
         p_type,
         ,_type,
         o_type,
         p_type,
         t_type,
         c_type,
         o_type,
         n_type,
         t_type,
         e_type,
         n_type,
         t_type,
         m_type,
         e_type,
         m_type,
         b_type,
         e_type,
         r_type,
         s_type,
         h_type,
         i_type,
         p_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let af_field = DictEntry { 
       key: Vec::from("AF"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
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
         s_type,
         ]_type,
         ;_type,
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
]),
       opt: DictKeySpec::Optional,
    }; 
    let ca_field = DictEntry { 
       key: Vec::from("ca"), 
       opt: DictKeySpec::Optional,
    }; 
    let ca_field = DictEntry { 
       key: Vec::from("CA"), 
       opt: DictKeySpec::Optional,
    }; 
    let bm_field = DictEntry { 
       key: Vec::from("BM"), 
       opt: DictKeySpec::Optional,
    }; 
    let lang_field = DictEntry { 
       key: Vec::from("Lang"), 
       opt: DictKeySpec::Optional,
    }; 
    let lastmodified_field = DictEntry { 
       key: Vec::from("LastModified"), 
       opt: DictKeySpec::Optional,
    }; 
    let version_field = DictEntry { 
       key: Vec::from("Version"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         __type,
         u_type,
         n_type,
         i_type,
         v_type,
         e_type,
         r_type,
         s_type,
         a_type,
         l_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let annotstates_field = DictEntry { 
       key: Vec::from("AnnotStates"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         a_type,
         n_type,
         n_type,
         o_type,
         t_type,
         s_type,
         t_type,
         a_type,
         t_type,
         e_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    let fontfauxing_field = DictEntry { 
       key: Vec::from("FontFauxing"), 
       chk: Rc::new(PDFType::Disjunct(vec![
         [_type,
         a_type,
         r_type,
         r_type,
         a_type,
         y_type,
         o_type,
         f_type,
         f_type,
         o_type,
         n_type,
         t_type,
         s_type,
         ]_type,
]),
       opt: DictKeySpec::Optional,
    }; 
    TypeCheck::new(
    tctx,
    "annottrapnetwork",
    Rc::new(PDFType::Dict(vec![
      type_field,
      subtype_field,
      rect_field,
      contents_field,
      p_field,
      nm_field,
      m_field,
      f_field,
      ap_field,
      as_field,
      border_field,
      c_field,
      structparent_field,
      oc_field,
      af_field,
      ca_field,
      ca_field,
      bm_field,
      lang_field,
      lastmodified_field,
      version_field,
      annotstates_field,
      fontfauxing_field,
   ]))
}