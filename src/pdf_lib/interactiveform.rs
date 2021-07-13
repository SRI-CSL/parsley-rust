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
use crate::pdf_lib::arrayoffields::arrayoffields_type;
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
use crate::pdf_lib::resource::resource_type;
use crate::pdf_lib::stream::stream_type;
use std::rc::Rc;
pub fn interactiveform_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_8 = stream_type(tctx);
    let assignment_7 = arrayofstreamsgeneral_type(tctx);
    let assignment_integer_6 =
        TypeCheck::new(tctx, "q", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_5 = TypeCheck::new(tctx, "da", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_4 = resource_type(tctx);
    let assignment_integer_2 = TypeCheck::new(
        tctx,
        "sigflags",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_bool_1 = TypeCheck::new(
        tctx,
        "needappearances",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_0 = arrayoffields_type(tctx);
    let assignment_3 = arrayoffields_type(tctx);
    let dis_3 = TypeCheck::new(
        tctx,
        "xfa",
        Rc::new(PDFType::Disjunct(vec![assignment_7, assignment_8])),
    );
    let dis_2 = TypeCheck::new(tctx, "dr", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_1 = TypeCheck::new(tctx, "co", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let dis_0 = TypeCheck::new(
        tctx,
        "fields",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let fields_field = DictEntry {
        key: Vec::from("Fields"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let needappearances_field = DictEntry {
        key: Vec::from("NeedAppearances"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let sigflags_field = DictEntry {
        key: Vec::from("SigFlags"),
        chk: assignment_integer_2,

        opt: DictKeySpec::Optional,
    };
    let co_field = DictEntry {
        key: Vec::from("CO"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let dr_field = DictEntry {
        key: Vec::from("DR"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let da_field = DictEntry {
        key: Vec::from("DA"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let q_field = DictEntry {
        key: Vec::from("Q"),
        chk: assignment_integer_6,

        opt: DictKeySpec::Optional,
    };
    let xfa_field = DictEntry {
        key: Vec::from("XFA"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "interactiveform",
        Rc::new(PDFType::Dict(vec![
            fields_field,
            needappearances_field,
            sigflags_field,
            co_field,
            dr_field,
            da_field,
            q_field,
            xfa_field,
        ])),
    )
}
