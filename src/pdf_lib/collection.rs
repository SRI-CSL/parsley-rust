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
use crate::pdf_lib::collectioncolors::collectioncolors_type;
use crate::pdf_lib::collectionschema::collectionschema_type;
use crate::pdf_lib::collectionsort::collectionsort_type;
use crate::pdf_lib::collectionsplit::collectionsplit_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::folder::folder_type;
use crate::pdf_lib::navigator::navigator_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn collection_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_6 = collectionsplit_type(tctx);
    let assignment_5 = folder_type(tctx);
    let assignment_4 = collectionsort_type(tctx);
    let assignment_3 = collectioncolors_type(tctx);
    let assignment_2 = navigator_type(tctx);
    let assignment_1 = TypeCheck::new(tctx, "d", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_0 = collectionschema_type(tctx);
    let dis_5 = TypeCheck::new(
        tctx,
        "split",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "folders",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_3 = TypeCheck::new(tctx, "sort", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_2 = TypeCheck::new(
        tctx,
        "colors",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "navigator",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "schema",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_view = ChoicePred(
        String::from("Invalid View"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("D"))),
            PDFObjT::Name(NameT::new(Vec::from("T"))),
            PDFObjT::Name(NameT::new(Vec::from("H"))),
            PDFObjT::Name(NameT::new(Vec::from("C"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Collection")))],
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
    let schema_field = DictEntry {
        key: Vec::from("Schema"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let view_field = DictEntry {
        key: Vec::from("View"),
        chk: TypeCheck::new_refined(
            tctx,
            "view",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_view),
        ),
        opt: DictKeySpec::Optional,
    };
    let navigator_field = DictEntry {
        key: Vec::from("Navigator"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let colors_field = DictEntry {
        key: Vec::from("Colors"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let sort_field = DictEntry {
        key: Vec::from("Sort"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let folders_field = DictEntry {
        key: Vec::from("Folders"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let split_field = DictEntry {
        key: Vec::from("Split"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "collection",
        Rc::new(PDFType::Dict(vec![
            type_field,
            schema_field,
            d_field,
            view_field,
            navigator_field,
            colors_field,
            sort_field,
            folders_field,
            split_field,
        ])),
    )
}
