extern crate parsley_rust;
use parsley_rust::pcore::parsebuffer;
use parsley_rust::pcore::prim_ascii;

fn main() {
    let mut ascii_parser = prim_ascii::AsciiChar::new_guarded(Box::new(|c: &char| *c == 'A'));
    let mut v : Vec<u8> = Vec::new();
    v.push(65);   // 'A'
    v.push(128);  // non-ascii
    v.push(0);    // nul; ascii
    let mut pb = parsebuffer::ParseBuffer::new(v);

    println!("I'm using the library: {:?}", pb);
}

