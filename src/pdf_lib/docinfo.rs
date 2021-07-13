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
pub fn docinfo_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_date_7 = mk_date_typchk(tctx);
    let assignment_date_6 = mk_date_typchk(tctx);
    let assignment_5 = TypeCheck::new(
        tctx,
        "producer",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_4 = TypeCheck::new(
        tctx,
        "creator",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_3 = TypeCheck::new(
        tctx,
        "keywords",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_2 = TypeCheck::new(
        tctx,
        "subject",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_1 = TypeCheck::new(
        tctx,
        "author",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_0 = TypeCheck::new(
        tctx,
        "title",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_7]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_6]));
    let dis_1 = TypeCheck::new(tctx, "moddate", assignments_disjuncts_1);
    let dis_0 = TypeCheck::new(tctx, "creationdate", assignments_disjuncts_0);
    let choices_trapped = ChoicePred(
        String::from("Invalid Trapped"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("True"))),
            PDFObjT::Name(NameT::new(Vec::from("False"))),
            PDFObjT::Name(NameT::new(Vec::from("Unknown"))),
        ],
    );
    let title_field = DictEntry {
        key: Vec::from("Title"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let author_field = DictEntry {
        key: Vec::from("Author"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let subject_field = DictEntry {
        key: Vec::from("Subject"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let keywords_field = DictEntry {
        key: Vec::from("Keywords"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let creator_field = DictEntry {
        key: Vec::from("Creator"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let producer_field = DictEntry {
        key: Vec::from("Producer"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let creationdate_field = DictEntry {
        key: Vec::from("CreationDate"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let moddate_field = DictEntry {
        key: Vec::from("ModDate"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let trapped_field = DictEntry {
        key: Vec::from("Trapped"),
        chk: TypeCheck::new_refined(
            tctx,
            "trapped",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_trapped),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "docinfo",
        Rc::new(PDFType::Dict(vec![
            title_field,
            author_field,
            subject_field,
            keywords_field,
            creator_field,
            producer_field,
            creationdate_field,
            moddate_field,
            trapped_field,
        ])),
    )
}
