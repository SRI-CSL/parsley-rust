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
use crate::pdf_lib::richmediawindow::richmediawindow_type;
use std::rc::Rc;
pub fn threedactivation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_bool_3 = TypeCheck::new(
        tctx,
        "transparent",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_2 = richmediawindow_type(tctx);
    let assignment_bool_1 =
        TypeCheck::new(tctx, "np", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_bool_0 =
        TypeCheck::new(tctx, "tb", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let dis_0 = TypeCheck::new(
        tctx,
        "window",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let choices_style = ChoicePred(
        String::from("Invalid Style"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Embedded"))),
            PDFObjT::Name(NameT::new(Vec::from("Windowed"))),
        ],
    );
    let choices_dis = ChoicePred(
        String::from("Invalid DIS"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("U"))),
            PDFObjT::Name(NameT::new(Vec::from("I"))),
            PDFObjT::Name(NameT::new(Vec::from("L"))),
        ],
    );
    let choices_d = ChoicePred(
        String::from("Invalid D"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("PC"))),
            PDFObjT::Name(NameT::new(Vec::from("PI"))),
            PDFObjT::Name(NameT::new(Vec::from("XD"))),
        ],
    );
    let choices_ais = ChoicePred(
        String::from("Invalid AIS"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("I"))),
            PDFObjT::Name(NameT::new(Vec::from("L"))),
        ],
    );
    let choices_a = ChoicePred(
        String::from("Invalid A"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("PO"))),
            PDFObjT::Name(NameT::new(Vec::from("PV"))),
            PDFObjT::Name(NameT::new(Vec::from("XA"))),
        ],
    );
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: TypeCheck::new_refined(
            tctx,
            "a",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_a),
        ),
        opt: DictKeySpec::Optional,
    };
    let ais_field = DictEntry {
        key: Vec::from("AIS"),
        chk: TypeCheck::new_refined(
            tctx,
            "ais",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_ais),
        ),
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: TypeCheck::new_refined(
            tctx,
            "d",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_d),
        ),
        opt: DictKeySpec::Optional,
    };
    let dis_field = DictEntry {
        key: Vec::from("DIS"),
        chk: TypeCheck::new_refined(
            tctx,
            "dis",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_dis),
        ),
        opt: DictKeySpec::Optional,
    };
    let tb_field = DictEntry {
        key: Vec::from("TB"),
        chk: assignment_bool_0,

        opt: DictKeySpec::Optional,
    };
    let np_field = DictEntry {
        key: Vec::from("NP"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let style_field = DictEntry {
        key: Vec::from("Style"),
        chk: TypeCheck::new_refined(
            tctx,
            "style",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_style),
        ),
        opt: DictKeySpec::Optional,
    };
    let window_field = DictEntry {
        key: Vec::from("Window"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let transparent_field = DictEntry {
        key: Vec::from("Transparent"),
        chk: assignment_bool_3,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "threedactivation",
        Rc::new(PDFType::Dict(vec![
            a_field,
            ais_field,
            d_field,
            dis_field,
            tb_field,
            np_field,
            style_field,
            window_field,
            transparent_field,
        ])),
    )
}
