use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::PDFObjT;
use crate::pdf_lib::common_data_structures::mk_parent_typchk;
use crate::pdf_lib::page::{page_type, template_type};
use crate::pdf_lib::pdf_prim::NameT;
use crate::pdf_lib::pdf_type_check::{
    ChoicePred, DictEntry, DictKeySpec, IndirectSpec, PDFPrimType, PDFType, Predicate, TypeCheck,
    TypeCheckContext, TypeCheckError,
};
use std::rc::Rc;

fn mk_pages_check(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pred = ChoicePred(
        String::from("Pages not present."),
        vec![PDFObjT::Name(NameT::new(Vec::from("Pages")))],
    );
    TypeCheck::new_refined(
        tctx,
        "pages",
        Rc::new(PDFType::PrimType(PDFPrimType::Name)),
        Rc::new(pred),
    )
}

fn mk_count_typchk(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    TypeCheck::new(
        tctx,
        "count",
        Rc::new(PDFType::PrimType(PDFPrimType::Integer)),
    )
}

pub fn root_page_tree(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pages = DictEntry {
        key: Vec::from("Type"),
        chk: mk_pages_check(tctx), // this must be a NameT
        opt: DictKeySpec::Required,
    };
    let count = DictEntry {
        key: Vec::from("Count"),
        chk: mk_count_typchk(tctx),
        opt: DictKeySpec::Required,
    };
    let opts = Rc::new(PDFType::Disjunct(vec![
        non_root_page_tree(tctx),
        page_type(tctx),
        template_type(tctx),
    ]));
    let elem = TypeCheck::new_indirect(tctx, "kid", opts, IndirectSpec::Required);
    let kids = Rc::new(PDFType::Array { elem, size: None });
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: TypeCheck::new(tctx, "kids", kids),
        opt: DictKeySpec::Required,
    };
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: mk_parent_typchk(tctx),
        opt: DictKeySpec::Forbidden,
    };
    TypeCheck::new(
        tctx,
        "root-page-tree",
        Rc::new(PDFType::Dict(vec![pages, count, kids, parent])),
    )
}

pub fn non_root_page_tree(tctx: &mut TypeCheckContext) -> Rc<TypeCheck> {
    let pages = DictEntry {
        key: Vec::from("Type"),
        chk: mk_pages_check(tctx), // this must be a NameT
        opt: DictKeySpec::Required,
    };
    let count = DictEntry {
        key: Vec::from("Count"),
        chk: mk_count_typchk(tctx),
        opt: DictKeySpec::Required,
    };
    let opts = Rc::new(PDFType::Disjunct(vec![
        page_type(tctx),
        TypeCheck::new_named("root-non-page-tree"),
        template_type(tctx),
    ]));
    let elem = TypeCheck::new_indirect(tctx, "kid", opts, IndirectSpec::Required);
    let kids = Rc::new(PDFType::Array { elem, size: None });
    let kids = DictEntry {
        key: Vec::from("Kids"),
        chk: TypeCheck::new(tctx, "kids", kids),
        opt: DictKeySpec::Required,
    };
    // Don't apply checks upward in the tree, otherwise we get into an
    // infinite loop.  Just ensure that the key is present.
    let parent = DictEntry {
        key: Vec::from("Parent"),
        chk: mk_parent_typchk(tctx),
        opt: DictKeySpec::Required,
    };
    TypeCheck::new(
        tctx,
        "root-non-page-tree",
        Rc::new(PDFType::Dict(vec![pages, count, kids, parent])),
    )
}

struct ReferencePredicate;

impl Predicate for ReferencePredicate {
    fn check(&self, obj: &Rc<LocatedVal<PDFObjT>>) -> Option<LocatedVal<TypeCheckError>> {
        if let PDFObjT::Array(ref s) = obj.val() {
            for c in s.objs() {
                if let PDFObjT::Reference(ref _s2) = c.val() {
                } else {
                    return Some(obj.place(TypeCheckError::PredicateError(
                        "Reference expected".to_string(),
                    )))
                }
            }
            None
        } else {
            Some(obj.place(TypeCheckError::PredicateError(
                "Reference wasn't an Array".to_string(),
            )))
        }
    }
}

#[cfg(test)]

mod test_page_tree {
    use super::super::super::pcore::parsebuffer::{LocatedVal, ParseBuffer};
    use super::super::pdf_obj::{parse_pdf_obj, IndirectT, PDFObjContext};
    use super::super::pdf_type_check::{check_type, TypeCheckContext};
    use super::{non_root_page_tree, root_page_tree};
    use std::rc::Rc;

    fn mk_new_context() -> PDFObjContext { PDFObjContext::new(10) }

