#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::{ File };
use std::io;
use std::io::{ prelude::*, BufReader };
use std::path::{ Path };

mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("No file found");
    let reader = BufReader::new(file);
    let content: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let tokens = tokenizer::run(&content);
    // let output = parser::run(&tokens);
}
