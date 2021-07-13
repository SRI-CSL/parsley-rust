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
use crate::pdf_lib::bead::bead_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::thread::thread_type;
use std::rc::Rc;
pub fn beadfirst_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_rectangle_6 = mk_rectangle_typchk(tctx);
    let assignment_5 = pageobject_type(tctx);
    let assignment_2 = bead_type(tctx);
    let assignment_4 = bead_type(tctx);
    let assignment_1 = beadfirst_type(tctx);
    let assignment_3 = beadfirst_type(tctx);
    let assignment_0 = thread_type(tctx);
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_6]));
    let dis_4 = TypeCheck::new(tctx, "r", assignments_disjuncts_0);
    let dis_3 = TypeCheck::new(tctx, "p", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_2 = TypeCheck::new(
        tctx,
        "v",
        Rc::new(PDFType::Disjunct(vec![assignment_3, assignment_4])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "n",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let dis_0 = TypeCheck::new(tctx, "t", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Bead")))],
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
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let n_field = DictEntry {
        key: Vec::from("N"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: dis_2,
        opt: DictKeySpec::Required,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_3,
        opt: DictKeySpec::Required,
    };
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: dis_4,
        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "beadfirst",
        Rc::new(PDFType::Dict(vec![
            type_field, t_field, n_field, v_field, p_field, r_field,
        ])),
    )
}