    // Page Tree Non-Root Node Tests
    #[test]
    fn test_root_page_tree() {
        let mut tctx = TypeCheckContext::new();
        let mut ctxt = mk_new_context();
        let page_1 = Vec::from(
            "<<
        /Parent 3 0 R
        /Annots [7 0 R 8 0 R 9 0 R 10 0 R 11 0 R 12 0 R]
        /Resources <<
        /Font <<
        /PLSIQB+Arial-BoldMT 13 0 R
        >>

        /XObject <<
        /HiQPdf_cegclgkofmhfkigcpjjhbeblpoekmnaj 14 0 R
        /HiQPdf_leaaobjohjkdkkbibmeielinnefhgagb 15 0 R
        /HiQPdf_enckkhlmoiokklenbnaknaojkeebldab 16 0 R
        >>

        /ProcSet [/PDF /Text /ImageC]
        /ExtGState <<
        /HiQPdf_kbdahdnmdhgokmhfpielfkjngbhhdkmm 17 0 R
        /HiQPdf_bahcpcpdpmjekbbdafgfphaelffjbdan 18 0 R
        >>

        >>

        /Contents [19 0 R 20 0 R]
        /Type /Page
        /MediaBox [0.00000 0.00000 612.00000 792.00000]
        >>
        "
            .as_bytes(),
        );
        let mut page1_pb = ParseBuffer::new(page_1);
        let page1_obj = parse_pdf_obj(&mut ctxt, &mut page1_pb).unwrap();
        let i1 = IndirectT::new(4, 0, Rc::new(page1_obj));
        let l1 = LocatedVal::new(i1, 0, 4);
        ctxt.register_obj(&l1);
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3
        // >>".as_bytes());
        let v = Vec::from("<</Type /Pages /Kids [4 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = root_page_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
    #[test]
    fn test_non_root_page_tree_not_wrong() {
        let mut tctx = TypeCheckContext::new();
        let mut ctxt = mk_new_context();
        let page_1 = Vec::from(
            "<<
        /Parent 3 0 R
        /Annots [7 0 R 8 0 R 9 0 R 10 0 R 11 0 R 12 0 R]
        /Resources <<
        /Font <<
        /PLSIQB+Arial-BoldMT 13 0 R
        >>

        /XObject <<
        /HiQPdf_cegclgkofmhfkigcpjjhbeblpoekmnaj 14 0 R
        /HiQPdf_leaaobjohjkdkkbibmeielinnefhgagb 15 0 R
        /HiQPdf_enckkhlmoiokklenbnaknaojkeebldab 16 0 R
        >>

        /ProcSet [/PDF /Text /ImageC]
        /ExtGState <<
        /HiQPdf_kbdahdnmdhgokmhfpielfkjngbhhdkmm 17 0 R
        /HiQPdf_bahcpcpdpmjekbbdafgfphaelffjbdan 18 0 R
        >>

        >>

        /Contents [19 0 R 20 0 R]
        /Type /Page
        /MediaBox [0.00000 0.00000 612.00000 792.00000]
        >>
        "
            .as_bytes(),
        );
        let mut page1_pb = ParseBuffer::new(page_1);
        let page1_obj = parse_pdf_obj(&mut ctxt, &mut page1_pb).unwrap();
        let i1 = IndirectT::new(10, 0, Rc::new(page1_obj));
        let l1 = LocatedVal::new(i1, 0, 4);
        ctxt.register_obj(&l1);
        //let v = Vec::from("<</Type /Pages /Kids [4 0 R  10 0 R 24 0 R ] /Count 3
        // >>".as_bytes());
        let v = Vec::from("<</Type /Pages /Parent [4 0 R] /Kids [10 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = non_root_page_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
    // Page Tree Non Root Node Tests End

    #[test]
    fn test_page_tree_not_wrong() {
        let mut tctx = TypeCheckContext::new();
        let mut ctxt = mk_new_context();
        let page_1 = Vec::from(
            "<<
        /Parent 3 0 R
        /Annots [7 0 R 8 0 R 9 0 R 10 0 R 11 0 R 12 0 R]
        /Resources <<
        /Font <<
        /PLSIQB+Arial-BoldMT 13 0 R
        >>

        /XObject <<
        /HiQPdf_cegclgkofmhfkigcpjjhbeblpoekmnaj 14 0 R
        /HiQPdf_leaaobjohjkdkkbibmeielinnefhgagb 15 0 R
        /HiQPdf_enckkhlmoiokklenbnaknaojkeebldab 16 0 R
        >>

        /ProcSet [/PDF /Text /ImageC]
        /ExtGState <<
        /HiQPdf_kbdahdnmdhgokmhfpielfkjngbhhdkmm 17 0 R
        /HiQPdf_bahcpcpdpmjekbbdafgfphaelffjbdan 18 0 R
        >>

        >>

        /Contents [19 0 R 20 0 R]
        /Type /Page
        /MediaBox [0.00000 0.00000 612.00000 792.00000]
        >>
        "
            .as_bytes(),
        );
        let mut page1_pb = ParseBuffer::new(page_1);
        let page1_obj = parse_pdf_obj(&mut ctxt, &mut page1_pb).unwrap();
        let i1 = IndirectT::new(10, 0, Rc::new(page1_obj));
        let l1 = LocatedVal::new(i1, 0, 4);
        ctxt.register_obj(&l1);
        let page_2 = Vec::from("<< /Type /Page
        /Parent 3 0 R
        /Annots [ 13 0 R 15 0 R 17 0 R 19 0 R 21 0 R 23 0 R 25 0 R 27 0 R 29 0 R 31 0 R 35 0 R 39 0 R ]
        /Contents 7 0 R
        >>
        ".as_bytes());
        let mut page2_pb = ParseBuffer::new(page_2);
        let page2_obj = parse_pdf_obj(&mut ctxt, &mut page2_pb).unwrap();
        let i2 = IndirectT::new(24, 0, Rc::new(page2_obj));
        let l2 = LocatedVal::new(i2, 0, 4);
        ctxt.register_obj(&l2);
        let v = Vec::from("<</Type /Pages /Kids [10 0 R 24 0 R ] /Count 3 >>".as_bytes());
        //let v = Vec::from("<< /Count 3 >>".as_bytes());
        let mut pb = ParseBuffer::new(v);
        let obj = parse_pdf_obj(&mut ctxt, &mut pb).unwrap();
        let typ = root_page_tree(&mut tctx);
        assert_eq!(check_type(&ctxt, &tctx, Rc::new(obj), typ), None);
    }
    // Page Tree Root Node Tests End
}
