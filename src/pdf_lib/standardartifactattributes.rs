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
use crate::pdf_lib::arrayof_4namesforattached::arrayof_4namesforattached_type;
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
pub fn standardartifactattributes_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = arrayof_4namesforattached_type(tctx);
    let assignment_rectangle_0 = mk_rectangle_typchk(tctx);
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_0]));
    let dis_1 = TypeCheck::new(
        tctx,
        "attached",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(tctx, "bbox", assignments_disjuncts_0);
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Header"))),
            PDFObjT::Name(NameT::new(Vec::from("Footer"))),
            PDFObjT::Name(NameT::new(Vec::from("Watermark"))),
            PDFObjT::Name(NameT::new(Vec::from("PageNum"))),
            PDFObjT::Name(NameT::new(Vec::from("Bates"))),
            PDFObjT::Name(NameT::new(Vec::from("LineNum"))),
            PDFObjT::Name(NameT::new(Vec::from("Redaction"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Pagination"))),
            PDFObjT::Name(NameT::new(Vec::from("Layout"))),
            PDFObjT::Name(NameT::new(Vec::from("Page"))),
            PDFObjT::Name(NameT::new(Vec::from("Inline"))),
        ],
    );
    let choices_o = ChoicePred(
        String::from("Invalid O"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Artifact")))],
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
    let bbox_field = DictEntry {
        key: Vec::from("BBox"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let attached_field = DictEntry {
        key: Vec::from("Attached"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let subtype_field = DictEntry {
        key: Vec::from("Subtype"),
        chk: TypeCheck::new_refined(
            tctx,
            "subtype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_subtype),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "standardartifactattributes",
        Rc::new(PDFType::Dict(vec![
            o_field,
            type_field,
            bbox_field,
            attached_field,
            subtype_field,
        ])),
    )
}
