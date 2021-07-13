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
pub fn projection_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_5 = TypeCheck::new(tctx, "os", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_4 = TypeCheck::new(tctx, "ps", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_3 = TypeCheck::new(tctx, "ps", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_2 = TypeCheck::new(tctx, "fov", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_1 = TypeCheck::new(tctx, "n", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_0 = TypeCheck::new(tctx, "f", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let choices_ob = ChoicePred(
        String::from("Invalid OB"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("W"))),
            PDFObjT::Name(NameT::new(Vec::from("H"))),
            PDFObjT::Name(NameT::new(Vec::from("Min"))),
            PDFObjT::Name(NameT::new(Vec::from("Max"))),
            PDFObjT::Name(NameT::new(Vec::from("Absolute"))),
        ],
    );
    let choices_cs = ChoicePred(
        String::from("Invalid CS"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("XNF"))),
            PDFObjT::Name(NameT::new(Vec::from("ANF"))),
        ],
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("O"))),
            PDFObjT::Name(NameT::new(Vec::from("P"))),
        ],
    );
    let subtype_field = DictEntry {
        key: Vec::from("Subtype"),
        chk: TypeCheck::new_refined(
            tctx,
            "subtype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_subtype),
        ),
        opt: DictKeySpec::Required,
    };
    let cs_field = DictEntry {
        key: Vec::from("CS"),
        chk: TypeCheck::new_refined(
            tctx,
            "cs",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_cs),
        ),
        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let n_field = DictEntry {
        key: Vec::from("N"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let fov_field = DictEntry {
        key: Vec::from("FOV"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let ps_field = DictEntry {
        key: Vec::from("PS"),
        chk: TypeCheck::new(
            tctx,
            "ps",
            Rc::new(PDFType::Disjunct(vec![assignment_3, assignment_4])),
        ),
        opt: DictKeySpec::Optional,
    };
    let os_field = DictEntry {
        key: Vec::from("OS"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let ob_field = DictEntry {
        key: Vec::from("OB"),
        chk: TypeCheck::new_refined(
            tctx,
            "ob",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_ob),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "projection",
        Rc::new(PDFType::Dict(vec![
            subtype_field,
            cs_field,
            f_field,
            n_field,
            fov_field,
            ps_field,
            os_field,
            ob_field,
        ])),
    )
}
