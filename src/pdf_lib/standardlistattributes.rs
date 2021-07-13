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
use std::rc::Rc;
pub fn standardlistattributes_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = TypeCheck::new(
        tctx,
        "continuedform",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_bool_0 = TypeCheck::new(
        tctx,
        "continuedlist",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let choices_listnumbering = ChoicePred(
        String::from("Invalid ListNumbering"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("None"))),
            PDFObjT::Name(NameT::new(Vec::from("Unordered"))),
            PDFObjT::Name(NameT::new(Vec::from("Description"))),
            PDFObjT::Name(NameT::new(Vec::from("Disc"))),
            PDFObjT::Name(NameT::new(Vec::from("Circle"))),
            PDFObjT::Name(NameT::new(Vec::from("Square"))),
            PDFObjT::Name(NameT::new(Vec::from("Ordered"))),
            PDFObjT::Name(NameT::new(Vec::from("Decimal"))),
            PDFObjT::Name(NameT::new(Vec::from("UpperRoman"))),
            PDFObjT::Name(NameT::new(Vec::from("LowerRoman"))),
            PDFObjT::Name(NameT::new(Vec::from("UpperAlpha"))),
            PDFObjT::Name(NameT::new(Vec::from("LowerAlpha"))),
        ],
    );
    let choices_o = ChoicePred(
        String::from("Invalid O"),
        vec![PDFObjT::Name(NameT::new(Vec::from("List")))],
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
    let listnumbering_field = DictEntry {
        key: Vec::from("ListNumbering"),
        chk: TypeCheck::new_refined(
            tctx,
            "listnumbering",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_listnumbering),
        ),
        opt: DictKeySpec::Optional,
    };
    let continuedlist_field = DictEntry {
        key: Vec::from("ContinuedList"),
        chk: assignment_bool_0,

        opt: DictKeySpec::Optional,
    };
    let continuedform_field = DictEntry {
        key: Vec::from("ContinuedForm"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "standardlistattributes",
        Rc::new(PDFType::Dict(vec![
            o_field,
            listnumbering_field,
            continuedlist_field,
            continuedform_field,
        ])),
    )
}
