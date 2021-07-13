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
use crate::pdf_lib::arrayofstringsascii::arrayofstringsascii_type;
use crate::pdf_lib::arrayofstringsbyte::arrayofstringsbyte_type;
use crate::pdf_lib::arrayofsubjectdn::arrayofsubjectdn_type;
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
use std::rc::Rc;
pub fn certseedvalue_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_10 =
        TypeCheck::new(tctx, "url", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_6 = arrayofsubjectdn_type(tctx);
    let assignment_7 = arrayofstringsascii_type(tctx);
    let assignment_5 = arrayofstringsascii_type(tctx);
    let assignment_4 = TypeCheck::new(
        tctx,
        "signaturepolicyhashalgorithm",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_3 = TypeCheck::new(
        tctx,
        "signaturepolicyhashvalue",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_2 = TypeCheck::new(
        tctx,
        "signaturepolicyoid",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_1 = arrayofstringsbyte_type(tctx);
    let assignment_9 = arrayofstringsbyte_type(tctx);
    let assignment_8 = arrayofstringsbyte_type(tctx);
    let assignment_integer_0 =
        TypeCheck::new(tctx, "ff", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let dis_5 = TypeCheck::new(tctx, "oid", Rc::new(PDFType::Disjunct(vec![assignment_9])));
    let dis_4 = TypeCheck::new(
        tctx,
        "issuer",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "keyusage",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "subjectdn",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "signaturepolicycommitmenttype",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "subject",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let choices_urltype = ChoicePred(
        String::from("Invalid URLType"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Browser")))],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("SVCert")))],
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
    let subject_field = DictEntry {
        key: Vec::from("Subject"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let signaturepolicyoid_field = DictEntry {
        key: Vec::from("SignaturePolicyOID"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let signaturepolicyhashvalue_field = DictEntry {
        key: Vec::from("SignaturePolicyHashValue"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let signaturepolicyhashalgorithm_field = DictEntry {
        key: Vec::from("SignaturePolicyHashAlgorithm"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let signaturepolicycommitmenttype_field = DictEntry {
        key: Vec::from("SignaturePolicyCommitmentType"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let subjectdn_field = DictEntry {
        key: Vec::from("SubjectDN"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let keyusage_field = DictEntry {
        key: Vec::from("KeyUsage"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let issuer_field = DictEntry {
        key: Vec::from("Issuer"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let oid_field = DictEntry {
        key: Vec::from("OID"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let url_field = DictEntry {
        key: Vec::from("URL"),
        chk: assignment_10,

        opt: DictKeySpec::Optional,
    };
    let urltype_field = DictEntry {
        key: Vec::from("URLType"),
        chk: TypeCheck::new_refined(
            tctx,
            "urltype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_urltype),
        ),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "certseedvalue",
        Rc::new(PDFType::Dict(vec![
            type_field,
            ff_field,
            subject_field,
            signaturepolicyoid_field,
            signaturepolicyhashvalue_field,
            signaturepolicyhashalgorithm_field,
            signaturepolicycommitmenttype_field,
            subjectdn_field,
            keyusage_field,
            issuer_field,
            oid_field,
            url_field,
            urltype_field,
        ])),
    )
}
