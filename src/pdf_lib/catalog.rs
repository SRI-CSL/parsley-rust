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
    mk_generic_indirect_dict_typchk, mk_generic_indirect_stream_typchk, mk_name_check,
    name_dictionary,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::page_tree::root_page_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckContext,
};
use std::rc::Rc;

fn mk_pagemode_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Invalid PageMode"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("UseNone"))),
            PDFObjT::Name(NameT::new(Vec::from("UseOutlines"))),
            PDFObjT::Name(NameT::new(Vec::from("UseThumbs"))),
            PDFObjT::Name(NameT::new(Vec::from("FullScreen"))),
            PDFObjT::Name(NameT::new(Vec::from("UseOC"))),
            PDFObjT::Name(NameT::new(Vec::from("UseAttachments"))),
        ],
    );
    TypeCheck::new_refined(
        tctx,
        "",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
        Rc::new(pred),
    )
}
fn mk_pagelayout_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Invalid PageLayout"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("SinglePage"))),
            PDFObjT::Name(NameT::new(Vec::from("OneColumn"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoColumnLeft"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoColumnRight"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoPageLeft"))),
            PDFObjT::Name(NameT::new(Vec::from("TwoPageRight"))),
        ],
    );
    TypeCheck::new_refined(
        tctx,
        "",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
        Rc::new(pred),
    )
}

fn mk_trapped_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Invalid Trapped value in info"),
        vec![
            PDFObjT::Name(NameT::new(Vec::from("True"))),
            PDFObjT::Name(NameT::new(Vec::from("False"))),
            PDFObjT::Name(NameT::new(Vec::from("Unknown"))),
        ],
    );
    TypeCheck::new_refined(
        tctx,
        "",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
        Rc::new(pred),
    )
}

