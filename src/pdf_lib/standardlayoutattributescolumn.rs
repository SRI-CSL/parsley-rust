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
use crate::pdf_lib::arrayof_3rgbnumbers::arrayof_3rgbnumbers_type;
use crate::pdf_lib::arrayof_4bordercolors::arrayof_4bordercolors_type;
use crate::pdf_lib::arrayof_4borderstylenames::arrayof_4borderstylenames_type;
use crate::pdf_lib::arrayof_4integers::arrayof_4integers_type;
use crate::pdf_lib::arrayofnumbersgeneral::arrayofnumbersgeneral_type;
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
pub fn standardlayoutattributescolumn_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_9 = arrayofnumbersgeneral_type(tctx);
    let assignment_8 = arrayofnumbersgeneral_type(tctx);
    let assignment_integer_7 = TypeCheck::new(
        tctx,
        "columncount",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_5 = arrayof_4integers_type(tctx);
    let assignment_4 = arrayof_4integers_type(tctx);
    let assignment_3 = arrayof_4borderstylenames_type(tctx);
    let assignment_1 = arrayof_4bordercolors_type(tctx);
    let assignment_0 = arrayof_3rgbnumbers_type(tctx);
    let assignment_2 = arrayof_3rgbnumbers_type(tctx);
    let assignment_6 = arrayof_3rgbnumbers_type(tctx);
    let dis_7 = TypeCheck::new(
        tctx,
        "columnwidths",
        Rc::new(PDFType::Disjunct(vec![assignment_9])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "columngap",
        Rc::new(PDFType::Disjunct(vec![assignment_8])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "color",
        Rc::new(PDFType::Disjunct(vec![assignment_6])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "padding",
        Rc::new(PDFType::Disjunct(vec![assignment_5])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "borderthickness",
        Rc::new(PDFType::Disjunct(vec![assignment_4])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "borderstyle",
        Rc::new(PDFType::Disjunct(vec![assignment_3])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "bordercolor",
        Rc::new(PDFType::Disjunct(vec![assignment_1, assignment_2])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "backgroundcolor",
        Rc::new(PDFType::Disjunct(vec![assignment_0])),
    );
    let choices_writingmode = ChoicePred(
        String::from("Invalid WritingMode"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("LrTb"))),
            PDFObjT::Name(NameT::new(Vec::from("RlTb"))),
            PDFObjT::Name(NameT::new(Vec::from("tbRl"))),
            PDFObjT::Name(NameT::new(Vec::from("TbLr"))),
            PDFObjT::Name(NameT::new(Vec::from("LrBt"))),
            PDFObjT::Name(NameT::new(Vec::from("RlBt"))),
            PDFObjT::Name(NameT::new(Vec::from("BtRl"))),
            PDFObjT::Name(NameT::new(Vec::from("BtLr"))),
        ],
    );
    let choices_placement = ChoicePred(
        String::from("Invalid Placement"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("Block"))),
            PDFObjT::Name(NameT::new(Vec::from("Inline"))),
            PDFObjT::Name(NameT::new(Vec::from("Before"))),
            PDFObjT::Name(NameT::new(Vec::from("Start"))),
            PDFObjT::Name(NameT::new(Vec::from("End"))),
        ],
    );
    let choices_o = ChoicePred(
        String::from("Invalid O"),
        vec![PDFObjT::Name(NameT::new(Vec::from("Layout")))],
    );
    let o_field = DictEntry {
        key: Vec::from("O"),
        chk: TypeCheck::new_refined(
            tctx,
            "o",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_o),
        ),
        opt: DictKeySpec::Required,
    };
    let placement_field = DictEntry {
        key: Vec::from("Placement"),
        chk: TypeCheck::new_refined(
            tctx,
            "placement",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_placement),
        ),
        opt: DictKeySpec::Optional,
    };
    let writingmode_field = DictEntry {
        key: Vec::from("WritingMode"),
        chk: TypeCheck::new_refined(
            tctx,
            "writingmode",
            Rc::new(PDFType::PrimType(PDFPrimType::Name)),
            Rc::new(choices_writingmode),
        ),
        opt: DictKeySpec::Optional,
    };
    let backgroundcolor_field = DictEntry {
        key: Vec::from("BackgroundColor"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let bordercolor_field = DictEntry {
        key: Vec::from("BorderColor"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let borderstyle_field = DictEntry {
        key: Vec::from("BorderStyle"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let borderthickness_field = DictEntry {
        key: Vec::from("BorderThickness"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let padding_field = DictEntry {
        key: Vec::from("Padding"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let color_field = DictEntry {
        key: Vec::from("Color"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let columncount_field = DictEntry {
        key: Vec::from("ColumnCount"),
        chk: assignment_integer_7,

        opt: DictKeySpec::Optional,
    };
    let columngap_field = DictEntry {
        key: Vec::from("ColumnGap"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let columnwidths_field = DictEntry {
        key: Vec::from("ColumnWidths"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "standardlayoutattributescolumn",
        Rc::new(PDFType::Dict(vec![
            o_field,
            placement_field,
            writingmode_field,
            backgroundcolor_field,
            bordercolor_field,
            borderstyle_field,
            borderthickness_field,
            padding_field,
            color_field,
            columncount_field,
            columngap_field,
            columnwidths_field,
        ])),
    )
}
