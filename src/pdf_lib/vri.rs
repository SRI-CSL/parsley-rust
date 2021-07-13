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
use crate::pdf_lib::arrayofstreamsgeneral::arrayofstreamsgeneral_type;
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
use crate::pdf_lib::stream::stream_type;
use std::rc::Rc;
pub fn vri_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = stream_type(tctx);
    let assignment_date_3 = mk_date_typchk(tctx);
    let assignment_1 = arrayofstreamsgeneral_type(tctx);
    let assignment_0 = arrayofstreamsgeneral_type(tctx);
    let assignment_2 = arrayofstreamsgeneral_type(tctx);
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_3]));
    let dis_4 = TypeCheck::new(tctx, "ts", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_3 = TypeCheck::new(tctx, "tu", assignments_disjuncts_0);
    let dis_2 = TypeCheck::new(tctx, "ocsp", Rc::new(PDFType::Disjunct(vec![assignment_2])));
    let dis_1 = TypeCheck::new(tctx, "crl", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let dis_0 = TypeCheck::new(tctx, "cert", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("VRI")))],
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
    let cert_field = DictEntry {
        key: Vec::from("Cert"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let crl_field = DictEntry {
        key: Vec::from("CRL"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let ocsp_field = DictEntry {
        key: Vec::from("OCSP"),
        chk: dis_2,
        opt: DictKeySpec::Required,
    };
    let tu_field = DictEntry {
        key: Vec::from("TU"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let ts_field = DictEntry {
        key: Vec::from("TS"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "vri",
        Rc::new(PDFType::Dict(vec![
            type_field, cert_field, crl_field, ocsp_field, tu_field, ts_field,
        ])),
    )
}
