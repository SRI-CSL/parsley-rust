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
use crate::pdf_lib::docmdptransformparameters::docmdptransformparameters_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::universalarray::universalarray_type;
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use std::rc::Rc;
pub fn signaturereferencedocmdp_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_4 = TypeCheck::new(
        tctx,
        "digestmethod",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_3 = stream_type(tctx);
    let assignment_2 = universaldictionary_type(tctx);
    let assignment_1 = universalarray_type(tctx);
    let assignment_0 = docmdptransformparameters_type(tctx);
    let dis_1 = TypeCheck::new(
        tctx,
        "data",
        Rc::new(PDFType::Disjunct(vec![
            assignment_1,
            assignment_2,
            assignment_3,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "transformparams",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_transformmethod = ChoicePred(
        String::from("Invalid TransformMethod"),
        vec![PDFObjT::Name(NameT::new(Vec::from("DocMDP")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("SigRef")))],
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
    let transformmethod_field = DictEntry {
        key: Vec::from("TransformMethod"),
        chk: TypeCheck::new_refined(
            tctx,
            "transformmethod",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_transformmethod),
        ),
        opt: DictKeySpec::Required,
    };
    let transformparams_field = DictEntry {
        key: Vec::from("TransformParams"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let data_field = DictEntry {
        key: Vec::from("Data"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let digestmethod_field = DictEntry {
        key: Vec::from("DigestMethod"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "signaturereferencedocmdp",
        Rc::new(PDFType::Dict(vec![
            type_field,
            transformmethod_field,
            transformparams_field,
            data_field,
            digestmethod_field,
        ])),
    )
}
