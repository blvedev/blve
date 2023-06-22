mod generate_js;
mod orig_html_struct;
mod structs;
mod transformers;
use std::fs;

use crate::generate_js::generate_js_from_blocks;
use blve_parser::parse_blve_file;

fn main() -> Result<(), String> {
    // get the file path from the command line
    let args: Vec<String> = std::env::args().collect();
    let a = fs::read_to_string(format!("./sample/{}.blv", args[1])).unwrap();
    let b = parse_blve_file(a.as_str()).unwrap();
    let code = generate_js_from_blocks(&b)?;
    println!("{}", code.0);
    Ok(())
}
