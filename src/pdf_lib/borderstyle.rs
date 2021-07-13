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
use crate::pdf_lib::arrayofdashpatterns::arrayofdashpatterns_type;
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
pub fn borderstyle_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_1 = arrayofdashpatterns_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "w", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let dis_0 = TypeCheck::new(tctx, "d", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("S"))),
            PDFObjT::Name(NameT::new(Vec::from("B"))),
            PDFObjT::Name(NameT::new(Vec::from("D"))),
            PDFObjT::Name(NameT::new(Vec::from("I"))),
            PDFObjT::Name(NameT::new(Vec::from("U"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Border")))],
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
    let w_field = DictEntry {
        key: Vec::from("W"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: TypeCheck::new_refined(
            tctx,
            "s",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_s),
        ),
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "borderstyle",
        Rc::new(PDFType::Dict(vec![type_field, w_field, s_field, d_field])),
    )
}
