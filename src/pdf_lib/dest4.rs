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
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn dest4_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_0 = pageobject_type(tctx);
    let dis_3 = TypeCheck::new(tctx, "4", Rc::new(PDFType::Disjunct(vec![])));
    let dis_2 = TypeCheck::new(tctx, "3", Rc::new(PDFType::Disjunct(vec![])));
    let dis_1 = TypeCheck::new(tctx, "2", Rc::new(PDFType::Disjunct(vec![])));
    let dis_0 = TypeCheck::new(tctx, "0", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_one = ChoicePred(
        String::from("Invalid 1"),
        vec![PDFObjT::Name(NameT::new(Vec::from("FitR")))],
    );
    let zero_field = DictEntry {
        key: Vec::from("0"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let one_field = DictEntry {
        key: Vec::from("1"),
        chk: TypeCheck::new_refined(
            tctx,
            "1",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_one),
        ),
        opt: DictKeySpec::Required,
    };
    let two_field = DictEntry {
        key: Vec::from("2"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let three_field = DictEntry {
        key: Vec::from("3"),
        chk: dis_2,
        opt: DictKeySpec::Required,
    };
    let four_field = DictEntry {
        key: Vec::from("4"),
        chk: dis_3,
        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "dest4",
        Rc::new(PDFType::Dict(vec![
            zero_field,
            one_field,
            two_field,
            three_field,
            four_field,
        ])),
    )
}
