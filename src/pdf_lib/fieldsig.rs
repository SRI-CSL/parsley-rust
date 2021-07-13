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
use crate::pdf_lib::addactionformfield::addactionformfield_type;
use crate::pdf_lib::arrayoffields::arrayoffields_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::doctimestamp::doctimestamp_type;
use crate::pdf_lib::field::field_type;
use crate::pdf_lib::fieldbtn::fieldbtn_type;
use crate::pdf_lib::fieldch::fieldch_type;
use crate::pdf_lib::fieldtx::fieldtx_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::sigfieldlock::sigfieldlock_type;
use crate::pdf_lib::sigfieldseedvalue::sigfieldseedvalue_type;
use crate::pdf_lib::signature::signature_type;
use crate::pdf_lib::stream::stream_type;
use std::rc::Rc;
pub fn fieldsig_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_19 = sigfieldseedvalue_type(tctx);
    let assignment_18 = sigfieldlock_type(tctx);
    let assignment_17 = stream_type(tctx);
    let assignment_16 = TypeCheck::new(tctx, "ds", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_15 = TypeCheck::new(tctx, "da", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_14 = addactionformfield_type(tctx);
    let assignment_11 = doctimestamp_type(tctx);
    let assignment_13 = doctimestamp_type(tctx);
    let assignment_10 = signature_type(tctx);
    let assignment_12 = signature_type(tctx);
    let assignment_integer_9 =
        TypeCheck::new(tctx, "ff", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_8 = TypeCheck::new(tctx, "tm", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_7 = TypeCheck::new(tctx, "tu", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_6 = TypeCheck::new(tctx, "t", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_5 = arrayoffields_type(tctx);
    let assignment_4 = field_type(tctx);
    let assignment_3 = fieldsig_type(tctx);
    let assignment_2 = fieldch_type(tctx);
    let assignment_1 = fieldbtn_type(tctx);
    let assignment_0 = fieldtx_type(tctx);
    let dis_7 = TypeCheck::new(tctx, "sv", Rc::new(PDFType::Disjunct(vec![assignment_19])));
    let dis_6 = TypeCheck::new(
        tctx,
        "lock",
        Rc::new(PDFType::Disjunct(vec![assignment_18])),
    );
    let dis_5 = TypeCheck::new(tctx, "rv", Rc::new(PDFType::Disjunct(vec![assignment_17])));
    let dis_4 = TypeCheck::new(tctx, "aa", Rc::new(PDFType::Disjunct(vec![assignment_14])));
    let dis_3 = TypeCheck::new(
        tctx,
        "dv",
        Rc::new(PDFType::Disjunct(vec![assignment_12, assignment_13])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "v",
        Rc::new(PDFType::Disjunct(vec![assignment_10, assignment_11])),
    );
    let dis_1 = TypeCheck::new(tctx, "kids", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_0 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
            assignment_4,
        ])),
    );
    let choices_q = ChoicePred(
        String::from("Invalid Q"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("0"))),
            PDFObjT::Name(NameT::new(Vec::from("1"))),
            PDFObjT::Name(NameT::new(Vec::from("2"))),
        ],
    );
    let choices_ft = ChoicePred(
        String::from("Invalid FT"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Sig")))],
    );
    let ft_field = DictEntry {
        key: Vec::from("FT"),
        chk: TypeCheck::new_refined(
            tctx,
            "ft",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_ft),
        ),
        opt: DictKeySpec::Optional,
    };
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let kids_field = DictEntry {
        key: Vec::from("Kids"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: assignment_6,

        opt: DictKeySpec::Required,
    };
    let tu_field = DictEntry {
        key: Vec::from("TU"),
        chk: assignment_7,

        opt: DictKeySpec::Optional,
    };
    let tm_field = DictEntry {
        key: Vec::from("TM"),
        chk: assignment_8,

        opt: DictKeySpec::Optional,
    };
    let ff_field = DictEntry {
        key: Vec::from("Ff"),
        chk: assignment_integer_9,

        opt: DictKeySpec::Optional,
    };
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let dv_field = DictEntry {
        key: Vec::from("DV"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let aa_field = DictEntry {
        key: Vec::from("AA"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let da_field = DictEntry {
        key: Vec::from("DA"),
        chk: assignment_15,

        opt: DictKeySpec::Optional,
    };
    let q_field = DictEntry {
        key: Vec::from("Q"),
        chk: TypeCheck::new_refined(
            tctx,
            "q",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_q),
        ),
        opt: DictKeySpec::Optional,
    };
    let ds_field = DictEntry {
        key: Vec::from("DS"),
        chk: assignment_16,

        opt: DictKeySpec::Optional,
    };
    let rv_field = DictEntry {
        key: Vec::from("RV"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let lock_field = DictEntry {
        key: Vec::from("Lock"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let sv_field = DictEntry {
        key: Vec::from("SV"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "fieldsig",
        Rc::new(PDFType::Dict(vec![
            ft_field,
            parent_field,
            kids_field,
            t_field,
            tu_field,
            tm_field,
            ff_field,
            v_field,
            dv_field,
            aa_field,
            da_field,
            q_field,
            ds_field,
            rv_field,
            lock_field,
            sv_field,
        ])),
    )
}
