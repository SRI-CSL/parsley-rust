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
use crate::pdf_lib::arrayofnamesgeneral::arrayofnamesgeneral_type;
use crate::pdf_lib::arrayofsoftwareidentifiers::arrayofsoftwareidentifiers_type;
use crate::pdf_lib::arrayofstringstext::arrayofstringstext_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::minimumbitdepth::minimumbitdepth_type;
use crate::pdf_lib::minimumscreensize::minimumscreensize_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn mediacriteria_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_9 = arrayofstringstext_type(tctx);
    let assignment_8 = arrayofnamesgeneral_type(tctx);
    let assignment_7 = arrayofsoftwareidentifiers_type(tctx);
    let assignment_6 = minimumscreensize_type(tctx);
    let assignment_5 = minimumbitdepth_type(tctx);
    let assignment_integer_4 =
        TypeCheck::new(tctx, "r", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_bool_3 =
        TypeCheck::new(tctx, "s", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_bool_2 =
        TypeCheck::new(tctx, "o", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_bool_1 =
        TypeCheck::new(tctx, "c", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_bool_0 =
        TypeCheck::new(tctx, "a", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let dis_4 = TypeCheck::new(tctx, "l", Rc::new(PDFType::Disjunct(vec![assignment_9])));
    let dis_3 = TypeCheck::new(tctx, "p", Rc::new(PDFType::Disjunct(vec![assignment_8])));
    let dis_2 = TypeCheck::new(tctx, "v", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_1 = TypeCheck::new(tctx, "z", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_0 = TypeCheck::new(tctx, "d", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("MediaCriteria")))],
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
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: assignment_bool_0,

        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: assignment_bool_2,

        opt: DictKeySpec::Optional,
    };
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: assignment_bool_3,

        opt: DictKeySpec::Optional,
    };
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: assignment_integer_4,

        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let z_field = DictEntry {
        key: Vec::from("Z"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let l_field = DictEntry {
        key: Vec::from("L"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "mediacriteria",
        Rc::new(PDFType::Dict(vec![
            type_field, a_field, c_field, o_field, s_field, r_field, d_field, z_field, v_field,
            p_field, l_field,
        ])),
    )
}
