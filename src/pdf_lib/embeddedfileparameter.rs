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
use crate::pdf_lib::mac::mac_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn embeddedfileparameter_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = TypeCheck::new(
        tctx,
        "checksum",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_3 = mac_type(tctx);
    let assignment_date_1 = mk_date_typchk(tctx);
    let assignment_date_2 = mk_date_typchk(tctx);
    let assignment_integer_0 = TypeCheck::new(
        tctx,
        "size",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignments_disjuncts_1 = Rc::new(PDFType::Disjunct(vec![assignment_date_2]));
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_1]));
    let dis_2 = TypeCheck::new(tctx, "mac", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_1 = TypeCheck::new(tctx, "moddate", assignments_disjuncts_1);
    let dis_0 = TypeCheck::new(tctx, "creationdate", assignments_disjuncts_0);
    let size_field = DictEntry {
        key: Vec::from("Size"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let creationdate_field = DictEntry {
        key: Vec::from("CreationDate"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let moddate_field = DictEntry {
        key: Vec::from("ModDate"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let mac_field = DictEntry {
        key: Vec::from("Mac"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let checksum_field = DictEntry {
        key: Vec::from("CheckSum"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "embeddedfileparameter",
        Rc::new(PDFType::Dict(vec![
            size_field,
            creationdate_field,
            moddate_field,
            mac_field,
            checksum_field,
        ])),
    )
}
