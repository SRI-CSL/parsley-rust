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
use crate::pdf_lib::universalarray::universalarray_type;
use std::rc::Rc;
pub fn opiversion20dict_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_8 = TypeCheck::new(
        tctx,
        "includedimagequality",
        Rc::new(PDFType::PrimType(PDFPrimType::Real)),
    );
    let assignment_7 = arrayof_2integers_type(tctx);
    let assignment_6 = universalarray_type(tctx);
    let assignment_bool_5 = TypeCheck::new(
        tctx,
        "overprint",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_rectangle_4 = mk_rectangle_typchk(tctx);
    let assignment_3 = arrayof_2numbers_type(tctx);
    let assignment_2 = arrayoftags_type(tctx);
    let assignment_1 = TypeCheck::new(
        tctx,
        "mainimage",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_0 = filespecification_type(tctx);
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_rectangle_4]));
    let dis_5 = TypeCheck::new(
        tctx,
        "includedimagedimensions",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_4 = TypeCheck::new(tctx, "inks", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_3 = TypeCheck::new(tctx, "croprect", assignments_disjuncts_0);
    let dis_2 = TypeCheck::new(tctx, "size", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_1 = TypeCheck::new(tctx, "tags", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_0 = TypeCheck::new(tctx, "f", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_version = ChoicePred(
        String::from("Invalid Version"),
        vec![PDFObjT::Name(NameT::new(Vec::from("2.0")))],
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
    let mainimage_field = DictEntry {
        key: Vec::from("MainImage"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let tags_field = DictEntry {
        key: Vec::from("Tags"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let size_field = DictEntry {
        key: Vec::from("Size"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let croprect_field = DictEntry {
        key: Vec::from("CropRect"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let overprint_field = DictEntry {
        key: Vec::from("Overprint"),
        chk: assignment_bool_5,

        opt: DictKeySpec::Optional,
    };
    let inks_field = DictEntry {
        key: Vec::from("Inks"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let includedimagedimensions_field = DictEntry {
        key: Vec::from("IncludedImageDimensions"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let includedimagequality_field = DictEntry {
        key: Vec::from("IncludedImageQuality"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "opiversion20dict",
        Rc::new(PDFType::Dict(vec![
            type_field,
            version_field,
            f_field,
            mainimage_field,
            tags_field,
            size_field,
            croprect_field,
            overprint_field,
            inks_field,
            includedimagedimensions_field,
            includedimagequality_field,
        ])),
    )
}
