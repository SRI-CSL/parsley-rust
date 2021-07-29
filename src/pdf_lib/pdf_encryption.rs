use std::rc::Rc;
use log::{debug, error, log, Level, LevelFilter};
use stringprep::{saslprep};

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, Location,
    ParseBufferT, ParseResult, ParsleyParser,
};


use super::pdf_obj::{
    IndirectP, IndirectT, PDFObjContext, PDFObjT, StreamT, DictT
};

use super::pdf_prim::{StreamContentT};
use sha2::{Sha256, Sha512, Digest};

// Encryption Dictionary.
// This struct contains the data required to decrypt an encrypted stream object.
pub struct PDFEncryptionDict {
    d: IndirectT,
    v: i64,
    length: i64
}

impl PDFEncryptionDict {
    pub fn new(d: IndirectT, v : i64, l: i64) -> PDFEncryptionDict {
        return PDFEncryptionDict{d: d, v: v , length: l}
    }

    pub fn get_v(&self) ->  i64 {
        return self.v;
    }

    pub fn get_length(&self) -> i64 {
        return self.length;
    }

    pub fn get_dict(&self) -> &IndirectT {
        return &self.d;
    }
}

fn sha256_hash(s : String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(s);

    // read hash digest and consume hasher
    return format!("{:X}", hasher.finalize());
}

/* 7.6.4.3.4 - compute SHA256 hash
 */
fn hash_algo_2_b(pass : String) -> String {

    let K = sha256_hash(pass);

    return K;
}


fn sasl_and_truncate_owner_pass(owner_pass: &str) -> String {
    // 1. SASL prep the string
    match saslprep(owner_pass) {
        Ok(s) => {
            // 2. truncate the UTF-8 representation to 127 bytes if it is longer than 127 bytes.
            if s.len() >= 127 {
                let st = s.to_string();
                let trunc_pass_str = & st[..127];
                return trunc_pass_str.to_string();
            }
            return s.to_string();
        },
        Err(e) => return panic!(e)
    };
}

// check if owner password is correct
// PDF spec [2017] section: 7.6.4.3.3
fn validate_owner_password (owner_pass : & str, OVS : String, U : String, OPH : String) {
    // step 1 && 2
    let hash_input = format!("{}{}{}", sasl_and_truncate_owner_pass(owner_pass), OVS, U);
    let s = hash_algo_2_b(hash_input);
    println!("hashed string is: {}", s);
}

fn compute_file_enc_key(owner_password : & str) {

}

/*
 * Decrypt an encrypted stream object.
 */
pub fn decrypt_stream(ctxt: &PDFObjContext, xref_obj : &LocatedVal<IndirectT>, s: &StreamT) {
    if !ctxt.is_encrypted() {
        panic!("Stream is not encrypted....");
    } else {
        println!("Started the decryption function....\n");

        let content = s.stream().val();

        // generate the file encryption key
        // compute_file_enc_key(usr_password);


        if let Some(d) = ctxt.get_encryption_dict() {
            if let PDFObjT::Dict(dict_obj) = d.get_dict().obj().val() {
                if let Some(o_obj) = dict_obj.get(b"O") {
                    if let PDFObjT::String(s) = o_obj.val() {
                        println!("read O: {:?} {}", s, s.len());
                    } else {
                        panic!("string extraction from O in enc dict failed\n");
                    }
                } else {
                    panic!("O key is missing in encryption dict.\n");
                }
            } else {
                panic!("PDFObjT::Dict casting failed\n");
            }
        } else {
            panic!("FATAL ERROR: NO ENCRYPTION DICTIONARY FOUND!\n");
        }

        //validate_owner_password("testing000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012312");
    }
}
