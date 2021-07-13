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
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;
pub fn legalattestation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_21 = TypeCheck::new(
        tctx,
        "attestation",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_bool_20 = TypeCheck::new(
        tctx,
        "optionalcontent",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_integer_19 = TypeCheck::new(
        tctx,
        "annotations",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_18 = TypeCheck::new(
        tctx,
        "devdepgs_fl",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_17 = TypeCheck::new(
        tctx,
        "devdepgs_bg",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_16 = TypeCheck::new(
        tctx,
        "devdepgs_ucr",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_15 = TypeCheck::new(
        tctx,
        "devdepgs_tr",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_14 = TypeCheck::new(
        tctx,
        "devdepgs_ht",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_13 = TypeCheck::new(
        tctx,
        "devdepgs_op",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_12 = TypeCheck::new(
        tctx,
        "nonembeddedfonts",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_11 = TypeCheck::new(
        tctx,
        "externalopidicts",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_10 = TypeCheck::new(
        tctx,
        "externalrefxobjects",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_9 = TypeCheck::new(
        tctx,
        "truetypefonts",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_8 = TypeCheck::new(
        tctx,
        "externalstreams",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_7 = TypeCheck::new(
        tctx,
        "alternateimages",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_6 = TypeCheck::new(
        tctx,
        "gotoremoteactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_5 = TypeCheck::new(
        tctx,
        "hideannotationactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_4 = TypeCheck::new(
        tctx,
        "soundactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_3 = TypeCheck::new(
        tctx,
        "movieactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_2 = TypeCheck::new(
        tctx,
        "uriactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_1 = TypeCheck::new(
        tctx,
        "launchactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_0 = TypeCheck::new(
        tctx,
        "javascriptactions",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let javascriptactions_field = DictEntry {
        key: Vec::from("JavaScriptActions"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let launchactions_field = DictEntry {
        key: Vec::from("LaunchActions"),
        chk: assignment_integer_1,

        opt: DictKeySpec::Optional,
    };
    let uriactions_field = DictEntry {
        key: Vec::from("URIActions"),
        chk: assignment_integer_2,

        opt: DictKeySpec::Optional,
    };
    let movieactions_field = DictEntry {
        key: Vec::from("MovieActions"),
        chk: assignment_integer_3,

        opt: DictKeySpec::Optional,
    };
    let soundactions_field = DictEntry {
        key: Vec::from("SoundActions"),
        chk: assignment_integer_4,

        opt: DictKeySpec::Optional,
    };
    let hideannotationactions_field = DictEntry {
        key: Vec::from("HideAnnotationActions"),
        chk: assignment_integer_5,

        opt: DictKeySpec::Optional,
    };
    let gotoremoteactions_field = DictEntry {
        key: Vec::from("GoToRemoteActions"),
        chk: assignment_integer_6,

        opt: DictKeySpec::Optional,
    };
    let alternateimages_field = DictEntry {
        key: Vec::from("AlternateImages"),
        chk: assignment_integer_7,

        opt: DictKeySpec::Optional,
    };
    let externalstreams_field = DictEntry {
        key: Vec::from("ExternalStreams"),
        chk: assignment_integer_8,

        opt: DictKeySpec::Optional,
    };
    let truetypefonts_field = DictEntry {
        key: Vec::from("TrueTypeFonts"),
        chk: assignment_integer_9,

        opt: DictKeySpec::Optional,
    };
    let externalrefxobjects_field = DictEntry {
        key: Vec::from("ExternalRefXobjects"),
        chk: assignment_integer_10,

        opt: DictKeySpec::Optional,
    };
    let externalopidicts_field = DictEntry {
        key: Vec::from("ExternalOPIdicts"),
        chk: assignment_integer_11,

        opt: DictKeySpec::Optional,
    };
    let nonembeddedfonts_field = DictEntry {
        key: Vec::from("NonEmbeddedFonts"),
        chk: assignment_integer_12,

        opt: DictKeySpec::Optional,
    };
    let devdepgs_op_field = DictEntry {
        key: Vec::from("DevDepGS_OP"),
        chk: assignment_integer_13,

        opt: DictKeySpec::Optional,
    };
    let devdepgs_ht_field = DictEntry {
        key: Vec::from("DevDepGS_HT"),
        chk: assignment_integer_14,

        opt: DictKeySpec::Optional,
    };
    let devdepgs_tr_field = DictEntry {
        key: Vec::from("DevDepGS_TR"),
        chk: assignment_integer_15,

        opt: DictKeySpec::Optional,
    };
    let devdepgs_ucr_field = DictEntry {
        key: Vec::from("DevDepGS_UCR"),
        chk: assignment_integer_16,

        opt: DictKeySpec::Optional,
    };
    let devdepgs_bg_field = DictEntry {
        key: Vec::from("DevDepGS_BG"),
        chk: assignment_integer_17,

        opt: DictKeySpec::Optional,
    };
    let devdepgs_fl_field = DictEntry {
        key: Vec::from("DevDepGS_FL"),
        chk: assignment_integer_18,

        opt: DictKeySpec::Optional,
    };
    let annotations_field = DictEntry {
        key: Vec::from("Annotations"),
        chk: assignment_integer_19,

        opt: DictKeySpec::Optional,
    };
    let optionalcontent_field = DictEntry {
        key: Vec::from("OptionalContent"),
        chk: assignment_bool_20,

        opt: DictKeySpec::Optional,
    };
    let attestation_field = DictEntry {
        key: Vec::from("Attestation"),
        chk: assignment_21,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "legalattestation",
        Rc::new(PDFType::Dict(vec![
            javascriptactions_field,
            launchactions_field,
            uriactions_field,
            movieactions_field,
            soundactions_field,
            hideannotationactions_field,
            gotoremoteactions_field,
            alternateimages_field,
            externalstreams_field,
            truetypefonts_field,
            externalrefxobjects_field,
            externalopidicts_field,
            nonembeddedfonts_field,
            devdepgs_op_field,
            devdepgs_ht_field,
            devdepgs_tr_field,
            devdepgs_ucr_field,
            devdepgs_bg_field,
            devdepgs_fl_field,
            annotations_field,
            optionalcontent_field,
            attestation_field,
        ])),
    )
}
