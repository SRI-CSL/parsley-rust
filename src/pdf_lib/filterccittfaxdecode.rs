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
pub fn filterccittfaxdecode_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_integer_7 = TypeCheck::new(
        tctx,
        "damagedrowsbeforeerror",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_bool_6 = TypeCheck::new(
        tctx,
        "blackis1",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_5 = TypeCheck::new(
        tctx,
        "endofblock",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_integer_4 = TypeCheck::new(
        tctx,
        "rows",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_integer_3 = TypeCheck::new(
        tctx,
        "columns",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    );
    let assignment_bool_2 = TypeCheck::new(
        tctx,
        "encodedbytealign",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_bool_1 = TypeCheck::new(
        tctx,
        "endofline",
        Rc::new(PDFType::PrimType(PDFPrimType::Bool)),
    );
    let assignment_integer_0 =
        TypeCheck::new(tctx, "k", Rc::new(PDFType::PrimType(PDFPrimType::Integer)));
    let k_field = DictEntry {
        key: Vec::from("K"),
        chk: assignment_integer_0,

        opt: DictKeySpec::Optional,
    };
    let endofline_field = DictEntry {
        key: Vec::from("EndOfLine"),
        chk: assignment_bool_1,

        opt: DictKeySpec::Optional,
    };
    let encodedbytealign_field = DictEntry {
        key: Vec::from("EncodedByteAlign"),
        chk: assignment_bool_2,

        opt: DictKeySpec::Optional,
    };
    let columns_field = DictEntry {
        key: Vec::from("Columns"),
        chk: assignment_integer_3,

        opt: DictKeySpec::Optional,
    };
    let rows_field = DictEntry {
        key: Vec::from("Rows"),
        chk: assignment_integer_4,

        opt: DictKeySpec::Optional,
    };
    let endofblock_field = DictEntry {
        key: Vec::from("EndOfBlock"),
        chk: assignment_bool_5,

        opt: DictKeySpec::Optional,
    };
    let blackis1_field = DictEntry {
        key: Vec::from("BlackIs1"),
        chk: assignment_bool_6,

        opt: DictKeySpec::Optional,
    };
    let damagedrowsbeforeerror_field = DictEntry {
        key: Vec::from("DamagedRowsBeforeError"),
        chk: assignment_integer_7,

        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "filterccittfaxdecode",
        Rc::new(PDFType::Dict(vec![
            k_field,
            endofline_field,
            encodedbytealign_field,
            columns_field,
            rows_field,
            endofblock_field,
            blackis1_field,
            damagedrowsbeforeerror_field,
        ])),
    )
}
