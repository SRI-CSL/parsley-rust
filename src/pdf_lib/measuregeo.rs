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
use crate::pdf_lib::arrayof3dtransmatrix::arrayof3dtransmatrix_type;
use crate::pdf_lib::arrayof_3names::arrayof_3names_type;
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::geographiccoordinatesystem::geographiccoordinatesystem_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::projectedcoordinatesystem::projectedcoordinatesystem_type;
use std::rc::Rc;
pub fn measuregeo_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_8 = arrayof3dtransmatrix_type(tctx);
    let assignment_5 = arrayof_3names_type(tctx);
    let assignment_2 = projectedcoordinatesystem_type(tctx);
    let assignment_4 = projectedcoordinatesystem_type(tctx);
    let assignment_1 = geographiccoordinatesystem_type(tctx);
    let assignment_3 = geographiccoordinatesystem_type(tctx);
    let assignment_0 = arrayofnumbersgeneral_type(tctx);
    let assignment_7 = arrayofnumbersgeneral_type(tctx);
    let assignment_6 = arrayofnumbersgeneral_type(tctx);
    let dis_6 = TypeCheck::new(tctx, "pcsm", Rc::new(PDFType::Disjunct(vec![assignment_8])));
    let dis_5 = TypeCheck::new(tctx, "lpts", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_4 = TypeCheck::new(tctx, "gpts", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_3 = TypeCheck::new(tctx, "pdu", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_2 = TypeCheck::new(
        tctx,
        "dcs",
        Rc::new(PDFType::Disjunct(vec![assignment_3, assignment_4])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "gcs",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "bounds",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![PDFObjT::Name(NameT::new(Vec::from("GEO")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Measure")))],
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
    let subtype_field = DictEntry {
        key: Vec::from("Subtype"),
        chk: TypeCheck::new_refined(
            tctx,
            "subtype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_subtype),
        ),
        opt: DictKeySpec::Optional,
    };
    let bounds_field = DictEntry {
        key: Vec::from("Bounds"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let gcs_field = DictEntry {
        key: Vec::from("GCS"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let dcs_field = DictEntry {
        key: Vec::from("DCS"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let pdu_field = DictEntry {
        key: Vec::from("PDU"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let gpts_field = DictEntry {
        key: Vec::from("GPTS"),
        chk: dis_4,
        opt: DictKeySpec::Required,
    };
    let lpts_field = DictEntry {
        key: Vec::from("LPTS"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let pcsm_field = DictEntry {
        key: Vec::from("PCSM"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "measuregeo",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            bounds_field,
            gcs_field,
            dcs_field,
            pdu_field,
            gpts_field,
            lpts_field,
            pcsm_field,
        ])),
    )
}
