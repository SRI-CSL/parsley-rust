// Copyright (c) 2019-2020 SRI International.
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

use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{ArrayT, DictKey, DictT, ObjectId, PDFObjT};
use super::pdf_prim::{IntegerT, NameT};
use format_num::NumberFormat;
use log::{log, Level};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::process;
use std::rc::Rc;
use std::str;

macro_rules! exit_log {
    ($pos:expr, $($arg:tt)+) => ({
        log!(Level::Error, "at {:>10} ({:#x}) - {}", $pos, $pos, format_args!($($arg)+));
        process::exit(1)
    })
}

fn serialize_dict(map: &BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let mut start = "<<".as_bytes().to_vec();
    let mut end = ">>".as_bytes().to_vec();
    let mut res: Vec<u8> = vec![];
    result.append(&mut start);
    for (key, value) in map {
        let mut slash = "/".as_bytes().to_vec();
        res.append(&mut slash);
        let n = key.as_slice().to_vec();
        for i in n {
            if i <= 33
                || i >= 126
                || i == 40
                || i == 41
                || i == 47
                || i == 60
                || i == 62
                || i == 14
                || i == 37
                || i == 123
                || i == 125
                || i == 91
                || i == 93
            {
                let escaped = format!("#{:02X}", i);
                let mut x = escaped.to_ascii_uppercase().as_bytes().to_vec();
                res.append(&mut x);
            } else {
                res.push(i);
            }
        }
        //res.append(&mut key.as_slice().to_vec());
        let mut whitespace = " ".as_bytes().to_vec();
        res.append(&mut whitespace);
        let mut value_result = evaluate(value.val());
        res.append(&mut value_result);
        let mut whitespace = " ".as_bytes().to_vec();
        res.append(&mut whitespace);
        result.append(&mut res);
    }
    result.append(&mut end);
    return result
}

pub fn serializer(
    object_ids: Vec<ObjectId>, objects: Vec<Rc<LocatedVal<PDFObjT>>>, root_id: ObjectId,
    ofile: &mut fs::File, info_id: Option<ObjectId>,
) {
    let mut offset_list: Vec<u32> = vec![];
    let mut offset_counter = 0; // Tracks the offset of each object
                                // offset_counter is used to create the xref table entries
    let mut xrefs: Vec<u128> = vec![];
    let filehandler = ofile;
    let mut xref_output: Vec<u8> = vec![];
    match filehandler.write("%PDF-2.0\x0a".as_bytes()) {
        Ok(d) => {
            offset_counter = offset_counter + d;
        },
        Err(_) => {},
    };
    let mut max = 0; // to compute Size key in Info
    for id in 0 .. object_ids.len() {
        let curobject = objects[id].clone();
        offset_list.push(object_ids[id].0 as u32);
        xrefs.push(offset_counter as u128);
        /*
         * This is not a smart way of creating the xref table
         * We create an entry for each object present
         */
        let cur = curobject.val();
        let x_1 = format!("{} 1\x0a", object_ids[id].0);
        let mut x_a1 = x_1.as_bytes().to_vec();
        let x_2 = format!("{:010} {:05} n \x0a", offset_counter, object_ids[id].1);
        let mut x_a2 = x_2.as_bytes().to_vec();
        xref_output.append(&mut x_a1);
        xref_output.append(&mut x_a2);
        let objname = format!("{} {} obj\x0a", object_ids[id].0, object_ids[id].1);
        match filehandler.write(objname.as_bytes()) {
            Ok(d) => {
                offset_counter = offset_counter + d;
            },
            Err(_) => {},
        };
        let g = generate(cur);
        match filehandler.write(&g) {
            Ok(d) => {
                offset_counter = offset_counter + d;
            },
            Err(_) => {},
        };
        match filehandler.write("\x0aendobj\x0a".as_bytes()) {
            Ok(d) => {
                offset_counter = offset_counter + d;
            },
            Err(_) => {
                exit_log!(offset_counter, "Error writing endobj");
            },
        };
        // max keeps track of the max entry for the Size entry in info
        if object_ids[id].0 > max {
            max = object_ids[id].0;
        }
    }
    match filehandler.write("xref\x0a".as_bytes()) {
        Ok(_) => {},
        Err(_) => {
            exit_log!(offset_counter, "Error writing xref table");
        },
    }
    match filehandler.write(&xref_output) {
        Ok(_) => {},
        Err(_) => {
            exit_log!(offset_counter, "Error writing xref table");
        },
    }
    // Need to put trailer here
    let trailer = match info_id {
        Some(info) => {
            format!(
                "trailer << /Root {} {} R /Size {} /Info {} {} R >> \x0a",
                root_id.0,
                root_id.1,
                max + 1,
                info.0,
                info.1,
            )
        },
        None => {
            format!(
                "trailer << /Root {} {} R /Size {} >> \x0a",
                root_id.0,
                root_id.1,
                max + 1,
            )
        },
    };
    match filehandler.write(trailer.as_bytes()) {
        Ok(_) => {},
        Err(_) => {
            exit_log!(offset_counter, "Error writing trailer");
        },
    }
    let startxref = format!("startxref\x0a{}\x0a", offset_counter);
    match filehandler.write(startxref.as_bytes()) {
        Ok(d) => {
            offset_counter = offset_counter + d;
        },
        Err(_) => {
            exit_log!(offset_counter, "Error writing string startxref");
        },
    }
    match filehandler.write("%%EOF\x0a".as_bytes()) {
        Ok(_) => {},
        Err(_) => {
            exit_log!(offset_counter, "Error writing string EOF");
        },
    }
}

