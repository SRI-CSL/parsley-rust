#[macro_use]
extern crate log;
extern crate env_logger;
extern crate log_panics;

use std::io::prelude::*;
use std::net::TcpStream;
use env_logger::Builder;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::panic;
use std::path::Path;
use std::process;
use std::rc::Rc;
use std::convert::TryInto;
use std::collections::{VecDeque, BTreeSet};
use parsley_rust::pcore::parsebuffer::{ParseBuffer, ParsleyParser, Location, LocatedVal};

fn main() -> std::io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;
    {
        let mut string = "abcd";
        stream.write(b"abcd")?;
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;
        println!("Printing response: {}", buffer);
    }
    Ok(())
}
