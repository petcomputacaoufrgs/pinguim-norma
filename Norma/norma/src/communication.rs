use wasm_bindgen::prelude::*;
// use std::collections::HashMap;
// use num_bigint::BigUint;
// use serde::{Serialize, Deserialize};

/*
 * Para importar funções e estruturas padrões do JS pro Rust
 */
#[wasm_bindgen]
extern "C" {
    pub fn alert();
}

/*
 * Para importar funções e estrutura de um arquivo JS pro Rust
 */
#[wasm_bindgen(module = "/../communication.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn foo();
}

/*
 * Estruturas a serem exportadas para o JS,
 * assim como possiveis implementações de métodos
 */


/*
 * Funções a serem exportadas para o JS
 */