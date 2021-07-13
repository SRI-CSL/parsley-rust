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
use crate::pdf_lib::arrayof3dcrosssection::arrayof3dcrosssection_type;
use crate::pdf_lib::arrayof3dmeasure::arrayof3dmeasure_type;
use crate::pdf_lib::arrayof3dnode::arrayof3dnode_type;
use crate::pdf_lib::arrayof3dtransmatrix::arrayof3dtransmatrix_type;
use crate::pdf_lib::arrayofstringstext::arrayofstringstext_type;
use crate::pdf_lib::arrayofviewparams::arrayofviewparams_type;
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
use crate::pdf_lib::projection::projection_type;
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::threedbackground::threedbackground_type;
use crate::pdf_lib::threedlightingscheme::threedlightingscheme_type;
use crate::pdf_lib::threedrendermode::threedrendermode_type;
use crate::pdf_lib::xobjectformtype1::xobjectformtype1_type;
use std::rc::Rc;
pub fn threedviewaddentries_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_15 = arrayofviewparams_type(tctx);
    let assignment_14 = stream_type(tctx);
    let assignment_bool_13 =
        TypeCheck::new(tctx, "nr", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_12 = arrayof3dnode_type(tctx);
    let assignment_11 = arrayof3dcrosssection_type(tctx);
    let assignment_10 = threedlightingscheme_type(tctx);
    let assignment_9 = threedrendermode_type(tctx);
    let assignment_8 = threedbackground_type(tctx);
    let assignment_7 = xobjectformtype1_type(tctx);
    let assignment_6 = projection_type(tctx);
    let assignment_5 = TypeCheck::new(tctx, "co", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_4 = arrayofstringstext_type(tctx);
    let assignment_3 = arrayof3dtransmatrix_type(tctx);
    let assignment_2 = arrayof3dmeasure_type(tctx);
    let assignment_1 = TypeCheck::new(tctx, "ms", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let assignment_0 = TypeCheck::new(tctx, "xn", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let dis_11 = TypeCheck::new(
        tctx,
        "params",
        Rc::new(PDFType::Disjunct(vec![assignment_15])),
    );
    let dis_10 = TypeCheck::new(
        tctx,
        "snapshot",
        Rc::new(PDFType::Disjunct(vec![assignment_14])),
    );
    let dis_9 = TypeCheck::new(tctx, "na", Rc::new(PDFType::Disjunct(vec![assignment_12])));
    let dis_8 = TypeCheck::new(tctx, "sa", Rc::new(PDFType::Disjunct(vec![assignment_11])));
    let dis_7 = TypeCheck::new(tctx, "ls", Rc::new(PDFType::Disjunct(vec![assignment_10])));
    let dis_6 = TypeCheck::new(tctx, "rm", Rc::new(PDFType::Disjunct(vec![assignment_9])));
    let dis_5 = TypeCheck::new(tctx, "bg", Rc::new(PDFType::Disjunct(vec![assignment_8])));
    let dis_4 = TypeCheck::new(tctx, "o", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_3 = TypeCheck::new(tctx, "p", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_2 = TypeCheck::new(
        tctx,
        "u3dpath",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_1 = TypeCheck::new(tctx, "c2w", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(tctx, "ma", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let choices_in = ChoicePred(
        String::from("Invalid IN"),
        vec![PDFObjT::Name(NameT::new(Vec::from("@XN")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("3DView")))],
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
    let xn_field = DictEntry {
        key: Vec::from("XN"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let in_field = DictEntry {
        key: Vec::from("IN"),
        chk: TypeCheck::new_refined(
            tctx,
            "in",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_in),
        ),
        opt: DictKeySpec::Optional,
    };
    let ms_field = DictEntry {
        key: Vec::from("MS"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let ma_field = DictEntry {
        key: Vec::from("MA"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let c2w_field = DictEntry {
        key: Vec::from("C2W"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let u3dpath_field = DictEntry {
        key: Vec::from("U3DPath"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let co_field = DictEntry {
        key: Vec::from("CO"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let bg_field = DictEntry {
        key: Vec::from("BG"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let rm_field = DictEntry {
        key: Vec::from("RM"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let ls_field = DictEntry {
        key: Vec::from("LS"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let sa_field = DictEntry {
        key: Vec::from("SA"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let na_field = DictEntry {
        key: Vec::from("NA"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    let nr_field = DictEntry {
        key: Vec::from("NR"),
        chk: assignment_bool_13,

        opt: DictKeySpec::Optional,
    };
    let snapshot_field = DictEntry {
        key: Vec::from("Snapshot"),
        chk: dis_10,
        opt: DictKeySpec::Optional,
    };
    let params_field = DictEntry {
        key: Vec::from("Params"),
        chk: dis_11,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "threedviewaddentries",
        Rc::new(PDFType::Dict(vec![
            type_field,
            xn_field,
            in_field,
            ms_field,
            ma_field,
            c2w_field,
            u3dpath_field,
            co_field,
            p_field,
            o_field,
            bg_field,
            rm_field,
            ls_field,
            sa_field,
            na_field,
            nr_field,
            snapshot_field,
            params_field,
        ])),
    )
}
