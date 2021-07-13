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
use crate::pdf_lib::arrayofnamesforprocset::arrayofnamesforprocset_type;
use crate::pdf_lib::colorspacemap::colorspacemap_type;
use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_indirect_array_typchk, mk_generic_indirect_dict_typchk,
    mk_generic_indirect_stream_typchk, mk_name_check, mk_rectangle_typchk, name_dictionary,
};
use crate::pdf_lib::fontmap::fontmap_type;
use crate::pdf_lib::graphicsstateparametermap::graphicsstateparametermap_type;
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::patternmap::patternmap_type;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use crate::pdf_lib::shadingmap::shadingmap_type;
use crate::pdf_lib::universaldictionary::universaldictionary_type;
use crate::pdf_lib::xobjectmap::xobjectmap_type;
use std::rc::Rc;
pub fn resource_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_7 = universaldictionary_type(tctx);
    let assignment_6 = arrayofnamesforprocset_type(tctx);
    let assignment_5 = fontmap_type(tctx);
    let assignment_4 = xobjectmap_type(tctx);
    let assignment_3 = shadingmap_type(tctx);
    let assignment_2 = patternmap_type(tctx);
    let assignment_1 = colorspacemap_type(tctx);
    let assignment_0 = graphicsstateparametermap_type(tctx);
    let dis_7 = TypeCheck::new(
        tctx,
        "properties",
        Rc::new(PDFType::Disjunct(vec![assignment_7])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "procset",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_5 = TypeCheck::new(tctx, "font", Rc::new(PDFType::Disjunct(vec![assignment_5])));
    let dis_4 = TypeCheck::new(
        tctx,
        "xobject",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "shading",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "pattern",
        Rc::new(PDFType::Disjunct(vec![assignment_2])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "colorspace",
        Rc::new(PDFType::Disjunct(vec![assignment_1])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "extgstate",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let extgstate_field = DictEntry {
        key: Vec::from("ExtGState"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let colorspace_field = DictEntry {
        key: Vec::from("ColorSpace"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let pattern_field = DictEntry {
        key: Vec::from("Pattern"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let shading_field = DictEntry {
        key: Vec::from("Shading"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let xobject_field = DictEntry {
        key: Vec::from("XObject"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let font_field = DictEntry {
        key: Vec::from("Font"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let procset_field = DictEntry {
        key: Vec::from("ProcSet"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let properties_field = DictEntry {
        key: Vec::from("Properties"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "resource",
        Rc::new(PDFType::Dict(vec![
            extgstate_field,
            colorspace_field,
            pattern_field,
            shading_field,
            xobject_field,
            font_field,
            procset_field,
            properties_field,
        ])),
    )
}
