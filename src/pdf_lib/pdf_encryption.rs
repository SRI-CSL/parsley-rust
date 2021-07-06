use std::rc::Rc;
use log::{debug, error, log, Level, LevelFilter};

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, Location,
    ParseBufferT, ParseResult, ParsleyParser,
};


use super::pdf_obj::{
    IndirectP, IndirectT, PDFObjContext, PDFObjT, StreamT, DictT
};

use super::pdf_prim::{StreamContentT};

// Encryption Dictionary.
// This struct contains the data required to decrypt an encrypted stream object.
pub struct PDFEncryptionDict {
    v: i64,
    filter: i64,
    length: i64
}

pub fn decrypt_stream(ctxt: &PDFObjContext, s: &StreamT) {
    if !ctxt.is_encrypted() {
        panic!("Stream is not encrypted....");
    } else {
        debug!("started the decryption...");

        // let v = match s.filters() {
        //     Ok(v) => v,
        //     Err(e) => return panic!(e)
        // };

        // for (count, x) in v.iter().enumerate() {
        //     println!("{:?}", x.name());
        // }

        let content = s.stream().val();
        debug!("content start: {:?} size: {:?} content: {:?}",
               content.start(),
               content.size(),
               content.content());
    }
}

