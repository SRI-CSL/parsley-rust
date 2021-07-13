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
use crate::pdf_lib::optcontentcreatorinfo::optcontentcreatorinfo_type;
use crate::pdf_lib::optcontentexport::optcontentexport_type;
use crate::pdf_lib::optcontentlanguage::optcontentlanguage_type;
use crate::pdf_lib::optcontentpageelement::optcontentpageelement_type;
use crate::pdf_lib::optcontentprint::optcontentprint_type;
use crate::pdf_lib::optcontentuser::optcontentuser_type;
use crate::pdf_lib::optcontentview::optcontentview_type;
use crate::pdf_lib::optcontentzoom::optcontentzoom_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn optcontentusage_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = optcontentpageelement_type(tctx);
    let assignment_6 = optcontentuser_type(tctx);
    let assignment_5 = optcontentview_type(tctx);
    let assignment_4 = optcontentprint_type(tctx);
    let assignment_3 = optcontentzoom_type(tctx);
    let assignment_2 = optcontentexport_type(tctx);
    let assignment_1 = optcontentlanguage_type(tctx);
    let assignment_0 = optcontentcreatorinfo_type(tctx);
    let dis_7 = TypeCheck::new(
        tctx,
        "pageelement",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_6 = TypeCheck::new(tctx, "user", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_5 = TypeCheck::new(tctx, "view", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_4 = TypeCheck::new(
        tctx,
        "print",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_3 = TypeCheck::new(tctx, "zoom", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_2 = TypeCheck::new(
        tctx,
        "export",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "language",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "creatorinfo",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let creatorinfo_field = DictEntry {
        key: Vec::from("CreatorInfo"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let language_field = DictEntry {
        key: Vec::from("Language"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let export_field = DictEntry {
        key: Vec::from("Export"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let zoom_field = DictEntry {
        key: Vec::from("Zoom"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let print_field = DictEntry {
        key: Vec::from("Print"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let view_field = DictEntry {
        key: Vec::from("View"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let user_field = DictEntry {
        key: Vec::from("User"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let pageelement_field = DictEntry {
        key: Vec::from("PageElement"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "optcontentusage",
        Rc::new(PDFType::Dict(vec![
            creatorinfo_field,
            language_field,
            export_field,
            zoom_field,
            print_field,
            view_field,
            user_field,
            pageelement_field,
        ])),
    )
}
