use wasm_bindgen::prelude::*;
// use std::collections::HashMap;
// use num_bigint::BigUint;
use serde::{Serialize, Deserialize};



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
#[derive(Debug, Clone)]
pub struct DataExporter {
    lines: JsValue,
    interpreter: *mut Temp,
}

impl DataExporter {
    pub fn new(lines: JsValue, interpreter: &mut Temp) -> Self {
        Self {
            lines,
            interpreter
        }
    }

    pub fn from(lines: IndexedLineList, interpreter: Temp) -> Self {
        Self {
            lines: JsValue::from_serde(&lines).unwrap(),
            interpreter: &mut interpreter.clone()
        }
    }
}

#[wasm_bindgen]
impl DataExporter {

    #[wasm_bindgen(js_name = getLines)]
    pub fn lines(&self) -> JsValue {
        self.lines.clone()
    }

    #[wasm_bindgen(js_name = getInterpreter)]
    pub fn interpreter(&self) -> *mut Temp {
        self.interpreter
    }
}

/*
 * Funções a serem exportadas para o JS
 */

#[wasm_bindgen(js_name=getValue)]
pub fn return_value_from_temp(t: &mut Temp) -> usize {
    t.a
}