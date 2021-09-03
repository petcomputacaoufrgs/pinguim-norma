use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::compiler::instruction::*;

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

    pub fn from_instruction(label: String, instruction: Instruction) -> Self {
        Self { label, line: instruction.to_string() }
    }
}


// Estrutura para exportar valores ao JS
#[wasm_bindgen]
pub struct DataExporter {
    lines: Vec<IndexedLine>,
    interpreter: Temp,
}

impl DataExporter {
    pub fn new(lines: Vec<IndexedLine>, interpreter: Temp) -> Self {
        Self {
            lines,
            interpreter
        }
    }
}

#[wasm_bindgen]
impl DataExporter {

    // Exporta Linhas de código como 
    #[wasm_bindgen(js_name = getLines)]
    pub fn lines_as_json(&self) -> JsValue {
        JsValue::from_serde(&self.lines).unwrap()
    }

    // Implementação futura: retornar o valor do interpretador
    #[wasm_bindgen]
    pub fn get_result(&mut self) -> String {
        String::from("2021")
    }

    // Start machine
    // #[wasm_bindgen(js_name = startMachine)]
    // pub fn start_machine(&mut self, input: String) {
    //     self.interpreter.reset();
    //     self.interpreter.input(input);
    // }
}