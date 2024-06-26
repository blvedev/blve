use blve_generator::blve_compile_from_block;
use blve_parser::parse_blve_file;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BlveCompilerOutput {
    js: String,
    css: Option<String>,
}

#[wasm_bindgen]
impl BlveCompilerOutput {
    #[wasm_bindgen(getter)]
    pub fn js(&self) -> String {
        self.js.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn css(&self) -> Option<String> {
        match self.css {
            Some(ref s) => Some(s.clone()),
            None => None,
        }
    }
}

#[wasm_bindgen]
pub fn compile(
    blve_code: String,
    no_export: Option<bool>,
    export_name: Option<String>,
    runtime_path: Option<String>,
) -> Result<BlveCompilerOutput, String> {
    let blocks = match parse_blve_file(&blve_code) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }?;
    let code = blve_compile_from_block(&blocks, no_export, export_name, runtime_path)?;
    Ok(BlveCompilerOutput {
        js: code.0,
        css: code.1,
    })
}
