use crate::pdf_lib::common_data_structures::structures::{
    mk_generic_dict_typchk, mk_name_check, mk_rectangle_typchk,
};
use crate::pdf_lib::pdf_type_check::{
    mk_date_typchk, DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck,
};
//use crate::pdf_lib::number_tree::{}
use std::rc::Rc;

fn page_type() -> TypeCheck {
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check("Not a Page".to_string(), "Page".to_string()),
        opt: DictKeySpec::Required,
    };
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Required,
    };
    let lastmodified = DictEntry {
        key: Vec::from("LastModified"),
        chk: mk_date_typchk(),
        opt: DictKeySpec::Optional,
    };
    let resources = DictEntry {
        key: Vec::from("Resources"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let mediabox = DictEntry {
        key: Vec::from("MediaBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let cropbox = DictEntry {
        key: Vec::from("CropBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let bleedbox = DictEntry {
        key: Vec::from("BleedBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let trimbox = DictEntry {
        key: Vec::from("TrimBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let artbox = DictEntry {
        key: Vec::from("ArtBox"),
        chk: mk_rectangle_typchk(),
        opt: DictKeySpec::Optional,
    };
    let boxcolorinfo = DictEntry {
        key: Vec::from("BoxColorInfo"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let contents = DictEntry {
        key: Vec::from("Contents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let rotate = DictEntry {
        key: Vec::from("Rotate"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
            PDFPrimType::Integer,
        )))),
        opt: DictKeySpec::Optional,
    };
    let group = DictEntry {
        key: Vec::from("Group"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let thumb = DictEntry {
        key: Vec::from("Thumb"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Stream(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let b = DictEntry {
        key: Vec::from("B"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let dur = DictEntry {
        key: Vec::from("Dur"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
            PDFPrimType::Integer,
        )))),
        opt: DictKeySpec::Optional,
    };
    let trans = DictEntry {
        key: Vec::from("Trans"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let annots = DictEntry {
        key: Vec::from("Annots"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let aa = DictEntry {
        key: Vec::from("AA"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let metadata = DictEntry {
        key: Vec::from("Metadata"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Stream(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let pieceinfo = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let structparents = DictEntry {
        key: Vec::from("StructParents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
            PDFPrimType::Integer,
        )))),
        opt: DictKeySpec::Optional,
    };
    let id = DictEntry {
        key: Vec::from("ID"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let pz = DictEntry {
        key: Vec::from("PZ"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
            PDFPrimType::Integer,
        )))),
        opt: DictKeySpec::Optional,
    };
    let separationinfo = DictEntry {
        key: Vec::from("SeparationInfo"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let tabs = DictEntry {
        key: Vec::from("Tabs"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let templateinstantiated = DictEntry {
        key: Vec::from("TemplateInstantiated"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let pressteps = DictEntry {
        key: Vec::from("PresSteps"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    let userunit = DictEntry {
        key: Vec::from("UserUnit"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
            PDFPrimType::Integer,
        )))),
        opt: DictKeySpec::Optional,
    };
    let vp = DictEntry {
        key: Vec::from("VP"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let af = DictEntry {
        key: Vec::from("AF"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let outputintents = DictEntry {
        key: Vec::from("OutputIntents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Any))),
        opt: DictKeySpec::Optional,
    };
    let dpart = DictEntry {
        key: Vec::from("DpPart"),
        chk: mk_generic_dict_typchk(),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(Rc::new(PDFType::Dict(vec![
        typ,
        parent,
        lastmodified,
        resources,
        mediabox,
        cropbox,
        bleedbox,
        trimbox,
        artbox,
        boxcolorinfo,
        contents,
        rotate,
        group,
        thumb,
        b,
        dur,
        trans,
        annots,
        aa,
        metadata,
        pieceinfo,
        structparents,
        id,
        pz,
        separationinfo,
        tabs,
        templateinstantiated,
        pressteps,
        userunit,
        vp,
        af,
        outputintents,
        dpart,
    ])))
}
#[cfg(test)]

mod test_page {
    use super::super::super::pcore::parsebuffer::{LocatedVal, ParseBuffer};
    use super::super::pdf_obj::{parse_pdf_obj, IndirectT, PDFObjContext, PDFObjT};
    use super::super::pdf_prim::IntegerT;

    use super::super::pdf_type_check::check_type;
    use super::page_type;
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn test_page() {
        let mut ctxt = mk_new_context();
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3
        // >>".as_bytes());
        let v = Vec::from("<</Type /Page /Parent 4 0 R /MediaBox [0 0 612 792] /Resources  <</Font <</F3 7 0 R /F5 9 0 R /F7 11 0 R >>   >>          /Contents 12 0 R /Annots [23 0 R 24 0 R ]       >> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let i = IndirectT::new(
            4,
            0,
            Rc::new(LocatedVal::new(PDFObjT::Integer(IntegerT::new(5)), 0, 1)),
        );
        let l = LocatedVal::new(i, 0, 4);
        ctxt.register_obj(&l);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        let typ = page_type();
        assert_eq!(check_type(&ctxt, Rc::new(obj), Rc::new(typ)), None);
    }
}