/*
 * This function makes recursive calls in one location and computes the final
 * string output from a PDF object parsed
 * serialize_dict in turn calls evaluate
 */
fn evaluate(obj: &PDFObjT) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    match obj {
        PDFObjT::Array(d) => {
            let mut start = "[".as_bytes().to_vec();
            let mut end = " ]".as_bytes().to_vec();
            result.append(&mut start);
            for obj in d.objs() {
                let mut whitespace = " ".as_bytes().to_vec();
                let mut res = evaluate(obj.val()); // Recursive call
                result.append(&mut whitespace);
                result.append(&mut res);
            }
            result.append(&mut end);
        },
        PDFObjT::Dict(d) => {
            let map = d.map();
            let mut res = serialize_dict(&map);
            result.append(&mut res);
        },
        PDFObjT::Stream(d) => {
            let mut start = "\x0astream\x0a".as_bytes().to_vec();
            let mut end = "\x0aendstream".as_bytes().to_vec();
            let stream_dict = d.dict().val().map();
            let mut stream_result = serialize_dict(stream_dict);
            result.append(&mut stream_result);
            result.append(&mut start);
            let mut content = d.content().to_vec();
            result.append(&mut content);
            result.append(&mut end);
        },
        PDFObjT::Reference(d) => {
            let mut res = format!("{:?} {:?} R", d.num(), d.gen()).as_bytes().to_vec();
            result.append(&mut res);
        },
        PDFObjT::Boolean(d) => {
            if *d {
                let mut res = "true".as_bytes().to_vec();
                result.append(&mut res);
            } else {
                let mut res = "false".as_bytes().to_vec();
                result.append(&mut res);
            }
        },
        PDFObjT::String(d) => match str::from_utf8(d) {
            Ok(x) => {
                let mut start = "(".as_bytes().to_vec();
                let mut end = ")".as_bytes().to_vec();
                let mut d2: Vec<u8> = vec![];
                let d1 = x.as_bytes().to_vec();
                for d in d1 {
                    let num = NumberFormat::new();
                    let x = num.format("o", d);
                    let mut x1 = format!("\\{:03}", x).as_bytes().to_vec();
                    d2.append(&mut x1)
                }
                result.append(&mut start);
                result.append(&mut d2);
                result.append(&mut end);
            },
            Err(_) => {
                let mut s = "<".to_string();
                for character in d {
                    let f = format!("{:02X}", character);
                    s.push_str(&f);
                }
                s.push_str(">");
                let mut vec_s = s.as_bytes().to_vec();
                result.append(&mut vec_s);
            },
        },
        PDFObjT::Name(d) => {
            let mut res = "/".as_bytes().to_vec();
            let n = d.normalize();
            for i in n {
                if i <= 33
                    || i >= 126
                    || i == 40
                    || i == 41
                    || i == 47
                    || i == 14
                    || i == 60
                    || i == 62
                    || i == 37
                    || i == 123
                    || i == 125
                    || i == 91
                    || i == 93
                {
                    let escaped = format!("#{:02X}", i);
                    let mut x = escaped.to_ascii_uppercase().as_bytes().to_vec();
                    res.append(&mut x);
                } else {
                    res.push(i);
                }
            }
            result.append(&mut res);
        },
        PDFObjT::Null(_) => {
            let mut res = "null".as_bytes().to_vec();
            result.append(&mut res);
        },
        PDFObjT::Comment(_) => {
            // Ignoring comments
        },
        PDFObjT::Integer(d) => {
            let mut res = format!("{:?}", d.int_val()).as_bytes().to_vec();
            result.append(&mut res);
        },
        PDFObjT::Real(d) => {
            let mut res = format!("{:?}", d.val()).as_bytes().to_vec();
            result.append(&mut res);
        },
    }
    return result
}

fn generate(obj: &PDFObjT) -> Vec<u8> { return evaluate(obj) }

