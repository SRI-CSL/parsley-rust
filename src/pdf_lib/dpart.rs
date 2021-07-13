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
use crate::pdf_lib::arrayofdpartarrays::arrayofdpartarrays_type;
use crate::pdf_lib::arrayoffilespecifications::arrayoffilespecifications_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::dpartroot::dpartroot_type;
use crate::pdf_lib::metadata::metadata_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::pageobject::pageobject_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use std::rc::Rc;
pub fn dpart_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = metadata_type(tctx);
    let assignment_6 = arrayoffilespecifications_type(tctx);
    let assignment_5 = universaldictionary_type(tctx);
    let assignment_4 = pageobject_type(tctx);
    let assignment_3 = pageobject_type(tctx);
    let assignment_2 = arrayofdpartarrays_type(tctx);
    let assignment_1 = dpartroot_type(tctx);
    let assignment_0 = dpart_type(tctx);
    let dis_6 = TypeCheck::new(
        tctx,
        "metadata",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_5 = TypeCheck::new(tctx, "af", Rc::new(PDFType::Disjunct(vec![assignment_6])));
    let dis_4 = TypeCheck::new(tctx, "dpm", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_3 = TypeCheck::new(tctx, "end", Rc::new(PDFType::Disjunct(vec![assignment_4])));
    let dis_2 = TypeCheck::new(
        tctx,
        "start",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "dparts",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "parent",
        Rc::new(PDFType::Disjunct(vec![assignment_0, assignment_1])),
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("DPart")))],
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
    let parent_field = DictEntry {
        key: Vec::from("Parent"),
        chk: dis_0,
        opt: DictKeySpec::Required,
    };
    let dparts_field = DictEntry {
        key: Vec::from("DParts"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let start_field = DictEntry {
        key: Vec::from("Start"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let end_field = DictEntry {
        key: Vec::from("End"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let dpm_field = DictEntry {
        key: Vec::from("DPM"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let af_field = DictEntry {
        key: Vec::from("AF"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let metadata_field = DictEntry {
        key: Vec::from("Metadata"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "dpart",
        Rc::new(PDFType::Dict(vec![
            type_field,
            parent_field,
            dparts_field,
            start_field,
            end_field,
            dpm_field,
            af_field,
            metadata_field,
        ])),
    )
}
