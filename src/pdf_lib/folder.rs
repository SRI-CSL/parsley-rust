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
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
use crate::pdf_lib::collectionitem::collectionitem_type;
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
use crate::pdf_lib::thumbnail::thumbnail_type;
use std::rc::Rc;
pub fn folder_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_10 = arrayofnumbersgeneral_type(tctx);
    let assignment_9 = thumbnail_type(tctx);
    let assignment_date_7 = mk_date_typchk(tctx);
    let assignment_date_8 = mk_date_typchk(tctx);
    let assignment_6 = TypeCheck::new(
        tctx,
        "desc",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_5 = collectionitem_type(tctx);
    let assignment_2 = folder_type(tctx);
    let assignment_4 = folder_type(tctx);
    let assignment_3 = folder_type(tctx);
    let assignment_1 = TypeCheck::new(
        tctx,
        "name",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_integer_0 =
        TypeCheck::new(tctx, "id", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_8]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_7]));
    let dis_7 = TypeCheck::new(
        tctx,
        "free",
        Rc::new(PDFType::Disjunct(vec![assignment_10])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "thumb",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_5 = TypeCheck::new(tctx, "moddate", assignments_disjuncts_1);
    let dis_4 = TypeCheck::new(tctx, "creationdate", assignments_disjuncts_0);
    let dis_3 = TypeCheck::new(tctx, "ci", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_2 = TypeCheck::new(tctx, "next", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_1 = TypeCheck::new(
        tctx,
        "child",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Folder")))],
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
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Required,
    };
    let name_field = DictEntry {
        key: Vec::from("Name"),
        chk: assignment_1,

        opt: DictKeySpec::Required,
    };
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let child_field = DictEntry {
        key: Vec::from("Child"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let next_field = DictEntry {
        key: Vec::from("Next"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let ci_field = DictEntry {
        key: Vec::from("CI"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let desc_field = DictEntry {
        key: Vec::from("Desc"),
        chk: assignment_6,

        opt: DictKeySpec::Optional,
    };
    let creationdate_field = DictEntry {
        key: Vec::from("CreationDate"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let moddate_field = DictEntry {
        key: Vec::from("ModDate"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let thumb_field = DictEntry {
        key: Vec::from("Thumb"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let free_field = DictEntry {
        key: Vec::from("Free"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "folder",
        Rc::new(PDFType::Dict(vec![
            type_field,
            id_field,
            name_field,
            parent_field,
            child_field,
            next_field,
            ci_field,
            desc_field,
            creationdate_field,
            moddate_field,
            thumb_field,
            free_field,
        ])),
    )
}