fn reducer_replace_pagemode(
    mut root_dict: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>>,
) -> BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>> {
    // Extract the Lang key
    let pagemodekey = DictKey::new("PageMode".as_bytes().to_vec());
    let s = root_dict.remove(&pagemodekey);
    if s.is_some() {
        let unwrapped_field = s.unwrap();
        let val = unwrapped_field.val();
        let start = unwrapped_field.start();
        let end = unwrapped_field.end();
        match val {
            PDFObjT::Name(v) => {
                let options = [
                    "UseNone".as_bytes(),
                    "UseOutlines".as_bytes(),
                    "UseThumbs".as_bytes(),
                    "FullScreen".as_bytes(),
                    "UseOC".as_bytes(),
                    "UseAttachments".as_bytes(),
                ];
                if options.contains(&v.val()) {
                    root_dict.insert(pagemodekey, unwrapped_field);
                } else {
                    root_dict.insert(
                        pagemodekey,
                        Rc::new(LocatedVal::new(
                            PDFObjT::Name(NameT::new("UseNone".as_bytes().to_vec())),
                            start,
                            end,
                        )),
                    );
                }
            },
            _ => {},
        }
    }
    root_dict
}

fn reducer_remove_lang(
    mut root_dict: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>>,
) -> BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>> {
    // Extract the Lang key
    let langkey = DictKey::new("Lang".as_bytes().to_vec());
    let s = root_dict.remove(&langkey);
    if s.is_some() {
        let unwrapped_field = s.unwrap();
        let val = unwrapped_field.val();
        match val {
            PDFObjT::String(_) => {
                root_dict.insert(langkey, unwrapped_field);
            },
            _ => {},
        }
    }
    root_dict
}

fn reducer_remove_null_entries(
    object_ids: Vec<ObjectId>, objects: Vec<Rc<LocatedVal<PDFObjT>>>,
) -> (Vec<ObjectId>, Vec<Rc<LocatedVal<PDFObjT>>>) {
    let mut robject_ids: Vec<ObjectId> = vec![];
    let mut robjects: Vec<Rc<LocatedVal<PDFObjT>>> = vec![];
    for id in 0 .. object_ids.len() {
        let mut map_tmp: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>> = BTreeMap::new();
        let val = objects[id].val().clone();
        /*
         * If it is a Dict entry, check if there is a key with the NullType in value
         * Ignore all such keys since they are as good as not present
         * For any other type of object, just write it as is---no traversals done
         */
        match val {
            PDFObjT::Dict(d) => {
                let m = d.clone();
                let mut map = m.map().clone();
                for (key, value) in &mut map {
                    if *value.val() != PDFObjT::Null(()) {
                        map_tmp.insert(key.clone(), Rc::clone(value));
                    }
                }
                let dict = PDFObjT::Dict(DictT::new(map_tmp.clone()));
                let tmp = Rc::new(LocatedVal::new(
                    dict,
                    objects[id].start(),
                    objects[id].end(),
                ));
                robject_ids.push(object_ids[id].clone());
                robjects.push(tmp);
            },
            _ => {
                robject_ids.push(object_ids[id].clone());
                robjects.push(objects[id].clone());
            },
        }
    }
    return (robject_ids, robjects)
}

/*
 * Fix issues in Page Tree Nodes
 * If the Count or Kids keys are not present, we restore them
 */
