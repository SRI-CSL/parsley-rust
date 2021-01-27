use crate::pdf_lib::common_data_structures::{
    mk_array_of_dict_typchk, mk_date_typchk, mk_generic_array_typchk, mk_generic_dict_typchk,
    mk_generic_stream_typchk, mk_name_check, mk_number_typchk, mk_parent_typchk,
    mk_rectangle_typchk, resources,
};
use crate::pdf_lib::pdf_obj::PDFObjT;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckContext,
};
use std::rc::Rc;

fn mk_tabs_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Invalid PageLayout"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("R"))),
            PDFObjT::Name(NameT::new(Vec::from("C"))),
            PDFObjT::Name(NameT::new(Vec::from("S"))),
            PDFObjT::Name(NameT::new(Vec::from("A"))),
            PDFObjT::Name(NameT::new(Vec::from("W"))),
        ],
    );
    TypeCheck::new_refined(
        tctx,
        "",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
        Rc::new(pred),
    )
}

fn mk_contents_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    // This is either a stream or an array of streams.
    let stm = TypeCheck::new(tctx, "", Rc::new(PDFType::Stream(vec![])));
    let arr = TypeCheck::new(
        tctx,
        "",
        Rc::new(PDFType::Array {
            elem: Rc::clone(&stm),
            size: None,
        }),
    );
    TypeCheck::new(tctx, "", Rc::new(PDFType::Disjunct(vec![stm, arr])))
}

