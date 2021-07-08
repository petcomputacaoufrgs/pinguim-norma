use wasm_bindgen::prelude::*;

mod norma;
mod communication;

/*
    Simulador de máquina Norma
    Criado pelo grupo PET Computação, baseado no modelo do
    professor Rodrigo Machado, para a matéria de
    Teoria da Computação
*/

#[wasm_bindgen(js_name = compile_text)]
pub fn compile(text: String) -> JsValue {
    //Tokeniza
    //Parseia
    //[...]
    //Retorna
    JsValue::from(text)
}

#[wasm_bindgen]
pub fn run_all() {

}

#[wasm_bindgen]
pub fn run_step() {

}