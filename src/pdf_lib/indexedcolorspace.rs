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
use crate::pdf_lib::calgraycolorspace::calgraycolorspace_type;
use crate::pdf_lib::calrgbcolorspace::calrgbcolorspace_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::devicencolorspace::devicencolorspace_type;
use crate::pdf_lib::iccbasedcolorspace::iccbasedcolorspace_type;
use crate::pdf_lib::labcolorspace::labcolorspace_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::separationcolorspace::separationcolorspace_type;
use crate::pdf_lib::stream::stream_type;
use std::rc::Rc;
pub fn indexedcolorspace_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = stream_type(tctx);
    let assignment_integer_6 =
        TypeCheck::new(tctx, "2", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_5 = devicencolorspace_type(tctx);
    let assignment_4 = separationcolorspace_type(tctx);
    let assignment_3 = iccbasedcolorspace_type(tctx);
    let assignment_2 = labcolorspace_type(tctx);
    let assignment_1 = calrgbcolorspace_type(tctx);
    let assignment_0 = calgraycolorspace_type(tctx);
    let dis_1 = TypeCheck::new(tctx, "3", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_0 = TypeCheck::new(
        tctx,
        "1",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
            assignment_4,
            assignment_5,
        ])),
    );
    let choices_zero = ChoicePred(
        String::from("Invalid 0"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Indexed")))],
    );
    let zero_field = DictEntry {
        key: Vec::from("0"),
        chk: TypeCheck::new_refined(
            tctx,
            "0",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_zero),
        ),
        opt: DictKeySpec::Required,
    };
    let one_field = DictEntry {
        key: Vec::from("1"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let two_field = DictEntry {
        key: Vec::from("2"),
        chk: assignment_integer_6,

        opt: DictKeySpec::Required,
    };
    let three_field = DictEntry {
        key: Vec::from("3"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "indexedcolorspace",
        Rc::new(PDFType::Dict(vec![
            zero_field,
            one_field,
            two_field,
            three_field,
        ])),
    )
}
