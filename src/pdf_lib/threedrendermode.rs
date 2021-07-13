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
use crate::pdf_lib::arrayof_4colourspaceentries::arrayof_4colourspaceentries_type;
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
pub fn threedrendermode_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_3 = TypeCheck::new(tctx, "cv", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_2 = TypeCheck::new(tctx, "o", Rc::new(PDFType::PrimType(PDFPrimType::Real)));
    let assignment_1 = arrayof_4colourspaceentries_type(tctx);
    let assignment_0 = arrayof_4colourspaceentries_type(tctx);
    let dis_1 = TypeCheck::new(tctx, "fc", Rc::new(PDFType::Disjunct(vec![assignment_1])));
    let dis_0 = TypeCheck::new(tctx, "ac", Rc::new(PDFType::Disjunct(vec![assignment_0])));
    let choices_subtype = ChoicePred(
        String::from("Invalid Subtype"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Solid"))),
            PDFObjT::Name(NameT::new(Vec::from("SolidWireframe"))),
            PDFObjT::Name(NameT::new(Vec::from("Transparent"))),
            PDFObjT::Name(NameT::new(Vec::from("TransparentWireframe"))),
            PDFObjT::Name(NameT::new(Vec::from("BoundingBox"))),
            PDFObjT::Name(NameT::new(Vec::from("TransparentBoundingBox"))),
            PDFObjT::Name(NameT::new(Vec::from("TransparentBoundingBoxOutline"))),
            PDFObjT::Name(NameT::new(Vec::from("Wireframe"))),
            PDFObjT::Name(NameT::new(Vec::from("ShadedWireframe"))),
            PDFObjT::Name(NameT::new(Vec::from("HiddenWireframe"))),
            PDFObjT::Name(NameT::new(Vec::from("Vertices"))),
            PDFObjT::Name(NameT::new(Vec::from("ShadedVertices"))),
            PDFObjT::Name(NameT::new(Vec::from("Illustration"))),
            PDFObjT::Name(NameT::new(Vec::from("SolidOutline"))),
            PDFObjT::Name(NameT::new(Vec::from("ShadedIllustration"))),
        ],
    );
    let choices_type = ChoicePred(
        String::from("Invalid Type"),
        vec![PDFObjT::Name(NameT::new(Vec::from("3DRenderMode")))],
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
    let subtype_field = DictEntry {
        key: Vec::from("Subtype"),
        chk: TypeCheck::new_refined(
            tctx,
            "subtype",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_subtype),
        ),
        opt: DictKeySpec::Required,
    };
    let ac_field = DictEntry {
        key: Vec::from("AC"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let fc_field = DictEntry {
        key: Vec::from("FC"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: assignment_2,

        opt: DictKeySpec::Optional,
    };
    let cv_field = DictEntry {
        key: Vec::from("CV"),
        chk: assignment_3,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "threedrendermode",
        Rc::new(PDFType::Dict(vec![
            type_field,
            subtype_field,
            ac_field,
            fc_field,
            o_field,
            cv_field,
        ])),
    )
}
