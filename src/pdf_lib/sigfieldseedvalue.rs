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
use crate::pdf_lib::arrayofdigestmethod::arrayofdigestmethod_type;
use crate::pdf_lib::arrayofnamesgeneral::arrayofnamesgeneral_type;
use crate::pdf_lib::arrayofstringstext::arrayofstringstext_type;
use crate::pdf_lib::certseedvalue::certseedvalue_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::mdpdict::mdpdict_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::timestampdict::timestampdict_type;
use std::rc::Rc;
pub fn sigfieldseedvalue_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_11 = TypeCheck::new(
        tctx,
        "appearancefilter",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_bool_10 = TypeCheck::new(
        tctx,
        "addrevinfo",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_8 = timestampdict_type(tctx);
    let assignment_7 = mdpdict_type(tctx);
    let assignment_9 = arrayofstringstext_type(tctx);
    let assignment_6 = arrayofstringstext_type(tctx);
    let assignment_5 = certseedvalue_type(tctx);
    let assignment_integer_4 =
        TypeCheck::new(tctx, "v", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_3 = arrayofdigestmethod_type(tctx);
    let assignment_2 = arrayofnamesgeneral_type(tctx);
    let assignment_1 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_integer_0 =
        TypeCheck::new(tctx, "ff", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let dis_6 = TypeCheck::new(
        tctx,
        "legalattestation",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "timestamp",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_4 = TypeCheck::new(tctx, "mdp", Rc::new(PDFType::Disjunct(vec![assignment_7])));
    let dis_3 = TypeCheck::new(
        tctx,
        "reasons",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_2 = TypeCheck::new(tctx, "cert", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_1 = TypeCheck::new(
        tctx,
        "digestmethod",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "subfilter",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let choices_lockdocument = ChoicePred(
        String::from("Invalid LockDocument"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("true"))),
            PDFObjT::Name(NameT::new(Vec::from("false"))),
            PDFObjT::Name(NameT::new(Vec::from("auto"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("SV")))],
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
    let ff_field = DictEntry {
        key: Vec::from("Ff"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let subfilter_field = DictEntry {
        key: Vec::from("SubFilter"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let digestmethod_field = DictEntry {
        key: Vec::from("DigestMethod"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: assignment_integer_4,

        opt: DictKeySpec::Optional,
    };
    let cert_field = DictEntry {
        key: Vec::from("Cert"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let reasons_field = DictEntry {
        key: Vec::from("Reasons"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let mdp_field = DictEntry {
        key: Vec::from("MDP"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let timestamp_field = DictEntry {
        key: Vec::from("TimeStamp"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let legalattestation_field = DictEntry {
        key: Vec::from("LegalAttestation"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let addrevinfo_field = DictEntry {
        key: Vec::from("AddRevInfo"),
        chk: assignment_bool_10,

        opt: DictKeySpec::Optional,
    };
    let lockdocument_field = DictEntry {
        key: Vec::from("LockDocument"),
        chk: TypeCheck::new_refined(
            tctx,
            "lockdocument",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_lockdocument),
        ),
        opt: DictKeySpec::Optional,
    };
    let appearancefilter_field = DictEntry {
        key: Vec::from("AppearanceFilter"),
        chk: assignment_11,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "sigfieldseedvalue",
        Rc::new(PDFType::Dict(vec![
            type_field,
            ff_field,
            filter_field,
            subfilter_field,
            digestmethod_field,
            v_field,
            cert_field,
            reasons_field,
            mdp_field,
            timestamp_field,
            legalattestation_field,
            addrevinfo_field,
            lockdocument_field,
            appearancefilter_field,
        ])),
    )
}
