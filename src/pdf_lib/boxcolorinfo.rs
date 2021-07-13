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
use crate::pdf_lib::boxstyle::boxstyle_type;
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
pub fn boxcolorinfo_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = boxstyle_type(tctx);
    let assignment_0 = boxstyle_type(tctx);
    let assignment_2 = boxstyle_type(tctx);
    let assignment_3 = boxstyle_type(tctx);
    let dis_3 = TypeCheck::new(
        tctx,
        "artbox",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "trimbox",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "bleedbox",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "cropbox",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let cropbox_field = DictEntry {
        key: Vec::from("CropBox"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let bleedbox_field = DictEntry {
        key: Vec::from("BleedBox"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let trimbox_field = DictEntry {
        key: Vec::from("TrimBox"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let artbox_field = DictEntry {
        key: Vec::from("ArtBox"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "boxcolorinfo",
        Rc::new(PDFType::Dict(vec![
            cropbox_field,
            bleedbox_field,
            trimbox_field,
            artbox_field,
        ])),
    )
}
