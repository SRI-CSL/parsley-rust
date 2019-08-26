use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
pub mod pcore;
pub mod pdf_lib;
use pcore::parsebuffer::ParseBuffer;

pub fn parse_file<'a>(file_path: &'a str) -> pcore::parsebuffer::ParseBuffer {
    let path = env::current_dir();
    if let Err(_) = path {
        println!("Cannot get current dir!");
    }
    let mut path = path.unwrap();
    path.push(file_path);
    let _display = path.as_path().display();
    // let display_var: &'a std::path::Display = &_display;
    // return display_var;
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path.as_path()) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", _display, why.description()),
        Ok(file) => file,
    };
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", _display, why.description()),
        Ok(_)    => ()
    }
    let p = ParseBuffer::new(Vec::from(s.as_bytes()));
    //let pb:&'a pcore::parsebuffer::ParseBuffer = &p;
    println!("{}", &p);
    return p;
}
