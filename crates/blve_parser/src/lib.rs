mod parse2;
mod parser1;
mod parsers;
mod structs;
mod swc_parser;

use parse2::parse2;
use parser1::parse1;
pub use structs::detailed_blocks::DetailedBlock;
pub use structs::detailed_meta_data::{DetailedMetaData, UseComponentStatement};

pub fn parse_blve_file(input: &str) -> Result<DetailedBlock, String> {
    let parsed_items = match parse1(input) {
        Ok(r) => {
            let (_, parsed_items) = r;
            Ok(parsed_items)
        }
        Err(e) => Err(e.to_string()),
    }?;

    let detailed_block = match parse2(parsed_items) {
        Ok(r) => r,
        Err(e) => return Err(format!("{:?}", e)),
    };

    Ok(detailed_block)
}
