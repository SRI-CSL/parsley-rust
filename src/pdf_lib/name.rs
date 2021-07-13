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
use crate::pdf_lib::actionecmascript::actionecmascript_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::dest0::dest0_type;
use crate::pdf_lib::dest1::dest1_type;
use crate::pdf_lib::dest4::dest4_type;
use crate::pdf_lib::destdict::destdict_type;
use crate::pdf_lib::destxyz::destxyz_type;
use crate::pdf_lib::embeddedfilestream::embeddedfilestream_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::renditionmedia::renditionmedia_type;
use crate::pdf_lib::renditionselector::renditionselector_type;
use crate::pdf_lib::slideshow::slideshow_type;
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use crate::pdf_lib::webcaptureimageset::webcaptureimageset_type;
use crate::pdf_lib::webcapturepageset::webcapturepageset_type;
use crate::pdf_lib::xobjectformps::xobjectformps_type;
use crate::pdf_lib::xobjectformpspassthrough::xobjectformpspassthrough_type;
use crate::pdf_lib::xobjectformtype1::xobjectformtype1_type;
use std::rc::Rc;
pub fn name_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_18 = renditionselector_type(tctx);
    let assignment_17 = renditionmedia_type(tctx);
    let assignment_16 = slideshow_type(tctx);
    let assignment_15 = embeddedfilestream_type(tctx);
    let assignment_12 = webcapturepageset_type(tctx);
    let assignment_14 = webcapturepageset_type(tctx);
    let assignment_11 = webcaptureimageset_type(tctx);
    let assignment_13 = webcaptureimageset_type(tctx);
    let assignment_10 = universaldictionary_type(tctx);
    let assignment_9 = pageobject_type(tctx);
    let assignment_8 = actionecmascript_type(tctx);
    let assignment_7 = xobjectformpspassthrough_type(tctx);
    let assignment_6 = xobjectformps_type(tctx);
    let assignment_5 = xobjectformtype1_type(tctx);
    let assignment_4 = dest4_type(tctx);
    let assignment_3 = dest1_type(tctx);
    let assignment_2 = dest0_type(tctx);
    let assignment_1 = destxyz_type(tctx);
    let assignment_0 = destdict_type(tctx);
    let dis_9 = TypeCheck::new(
        tctx,
        "renditions",
        Rc::new(PDFType::Disjunct(vec![assignment_17, assignment_18])),
    );
    let dis_8 = TypeCheck::new(
        tctx,
        "alternatepresentations",
        Rc::new(PDFType::Disjunct(vec![assignment_16])),
    );
    let dis_7 = TypeCheck::new(
        tctx,
        "embeddedfiles",
        Rc::new(PDFType::Disjunct(vec![assignment_15])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "urls",
        Rc::new(PDFType::Disjunct(vec![assignment_13, assignment_14])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "ids",
        Rc::new(PDFType::Disjunct(vec![assignment_11, assignment_12])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "templates",
        Rc::new(PDFType::Disjunct(vec![assignment_10])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "pages",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "javascript",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "ap",
        Rc::new(PDFType::Disjunct(vec![
            assignment_5,
            assignment_6,
            assignment_7,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "dests",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
            assignment_4,
        ])),
    );
    let dests_field = DictEntry {
        key: Vec::from("Dests"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let ap_field = DictEntry {
        key: Vec::from("AP"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let javascript_field = DictEntry {
        key: Vec::from("JavaScript"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let pages_field = DictEntry {
        key: Vec::from("Pages"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let templates_field = DictEntry {
        key: Vec::from("Templates"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let ids_field = DictEntry {
        key: Vec::from("IDS"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let urls_field = DictEntry {
        key: Vec::from("URLS"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let embeddedfiles_field = DictEntry {
        key: Vec::from("EmbeddedFiles"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let alternatepresentations_field = DictEntry {
        key: Vec::from("AlternatePresentations"),
        chk: dis_8,
        opt: DictKeySpec::Optional,
    };
    let renditions_field = DictEntry {
        key: Vec::from("Renditions"),
        chk: dis_9,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "name",
        Rc::new(PDFType::Dict(vec![
            dests_field,
            ap_field,
            javascript_field,
            pages_field,
            templates_field,
            ids_field,
            urls_field,
            embeddedfiles_field,
            alternatepresentations_field,
            renditions_field,
        ])),
    )
}
