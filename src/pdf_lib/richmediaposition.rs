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
pub fn richmediaposition_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = TypeCheck::new(
        tctx,
        "voffset",
        Rc::new(PDFType::PrimType(PDFPrimType::Real)),
    );
    let assignment_0 = TypeCheck::new(
        tctx,
        "hoffset",
        Rc::new(PDFType::PrimType(PDFPrimType::Real)),
    );
    let choices_valign = ChoicePred(
        String::from("Invalid VAlign"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Near"))),
            PDFObjT::Name(NameT::new(Vec::from("Center"))),
            PDFObjT::Name(NameT::new(Vec::from("Far"))),
        ],
    );
    let choices_halign = ChoicePred(
        String::from("Invalid HAlign"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Near"))),
            PDFObjT::Name(NameT::new(Vec::from("Center"))),
            PDFObjT::Name(NameT::new(Vec::from("Far"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("RichMediaPosition")))],
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
    let halign_field = DictEntry {
        key: Vec::from("HAlign"),
        chk: TypeCheck::new_refined(
            tctx,
            "halign",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_halign),
        ),
        opt: DictKeySpec::Optional,
    };
    let valign_field = DictEntry {
        key: Vec::from("VAlign"),
        chk: TypeCheck::new_refined(
            tctx,
            "valign",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_valign),
        ),
        opt: DictKeySpec::Optional,
    };
    let hoffset_field = DictEntry {
        key: Vec::from("HOffset"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let voffset_field = DictEntry {
        key: Vec::from("VOffset"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "richmediaposition",
        Rc::new(PDFType::Dict(vec![
            type_field,
            halign_field,
            valign_field,
            hoffset_field,
            voffset_field,
        ])),
    )
}
