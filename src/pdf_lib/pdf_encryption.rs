use std::rc::Rc;
use log::{debug, error, log, Level, LevelFilter};
use stringprep::{saslprep};

use super::super::pcore::parsebuffer::{
    locate_value, ErrorKind, LocatedVal, Location,
    ParseBufferT, ParseResult, ParsleyParser,
};


use super::pdf_obj::{
    IndirectP, IndirectT, PDFObjContext, PDFObjT, StreamT, DictT, ArrayT
};

use super::pdf_prim::{StreamContentT, IntegerT};
use sha2::{Sha256, Sha512, Digest};
use std::mem;

use md5::{Md5};
use md5::Digest as md5digest;

// Encryption Dictionary.
// This struct contains the data required to decrypt an encrypted stream object.
pub struct PDFEncryptionDict {
    d: IndirectT,
    v: i64,
    length: i64,
    id_obj: Vec<Rc<LocatedVal<PDFObjT>>>
}

impl PDFEncryptionDict {
    pub fn new(d: IndirectT, v : i64, l: i64, id_obj: Vec<Rc<LocatedVal<PDFObjT>>>) -> PDFEncryptionDict {
        return PDFEncryptionDict{d: d, v: v , length: l, id_obj: id_obj}
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

// 7.6.4.3.2
// Algorithm 2: Computing a file encryption key in order to encrypt a
// document (revision 4 and earlier
fn compute_file_enc_key(d: &PDFEncryptionDict) {
    // TODO: sameed
    println!("in compute_file_enc_key\n");
    let padding_str = vec![0x28, 0xBF, 0x4E, 0x5E, 0x4E, 0x75, 0x8A,
                           0x41, 0x64, 0x00, 0x4E, 0x56, 0xFF, 0xFA,
                           0x01, 0x08, 0x2E, 0x2E, 0x00, 0xB6, 0xD0,
                           0x68, 0x3E, 0x80, 0x2F, 0x0C, 0xA9, 0xFE,
                           0x64, 0x53, 0x69, 0x7A];

    // get the enc dict and get the user password
    if let PDFObjT::Dict(dict_obj) = d.get_dict().obj().val() {
        if let Some(o_obj) = dict_obj.get(b"O") {
            if let PDFObjT::String(owner_pass) = o_obj.val() {
                let mut tmp : Vec<u8> = [].to_vec();
                // let pass = "";
                // tmp.extend(pass.iter().copied());
                tmp.extend(padding_str.iter().copied());
                // take the first 32 bytes
                // let u_pass = tmp.iter().take(32);
                let mut hasher = Md5::new();
                // padding string
                hasher.update(tmp);
                // O value
                hasher.update(owner_pass);
                if let Some(p_obj) = dict_obj.get(b"P") {
                    if let PDFObjT::Integer(p_val) = p_obj.val() {
                        unsafe {
                            let unsigned_p_val = p_val.int_val() as u32;
                            let mut p = mem::transmute::<u32, [u8; 4]>(unsigned_p_val);
                            p.reverse();
                            // P value
                            hasher.update(p);
                        }
                    } else {
                        panic!("cannot read P value from dict!\n");
                    }
                } else {
                    panic!("dict does not have P value")
                }

                // ID object value
                match d.id_obj[0].val() {
                    PDFObjT::String(x) => {
                        hasher.update(x)
                    },
                    _ => {
                        panic!("ID object is invalid: {:?}", d.id_obj[0].val());
                    },
                };

                if let Some(r_obj)  = dict_obj.get(b"R"){
                    if let PDFObjT::Integer(r) = r_obj.val() {
                        if r.int_val() >= 4 {
                            panic!("revision version 4 not implemented yet!\n");
                        }

                        // finish hasher
                        let digest = hasher.finalize();
                        debug!("Result of hash is: {:?}\n", digest);

                        if r.int_val() >= 3 {
                            panic!("revision >= 3 not implemented yet!\n");
                        }

                        // take slice of hasher as result depending on revision number
                        let result;
                        if r.int_val() == 2 {
                             result = &digest[0 .. 5];
                        }
                        if let Some(l_obj) = dict_obj.get(b"Length") {
                            if let PDFObjT::Integer(l) = l_obj.val() {
                                let result = &digest[0 .. l.int_val()];
                                println!("\n key is {:?}\n", l.int_val());
                            }
                        }
                    }
                }
            } else {
                panic!("string extraction from O in enc dict failed\n");
            }
        } else {
            panic!("O key is missing in encryption dict.\n");
        }
    }
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
                        println!("TODO finish this:: read O: {:?} {}", s, s.len());
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
    }
}

/* decrypt root stream */
pub fn decrypt_root_stream(ctxt: &PDFObjContext, xref_obj : &Rc<LocatedVal<PDFObjT>>, s: &StreamT) {
    if !ctxt.is_encrypted() {
        panic!("Stream is not encrypted....");
    } else {
        println!("Started the decrypt_root_stream function....\n");

        let content = s.stream().val();


        if let Some(d) = ctxt.get_encryption_dict() {
            compute_file_enc_key(d);
            if let PDFObjT::Dict(dict_obj) = d.get_dict().obj().val() {
                println!("printing dict_obj: {:?}\n", dict_obj);
                // generate the file encryption key
                if let Some(u_obj) = dict_obj.get(b"U") {
                    if let PDFObjT::String(usr_password) = u_obj.val() {
                        println!("read u: {:?} {}", usr_password, usr_password.len());

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
    }
}
