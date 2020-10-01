mod structures {
    use super::super::super::pcore::parsebuffer::{ParseBuffer};
    use super::super::pdf_obj::{PDFObjContext, PDFObjT, parse_pdf_obj};
    use crate::pdf_lib::pdf_prim::NameT;
    use super::super::pdf_type_check::{
        check_type, DictEntry, DictKeySpec, PDFPrimType, PDFType, TypeCheck, TypeCheckError, ChoicePred
    };
    use std::rc::Rc;
    fn rectangle_check() -> Rc<TypeCheck> {
        Rc::new(TypeCheck::new(Rc::new(PDFType::Array {
            elem: Rc::new(TypeCheck::new(Rc::new(PDFType::PrimType(
                                  PDFPrimType::Integer,
                                  )))),
                                  size: Some(4),
        })))
    }
}
