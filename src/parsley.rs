extern crate parsley_rust;
use parsley_rust::pcore::parsebuffer;
use parsley_rust::pcore::prim_ascii;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = String::new();
    if args.len() == 2
    {
        filename = String::from(&args[1]);
    }
    else
    {
        process::exit(0x0100);
    }
    parsley_rust::parse_file(&filename);
}

