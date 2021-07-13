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
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::mediaclipdata::mediaclipdata_type;
use crate::pdf_lib::mediaclipsection::mediaclipsection_type;
use crate::pdf_lib::mediaplayparameters::mediaplayparameters_type;
use crate::pdf_lib::mediascreenparameters::mediascreenparameters_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::renditionbe::renditionbe_type;
use crate::pdf_lib::renditionmh::renditionmh_type;
use std::rc::Rc;
pub fn renditionmedia_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_6 = mediascreenparameters_type(tctx);
    let assignment_5 = mediaplayparameters_type(tctx);
    let assignment_4 = mediaclipdata_type(tctx);
    let assignment_3 = mediaclipsection_type(tctx);
    let assignment_2 = renditionbe_type(tctx);
    let assignment_1 = renditionmh_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "n", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let dis_4 = TypeCheck::new(tctx, "sp", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_3 = TypeCheck::new(tctx, "p", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_2 = TypeCheck::new(
        tctx,
        "c",
        Rc::new(PDFType::Disjunct(vec![assignment_3, assignment_4])),
    );
    let dis_1 = TypeCheck::new(tctx, "be", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_0 = TypeCheck::new(tctx, "mh", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![PDFObjT::Name(NameT::new(Vec::from("MR")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Rendition")))],
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
    let mh_field = DictEntry {
        key: Vec::from("MH"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let be_field = DictEntry {
        key: Vec::from("BE"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let sp_field = DictEntry {
        key: Vec::from("SP"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "renditionmedia",
        Rc::new(PDFType::Dict(vec![
            type_field, s_field, n_field, mh_field, be_field, c_field, p_field, sp_field,
        ])),
    )
}
