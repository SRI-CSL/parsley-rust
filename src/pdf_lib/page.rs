use super::super::pcore::parsebuffer::{LocatedVal};
use super::pdf_obj::{PDFObjContext, PDFObjT};
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckError, Predicate, ChoicePred, mk_date_typchk
};
use crate::pdf_lib::common_data_structures::structures::{mk_reference_typchk, mk_rectangle_typchk, mk_name_check};
//use crate::pdf_lib::number_tree::{}
use std::rc::Rc;


fn page_type() -> TypeCheck {
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check("Page".to_string()),
        opt: DictKeySpec::Required,
    };
    let parent =  DictEntry {
        key: Vec::from("Parent"),
        chk: mk_reference_typchk(),
        opt: DictKeySpec::Required,
    };
    let lastmodified =  DictEntry {
        key: Vec::from("LastModified"),
        chk: mk_date_typchk,
        opt: DictKeySpec::Optional,
    };
    let resources =  DictEntry {
        key: Vec::from("Resources"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let mediabox =  DictEntry {
        key: Vec::from("MediaBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let cropbox =  DictEntry {
        key: Vec::from("CropBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let bleedbox =  DictEntry {
        key: Vec::from("BleedBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let trimbox =  DictEntry {
        key: Vec::from("TrimBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let artbox =  DictEntry {
        key: Vec::from("ArtBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let boxcolorinfo =  DictEntry {
        key: Vec::from("BoxColorInfo"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let contents =  DictEntry {
        key: Vec::from("Contents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let rotate =  DictEntry {
        key: Vec::from("Rotate"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Integer)))),
        opt: DictKeySpec::Optional,
    };
    let group =  DictEntry {
        key: Vec::from("Group"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let thumb =  DictEntry {
        key: Vec::from("Thumb"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Stream(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let b =  DictEntry {
        key: Vec::from("B"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let dur =  DictEntry {
        key: Vec::from("Dur"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Integer)))),
        opt: DictKeySpec::Optional,
    };
    let trans =  DictEntry {
        key: Vec::from("Trans"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let annots =  DictEntry {
        key: Vec::from("Annots"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let aa =  DictEntry {
        key: Vec::from("AA"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let metadata =  DictEntry {
        key: Vec::from("Metadata"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Stream(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let pieceinfo =  DictEntry {
        key: Vec::from("PieceInfo"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let structparents =  DictEntry {
        key: Vec::from("StructParents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Integer)))),
        opt: DictKeySpec::Optional,
    };
    let id =  DictEntry {
        key: Vec::from("ID"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let pz =  DictEntry {
        key: Vec::from("PZ"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Integer)))),
        opt: DictKeySpec::Optional,
    };
    let separationinfo =  DictEntry {
        key: Vec::from("SeparationInfo"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let tabs =  DictEntry {
        key: Vec::from("Tabs"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let templateinstantiated =  DictEntry {
        key: Vec::from("Tabs"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let pressteps =  DictEntry {
        key: Vec::from("PresSteps"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let userunit =  DictEntry {
        key: Vec::from("UserUnit"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Integer)))),
        opt: DictKeySpec::Optional,
    };
    let vp =  DictEntry {
        key: Vec::from("VP"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let af =  DictEntry {
        key: Vec::from("AF"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let outputintents =  DictEntry {
        key: Vec::from("OutputIntents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let dpart =  DictEntry {
        key: Vec::from("DpPart"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(Rc::new(PDFType::Dict(vec![typ, parent, lastmodified, resources, mediabox, cropbox, 
                                        bleedbox, trimbox, artbox, boxcolorinfo, contents, rotate, group, thumb, 
                                        b, dur, trans, annots, aa, metadata, pieceinfo, structparents, id, pz, separationinfo, 
                                        tabs, templateinstantiated, pressteps, userunit, vp, af, outputintents, dpart])))

}
