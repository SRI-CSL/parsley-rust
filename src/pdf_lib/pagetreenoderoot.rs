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
use crate::pdf_lib::arrayofpagetreenodekids::arrayofpagetreenodekids_type;
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
use crate::pdf_lib::resource::resource_type;
use std::rc::Rc;
pub fn pagetreenoderoot_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_5 = TypeCheck::new(
        tctx,
        "rotate",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_rectangle_4 = mk_rectangle_typchk(tctx);
    let assignment_rectangle_3 = mk_rectangle_typchk(tctx);
    let assignment_2 = resource_type(tctx);
    let assignment_integer_1 = TypeCheck::new(
        tctx,
        "count",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_0 = arrayofpagetreenodekids_type(tctx);
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_4]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_3]));
    let dis_3 = TypeCheck::new(tctx, "cropbox", assignments_disjuncts_1);
    let dis_2 = TypeCheck::new(tctx, "mediabox", assignments_disjuncts_0);
    let dis_1 = TypeCheck::new(
        tctx,
        "resources",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_0 = TypeCheck::new(tctx, "kids", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Pages")))],
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
    let kids_field = DictEntry {
        key: Vec::from("Kids"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let count_field = DictEntry {
        key: Vec::from("Count"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Required,
    };
    let resources_field = DictEntry {
        key: Vec::from("Resources"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let mediabox_field = DictEntry {
        key: Vec::from("MediaBox"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let cropbox_field = DictEntry {
        key: Vec::from("CropBox"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let rotate_field = DictEntry {
        key: Vec::from("Rotate"),
        chk: assignment_integer_5,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "pagetreenoderoot",
        Rc::new(PDFType::Dict(vec![
            type_field,
            kids_field,
            count_field,
            resources_field,
            mediabox_field,
            cropbox_field,
            rotate_field,
        ])),
    )
}
