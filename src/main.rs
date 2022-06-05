mod compiler;
use compiler::{compile};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];
    let mut file = File::open(source)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    compile(source.to_string(), contents.to_string());
    Ok(())
}
