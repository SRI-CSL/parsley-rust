use super::super::pcore::parsebuffer::{LocatedVal};
use super::pdf_obj::{PDFObjContext, PDFObjT};
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckError, Predicate, ChoicePred
};
use crate::pdf_lib::common_data_structures::structures::{mk_reference_typchk, mk_name_check};
//use crate::pdf_lib::number_tree::{}
use std::rc::Rc;

fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

fn mk_af_typchk() -> Rc<TypeCheck> {
    Rc::new(TypeCheck::new(Rc::new(PDFType::Array {
        elem: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])
                              ))),
                              size: None,
    })))
}
// Errata: extensions, af, dpartroot, dss


fn catalog_type() -> TypeCheck {
    // Row 1
    //TypeCheck::new(Rc::new(PDFType::Dict(vec![typ, version, extensions, pages, pagelabels, names, dests, viewerpreferences, pagelayout,
    let typ = DictEntry {
        key: Vec::from("Type"),
        chk: mk_name_check("Catalog".to_string()),
        opt: DictKeySpec::Required,
    };
    let version = DictEntry {
        key: Vec::from("Version"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Name)))), // TODO: Maybe make a whitelist of version numbers?
        opt: DictKeySpec::Optional,
    };
    let extensions = DictEntry {
        key: Vec::from("Extensions"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let pages = DictEntry {
        key: Vec::from("Pages"),
        chk: mk_reference_typchk(),
        opt: DictKeySpec::Optional,
    };
    let pagelabels = DictEntry {
        key: Vec::from("PageLabels"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let names = DictEntry {
        key: Vec::from("Names"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let dests = DictEntry {
        key: Vec::from("Dests"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let viewerpreferences = DictEntry {
        key: Vec::from("ViewerPreferences"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let pagelayout = DictEntry {
        key: Vec::from("PageLayout"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    //Row 2
    //pagemode, outlines, threads, openaction, aa, uri, acroform, 
    let pagemode = DictEntry {
        key: Vec::from("PageMode"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let outlines = DictEntry {
        key: Vec::from("Outlines"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let threads = DictEntry {
        key: Vec::from("Threads"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let openaction = DictEntry {
        key: Vec::from("OpenAction"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let aa = DictEntry {
        key: Vec::from("AA"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let uri = DictEntry {
        key: Vec::from("Uri"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let acroform = DictEntry {
        key: Vec::from("AcroForm"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };

    //Row 3
    //metadata, structtreeroot, markinfo, lang, spiderinfo, outputintents, pieceinfo, ocproperties, 
    let metadata = DictEntry {
        key: Vec::from("Metadata"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let structtreeroot = DictEntry {
        key: Vec::from("StructTreeRoot"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let markinfo = DictEntry {
        key: Vec::from("MarkInfo"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let lang = DictEntry {
        key: Vec::from("Lang"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let spiderinfo = DictEntry {
        key: Vec::from("SpiderInfo"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let outputintents = DictEntry {
        key: Vec::from("OutputIntents"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let pieceinfo = DictEntry {
        key: Vec::from("PieceInfo"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let ocproperties = DictEntry {
        key: Vec::from("OcProperties"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };

    // Row 4
    //perms, legal, requirements, collection, needsrendering, dss, af, dpartroot])))
    let perms = DictEntry {
        key: Vec::from("Perms"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let legal = DictEntry {
        key: Vec::from("Legal"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let requirements = DictEntry {
        key: Vec::from("Requirements"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let collection = DictEntry {
        key: Vec::from("Collection"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let needsrendering = DictEntry {
        key: Vec::from("NeedsRendering"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(PDFPrimType::Bool)))),
        opt: DictKeySpec::Optional,
    };
    let dss = DictEntry {
        key: Vec::from("DSS"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    let af = DictEntry {
        key: Vec::from("AF"),
        chk: mk_af_typchk(),
        opt: DictKeySpec::Optional,
    };
    let dpartroot = DictEntry {
        key: Vec::from("DPartRoot"),
        chk: Rc::new(TypeCheck::new(Rc::new(PDFType::Dict(vec![])))),
        opt: DictKeySpec::Optional,
    };
    TypeCheck::new(Rc::new(PDFType::Dict(vec![typ, version, extensions, pages, pagelabels, names, dests, viewerpreferences, pagelayout,
                                                pagemode, outlines, threads, openaction, aa, uri, acroform, 
                                                metadata, structtreeroot, markinfo, lang, spiderinfo, outputintents, pieceinfo, ocproperties, 
                                                perms, legal, requirements, collection, needsrendering, dss, af, dpartroot])))
}