pub fn info_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let title = DictEntry {
        key: Vec::from("Title"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let author = DictEntry {
        key: Vec::from("Author"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let subject = DictEntry {
        key: Vec::from("Subject"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let keywords = DictEntry {
        key: Vec::from("Keywords"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let creator = DictEntry {
        key: Vec::from("Creator"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let producer = DictEntry {
        key: Vec::from("Producer"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let creationdate = DictEntry {
        key: Vec::from("CreationDate"),
        chk: mk_date_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let moddate = DictEntry {
        key: Vec::from("ModDate"),
        chk: mk_date_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let trapped = DictEntry {
        key: Vec::from("Trapped"),
        chk: mk_trapped_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "template",
        Rc::new(PDFType::Dict(
            vec![
                title,
                author,
                subject,
                keywords,
                creator,
                producer,
                creationdate,
                moddate,
                trapped,
            ],
            None,
        )),
    )
}

// Errata: extensions, af, dpartroot, dss

pub fn catalog_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    // Row 1
    //TypeCheck::new(Rc::new(PDFType::Dict(vec![typ, version, extensions, pages,
    // pagelabels, names, dests, viewerpreferences, pagelayout,
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check("Catalog", "Not a Catalog", tctx),
        opt: DictKeySpec::Required,
    };
    let version = DictEntry {
        key: Vec::from("Version"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::Name))), /* TODO: Maybe make a whitelist of version numbers? */
        opt: DictKeySpec::Optional,
    };
    let extensions = DictEntry {
        key: Vec::from("Extensions"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let pages = DictEntry {
        key: Vec::from("Pages"),
        chk: root_page_tree(tctx),
        opt: DictKeySpec::Required,
    };
    let pagelabels = DictEntry {
        key: Vec::from("PageLabels"),
        chk: number_tree(tctx),
        opt: DictKeySpec::Optional,
    };
    let names = DictEntry {
        key: Vec::from("Names"),
        chk: name_dictionary(tctx),
        opt: DictKeySpec::Optional,
    };
    let dests_option_1 = mk_generic_dict_typchk(tctx);
    let dests_option_2 = mk_generic_indirect_dict_typchk(tctx);
    let dests = DictEntry {
        key: Vec::from("Dests"),
        chk: TypeCheck::new(
            tctx,
            "",
            Rc::new(PDFType::Disjunct(vec![dests_option_1, dests_option_2])),
        ),
        opt: DictKeySpec::Optional,
    };
    let viewerpreferences = DictEntry {
        key: Vec::from("ViewerPreferences"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let pagelayout = DictEntry {
        key: Vec::from("PageLayout"),
        chk: mk_pagelayout_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    //Row 2
    //pagemode, outlines, threads, openaction, aa, uri, acroform,
    let pagemode = DictEntry {
        key: Vec::from("PageMode"),
        chk: mk_pagemode_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let outlines = DictEntry {
        key: Vec::from("Outlines"),
        chk: mk_generic_indirect_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let threads = DictEntry {
        key: Vec::from("Threads"),
        chk: mk_generic_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let garray = mk_generic_array_typchk(tctx);
    let gdict = mk_generic_dict_typchk(tctx);
    let openaction = DictEntry {
        key: Vec::from("OpenAction"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::Disjunct(vec![garray, gdict]))),
        opt: DictKeySpec::Optional,
    };
    let aa = DictEntry {
        key: Vec::from("AA"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let uri = DictEntry {
        key: Vec::from("URI"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let acroform = DictEntry {
        key: Vec::from("AcroForm"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };

    //Row 3
    //metadata, structtreeroot, markinfo, lang, spiderinfo, outputintents,
    // pieceinfo, ocproperties,
    let metadata = DictEntry {
        key: Vec::from("Metadata"),
        chk: mk_generic_indirect_stream_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let structtreeroot = DictEntry {
        key: Vec::from("StructTreeRoot"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let markinfo = DictEntry {
        key: Vec::from("MarkInfo"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let lang = DictEntry {
        key: Vec::from("Lang"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::String))),
        opt: DictKeySpec::Optional,
    };
    let spiderinfo = DictEntry {
        key: Vec::from("SpiderInfo"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let outputintents = DictEntry {
        key: Vec::from("OutputIntents"),
        chk: mk_generic_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let pieceinfo = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let ocproperties = DictEntry {
        key: Vec::from("OCProperties"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional, // FIXME: "Required if the document contains optional content"
    };

    // Row 4
    //perms, legal, requirements, collection, needsrendering, dss, af,
    // dpartroot])))
    let perms = DictEntry {
        key: Vec::from("Perms"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let legal = DictEntry {
        key: Vec::from("Legal"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let requirements = DictEntry {
        key: Vec::from("Requirements"),
        chk: mk_generic_array_typchk(tctx), // FIXME: array of dicts
        opt: DictKeySpec::Optional,
    };
    let collection = DictEntry {
        key: Vec::from("Collection"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let needsrendering = DictEntry {
        key: Vec::from("NeedsRendering"),
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::Bool))),
        opt: DictKeySpec::Optional,
    };
    let dss = DictEntry {
        key: Vec::from("DSS"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let af = DictEntry {
        key: Vec::from("AF"),
        chk: mk_array_of_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let dpartroot = DictEntry {
        key: Vec::from("DPartRoot"),
        chk: mk_generic_dict_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(
        tctx,
        "catalog",
        Rc::new(PDFType::Dict(
            vec![
                typ,
                version,
                extensions,
                pages,
                pagelabels,
                names,
                dests,
                viewerpreferences,
                pagelayout,
                pagemode,
                outlines,
                threads,
                openaction,
                aa,
                uri,
                acroform,
                metadata,
                structtreeroot,
                markinfo,
                lang,
                spiderinfo,
                outputintents,
                pieceinfo,
                ocproperties,
                perms,
                legal,
                requirements,
                collection,
                needsrendering,
                dss,
                af,
                dpartroot,
            ],
            None,
        )),
    )
}
#[cfg(test)]
mod test_name_tree {
    use super::catalog_type;
    use crate::pcore::parsebuffer::ParseBuffer;
    use crate::pdf_lib::pdf_obj::{parse_pdf_indirect_obj, parse_pdf_obj, PDFObjContext};
    use crate::pdf_lib::pdf_type_check::{check_type, TypeCheckContext};
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn test_catalog() {
        // set up the context
        let mut ctxt = mk_new_context();

        let v = Vec::from("2 0 obj <</Type /Pages /Kids [4 0 R] /Count 1 >> endobj".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let _root = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        let v = Vec::from(
            "48 0 obj <</Length 0>> stream
\nendstream endobj"
                .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let _content = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        let v = Vec::from(
            "4 0 obj <<
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
        >>
        /ProcSet [ /PDF /Text /ImageC ]
        /ExtGState <<
        /GS0 221 0 R
        /GS1 222 0 R
        >>
        >>
        /Type /Page
        >> endobj
        "
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let _page = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        let v = Vec::from(
            "3 0 obj <</Title ( Chapter 1 )
        /Parent 21 0 R
        /Next 26 0 R
        /First 23 0 R
        /Last 25 0 R
        /Count 3
        /Dest [3 0 R /XYZ 0 792 0]
        >> endobj
        "
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let _outline = parse_pdf_indirect_obj(&mut ctxt, &mut pb).unwrap();

        // parse the test object
        let v = Vec::from(
            "<</Type /Catalog
  /Pages 2 0 R
  /PageMode /UseOutlines
  /Outlines 3 0 R
  >>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();

        // check
        let mut tctx = TypeCheckContext::new();
        let typ = catalog_type(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
}
