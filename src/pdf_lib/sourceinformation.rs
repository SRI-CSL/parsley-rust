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
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::urlalias::urlalias_type;
use crate::pdf_lib::webcapturecommand::webcapturecommand_type;
use std::rc::Rc;
pub fn sourceinformation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_3 = webcapturecommand_type(tctx);
    let assignment_date_1 = mk_date_typchk(tctx);
    let assignment_date_2 = mk_date_typchk(tctx);
    let assignment_0 = urlalias_type(tctx);
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_2]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_1]));
    let dis_3 = TypeCheck::new(tctx, "c", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_2 = TypeCheck::new(tctx, "e", assignments_disjuncts_1);
    let dis_1 = TypeCheck::new(tctx, "ts", assignments_disjuncts_0);
    let dis_0 = TypeCheck::new(tctx, "au", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
        ],
    );
    let au_field = DictEntry {
        key: Vec::from("AU"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let ts_field = DictEntry {
        key: Vec::from("TS"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let e_field = DictEntry {
        key: Vec::from("E"),
        chk: dis_2,
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
        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "sourceinformation",
        Rc::new(PDFType::Dict(vec![
            au_field, ts_field, e_field, s_field, c_field,
        ])),
    )
}
