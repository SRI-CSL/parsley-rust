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

// Operator classification
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpType {
    Compat,
    PathConstruction,
    PathPainting,
    PathClipping,
    InlineImage,
    MarkedContent,
    GeneralGraphics,
    SpecialGraphics,
    Color,
    TextState,
    TextObject,
    TextShow,
    TextPositioning,
    Shading,
    XObject,
    Type3Font,
}

// Very coarse approximation of argument type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArgType {
    Number,
    NumberArray,
    Name,
    Dict,
    String,
    // special cases
    NumberOrStringArray,
    NameOrDictionary,
    Star, /* arbitrary unchecked sequence */
}

// operator summary, Table A.1, page 844
pub const OPERATORS: &'static [(&str, (OpType, &[ArgType]))] = &[
    // compatibility, Table 33, page 112
    ("BX", (OpType::Compat, &[])),
    ("EX", (OpType::Compat, &[])),
    // graphics state, Table 56, page 164
    ("q", (OpType::GeneralGraphics, &[])),
    ("Q", (OpType::GeneralGraphics, &[])),
    (
        "cm",
        (
            OpType::SpecialGraphics,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    ("w", (OpType::GeneralGraphics, &[ArgType::Number])),
    ("J", (OpType::GeneralGraphics, &[ArgType::Number])),
    ("j", (OpType::GeneralGraphics, &[ArgType::Number])),
    ("M", (OpType::GeneralGraphics, &[ArgType::Number])),
    (
        "d",
        (
            OpType::GeneralGraphics,
            &[ArgType::NumberArray, ArgType::Number],
        ),
    ),
    ("ri", (OpType::GeneralGraphics, &[ArgType::Name])),
    ("i", (OpType::GeneralGraphics, &[ArgType::Number])),
    ("gs", (OpType::GeneralGraphics, &[ArgType::Dict])),
    // path construction, Table 58, page 170
    (
        "m",
        (
            OpType::PathConstruction,
            &[ArgType::Number, ArgType::Number],
        ),
    ),
    (
        "l",
        (
            OpType::PathConstruction,
            &[ArgType::Number, ArgType::Number],
        ),
    ),
    (
        "c",
        (
            OpType::PathConstruction,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    (
        "v",
        (
            OpType::PathConstruction,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    (
        "y",
        (
            OpType::PathConstruction,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    ("h", (OpType::PathConstruction, &[])),
    (
        "re",
        (
            OpType::PathConstruction,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    // path-painting, Table 59, page 173
    ("S", (OpType::PathPainting, &[])),
    ("s", (OpType::PathPainting, &[])),
    ("f", (OpType::PathPainting, &[])),
    ("F", (OpType::PathPainting, &[])),
    ("f*", (OpType::PathPainting, &[])),
    ("B", (OpType::PathPainting, &[])),
    ("B*", (OpType::PathPainting, &[])),
    ("b", (OpType::PathPainting, &[])),
    ("b*", (OpType::PathPainting, &[])),
    ("n", (OpType::PathPainting, &[])),
    // clipping path, Table 60, page 177
    ("W", (OpType::PathClipping, &[])),
    ("W*", (OpType::PathClipping, &[])),
    // color, Table 73, page 217
    ("CS", (OpType::Color, &[ArgType::Name])),
    ("cs", (OpType::Color, &[ArgType::Name])),
    // state-dependent arguments, temporarily treated as unchecked
    ("SC", (OpType::Color, &[ArgType::Star])),
    ("SCN", (OpType::Color, &[ArgType::Star])),
    ("sc", (OpType::Color, &[ArgType::Star])),
    ("scn", (OpType::Color, &[ArgType::Star])),
    ("G", (OpType::Color, &[ArgType::Number])),
    ("g", (OpType::Color, &[ArgType::Number])),
    (
        "RG",
        (
            OpType::Color,
            &[ArgType::Number, ArgType::Number, ArgType::Number],
        ),
    ),
    (
        "rg",
        (
            OpType::Color,
            &[ArgType::Number, ArgType::Number, ArgType::Number],
        ),
    ),
    (
        "K",
        (
            OpType::Color,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    (
        "k",
        (
            OpType::Color,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    // shading, Table 76, page 229
    ("sh", (OpType::Shading, &[ArgType::Name])),
    // XObject, Table 86, page 254
    ("Do", (OpType::XObject, &[ArgType::Name])),
    // inline image, Table 90, page 268
    ("BI", (OpType::InlineImage, &[])),
    ("ID", (OpType::InlineImage, &[])),
    ("EI", (OpType::InlineImage, &[])),
    // text state, Table 103, page 298
    ("Tc", (OpType::TextState, &[ArgType::Number])),
    ("Tw", (OpType::TextState, &[ArgType::Number])),
    ("Tz", (OpType::TextState, &[ArgType::Number])),
    ("TL", (OpType::TextState, &[ArgType::Number])),
    ("Tf", (OpType::TextState, &[ArgType::Name, ArgType::Number])),
    ("Tr", (OpType::TextState, &[ArgType::Number])),
    ("Ts", (OpType::TextState, &[ArgType::Number])),
    // text object, Table 105, page 306
    ("BT", (OpType::TextObject, &[])),
    ("ET", (OpType::TextObject, &[])),
    // text positioning, Table 106, page 308
    (
        "Td",
        (OpType::TextPositioning, &[ArgType::Number, ArgType::Number]),
    ),
    (
        "TD",
        (OpType::TextPositioning, &[ArgType::Number, ArgType::Number]),
    ),
    (
        "Tm",
        (
            OpType::TextPositioning,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    ("T*", (OpType::TextPositioning, &[])),
    // text showing, Table 107, page 309
    ("Tj", (OpType::TextShow, &[ArgType::String])),
    ("'", (OpType::TextShow, &[ArgType::String])),
    (
        "\"",
        (
            OpType::TextShow,
            &[ArgType::Number, ArgType::Number, ArgType::String],
        ),
    ),
    ("TJ", (OpType::TextShow, &[ArgType::NumberOrStringArray])), // special case
    // Type 3 font, Table 111, page 320
    (
        "d0",
        (OpType::Type3Font, &[ArgType::Number, ArgType::Number]),
    ),
    (
        "d1",
        (
            OpType::Type3Font,
            &[
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
                ArgType::Number,
            ],
        ),
    ),
    // marked-content, Table 352, page 721
    ("MP", (OpType::MarkedContent, &[ArgType::Name])),
    (
        "DP",
        (
            OpType::MarkedContent,
            &[ArgType::Name, ArgType::NameOrDictionary],
        ),
    ),
    ("BMC", (OpType::MarkedContent, &[ArgType::Name])),
    (
        "BDC",
        (
            OpType::MarkedContent,
            &[ArgType::Name, ArgType::NameOrDictionary],
        ),
    ),
    ("EMC", (OpType::MarkedContent, &[])),
];
