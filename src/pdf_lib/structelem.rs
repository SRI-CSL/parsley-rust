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
use crate::pdf_lib::arrayofattributes::arrayofattributes_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::arrayofnamesgeneral::arrayofnamesgeneral_type;
use crate::pdf_lib::arrayofstructelem::arrayofstructelem_type;
use crate::pdf_lib::arrayofstructelemkids::arrayofstructelemkids_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::filespecification::filespecification_type;
use crate::pdf_lib::markedcontentreference::markedcontentreference_type;
use crate::pdf_lib::namespace::namespace_type;
use crate::pdf_lib::nsoattributes::nsoattributes_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::objectreference::objectreference_type;
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::standardartifactattributes::standardartifactattributes_type;
use crate::pdf_lib::standardlayoutattributesblse::standardlayoutattributesblse_type;
use crate::pdf_lib::standardlayoutattributescolumn::standardlayoutattributescolumn_type;
use crate::pdf_lib::standardlayoutattributesilse::standardlayoutattributesilse_type;
use crate::pdf_lib::standardlistattributes::standardlistattributes_type;
use crate::pdf_lib::standardprintfieldattributes::standardprintfieldattributes_type;
use crate::pdf_lib::standardtableattributes::standardtableattributes_type;
use crate::pdf_lib::stream::stream_type;
use crate::pdf_lib::structtreeroot::structtreeroot_type;
use crate::pdf_lib::userpropertiesattributes::userpropertiesattributes_type;
use std::rc::Rc;
pub fn structelem_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_32 = TypeCheck::new(
        tctx,
        "phoneme",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_31 = TypeCheck::new(
        tctx,
        "phoneticalphabet",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
    );
    let assignment_30 = namespace_type(tctx);
    let assignment_29 = filespecification_type(tctx);
    let assignment_28 = arrayoffilespecifications_type(tctx);
    let assignment_27 = TypeCheck::new(
        tctx,
        "actualtext",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_26 = TypeCheck::new(tctx, "e", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_25 =
        TypeCheck::new(tctx, "alt", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_24 = TypeCheck::new(
        tctx,
        "lang",
        Rc::new(PDFType::PrimType(PDFPrimType::String)),
    );
    let assignment_23 = TypeCheck::new(tctx, "t", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_integer_22 =
        TypeCheck::new(tctx, "r", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let assignment_21 = arrayofnamesgeneral_type(tctx);
    let assignment_20 = stream_type(tctx);
    let assignment_19 = nsoattributes_type(tctx);
    let assignment_18 = userpropertiesattributes_type(tctx);
    let assignment_17 = standardartifactattributes_type(tctx);
    let assignment_16 = standardprintfieldattributes_type(tctx);
    let assignment_15 = standardlistattributes_type(tctx);
    let assignment_14 = standardtableattributes_type(tctx);
    let assignment_13 = standardlayoutattributesilse_type(tctx);
    let assignment_12 = standardlayoutattributescolumn_type(tctx);
    let assignment_11 = standardlayoutattributesblse_type(tctx);
    let assignment_10 = arrayofattributes_type(tctx);
    let assignment_9 = objectreference_type(tctx);
    let assignment_8 = markedcontentreference_type(tctx);
    let assignment_6 = arrayofstructelemkids_type(tctx);
    let assignment_5 = pageobject_type(tctx);
    let assignment_4 = arrayofstructelem_type(tctx);
    let assignment_3 = TypeCheck::new(tctx, "id", Rc::new(PDFType::PrimType(PDFPrimType::String)));
    let assignment_2 = structtreeroot_type(tctx);
    let assignment_1 = structelem_type(tctx);
    let assignment_7 = structelem_type(tctx);
    let assignment_0 = TypeCheck::new(tctx, "s", Rc::new(PDFType::PrimType(PDFPrimType::Name)));
    let dis_7 = TypeCheck::new(tctx, "ns", Rc::new(PDFType::Disjunct(vec![assignment_30])));
    let dis_6 = TypeCheck::new(
        tctx,
        "af",
        Rc::new(PDFType::Disjunct(vec![assignment_28, assignment_29])),
    );
    let dis_5 = TypeCheck::new(tctx, "c", Rc::new(PDFType::Disjunct(vec![assignment_21])));
    let dis_4 = TypeCheck::new(
        tctx,
        "a",
        Rc::new(PDFType::Disjunct(vec![
            assignment_10,
            assignment_11,
            assignment_12,
            assignment_13,
            assignment_14,
            assignment_15,
            assignment_16,
            assignment_17,
            assignment_18,
            assignment_19,
            assignment_20,
        ])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "k",
        Rc::new(PDFType::Disjunct(vec![
            assignment_6,
            assignment_7,
            assignment_8,
            assignment_9,
        ])),
    );
    let dis_2 = TypeCheck::new(tctx, "pg", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_1 = TypeCheck::new(tctx, "ref", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_0 = TypeCheck::new(
        tctx,
        "p",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("StructElem")))],
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
    let s_field = DictEntry {
        key: Vec::from("S"),
        chk: assignment_0,

        opt: DictKeySpec::Required,
    };
    let p_field = DictEntry {
        key: Vec::from("P"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let id_field = DictEntry {
        key: Vec::from("ID"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    let ref_field = DictEntry {
        key: Vec::from("Ref"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let pg_field = DictEntry {
        key: Vec::from("Pg"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let k_field = DictEntry {
        key: Vec::from("K"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let a_field = DictEntry {
        key: Vec::from("A"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let c_field = DictEntry {
        key: Vec::from("C"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let r_field = DictEntry {
        key: Vec::from("R"),
        chk: assignment_integer_22,

        opt: DictKeySpec::Optional,
    };
    let t_field = DictEntry {
        key: Vec::from("T"),
        chk: assignment_23,

        opt: DictKeySpec::Optional,
    };
    let lang_field = DictEntry {
        key: Vec::from("Lang"),
        chk: assignment_24,

        opt: DictKeySpec::Optional,
    };
    let alt_field = DictEntry {
        key: Vec::from("Alt"),
        chk: assignment_25,

        opt: DictKeySpec::Optional,
    };
    let e_field = DictEntry {
        key: Vec::from("E"),
        chk: assignment_26,

        opt: DictKeySpec::Optional,
    };
    let actualtext_field = DictEntry {
        key: Vec::from("ActualText"),
        chk: assignment_27,

        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let ns_field = DictEntry {
        key: Vec::from("NS"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    let phoneticalphabet_field = DictEntry {
        key: Vec::from("PhoneticAlphabet"),
        chk: assignment_31,

        opt: DictKeySpec::Optional,
    };
    let phoneme_field = DictEntry {
        key: Vec::from("Phoneme"),
        chk: assignment_32,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "structelem",
        Rc::new(PDFType::Dict(vec![
            type_field,
            s_field,
            p_field,
            id_field,
            ref_field,
            pg_field,
            k_field,
            a_field,
            c_field,
            r_field,
            t_field,
            lang_field,
            alt_field,
            e_field,
            actualtext_field,
            af_field,
            ns_field,
            phoneticalphabet_field,
            phoneme_field,
        ])),
    )
}
