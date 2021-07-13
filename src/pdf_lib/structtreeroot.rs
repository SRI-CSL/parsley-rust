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
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::arrayofnamespace::arrayofnamespace_type;
use crate::pdf_lib::arrayofstructelem::arrayofstructelem_type;
use crate::pdf_lib::classmap::classmap_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::markedcontentreference::markedcontentreference_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::objectreference::objectreference_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::rolemap::rolemap_type;
use crate::pdf_lib::structelem::structelem_type;
use std::rc::Rc;
pub fn structtreeroot_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_15 = filespecification_type(tctx);
    let assignment_13 = filespecification_type(tctx);
    let assignment_12 = arrayoffilespecifications_type(tctx);
    let assignment_14 = arrayoffilespecifications_type(tctx);
    let assignment_11 = arrayofnamespace_type(tctx);
    let assignment_10 = classmap_type(tctx);
    let assignment_9 = rolemap_type(tctx);
    let assignment_integer_8 = TypeCheck::new(
        tctx,
        "parenttreenextkey",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_7 = objectreference_type(tctx);
    let assignment_4 = objectreference_type(tctx);
    let assignment_3 = markedcontentreference_type(tctx);
    let assignment_6 = markedcontentreference_type(tctx);
    let assignment_1 = structelem_type(tctx);
    let assignment_2 = structelem_type(tctx);
    let assignment_5 = structelem_type(tctx);
    let assignment_0 = arrayofstructelem_type(tctx);
    let dis_7 = TypeCheck::new(
        tctx,
        "af",
        Rc::new(PDFType::Disjunct(vec![assignment_14, assignment_15])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "pronunciationlexicon",
        Rc::new(PDFType::Disjunct(vec![assignment_12, assignment_13])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "namespaces",
        Rc::new(PDFType::Disjunct(vec![assignment_11])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "classmap",
        Rc::new(PDFType::Disjunct(vec![assignment_10])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "rolemap",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "parenttree",
        Rc::new(PDFType::Disjunct(vec![
            assignment_5,
            assignment_6,
            assignment_7,
        ])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "idtree",
        Rc::new(PDFType::Disjunct(vec![
            assignment_2,
            assignment_3,
            assignment_4,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "k",
        Rc::new(PDFType::Disjunct(vec![assignment_0, assignment_1])),
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("StructTreeRoot")))],
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
    let k_field = DictEntry {
        key: Vec::from("K"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let idtree_field = DictEntry {
        key: Vec::from("IDTree"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let parenttree_field = DictEntry {
        key: Vec::from("ParentTree"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let parenttreenextkey_field = DictEntry {
        key: Vec::from("ParentTreeNextKey"),
        chk: assignment_integer_8,

        opt: DictKeySpec::Optional,
    };
    let rolemap_field = DictEntry {
        key: Vec::from("RoleMap"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let classmap_field = DictEntry {
        key: Vec::from("ClassMap"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let namespaces_field = DictEntry {
        key: Vec::from("Namespaces"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let pronunciationlexicon_field = DictEntry {
        key: Vec::from("PronunciationLexicon"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "structtreeroot",
        Rc::new(PDFType::Dict(vec![
            type_field,
            k_field,
            idtree_field,
            parenttree_field,
            parenttreenextkey_field,
            rolemap_field,
            classmap_field,
            namespaces_field,
            pronunciationlexicon_field,
            af_field,
        ])),
    )
}
