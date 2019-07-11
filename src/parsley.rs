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
        println!("{}", filename);
    }
    else
    {
        process::exit(0x0100);
    }
    let mut ascii_parser = prim_ascii::AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
    let mut v : Vec<u8> = Vec::new();
    v.push(65);   // 'A'
    v.push(128);  // non-ascii
    v.push(0);    // nul; ascii
    let mut pb = parsebuffer::ParseBuffer::new(v);

    println!("I'm using the library: {:?} {}", pb, filename);
}

