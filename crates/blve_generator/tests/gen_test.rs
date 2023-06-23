use std::{fs, vec};

use blve_generator::blve_compile_from_block;
use blve_parser::parse_blve_file;

fn main() -> Result<(), String> {
    // set ENV variable
    std::env::set_var("BLVE_TEST", "true");
    // get the file path from the command line
    let args: Vec<String> = std::env::args().collect();
    // if args.len() < 2 {
    let file_names = if args.len() > 1 {
        vec![args[1].clone()]
    } else {
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ]
    };
    for file_name in file_names {
        println!("./tests/cases/{}.blv", &file_name);
        let a = fs::read_to_string(format!("./tests/cases/{}.blv", &file_name)).unwrap();
        let b = parse_blve_file(a.as_str()).unwrap();
        let code = blve_compile_from_block(&b)?;
        println!("{}", code.0);
        fs::write(format!("./tests/cases/{}.js", file_name), code.0).unwrap();
    }
    Ok(())
}
