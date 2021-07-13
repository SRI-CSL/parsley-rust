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
use crate::pdf_lib::arrayof_3integers::arrayof_3integers_type;
use crate::pdf_lib::arrayofintegersgeneral::arrayofintegersgeneral_type;
use crate::pdf_lib::arrayofsignaturereferences::arrayofsignaturereferences_type;
use crate::pdf_lib::arrayofstringsbyte::arrayofstringsbyte_type;
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
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use std::rc::Rc;
pub fn signature_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_16 = TypeCheck::new(
        tctx,
        "prop_authtype",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_integer_15 = TypeCheck::new(
        tctx,
        "prop_authtime",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_14 = universaldictionary_type(tctx);
    let assignment_integer_13 =
        TypeCheck::new(tctx, "v", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_integer_12 =
        TypeCheck::new(tctx, "r", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_11 = TypeCheck::new(
        tctx,
        "contactinfo",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_10 = TypeCheck::new(
        tctx,
        "reason",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_9 = TypeCheck::new(
        tctx,
        "location",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_date_8 = mk_date_typchk(tctx);
    let assignment_7 = TypeCheck::new(
        tctx,
        "name",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_6 = arrayof_3integers_type(tctx);
    let assignment_5 = arrayofsignaturereferences_type(tctx);
    let assignment_4 = arrayofintegersgeneral_type(tctx);
    let assignment_3 = arrayofstringsbyte_type(tctx);
    let assignment_2 = TypeCheck::new(
        tctx,
        "contents",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_1 = TypeCheck::new(
        tctx,
        "subfilter",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_0 = TypeCheck::new(
        tctx,
        "filter",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignments_disjuncts_0 = Rc::new(PDFType::Disjunct(vec![assignment_date_8]));
    let dis_5 = TypeCheck::new(
        tctx,
        "prop_build",
        Rc::new(PDFType::Disjunct(vec![assignment_14])),
    );
    let dis_4 = TypeCheck::new(tctx, "m", assignments_disjuncts_0);
    let dis_3 = TypeCheck::new(
        tctx,
        "changes",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "reference",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "byterange",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_0 = TypeCheck::new(tctx, "cert", Rc::new(PDFType::Disjunct(vec![assignment_3])));
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Sig")))],
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
    let filter_field = DictEntry {
        key: Vec::from("Filter"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let subfilter_field = DictEntry {
        key: Vec::from("SubFilter"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let contents_field = DictEntry {
        key: Vec::from("Contents"),
        chk: assignment_2,

        opt: DictKeySpec::Required,
    };
    let cert_field = DictEntry {
        key: Vec::from("Cert"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let byterange_field = DictEntry {
        key: Vec::from("ByteRange"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let reference_field = DictEntry {
        key: Vec::from("Reference"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let changes_field = DictEntry {
        key: Vec::from("Changes"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let name_field = DictEntry {
        key: Vec::from("Name"),
        chk: assignment_7,

        opt: DictKeySpec::Optional,
    };
    let m_field = DictEntry {
        key: Vec::from("M"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let location_field = DictEntry {
        key: Vec::from("Location"),
        chk: assignment_9,

        opt: DictKeySpec::Optional,
    };
    let reason_field = DictEntry {
        key: Vec::from("Reason"),
        chk: assignment_10,

        opt: DictKeySpec::Optional,
    };
    let contactinfo_field = DictEntry {
        key: Vec::from("ContactInfo"),
        chk: assignment_11,

        opt: DictKeySpec::Optional,
    };
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: assignment_integer_12,

        opt: DictKeySpec::Optional,
    };
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: assignment_integer_13,

        opt: DictKeySpec::Optional,
    };
    let prop_build_field = DictEntry {
        key: Vec::from("Prop_Build"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let prop_authtime_field = DictEntry {
        key: Vec::from("Prop_AuthTime"),
        chk: assignment_integer_15,

        opt: DictKeySpec::Optional,
    };
    let prop_authtype_field = DictEntry {
        key: Vec::from("Prop_AuthType"),
        chk: assignment_16,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "signature",
        Rc::new(PDFType::Dict(vec![
            type_field,
            filter_field,
            subfilter_field,
            contents_field,
            cert_field,
            byterange_field,
            reference_field,
            changes_field,
            name_field,
            m_field,
            location_field,
            reason_field,
            contactinfo_field,
            r_field,
            v_field,
            prop_build_field,
            prop_authtime_field,
            prop_authtype_field,
        ])),
    )
}
