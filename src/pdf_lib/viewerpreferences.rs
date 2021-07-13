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
use crate::pdf_lib::arrayofintegersgeneral::arrayofintegersgeneral_type;
use crate::pdf_lib::arrayofnamesforenforce::arrayofnamesforenforce_type;
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
pub fn viewerpreferences_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_9 = arrayofnamesforenforce_type(tctx);
    let assignment_integer_8 = TypeCheck::new(
        tctx,
        "numcopies",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_7 = arrayofintegersgeneral_type(tctx);
    let assignment_bool_6 = TypeCheck::new(
        tctx,
        "picktraybypdfsize",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_5 = TypeCheck::new(
        tctx,
        "displaydoctitle",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_4 = TypeCheck::new(
        tctx,
        "centerwindow",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_3 = TypeCheck::new(
        tctx,
        "fitwindow",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_2 = TypeCheck::new(
        tctx,
        "hidewindowui",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_1 = TypeCheck::new(
        tctx,
        "hidemenubar",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_0 = TypeCheck::new(
        tctx,
        "hidetoolbar",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "enforce",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "printpagerange",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let choices_duplex = ChoicePred(
        String::from("Invalid Duplex"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Simplex"))),
            PDFObjT::Name(NameT::new(Vec::from("DuplexFlipShortEdge"))),
            PDFObjT::Name(NameT::new(Vec::from("DuplexFlipLongEdge"))),
        ],
    );
    let choices_printscaling = ChoicePred(
        String::from("Invalid PrintScaling"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("None"))),
            PDFObjT::Name(NameT::new(Vec::from("AppDefault"))),
        ],
    );
    let choices_printclip = ChoicePred(
        String::from("Invalid PrintClip"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("MediaBox"))),
            PDFObjT::Name(NameT::new(Vec::from("CropBox"))),
            PDFObjT::Name(NameT::new(Vec::from("BleedBox"))),
            PDFObjT::Name(NameT::new(Vec::from("TrimBox"))),
            PDFObjT::Name(NameT::new(Vec::from("ArtBox"))),
        ],
    );
    let choices_printarea = ChoicePred(
        String::from("Invalid PrintArea"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("MediaBox"))),
            PDFObjT::Name(NameT::new(Vec::from("CropBox"))),
            PDFObjT::Name(NameT::new(Vec::from("BleedBox"))),
            PDFObjT::Name(NameT::new(Vec::from("TrimBox"))),
            PDFObjT::Name(NameT::new(Vec::from("ArtBox"))),
        ],
    );
    let choices_viewclip = ChoicePred(
        String::from("Invalid ViewClip"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("MediaBox"))),
            PDFObjT::Name(NameT::new(Vec::from("CropBox"))),
            PDFObjT::Name(NameT::new(Vec::from("BleedBox"))),
            PDFObjT::Name(NameT::new(Vec::from("TrimBox"))),
            PDFObjT::Name(NameT::new(Vec::from("ArtBox"))),
        ],
    );
    let choices_viewarea = ChoicePred(
        String::from("Invalid ViewArea"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("MediaBox"))),
            PDFObjT::Name(NameT::new(Vec::from("CropBox"))),
            PDFObjT::Name(NameT::new(Vec::from("BleedBox"))),
            PDFObjT::Name(NameT::new(Vec::from("TrimBox"))),
            PDFObjT::Name(NameT::new(Vec::from("ArtBox"))),
        ],
    );
    let choices_direction = ChoicePred(
        String::from("Invalid Direction"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("L2R"))),
            PDFObjT::Name(NameT::new(Vec::from("R2L"))),
        ],
    );
    let choices_nonfullscreenpagemode = ChoicePred(
        String::from("Invalid NonFullScreenPageMode"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("UseNone"))),
            PDFObjT::Name(NameT::new(Vec::from("UseOutlines"))),
            PDFObjT::Name(NameT::new(Vec::from("UseThumbs"))),
            PDFObjT::Name(NameT::new(Vec::from("UseOC"))),
        ],
    );
    let hidetoolbar_field = DictEntry {
        key: Vec::from("HideToolbar"),
        chk: assignment_bool_0,

        opt: DictKeySpec::Optional,
    };
    let hidemenubar_field = DictEntry {
        key: Vec::from("HideMenubar"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let hidewindowui_field = DictEntry {
        key: Vec::from("HideWindowUI"),
        chk: assignment_bool_2,

        opt: DictKeySpec::Optional,
    };
    let fitwindow_field = DictEntry {
        key: Vec::from("FitWindow"),
        chk: assignment_bool_3,

        opt: DictKeySpec::Optional,
    };
    let centerwindow_field = DictEntry {
        key: Vec::from("CenterWindow"),
        chk: assignment_bool_4,

        opt: DictKeySpec::Optional,
    };
    let displaydoctitle_field = DictEntry {
        key: Vec::from("DisplayDocTitle"),
        chk: assignment_bool_5,

        opt: DictKeySpec::Optional,
    };
    let nonfullscreenpagemode_field = DictEntry {
        key: Vec::from("NonFullScreenPageMode"),
        chk: TypeCheck::new_refined(
            tctx,
            "nonfullscreenpagemode",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_nonfullscreenpagemode),
        ),
        opt: DictKeySpec::Optional,
    };
    let direction_field = DictEntry {
        key: Vec::from("Direction"),
        chk: TypeCheck::new_refined(
            tctx,
            "direction",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_direction),
        ),
        opt: DictKeySpec::Optional,
    };
    let viewarea_field = DictEntry {
        key: Vec::from("ViewArea"),
        chk: TypeCheck::new_refined(
            tctx,
            "viewarea",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_viewarea),
        ),
        opt: DictKeySpec::Optional,
    };
    let viewclip_field = DictEntry {
        key: Vec::from("ViewClip"),
        chk: TypeCheck::new_refined(
            tctx,
            "viewclip",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_viewclip),
        ),
        opt: DictKeySpec::Optional,
    };
    let printarea_field = DictEntry {
        key: Vec::from("PrintArea"),
        chk: TypeCheck::new_refined(
            tctx,
            "printarea",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_printarea),
        ),
        opt: DictKeySpec::Optional,
    };
    let printclip_field = DictEntry {
        key: Vec::from("PrintClip"),
        chk: TypeCheck::new_refined(
            tctx,
            "printclip",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_printclip),
        ),
        opt: DictKeySpec::Optional,
    };
    let printscaling_field = DictEntry {
        key: Vec::from("PrintScaling"),
        chk: TypeCheck::new_refined(
            tctx,
            "printscaling",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_printscaling),
        ),
        opt: DictKeySpec::Optional,
    };
    let duplex_field = DictEntry {
        key: Vec::from("Duplex"),
        chk: TypeCheck::new_refined(
            tctx,
            "duplex",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_duplex),
        ),
        opt: DictKeySpec::Optional,
    };
    let picktraybypdfsize_field = DictEntry {
        key: Vec::from("PickTrayByPDFSize"),
        chk: assignment_bool_6,

        opt: DictKeySpec::Optional,
    };
    let printpagerange_field = DictEntry {
        key: Vec::from("PrintPageRange"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let numcopies_field = DictEntry {
        key: Vec::from("NumCopies"),
        chk: assignment_integer_8,

        opt: DictKeySpec::Optional,
    };
    let enforce_field = DictEntry {
        key: Vec::from("Enforce"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "viewerpreferences",
        Rc::new(PDFType::Dict(vec![
            hidetoolbar_field,
            hidemenubar_field,
            hidewindowui_field,
            fitwindow_field,
            centerwindow_field,
            displaydoctitle_field,
            nonfullscreenpagemode_field,
            direction_field,
            viewarea_field,
            viewclip_field,
            printarea_field,
            printclip_field,
            printscaling_field,
            duplex_field,
            picktraybypdfsize_field,
            printpagerange_field,
            numcopies_field,
            enforce_field,
        ])),
    )
}
