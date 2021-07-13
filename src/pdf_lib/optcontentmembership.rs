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
use crate::pdf_lib::arrayofocg::arrayofocg_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::optcontentgroup::optcontentgroup_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::universalarray::universalarray_type;
use std::rc::Rc;
pub fn optcontentmembership_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_2 = universalarray_type(tctx);
    let assignment_1 = optcontentgroup_type(tctx);
    let assignment_0 = arrayofocg_type(tctx);
    let dis_1 = TypeCheck::new(tctx, "ve", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_0 = TypeCheck::new(
        tctx,
        "ocgs",
        Rc::new(PDFType::Disjunct(vec![assignment_0, assignment_1])),
    );
    let choices_p = ChoicePred(
        String::from("Invalid P"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("AllOn"))),
            PDFObjT::Name(NameT::new(Vec::from("AnyOn"))),
            PDFObjT::Name(NameT::new(Vec::from("AnyOff"))),
            PDFObjT::Name(NameT::new(Vec::from("AllOff"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("OCMD")))],
    );
    let type_field = DictEntry {
        key: Vec::from("Type"),
        chk: TypeCheck::new_refined(
            tctx,
            "type",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_type),
        ),
        opt: DictKeySpec::Required,
    };
    let ocgs_field = DictEntry {
        key: Vec::from("OCGs"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: TypeCheck::new_refined(
            tctx,
            "p",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_p),
        ),
        opt: DictKeySpec::Optional,
    };
    let ve_field = DictEntry {
        key: Vec::from("VE"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "optcontentmembership",
        Rc::new(PDFType::Dict(vec![
            type_field, ocgs_field, p_field, ve_field,
        ])),
    )
}