fn mk_generic_page_entries(tctx: &mut TypeCheckContext) -> Vec<DictEntry> {
    let lastmodified = DictEntry {
        key: Vec::from("LastModified"),
        chk: mk_date_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let resources = DictEntry {
        key: Vec::from("Resources"),
        chk: resources(tctx),
        opt: DictKeySpec::Optional,
    };
    let mediabox = DictEntry {
        key: Vec::from("MediaBox"),
        chk: mk_rectangle_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let cropbox = DictEntry {
        key: Vec::from("CropBox"),
        chk: mk_rectangle_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let bleedbox = DictEntry {
        key: Vec::from("BleedBox"),
        chk: mk_rectangle_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let trimbox = DictEntry {
        key: Vec::from("TrimBox"),
        chk: mk_rectangle_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let artbox = DictEntry {
        key: Vec::from("ArtBox"),
        chk: mk_rectangle_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let boxcolorinfo = DictEntry {
        key: Vec::from("BoxColorInfo"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let contents = DictEntry {
        key: Vec::from("Contents"),
        chk: mk_contents_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let rotate = DictEntry {
        key: Vec::from("Rotate"),
        chk: TypeCheck::new(
            tctx,
            "rotate",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        ),
        opt: DictKeySpec::Optional,
    };
    let group = DictEntry {
        key: Vec::from("Group"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let thumb = DictEntry {
        key: Vec::from("Thumb"),
        chk: mk_generic_stream_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let dur = DictEntry {
        key: Vec::from("Dur"),
        chk: mk_number_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let trans = DictEntry {
        key: Vec::from("Trans"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let annots = DictEntry {
        key: Vec::from("Annots"),
        chk: mk_generic_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let aa = DictEntry {
        key: Vec::from("AA"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let metadata = DictEntry {
        key: Vec::from("Metadata"),
        chk: mk_generic_stream_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let pieceinfo = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let structparents = DictEntry {
        key: Vec::from("StructParents"),
        chk: TypeCheck::new(
            tctx,
            "structparents",
            Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
        ),
        opt: DictKeySpec::Optional,
    };
    let id = DictEntry {
        key: Vec::from("ID"),
        // This needs to be a byte stream, indirect reference preferred.
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let pz = DictEntry {
        key: Vec::from("PZ"),
        chk: mk_number_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let separationinfo = DictEntry {
        key: Vec::from("SeparationInfo"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let tabs = DictEntry {
        key: Vec::from("Tabs"),
        chk: mk_tabs_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let templateinstantiated = DictEntry {
        key: Vec::from("TemplateInstantiated"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::Name))),
        opt: DictKeySpec::Optional,
    };
    let pressteps = DictEntry {
        key: Vec::from("PresSteps"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let userunit = DictEntry {
        key: Vec::from("UserUnit"),
        chk: mk_number_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let vp = DictEntry {
        key: Vec::from("VP"),
        chk: mk_generic_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let af = DictEntry {
        key: Vec::from("AF"),
        chk: mk_array_of_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let outputintents = DictEntry {
        key: Vec::from("OutputIntents"),
        chk: mk_generic_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let dpart = DictEntry {
        key: Vec::from("DPart"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    vec![
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
    ]
}

pub fn template_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check("Template", "Not a Template", tctx),
        opt: DictKeySpec::Required,
    };
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: mk_parent_typchk(tctx),
        opt: DictKeySpec::Forbidden,
    };

    let mut ents = Vec::new();
    ents.push(typ);
    ents.push(parent);

    let mut generic_ents = mk_generic_page_entries(tctx);
    ents.append(&mut generic_ents);

    TypeCheck::new(tctx, "template", Rc::new(PDFType::Dict(ents)))
}
pub fn page_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check("Page", "Not a Page", tctx),
        opt: DictKeySpec::Required,
    };
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: mk_parent_typchk(tctx),
        opt: DictKeySpec::Required,
    };
    let b = DictEntry {
        key: Vec::from("B"),
        chk: mk_generic_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };

    let mut ents = Vec::new();
    ents.push(typ);
    ents.push(parent);
    ents.push(b);

    let mut generic_ents = mk_generic_page_entries(tctx);
    ents.append(&mut generic_ents);

    TypeCheck::new(tctx, "page", Rc::new(PDFType::Dict(ents)))
}
#[cfg(test)]

mod test_page {
    use super::page_type;
    use crate::pcore::parsebuffer::ParseBuffer;
    use crate::pdf_lib::pdf_obj::{parse_pdf_indirect_obj, parse_pdf_obj, Marker, PDFObjContext};
    use crate::pdf_lib::pdf_type_check::{check_type, TypeCheckContext};
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn test_page() {
        let mut ctxt = mk_new_context();
        // populate context
        let v = Vec::from("4 0 obj << >> endobj");
        let mut pb = ParseBuffer::new(v);
        let _parent = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        let v = Vec::from(
            "12 0 obj << >> stream
         endstream
         endobj"
                .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let _content = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        // parse page object
        let v = Vec::from("<</Type /Page /Parent 4 0 R /MediaBox [0 0 612 792] /Resources  <</Font <</F3 7 0 R /F5 9 0 R /F7 11 0 R >> >>  /Contents 12 0 R /Annots [23 0 R 24 0 R ]>> ".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb, Marker::Obj).unwrap();

        // check
        let mut tctx = TypeCheckContext::new();
        let typ = page_type(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
    #[test]
    fn test_another_page() {
        let mut ctxt = mk_new_context();
        // populate context

        let pt_node = Vec::from("1 0 obj <<
        /Type /Pages
        /Count 118
        /Kids [ 3 0 R 4 0 R 5 0 R 6 0 R 7 0 R 8 0 R 9 0 R 10 0 R 11 0 R 12 0 R 13 0 R 14 0 R 15 0 R 16 0 R 17 0 R 18 0 R 19 0 R 20 0 R 21 0 R 22 0 R 23 0 R 24 0 R 25 0 R 26 0 R 27 0 R 28 0 R 29 0 R 30 0 R 31 0 R 32 0 R 33 0 R 34 0 R 35 0 R 36 0 R 37 0 R 38 0 R 39 0 R 40 0 R 41 0 R 42 0 R 43 0 R 44 0 R 45 0 R 46 0 R 47 0 R 48 0 R 49 0 R 50 0 R 51 0 R 52 0 R 53 0 R 54 0 R 55 0 R 56 0 R 57 0 R 58 0 R 59 0 R 60 0 R 61 0 R 62 0 R 63 0 R 64 0 R 65 0 R 66 0 R 67 0 R 68 0 R 69 0 R 70 0 R 71 0 R 72 0 R 73 0 R 74 0 R 75 0 R 76 0 R 77 0 R 78 0 R 79 0 R 80 0 R 81 0 R 82 0 R 83 0 R 84 0 R 85 0 R 86 0 R 87 0 R 88 0 R 89 0 R 90 0 R 91 0 R 92 0 R 93 0 R 94 0 R 95 0 R 96 0 R 97 0 R 98 0 R 99 0 R 100 0 R 101 0 R 102 0 R 103 0 R 104 0 R 105 0 R 106 0 R 107 0 R 108 0 R 109 0 R 110 0 R 111 0 R 112 0 R 113 0 R 114 0 R 115 0 R 116 0 R 117 0 R 118 0 R 119 0 R 120 0 R ]
        >> endobj
        ".as_bytes());
        let mut pt_node = ParseBuffer::new(pt_node);
        let _pt_node = parse_pdf_indirect_obj(&mut ctxt, &mut pt_node).unwrap();

        let page1 = Vec::from(
            "39 0 obj
        <<
        /CropBox [ 0 0 792 612 ]
        /Annots [741 0 R]
        /Parent 1 0 R
        /StructParents 127
        /Contents 750 0 R
        /Rotate 0
        /BleedBox [ 0 0 792 612 ]
        /ArtBox [ 0 0 792 612 ]
        /Group 759 0 R
        /MediaBox [ 0 0 792 612 ]
        /TrimBox [ 0 0 792 612 ]
        /Resources <<
        /XObject <<
        /Im0 170 0 R
        /Fm0 462 0 R
        >>
        /Shading <<
        /Sh0 176 0 R
        >>
        /ColorSpace <<
        /CS0 172 0 R
        /CS1 172 0 R
        >>
        /Font <<
        /TT0 184 0 R
        /TT1 180 0 R
        /TT2 188 0 R
        /TT3 196 0 R
        /C2_0 200 0 R
        /C2_1 207 0 R
        /C2_2 258 0 R
        >>
        /ProcSet [ /PDF /Text /ImageC ]
        /ExtGState <<
        /GS0 221 0 R
        /GS1 222 0 R
        /GS2 387 0 R
        /GS3 464 0 R
        >>
        >>
        /Type /Page
        >> endobj
        "
            .as_bytes(),
        );
        let mut page1 = ParseBuffer::new(page1);
        let _page1 = parse_pdf_indirect_obj(&mut ctxt, &mut page1).unwrap();

        let v = Vec::from(
            "750 0 obj << >> stream
         endstream
         endobj"
                .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let _content1 = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        let v = Vec::from(
            "48 0 obj << >> stream
         endstream
         endobj"
                .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let _content2 = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        let page2 = Vec::from(
            "<<
        /CropBox [ 0 0 792 612 ]
        /Annots [39 0 R]
        /Parent 1 0 R
        /StructParents 68
        /Contents 48 0 R
        /Rotate 0
        /BleedBox [ 0 0 792 612 ]
        /ArtBox [ 0 0 792 612 ]
        /MediaBox [ 0 0 792 612 ]
        /TrimBox [ 0 0 792 612 ]
        /Resources <<
        /XObject <<
        /Im0 170 0 R
        >>
        /Shading <<
        /Sh0 176 0 R
        >>
        /ColorSpace <<
        /CS0 172 0 R
        /CS1 172 0 R
        >>
        /Font <<
        /TT0 180 0 R
        /TT1 184 0 R
        /TT2 188 0 R
        /TT3 1884 0 R
        /TT4 192 0 R
        /C2_0 207 0 R
        /C2_1 2021 0 R
        >>
        /ProcSet [ /PDF /Text /ImageC ]
        /ExtGState <<
        /GS0 221 0 R
        /GS1 222 0 R
        >>
        >>
        /Type /Page
        >>
        "
            .as_bytes(),
        );
        let mut page2 = ParseBuffer::new(page2);
        let obj = parse_pdf_obj(&mut ctxt, &mut page2, Marker::Obj).unwrap();

        let mut tctx = TypeCheckContext::new();
        let typ = page_type(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
}
