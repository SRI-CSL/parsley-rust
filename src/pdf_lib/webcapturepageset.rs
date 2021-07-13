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
use crate::pdf_lib::arrayofsourceinformations::arrayofsourceinformations_type;
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
use crate::pdf_lib::sourceinformation::sourceinformation_type;
use crate::pdf_lib::universalarray::universalarray_type;
use std::rc::Rc;
pub fn webcapturepageset_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = TypeCheck::new(tctx, "tid", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_6 = TypeCheck::new(tctx, "t", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_date_5 = mk_date_typchk(tctx);
    let assignment_4 = TypeCheck::new(tctx, "ct", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_3 = sourceinformation_type(tctx);
    let assignment_2 = arrayofsourceinformations_type(tctx);
    let assignment_1 = universalarray_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "id", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_5]));
    let dis_2 = TypeCheck::new(tctx, "ts", assignments_disjuncts_0);
    let dis_1 = TypeCheck::new(
        tctx,
        "si",
        Rc::new(PDFType::Disjunct(vec![assignment_2, assignment_3])),
    );
    let dis_0 = TypeCheck::new(tctx, "o", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let choices_s = ChoicePred(
        String::from("Invalid S"),
        vec![PDFObjT::Name(NameT::new(Vec::from("SPS")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("SpiderContentSet")))],
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
        opt: DictKeySpec::Required,
    };
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let si_field = DictEntry {
        key: Vec::from("SI"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let ct_field = DictEntry {
        key: Vec::from("CT"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let ts_field = DictEntry {
        key: Vec::from("TS"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: assignment_6,

        opt: DictKeySpec::Optional,
    };
    let tid_field = DictEntry {
        key: Vec::from("TID"),
        chk: assignment_7,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "webcapturepageset",
        Rc::new(PDFType::Dict(vec![
            type_field, s_field, id_field, o_field, si_field, ct_field, ts_field, t_field,
            tid_field,
        ])),
    )
}
