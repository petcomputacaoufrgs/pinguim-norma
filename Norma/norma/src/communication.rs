use wasm_bindgen::prelude::*;
// use std::collections::HashMap;
// use num_bigint::BigUint;
use serde::{Serialize, Deserialize};
use crate::norma::Machine;

/*
 * Para importar funções e estruturas padrões do JS pro Rust
 */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: JsValue);
}

/*
 * Para importar funções e estrutura de um arquivo JS pro Rust
 */
#[wasm_bindgen(module = "/../communication.js")]
extern "C" {

}

/*
 * Estruturas a serem exportadas para o JS,
 * assim como possiveis implementações de métodos
 */

// Estrutura temporaria para debug
// Essa estrutura fará o papel temporariamente do Interpretador
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temp {
    pub a: usize
}

// Estrutura para armazenar uma linha indexada
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedLine {
    label: String,
    line: String
}

impl IndexedLine {
    pub fn new(label: String, line: String) -> Self {
        Self {label, line}
    }

    pub fn from(label: &str, line: &str) -> Self {
        Self {label: String::from(label), line: String::from(line)}
    }
}



// Estrutura para exportar vetor de linhas indexadas
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  IndexedLineList {
    lines: Vec<IndexedLine>,
}

impl IndexedLineList {
    pub fn new(lines: Vec<IndexedLine>) -> Self {
        Self { lines }
    }
}


// Estrutura para exportar valores ao JS
#[wasm_bindgen]
pub struct DataExporter {
    lines: JsValue,
    interpreter: Machine,
}

impl DataExporter {
    pub fn new(lines: JsValue, interpreter: Machine) -> Self {
        Self {
            lines,
            interpreter
        }
    }

    pub fn from(lines: IndexedLineList, interpreter: Machine) -> Self {
        Self {
            lines: JsValue::from_serde(&lines).unwrap(),
            interpreter
        }
    }
}

#[wasm_bindgen]
impl DataExporter {

    #[wasm_bindgen(js_name = getLines)]
    pub fn lines(&self) -> JsValue {
        self.lines.clone()
    }

    #[wasm_bindgen]
    pub fn get_result(&mut self) -> String {
        self.interpreter.inc("X");
        self.interpreter.get_value("X").to_str_radix(10)
    }
}

/*
 * Funções a serem exportadas para o JS
 */