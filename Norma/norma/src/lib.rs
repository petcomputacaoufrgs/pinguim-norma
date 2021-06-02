use wasm_bindgen::prelude::*;

mod norma;

// Import javascript functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}