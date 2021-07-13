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
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::universalarray::universalarray_type;
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use std::rc::Rc;
pub fn markedcontentreference_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_5 = TypeCheck::new(
        tctx,
        "mcid",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_3 = universaldictionary_type(tctx);
    let assignment_2 = universalarray_type(tctx);
    let assignment_1 = stream_type(tctx);
    let assignment_4 = stream_type(tctx);
    let assignment_0 = pageobject_type(tctx);
    let dis_2 = TypeCheck::new(
        tctx,
        "stmown",
        Rc::new(PDFType::Disjunct(vec![
            assignment_2,
            assignment_3,
            assignment_4,
        ])),
    );
    let dis_1 = TypeCheck::new(tctx, "stm", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let dis_0 = TypeCheck::new(tctx, "pg", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("MCR")))],
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
    let pg_field = DictEntry {
        key: Vec::from("Pg"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let stm_field = DictEntry {
        key: Vec::from("Stm"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let stmown_field = DictEntry {
        key: Vec::from("StmOwn"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let mcid_field = DictEntry {
        key: Vec::from("MCID"),
        chk: assignment_integer_5,

        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "markedcontentreference",
        Rc::new(PDFType::Dict(vec![
            type_field,
            pg_field,
            stm_field,
            stmown_field,
            mcid_field,
        ])),
    )
}
