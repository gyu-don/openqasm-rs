extern crate openqasm;

use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use openqasm::lexer;

fn main() {
    if let Some(filename) = env::args().nth(1) {
        let f = File::open(&filename).expect("Failed to open the target file.");
        let mut f = io::BufReader::new(f);
        let mut code = String::new();
        f.read_to_string(&mut code).expect("Failed to read the target file.");
        let mut tokens = lexer::tokenize(&code, Path::new(&filename));
        for tok in tokens {
            println!("{:?}", tok);
        }
    } else {
        println!("Usage: {} <filename>", env::args().nth(0).unwrap());
    }
}
