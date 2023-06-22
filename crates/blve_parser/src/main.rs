mod parse2;
mod parser1;
mod parsers;
mod structs;
mod swc_parser;
use nanoid::nanoid;
use parse2::parse2;
use parser1::parse1;
#[macro_use]
extern crate swc_common;

fn main() {
    let filenum = "3";
    let filepath = format!("./sample/{}.blv", filenum);
    let input = std::fs::read_to_string(filepath).unwrap();

    let a = parse1(input.as_str());

    match a {
        Ok((_, b)) => {
            let c = parse2(b);
            match c {
                Ok(d) => {
                    match &d.detailed_language_blocks.js {
                        Some(_js) => {}
                        None => {
                            // Err("No js block")
                        }
                    }
                    let id = nanoid!();
                    let filename = format!("{}_{}.blvestruct", filenum, id);
                    std::fs::write(filename, format!("{:#?}", d)).unwrap();
                    return;
                }
                Err(e) => {
                    println!("{:?}", e);
                    return;
                }
            }
        }
        Err(_) => println!("err"),
    };
}
