use super::pdf_obj::PDFObjT;
use crate::pdf_lib::common_data_structures::structures::name_dictionary;
use crate::pdf_lib::common_data_structures::structures::{
    mk_generic_array_typchk, mk_generic_dict_typchk, mk_generic_indirect_array_typchk,
    mk_generic_indirect_dict_typchk, mk_name_check,
};
use crate::pdf_lib::number_tree::number_tree;
use crate::pdf_lib::page_tree::root_page_tree;
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckContext,
};
use std::rc::Rc;

fn mk_af_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let elem = mk_generic_dict_typchk(tctx);
    TypeCheck::new(
        tctx,
        "",
        Rc::new(PDFType::Array {
            elem: elem,
            size: None,
        }),
    )
}
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
// Errata: extensions, af, dpartroot, dss

pub fn catalog_type(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    // Row 1
    //TypeCheck::new(Rc::new(PDFType::Dict(vec![typ, version, extensions, pages,
    // pagelabels, names, dests, viewerpreferences, pagelayout,
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check(
            "Not a Catalog".to_string(),
            "Catalog".to_string(),
            tctx,
        ),
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
    let dests = DictEntry {
        key: Vec::from("Dests"),
        chk: mk_generic_indirect_dict_typchk(tctx), // FIXME: indirect dict of names
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
        chk: mk_generic_indirect_array_typchk(tctx),
        opt: DictKeySpec::Optional,
    };
    let garray = mk_generic_array_typchk(tctx);
    let gdict = mk_generic_dict_typchk(tctx);
    let openaction = DictEntry {
        key: Vec::from("OpenAction"),
        chk: TypeCheck::new(
            tctx,
            "",
            Rc::new(PDFType::Disjunct(vec![
                garray,
                gdict,
            ])),
        ),
        opt: DictKeySpec::Optional,
    };
    let aa = DictEntry {
        key: Vec::from("AA"),
        chk: mk_generic_array_typchk(tctx),
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
        chk: TypeCheck::new(tctx, "", Rc::new(PDFType::PrimType(PDFPrimType::Bool))), /* FIXME: this needs to be a generic indirect stream */
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
        chk: TypeCheck::new(
            tctx,
            "",
            Rc::new(PDFType::PrimType(PDFPrimType::String)),
        ),
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
        chk: mk_generic_array_typchk(tctx),
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
        chk: mk_af_typchk(tctx),
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
        Rc::new(PDFType::Dict(vec![
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
        ])),
    )
}
#[cfg(test)]
mod test_name_tree {
    use super::super::super::pcore::parsebuffer::{LocatedVal, ParseBuffer};
    use super::super::pdf_obj::{parse_pdf_obj, DictT, IndirectT, PDFObjContext, PDFObjT};
    use super::super::pdf_prim::IntegerT;
    use super::super::pdf_type_check::{check_type, TypeCheckContext};
    use super::catalog_type;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    #[test]
    fn test_catalog() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();
        let i1 = IndirectT::new(
            2,
            0,
            Rc::new(LocatedVal::new(PDFObjT::Integer(IntegerT::new(5)), 0, 1)),
        );
        let l1 = LocatedVal::new(i1, 0, 4);

        let i2 = IndirectT::new(
            3,
            0,
            Rc::new(LocatedVal::new(
                PDFObjT::Dict(DictT::new(BTreeMap::new())),
                0,
                1,
            )),
        );
        let l2 = LocatedVal::new(i2, 0, 4);
        ctxt.register_obj(&l1);
        ctxt.register_obj(&l2);
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
        let typ = catalog_type(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }

    #[test]
    fn test_catalog_2() {
        let mut ctxt = mk_new_context();
        let mut tctx = TypeCheckContext::new();
        let i1 = IndirectT::new(
            2,
            0,
            Rc::new(LocatedVal::new(PDFObjT::Integer(IntegerT::new(5)), 0, 1)),
        );
        let l1 = LocatedVal::new(i1, 0, 4);

        let i2 = IndirectT::new(
            3,
            0,
            Rc::new(LocatedVal::new(
                PDFObjT::Dict(DictT::new(BTreeMap::new())),
                0,
                1,
            )),
        );
        let l2 = LocatedVal::new(i2, 0, 4);
        ctxt.register_obj(&l1);
        ctxt.register_obj(&l2);
        let v = Vec::from(
            "<</Type/Catalog/Pages 2 0 R/Lang(en-AU) /StructTreeRoot 75 0 R/MarkInfo<</Marked true>>>>"
            .as_bytes(),
        );
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = catalog_type(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
}
