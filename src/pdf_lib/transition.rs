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
pub fn transition_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_bool_2 =
        TypeCheck::new(tctx, "b", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_1 = TypeCheck::new(tctx, "ss", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_0 = TypeCheck::new(tctx, "d", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let choices_di = ChoicePred(
        String::from("Invalid Di"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("None];[0"))),
            PDFObjT::Name(NameT::new(Vec::from("90"))),
            PDFObjT::Name(NameT::new(Vec::from("180"))),
            PDFObjT::Name(NameT::new(Vec::from("270"))),
            PDFObjT::Name(NameT::new(Vec::from("315"))),
        ],
    );
    let choices_m = ChoicePred(
        String::from("Invalid M"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("I"))),
            PDFObjT::Name(NameT::new(Vec::from("O"))),
        ],
    );
    let choices_dm = ChoicePred(
        String::from("Invalid Dm"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("H"))),
            PDFObjT::Name(NameT::new(Vec::from("V"))),
        ],
    );
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Split"))),
            PDFObjT::Name(NameT::new(Vec::from("Blinds"))),
            PDFObjT::Name(NameT::new(Vec::from("Box"))),
            PDFObjT::Name(NameT::new(Vec::from("Wipe"))),
            PDFObjT::Name(NameT::new(Vec::from("Dissolve"))),
            PDFObjT::Name(NameT::new(Vec::from("Glitter"))),
            PDFObjT::Name(NameT::new(Vec::from("R"))),
            PDFObjT::Name(NameT::new(Vec::from("Fly"))),
            PDFObjT::Name(NameT::new(Vec::from("Push"))),
            PDFObjT::Name(NameT::new(Vec::from("Cover"))),
            PDFObjT::Name(NameT::new(Vec::from("Uncover"))),
            PDFObjT::Name(NameT::new(Vec::from("Fade"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Trans")))],
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
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let dm_field = DictEntry {
        key: Vec::from("Dm"),
        chk: TypeCheck::new_refined(
            tctx,
            "dm",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_dm),
        ),
        opt: DictKeySpec::Optional,
    };
    let m_field = DictEntry {
        key: Vec::from("M"),
        chk: TypeCheck::new_refined(
            tctx,
            "m",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_m),
        ),
        opt: DictKeySpec::Optional,
    };
    let di_field = DictEntry {
        key: Vec::from("Di"),
        chk: TypeCheck::new_refined(
            tctx,
            "di",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_di),
        ),
        opt: DictKeySpec::Optional,
    };
    let ss_field = DictEntry {
        key: Vec::from("SS"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let b_field = DictEntry {
        key: Vec::from("B"),
        chk: assignment_bool_2,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "transition",
        Rc::new(PDFType::Dict(vec![
            type_field, s_field, d_field, dm_field, m_field, di_field, ss_field, b_field,
        ])),
    )
}
