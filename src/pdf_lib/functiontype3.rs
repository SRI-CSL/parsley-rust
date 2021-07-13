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
use crate::pdf_lib::arrayoffunctions::arrayoffunctions_type;
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
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
use std::rc::Rc;
pub fn functiontype3_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_2 = arrayoffunctions_type(tctx);
    let assignment_1 = arrayofnumbersgeneral_type(tctx);
    let assignment_0 = arrayofnumbersgeneral_type(tctx);
    let assignment_4 = arrayofnumbersgeneral_type(tctx);
    let assignment_3 = arrayofnumbersgeneral_type(tctx);
    let dis_4 = TypeCheck::new(
        tctx,
        "encode",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "bounds",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "functions",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "range",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "domain",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_functiontype = ChoicePred(
        String::from("Invalid FunctionType"),
        vec![PDFObjT::Name(NameT::new(Vec::from("3")))],
    );
    let functiontype_field = DictEntry {
        key: Vec::from("FunctionType"),
        chk: TypeCheck::new_refined(
            tctx,
            "functiontype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_functiontype),
        ),
        opt: DictKeySpec::Required,
    };
    let domain_field = DictEntry {
        key: Vec::from("Domain"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let range_field = DictEntry {
        key: Vec::from("Range"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let functions_field = DictEntry {
        key: Vec::from("Functions"),
        chk: dis_2,
        opt: DictKeySpec::Required,
    };
    let bounds_field = DictEntry {
        key: Vec::from("Bounds"),
        chk: dis_3,
        opt: DictKeySpec::Required,
    };
    let encode_field = DictEntry {
        key: Vec::from("Encode"),
        chk: dis_4,
        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "functiontype3",
        Rc::new(PDFType::Dict(vec![
            functiontype_field,
            domain_field,
            range_field,
            functions_field,
            bounds_field,
            encode_field,
        ])),
    )
}
