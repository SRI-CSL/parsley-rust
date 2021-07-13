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
use crate::pdf_lib::arrayof_2integers::arrayof_2integers_type;
use crate::pdf_lib::arrayof_2numbers::arrayof_2numbers_type;
use crate::pdf_lib::arrayof_4integers::arrayof_4integers_type;
use crate::pdf_lib::arrayof_8numbers::arrayof_8numbers_type;
use crate::pdf_lib::arrayofintegersgeneral::arrayofintegersgeneral_type;
use crate::pdf_lib::arrayofopi13color::arrayofopi13color_type;
use crate::pdf_lib::arrayoftags::arrayoftags_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn opiversion13dict_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_15 = arrayoftags_type(tctx);
    let assignment_bool_14 = TypeCheck::new(
        tctx,
        "transparency",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_13 = arrayofintegersgeneral_type(tctx);
    let assignment_bool_11 = TypeCheck::new(
        tctx,
        "overprint",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_10 = TypeCheck::new(tctx, "tint", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_9 = arrayofopi13color_type(tctx);
    let assignment_8 = TypeCheck::new(
        tctx,
        "colortype",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_7 = arrayof_2numbers_type(tctx);
    let assignment_6 = arrayof_8numbers_type(tctx);
    let assignment_5 = arrayof_4integers_type(tctx);
    let assignment_rectangle_4 = mk_rectangle_typchk(tctx);
    let assignment_12 = arrayof_2integers_type(tctx);
    let assignment_3 = arrayof_2integers_type(tctx);
    let assignment_2 = TypeCheck::new(
        tctx,
        "comments",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_1 = TypeCheck::new(tctx, "id", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_0 = filespecification_type(tctx);
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_4]));
    let dis_9 = TypeCheck::new(
        tctx,
        "tags",
        Rc::new(PDFType::Disjunct(vec![assignment_15])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "grapmap",
        Rc::new(PDFType::Disjunct(vec![assignment_13])),
    );
    let dis_7 = TypeCheck::new(
        tctx,
        "imagetype",
        Rc::new(PDFType::Disjunct(vec![assignment_12])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "color",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "resolution",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "position",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "cropfixed",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_2 = TypeCheck::new(tctx, "croprect", assignments_disjuncts_0);
    let dis_1 = TypeCheck::new(tctx, "size", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_version = ChoicePred(
        String::from("Invalid Version"),
        vec![PDFObjT::Name(NameT::new(Vec::from("1.3")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("OPI")))],
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
    let version_field = DictEntry {
        key: Vec::from("Version"),
        chk: TypeCheck::new_refined(
            tctx,
            "version",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_version),
        ),
        opt: DictKeySpec::Required,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let comments_field = DictEntry {
        key: Vec::from("Comments"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let size_field = DictEntry {
        key: Vec::from("Size"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let croprect_field = DictEntry {
        key: Vec::from("CropRect"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let cropfixed_field = DictEntry {
        key: Vec::from("CropFixed"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let position_field = DictEntry {
        key: Vec::from("Position"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let resolution_field = DictEntry {
        key: Vec::from("Resolution"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let colortype_field = DictEntry {
        key: Vec::from("ColorType"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    let color_field = DictEntry {
        key: Vec::from("Color"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let tint_field = DictEntry {
        key: Vec::from("Tint"),
        chk: assignment_10,

        opt: DictKeySpec::Optional,
    };
    let overprint_field = DictEntry {
        key: Vec::from("Overprint"),
        chk: assignment_bool_11,

        opt: DictKeySpec::Optional,
    };
    let imagetype_field = DictEntry {
        key: Vec::from("ImageType"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let grapmap_field = DictEntry {
        key: Vec::from("GrapMap"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let transparency_field = DictEntry {
        key: Vec::from("Transparency"),
        chk: assignment_bool_14,

        opt: DictKeySpec::Optional,
    };
    let tags_field = DictEntry {
        key: Vec::from("Tags"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "opiversion13dict",
        Rc::new(PDFType::Dict(vec![
            type_field,
            version_field,
            f_field,
            id_field,
            comments_field,
            size_field,
            croprect_field,
            cropfixed_field,
            position_field,
            resolution_field,
            colortype_field,
            color_field,
            tint_field,
            overprint_field,
            imagetype_field,
            grapmap_field,
            transparency_field,
            tags_field,
        ])),
    )
}
