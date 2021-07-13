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
use crate::pdf_lib::actiongoto::actiongoto_type;
use crate::pdf_lib::actiongoto3dview::actiongoto3dview_type;
use crate::pdf_lib::actiongotodp::actiongotodp_type;
use crate::pdf_lib::actiongotoe::actiongotoe_type;
use crate::pdf_lib::actiongotor::actiongotor_type;
use crate::pdf_lib::actionhide::actionhide_type;
use crate::pdf_lib::actionimportdata::actionimportdata_type;
use crate::pdf_lib::actionlaunch::actionlaunch_type;
use crate::pdf_lib::actionmovie::actionmovie_type;
use crate::pdf_lib::actionnamed::actionnamed_type;
use crate::pdf_lib::actionrendition::actionrendition_type;
use crate::pdf_lib::actionresetform::actionresetform_type;
use crate::pdf_lib::actionrichmediaexecute::actionrichmediaexecute_type;
use crate::pdf_lib::actionsetocgstate::actionsetocgstate_type;
use crate::pdf_lib::actionsound::actionsound_type;
use crate::pdf_lib::actionsubmitform::actionsubmitform_type;
use crate::pdf_lib::actionthread::actionthread_type;
use crate::pdf_lib::actiontransition::actiontransition_type;
use crate::pdf_lib::actionuri::actionuri_type;
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
pub fn addactionscreenannotation_type<'a>(tctx: &'a mut TypeCheckContext) -> Rc<TypeCheck> {
    let assignment_39 = actionrichmediaexecute_type(tctx);
    let assignment_19 = actionrichmediaexecute_type(tctx);
    let assignment_139 = actionrichmediaexecute_type(tctx);
    let assignment_159 = actionrichmediaexecute_type(tctx);
    let assignment_119 = actionrichmediaexecute_type(tctx);
    let assignment_59 = actionrichmediaexecute_type(tctx);
    let assignment_79 = actionrichmediaexecute_type(tctx);
    let assignment_99 = actionrichmediaexecute_type(tctx);
    let assignment_118 = actionecmascript_type(tctx);
    let assignment_58 = actionecmascript_type(tctx);
    let assignment_158 = actionecmascript_type(tctx);
    let assignment_18 = actionecmascript_type(tctx);
    let assignment_38 = actionecmascript_type(tctx);
    let assignment_98 = actionecmascript_type(tctx);
    let assignment_138 = actionecmascript_type(tctx);
    let assignment_78 = actionecmascript_type(tctx);
    let assignment_57 = actiongoto3dview_type(tctx);
    let assignment_117 = actiongoto3dview_type(tctx);
    let assignment_17 = actiongoto3dview_type(tctx);
    let assignment_77 = actiongoto3dview_type(tctx);
    let assignment_137 = actiongoto3dview_type(tctx);
    let assignment_37 = actiongoto3dview_type(tctx);
    let assignment_157 = actiongoto3dview_type(tctx);
    let assignment_97 = actiongoto3dview_type(tctx);
    let assignment_76 = actiontransition_type(tctx);
    let assignment_96 = actiontransition_type(tctx);
    let assignment_36 = actiontransition_type(tctx);
    let assignment_136 = actiontransition_type(tctx);
    let assignment_16 = actiontransition_type(tctx);
    let assignment_116 = actiontransition_type(tctx);
    let assignment_56 = actiontransition_type(tctx);
    let assignment_156 = actiontransition_type(tctx);
    let assignment_55 = actionrendition_type(tctx);
    let assignment_155 = actionrendition_type(tctx);
    let assignment_95 = actionrendition_type(tctx);
    let assignment_115 = actionrendition_type(tctx);
    let assignment_75 = actionrendition_type(tctx);
    let assignment_135 = actionrendition_type(tctx);
    let assignment_15 = actionrendition_type(tctx);
    let assignment_35 = actionrendition_type(tctx);
    let assignment_94 = actionsetocgstate_type(tctx);
    let assignment_134 = actionsetocgstate_type(tctx);
    let assignment_74 = actionsetocgstate_type(tctx);
    let assignment_14 = actionsetocgstate_type(tctx);
    let assignment_54 = actionsetocgstate_type(tctx);
    let assignment_154 = actionsetocgstate_type(tctx);
    let assignment_114 = actionsetocgstate_type(tctx);
    let assignment_34 = actionsetocgstate_type(tctx);
    let assignment_153 = actionimportdata_type(tctx);
    let assignment_33 = actionimportdata_type(tctx);
    let assignment_13 = actionimportdata_type(tctx);
    let assignment_53 = actionimportdata_type(tctx);
    let assignment_93 = actionimportdata_type(tctx);
    let assignment_113 = actionimportdata_type(tctx);
    let assignment_73 = actionimportdata_type(tctx);
    let assignment_133 = actionimportdata_type(tctx);
    let assignment_152 = actionresetform_type(tctx);
    let assignment_92 = actionresetform_type(tctx);
    let assignment_12 = actionresetform_type(tctx);
    let assignment_52 = actionresetform_type(tctx);
    let assignment_112 = actionresetform_type(tctx);
    let assignment_72 = actionresetform_type(tctx);
    let assignment_32 = actionresetform_type(tctx);
    let assignment_132 = actionresetform_type(tctx);
    let assignment_31 = actionsubmitform_type(tctx);
    let assignment_111 = actionsubmitform_type(tctx);
    let assignment_51 = actionsubmitform_type(tctx);
    let assignment_91 = actionsubmitform_type(tctx);
    let assignment_131 = actionsubmitform_type(tctx);
    let assignment_11 = actionsubmitform_type(tctx);
    let assignment_71 = actionsubmitform_type(tctx);
    let assignment_151 = actionsubmitform_type(tctx);
    let assignment_130 = actionnamed_type(tctx);
    let assignment_10 = actionnamed_type(tctx);
    let assignment_150 = actionnamed_type(tctx);
    let assignment_90 = actionnamed_type(tctx);
    let assignment_110 = actionnamed_type(tctx);
    let assignment_50 = actionnamed_type(tctx);
    let assignment_70 = actionnamed_type(tctx);
    let assignment_30 = actionnamed_type(tctx);
    let assignment_129 = actionhide_type(tctx);
    let assignment_49 = actionhide_type(tctx);
    let assignment_149 = actionhide_type(tctx);
    let assignment_69 = actionhide_type(tctx);
    let assignment_9 = actionhide_type(tctx);
    let assignment_109 = actionhide_type(tctx);
    let assignment_89 = actionhide_type(tctx);
    let assignment_29 = actionhide_type(tctx);
    let assignment_8 = actionmovie_type(tctx);
    let assignment_48 = actionmovie_type(tctx);
    let assignment_28 = actionmovie_type(tctx);
    let assignment_68 = actionmovie_type(tctx);
    let assignment_108 = actionmovie_type(tctx);
    let assignment_148 = actionmovie_type(tctx);
    let assignment_88 = actionmovie_type(tctx);
    let assignment_128 = actionmovie_type(tctx);
    let assignment_107 = actionsound_type(tctx);
    let assignment_47 = actionsound_type(tctx);
    let assignment_67 = actionsound_type(tctx);
    let assignment_87 = actionsound_type(tctx);
    let assignment_7 = actionsound_type(tctx);
    let assignment_27 = actionsound_type(tctx);
    let assignment_127 = actionsound_type(tctx);
    let assignment_147 = actionsound_type(tctx);
    let assignment_46 = actionuri_type(tctx);
    let assignment_146 = actionuri_type(tctx);
    let assignment_86 = actionuri_type(tctx);
    let assignment_126 = actionuri_type(tctx);
    let assignment_26 = actionuri_type(tctx);
    let assignment_106 = actionuri_type(tctx);
    let assignment_66 = actionuri_type(tctx);
    let assignment_6 = actionuri_type(tctx);
    let assignment_65 = actionthread_type(tctx);
    let assignment_25 = actionthread_type(tctx);
    let assignment_5 = actionthread_type(tctx);
    let assignment_125 = actionthread_type(tctx);
    let assignment_145 = actionthread_type(tctx);
    let assignment_45 = actionthread_type(tctx);
    let assignment_105 = actionthread_type(tctx);
    let assignment_85 = actionthread_type(tctx);
    let assignment_84 = actionlaunch_type(tctx);
    let assignment_24 = actionlaunch_type(tctx);
    let assignment_144 = actionlaunch_type(tctx);
    let assignment_104 = actionlaunch_type(tctx);
    let assignment_44 = actionlaunch_type(tctx);
    let assignment_64 = actionlaunch_type(tctx);
    let assignment_124 = actionlaunch_type(tctx);
    let assignment_4 = actionlaunch_type(tctx);
    let assignment_63 = actiongotodp_type(tctx);
    let assignment_143 = actiongotodp_type(tctx);
    let assignment_123 = actiongotodp_type(tctx);
    let assignment_83 = actiongotodp_type(tctx);
    let assignment_23 = actiongotodp_type(tctx);
    let assignment_43 = actiongotodp_type(tctx);
    let assignment_103 = actiongotodp_type(tctx);
    let assignment_3 = actiongotodp_type(tctx);
    let assignment_142 = actiongotoe_type(tctx);
    let assignment_102 = actiongotoe_type(tctx);
    let assignment_122 = actiongotoe_type(tctx);
    let assignment_62 = actiongotoe_type(tctx);
    let assignment_42 = actiongotoe_type(tctx);
    let assignment_22 = actiongotoe_type(tctx);
    let assignment_82 = actiongotoe_type(tctx);
    let assignment_2 = actiongotoe_type(tctx);
    let assignment_21 = actiongotor_type(tctx);
    let assignment_121 = actiongotor_type(tctx);
    let assignment_81 = actiongotor_type(tctx);
    let assignment_41 = actiongotor_type(tctx);
    let assignment_61 = actiongotor_type(tctx);
    let assignment_1 = actiongotor_type(tctx);
    let assignment_141 = actiongotor_type(tctx);
    let assignment_101 = actiongotor_type(tctx);
    let assignment_20 = actiongoto_type(tctx);
    let assignment_120 = actiongoto_type(tctx);
    let assignment_0 = actiongoto_type(tctx);
    let assignment_60 = actiongoto_type(tctx);
    let assignment_40 = actiongoto_type(tctx);
    let assignment_140 = actiongoto_type(tctx);
    let assignment_80 = actiongoto_type(tctx);
    let assignment_100 = actiongoto_type(tctx);
    let dis_7 = TypeCheck::new(
        tctx,
        "pi",
        Rc::new(PDFType::Disjunct(vec![
            assignment_140,
            assignment_141,
            assignment_142,
            assignment_143,
            assignment_144,
            assignment_145,
            assignment_146,
            assignment_147,
            assignment_148,
            assignment_149,
            assignment_150,
            assignment_151,
            assignment_152,
            assignment_153,
            assignment_154,
            assignment_155,
            assignment_156,
            assignment_157,
            assignment_158,
            assignment_159,
        ])),
    );
    let dis_6 = TypeCheck::new(
        tctx,
        "pv",
        Rc::new(PDFType::Disjunct(vec![
            assignment_120,
            assignment_121,
            assignment_122,
            assignment_123,
            assignment_124,
            assignment_125,
            assignment_126,
            assignment_127,
            assignment_128,
            assignment_129,
            assignment_130,
            assignment_131,
            assignment_132,
            assignment_133,
            assignment_134,
            assignment_135,
            assignment_136,
            assignment_137,
            assignment_138,
            assignment_139,
        ])),
    );
    let dis_5 = TypeCheck::new(
        tctx,
        "pc",
        Rc::new(PDFType::Disjunct(vec![
            assignment_100,
            assignment_101,
            assignment_102,
            assignment_103,
            assignment_104,
            assignment_105,
            assignment_106,
            assignment_107,
            assignment_108,
            assignment_109,
            assignment_110,
            assignment_111,
            assignment_112,
            assignment_113,
            assignment_114,
            assignment_115,
            assignment_116,
            assignment_117,
            assignment_118,
            assignment_119,
        ])),
    );
    let dis_4 = TypeCheck::new(
        tctx,
        "po",
        Rc::new(PDFType::Disjunct(vec![
            assignment_80,
            assignment_81,
            assignment_82,
            assignment_83,
            assignment_84,
            assignment_85,
            assignment_86,
            assignment_87,
            assignment_88,
            assignment_89,
            assignment_90,
            assignment_91,
            assignment_92,
            assignment_93,
            assignment_94,
            assignment_95,
            assignment_96,
            assignment_97,
            assignment_98,
            assignment_99,
        ])),
    );
    let dis_3 = TypeCheck::new(
        tctx,
        "u",
        Rc::new(PDFType::Disjunct(vec![
            assignment_60,
            assignment_61,
            assignment_62,
            assignment_63,
            assignment_64,
            assignment_65,
            assignment_66,
            assignment_67,
            assignment_68,
            assignment_69,
            assignment_70,
            assignment_71,
            assignment_72,
            assignment_73,
            assignment_74,
            assignment_75,
            assignment_76,
            assignment_77,
            assignment_78,
            assignment_79,
        ])),
    );
    let dis_2 = TypeCheck::new(
        tctx,
        "d",
        Rc::new(PDFType::Disjunct(vec![
            assignment_40,
            assignment_41,
            assignment_42,
            assignment_43,
            assignment_44,
            assignment_45,
            assignment_46,
            assignment_47,
            assignment_48,
            assignment_49,
            assignment_50,
            assignment_51,
            assignment_52,
            assignment_53,
            assignment_54,
            assignment_55,
            assignment_56,
            assignment_57,
            assignment_58,
            assignment_59,
        ])),
    );
    let dis_1 = TypeCheck::new(
        tctx,
        "x",
        Rc::new(PDFType::Disjunct(vec![
            assignment_20,
            assignment_21,
            assignment_22,
            assignment_23,
            assignment_24,
            assignment_25,
            assignment_26,
            assignment_27,
            assignment_28,
            assignment_29,
            assignment_30,
            assignment_31,
            assignment_32,
            assignment_33,
            assignment_34,
            assignment_35,
            assignment_36,
            assignment_37,
            assignment_38,
            assignment_39,
        ])),
    );
    let dis_0 = TypeCheck::new(
        tctx,
        "e",
        Rc::new(PDFType::Disjunct(vec![
            assignment_0,
            assignment_1,
            assignment_2,
            assignment_3,
            assignment_4,
            assignment_5,
            assignment_6,
            assignment_7,
            assignment_8,
            assignment_9,
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
        ])),
    );
    let e_field = DictEntry {
        key: Vec::from("E"),
        chk: dis_0,
        opt: DictKeySpec::Optional,
    };
    let x_field = DictEntry {
        key: Vec::from("X"),
        chk: dis_1,
        opt: DictKeySpec::Optional,
    };
    let d_field = DictEntry {
        key: Vec::from("D"),
        chk: dis_2,
        opt: DictKeySpec::Optional,
    };
    let u_field = DictEntry {
        key: Vec::from("U"),
        chk: dis_3,
        opt: DictKeySpec::Optional,
    };
    let po_field = DictEntry {
        key: Vec::from("PO"),
        chk: dis_4,
        opt: DictKeySpec::Optional,
    };
    let pc_field = DictEntry {
        key: Vec::from("PC"),
        chk: dis_5,
        opt: DictKeySpec::Optional,
    };
    let pv_field = DictEntry {
        key: Vec::from("PV"),
        chk: dis_6,
        opt: DictKeySpec::Optional,
    };
    let pi_field = DictEntry {
        key: Vec::from("PI"),
        chk: dis_7,
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "addactionscreenannotation",
        Rc::new(PDFType::Dict(vec![
            e_field, x_field, d_field, u_field, po_field, pc_field, pv_field, pi_field,
        ])),
    )
}
