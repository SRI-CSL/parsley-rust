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
use crate::pdf_lib::arrayof_2stringsbyte::arrayof_2stringsbyte_type;
use crate::pdf_lib::collectionitem::collectionitem_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::encryptedpayload::encryptedpayload_type;
use crate::pdf_lib::filespecrf::filespecrf_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::thumbnail::thumbnail_type;
use std::rc::Rc;
pub fn filespecification_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_13 = encryptedpayload_type(tctx);
    let assignment_12 = thumbnail_type(tctx);
    let assignment_11 = collectionitem_type(tctx);
    let assignment_10 = TypeCheck::new(
        tctx,
        "desc",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_9 = filespecrf_type(tctx);
    let assignment_8 = filespecrf_type(tctx);
    let assignment_bool_7 =
        TypeCheck::new(tctx, "v", Rc::new(PDFType::PrimType(PDFPrimType::Bool)));
    let assignment_6 = arrayof_2stringsbyte_type(tctx);
    let assignment_5 = TypeCheck::new(
        tctx,
        "unix",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_4 = TypeCheck::new(tctx, "mac", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_3 = TypeCheck::new(tctx, "dos", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_2 = TypeCheck::new(tctx, "uf", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_1 = TypeCheck::new(tctx, "f", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_0 = TypeCheck::new(tctx, "fs", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let dis_5 = TypeCheck::new(tctx, "ep", Rc::new(PDFType::Disjunct(vec![assignment_13])));
    let dis_4 = TypeCheck::new(
        tctx,
        "thumb",
        Rc::new(PDFType::Disjunct(vec![assignment_12])),
    );
    let dis_3 = TypeCheck::new(tctx, "ci", Rc::new(PDFType::Disjunct(vec![assignment_11])));
    let dis_2 = TypeCheck::new(tctx, "rf", Rc::new(PDFType::Disjunct(vec![assignment_9])));
    let dis_1 = TypeCheck::new(tctx, "ef", Rc::new(PDFType::Disjunct(vec![assignment_8])));
    let dis_0 = TypeCheck::new(tctx, "id", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let choices_afrelationship = ChoicePred(
        String::from("Invalid AFRelationship"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Source"))),
            PDFObjT::Name(NameT::new(Vec::from("Data"))),
            PDFObjT::Name(NameT::new(Vec::from("Alternative"))),
            PDFObjT::Name(NameT::new(Vec::from("Supplement"))),
            PDFObjT::Name(NameT::new(Vec::from("EncryptedPayload"))),
            PDFObjT::Name(NameT::new(Vec::from("FormData"))),
            PDFObjT::Name(NameT::new(Vec::from("Schema"))),
            PDFObjT::Name(NameT::new(Vec::from("Unspecified"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Filespec")))],
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
    let fs_field = DictEntry {
        key: Vec::from("FS"),
        chk: assignment_0,

        opt: DictKeySpec::Optional,
    };
    let f_field = DictEntry {
        key: Vec::from("F"),
        chk: assignment_1,

        opt: DictKeySpec::Optional,
    };
    let uf_field = DictEntry {
        key: Vec::from("UF"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let dos_field = DictEntry {
        key: Vec::from("DOS"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let mac_field = DictEntry {
        key: Vec::from("Mac"),
        chk: assignment_4,

        opt: DictKeySpec::Optional,
    };
    let unix_field = DictEntry {
        key: Vec::from("Unix"),
        chk: assignment_5,

        opt: DictKeySpec::Optional,
    };
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let v_field = DictEntry {
        key: Vec::from("V"),
        chk: assignment_bool_7,

        opt: DictKeySpec::Optional,
    };
    let ef_field = DictEntry {
        key: Vec::from("EF"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let rf_field = DictEntry {
        key: Vec::from("RF"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let desc_field = DictEntry {
        key: Vec::from("Desc"),
        chk: assignment_10,

        opt: DictKeySpec::Optional,
    };
    let ci_field = DictEntry {
        key: Vec::from("CI"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let thumb_field = DictEntry {
        key: Vec::from("Thumb"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let afrelationship_field = DictEntry {
        key: Vec::from("AFRelationship"),
        chk: TypeCheck::new_refined(
            tctx,
            "afrelationship",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_afrelationship),
        ),
        opt: DictKeySpec::Optional,
    };
    let ep_field = DictEntry {
        key: Vec::from("EP"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "filespecification",
        Rc::new(PDFType::Dict(vec![
            type_field,
            fs_field,
            f_field,
            uf_field,
            dos_field,
            mac_field,
            unix_field,
            id_field,
            v_field,
            ef_field,
            rf_field,
            desc_field,
            ci_field,
            thumb_field,
            afrelationship_field,
            ep_field,
        ])),
    )
}