fn reducer_pages(
    object_ids: Vec<ObjectId>, objects: Vec<Rc<LocatedVal<PDFObjT>>>,
) -> (Vec<ObjectId>, Vec<Rc<LocatedVal<PDFObjT>>>) {
    let mut robject_ids: Vec<ObjectId> = vec![];
    let mut robjects: Vec<Rc<LocatedVal<PDFObjT>>> = vec![];
    for id in 0 .. object_ids.len() {
        let curpage = objects[id].val().clone();
        let start = objects[id].start();
        let end = objects[id].end();
        match curpage {
            PDFObjT::Dict(d) => {
                let pagekey = DictKey::new("Type".as_bytes().to_vec());
                let countkey = DictKey::new("Count".as_bytes().to_vec());
                let countkeycopy = DictKey::new("Count".as_bytes().to_vec());
                let kidskey = DictKey::new("Kids".as_bytes().to_vec());
                let m = d.clone();
                let mut map = m.map().clone();
                if map.get(&pagekey).is_some() {
                    match map.get(&pagekey).unwrap().val() {
                        PDFObjT::Name(n) => {
                            if n.val() == "Pages".as_bytes() {
                                let count = map.remove(&countkey);
                                let kids = map.remove(&kidskey);
                                // If kids is absent, we must rewrite the count key anyway
                                if kids.is_none() {
                                    map.insert(
                                        countkey,
                                        Rc::new(LocatedVal::new(
                                            PDFObjT::Integer(IntegerT::new(0)),
                                            start,
                                            end,
                                        )),
                                    );
                                    map.insert(
                                        kidskey,
                                        Rc::new(LocatedVal::new(
                                            PDFObjT::Array(ArrayT::new([].to_vec())),
                                            start,
                                            end,
                                        )),
                                    );
                                    robject_ids.push(object_ids[id]);
                                    robjects.push(Rc::new(LocatedVal::new(
                                        PDFObjT::Dict(DictT::new(map)),
                                        start,
                                        end,
                                    )));
                                }
                                // If count is absent and kids is there, check the length of the
                                // array and insert TODO: Maybe we
                                // should make this a corrective method? Make sure both match?
                                else if kids.is_some() && count.is_none() {
                                    let ukidsarray = kids.unwrap();
                                    let kidsarray = ukidsarray.val();
                                    match kidsarray {
                                        PDFObjT::Array(d) => {
                                            map.insert(
                                                countkeycopy,
                                                Rc::new(LocatedVal::new(
                                                    PDFObjT::Integer(IntegerT::new(
                                                        d.objs().len() as i64
                                                    )),
                                                    start,
                                                    end,
                                                )),
                                            );
                                            map.insert(kidskey, ukidsarray);
                                        },
                                        _ => {},
                                    }
                                    robject_ids.push(object_ids[id]);
                                    robjects.push(Rc::new(LocatedVal::new(
                                        PDFObjT::Dict(DictT::new(map)),
                                        start,
                                        end,
                                    )));
                                }
                            }
                        },
                        _ => {},
                    }
                // /Types key is missing, lets add it
                } else if map.get(&countkey).is_some() || map.get(&kidskey).is_some() {
                    map.insert(
                        pagekey,
                        Rc::new(LocatedVal::new(
                            PDFObjT::Name(NameT::new("Pages".as_bytes().to_vec())),
                            start,
                            end,
                        )),
                    );
                    robject_ids.push(object_ids[id]);
                    robjects.push(Rc::new(LocatedVal::new(
                        PDFObjT::Dict(DictT::new(map)),
                        start,
                        end,
                    )));
                }
                if robject_ids.len() == 0 || robject_ids[robject_ids.len() - 1] != object_ids[id] {
                    robject_ids.push(object_ids[id]);
                    robjects.push(Rc::clone(&objects[id]))
                }
            },
            _ => {
                robject_ids.push(object_ids[id]);
                robjects.push(Rc::clone(&objects[id]));
            },
        }
    }
    (robject_ids, robjects)
}

/* Reduce a root_obj: convert certain objects to the correct types if incorrect
   List of transformations
   1. If Lang key is a name object, convert it to a string
   2. If type key is missing in an object, and subtype is in a list, then add type key
   3. If PageMode or PageLayout not in list, change to the default value
   4. Only return the object ids and objects that have changed, including the root
*/
pub fn reduce(
    root_obj: &Rc<LocatedVal<PDFObjT>>, object_ids: Vec<ObjectId>,
    objects: Vec<Rc<LocatedVal<PDFObjT>>>, root_id: ObjectId,
) -> (
    //Rc<LocatedVal<PDFObjT>>,
    Vec<ObjectId>,
    Vec<Rc<LocatedVal<PDFObjT>>>,
) {
    let start = root_obj.start();
    let end = root_obj.end();
    let unwrapped_root = root_obj.val().clone();
    let mut return_objects: Vec<Rc<LocatedVal<PDFObjT>>> = vec![];
    let mut return_object_ids: Vec<ObjectId> = vec![];
    let mut new_root_map: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>> = BTreeMap::new();
    // Extract the BTreeMap from the root_obj and apply the transformations
    match unwrapped_root {
        PDFObjT::Dict(d) => {
            let m = d.clone();
            let map = m.map().clone();
            new_root_map = reducer_remove_lang(map);
            new_root_map = reducer_replace_pagemode(new_root_map);
            let (robject_ids, robjects) = reducer_pages(object_ids, objects);
            let (mut robject_ids, mut robjects) =
                reducer_remove_null_entries(robject_ids, robjects);

            return_object_ids.append(&mut robject_ids);
            return_objects.append(&mut robjects);
            // Add another reducers on the Catalog here
        },
        _ => {},
    }
    for id in 0 .. return_object_ids.len() {
        if return_object_ids[id] == root_id {
            let nroot_map = new_root_map.clone();
            let root_obj = Rc::new(LocatedVal::new(
                PDFObjT::Dict(DictT::new(nroot_map)),
                start,
                end,
            ));
            return_objects[id] = root_obj;
        }
    }
    (return_object_ids, return_objects)
}
