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
use crate::pdf_lib::addactionformfield::addactionformfield_type;
use crate::pdf_lib::arrayoffields::arrayoffields_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::fieldbtn::fieldbtn_type;
use crate::pdf_lib::fieldch::fieldch_type;
use crate::pdf_lib::fieldsig::fieldsig_type;
use crate::pdf_lib::fieldtx::fieldtx_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn field_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_10 = addactionformfield_type(tctx);
    let assignment_integer_9 =
        TypeCheck::new(tctx, "ff", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_8 = TypeCheck::new(tctx, "tm", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_7 = TypeCheck::new(tctx, "tu", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_6 = TypeCheck::new(tctx, "t", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_5 = arrayoffields_type(tctx);
    let assignment_4 = field_type(tctx);
    let assignment_3 = fieldsig_type(tctx);
    let assignment_2 = fieldch_type(tctx);
    let assignment_1 = fieldbtn_type(tctx);
    let assignment_0 = fieldtx_type(tctx);
    let dis_2 = TypeCheck::new(tctx, "aa", Rc::new(PDFType::Disjunct(vec![assignment_10])));
    let dis_1 = TypeCheck::new(tctx, "kids", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_0 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
            assignment_4,
        ])),
    );
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let kids_field = DictEntry {
        key: Vec::from("Kids"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: assignment_6,

        opt: DictKeySpec::Required,
    };
    let tu_field = DictEntry {
        key: Vec::from("TU"),
        chk: assignment_7,

        opt: DictKeySpec::Optional,
    };
    let tm_field = DictEntry {
        key: Vec::from("TM"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    let ff_field = DictEntry {
        key: Vec::from("Ff"),
        chk: assignment_integer_9,

        opt: DictKeySpec::Optional,
    };
    let aa_field = DictEntry {
        key: Vec::from("AA"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "field",
        Rc::new(PDFType::Dict(vec![
            parent_field,
            kids_field,
            t_field,
            tu_field,
            tm_field,
            ff_field,
            aa_field,
        ])),
    )
}
