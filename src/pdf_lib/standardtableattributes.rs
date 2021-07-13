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
use crate::pdf_lib::arrayofstringsbyte::arrayofstringsbyte_type;
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
pub fn standardtableattributes_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = TypeCheck::new(
        tctx,
        "short",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_3 = TypeCheck::new(
        tctx,
        "summary",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_2 = arrayofstringsbyte_type(tctx);
    let assignment_integer_1 = TypeCheck::new(
        tctx,
        "colspan",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_0 = TypeCheck::new(
        tctx,
        "rowspan",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "headers",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let choices_scope = ChoicePred(
        String::from("Invalid Scope"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Row"))),
            PDFObjT::Name(NameT::new(Vec::from("Column"))),
            PDFObjT::Name(NameT::new(Vec::from("Both"))),
        ],
    );
    let choices_o = ChoicePred(
        String::from("Invalid O"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Table")))],
    );
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: TypeCheck::new_refined(
            tctx,
            "o",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_o),
        ),
        opt: DictKeySpec::Required,
    };
    let rowspan_field = DictEntry {
        key: Vec::from("RowSpan"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let colspan_field = DictEntry {
        key: Vec::from("ColSpan"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Optional,
    };
    let headers_field = DictEntry {
        key: Vec::from("Headers"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let scope_field = DictEntry {
        key: Vec::from("Scope"),
        chk: TypeCheck::new_refined(
            tctx,
            "scope",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_scope),
        ),
        opt: DictKeySpec::Optional,
    };
    let summary_field = DictEntry {
        key: Vec::from("Summary"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let short_field = DictEntry {
        key: Vec::from("Short"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "standardtableattributes",
        Rc::new(PDFType::Dict(vec![
            o_field,
            rowspan_field,
            colspan_field,
            headers_field,
            scope_field,
            summary_field,
            short_field,
        ])),
    )
}
