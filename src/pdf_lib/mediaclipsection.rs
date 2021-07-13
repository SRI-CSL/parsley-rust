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
use crate::pdf_lib::arrayofmultilangtext::arrayofmultilangtext_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::mediaclipdata::mediaclipdata_type;
use crate::pdf_lib::mediaclipsectionmhbe::mediaclipsectionmhbe_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn mediaclipsection_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_5 = mediaclipsectionmhbe_type(tctx);
    let assignment_4 = mediaclipsectionmhbe_type(tctx);
    let assignment_3 = arrayofmultilangtext_type(tctx);
    let assignment_2 = mediaclipdata_type(tctx);
    let assignment_1 = mediaclipsection_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "n", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let dis_3 = TypeCheck::new(tctx, "be", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_2 = TypeCheck::new(tctx, "mh", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_1 = TypeCheck::new(tctx, "alt", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(
        tctx,
        "d",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![PDFObjT::Name(NameT::new(Vec::from("MCS")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("MediaClip")))],
    );
    let type_field = DictEntry {
        key: Vec::from("Type"),
        chk: TypeCheck::new_refined(
            tctx,
            "type",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_type),
        ),
        opt: DictKeySpec::Optional,
    };
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: TypeCheck::new_refined(
            tctx,
            "s",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_s),
        ),
        opt: DictKeySpec::Required,
    };
    let n_field = DictEntry {
        key: Vec::from("N"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let alt_field = DictEntry {
        key: Vec::from("Alt"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let mh_field = DictEntry {
        key: Vec::from("MH"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let be_field = DictEntry {
        key: Vec::from("BE"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "mediaclipsection",
        Rc::new(PDFType::Dict(vec![
            type_field, s_field, n_field, d_field, alt_field, mh_field, be_field,
        ])),
    )
}
