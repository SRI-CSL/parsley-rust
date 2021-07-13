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
use crate::pdf_lib::arrayofocconfig::arrayofocconfig_type;
use crate::pdf_lib::arrayofocg::arrayofocg_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::optcontentconfig::optcontentconfig_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn optcontentproperties_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_2 = arrayofocconfig_type(tctx);
    let assignment_1 = optcontentconfig_type(tctx);
    let assignment_0 = arrayofocg_type(tctx);
    let dis_2 = TypeCheck::new(
        tctx,
        "configs",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(tctx, "d", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let dis_0 = TypeCheck::new(tctx, "ocgs", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let ocgs_field = DictEntry {
        key: Vec::from("OCGs"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_1,
        opt: DictKeySpec::Required,
    };
    let configs_field = DictEntry {
        key: Vec::from("Configs"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "optcontentproperties",
        Rc::new(PDFType::Dict(vec![ocgs_field, d_field, configs_field])),
    )
}
