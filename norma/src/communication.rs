use crate::{
    compiler::{
        self,
        error::{Diagnostics, Error},
        position::Span,
    },
    interpreter::Interpreter,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

//
// - Checar erros de compilação.
//
// - Compilar código criando intepretador.
//
// - Obter instruções.
//
// - Obter registradores.
//
// - Executar passo do interpretador.
//
// - Executar passos do interpretador (e parar interpretador).
//
// - Resetar interpretador.

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableSpan {
    rendered: String,
    start: usize,
    end: usize,
}

impl ExportableSpan {
    pub fn new(span: Span) -> Self {
        Self {
            rendered: span.to_string(),
            start: span.start.index_utf16,
            end: span.end.index_utf16,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableError {
    message: String,
    span: Option<ExportableSpan>,
}

impl ExportableError {
    pub fn new(error: &Error) -> Self {
        Self {
            message: error.cause().to_string(),
            span: error.span().map(ExportableSpan::new),
        }
    }
}

fn export_diagnostics(diagnostics: &Diagnostics) -> JsValue {
    let errors: Vec<_> = diagnostics.iter().map(ExportableError::new).collect();
    JsValue::from_serde(errors)
}

#[wasm_bindgen]
pub fn check(source: &str) -> Result<(), JsValue> {
    match compiler::compile(source) {
        Ok(_) => Ok(()),

        Err(diagnostics) => Err(export_diagnostics(&diagnostics)),
    }
}

#[wasm_bindgen]
pub fn compile(source: &str) -> Result<InterpreterHandle, JsValue> {
    match compiler::compile(source) {
        Ok(interpreter) => Ok(InterpreterHandle::new(interpreter)),

        Err(diagnostics) => Err(export_diagnostics(&diagnostics)),
    }
}

#[wasm_bindgen]
pub struct InterpreterHandle {
    interpreter: Interpreter,
}

/*
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
extern "C" {}

/*
 * Estruturas a serem exportadas para o JS,
 * assim como possiveis implementações de métodos
 */

// Estrutura temporaria para debug
// Essa estrutura fará o papel temporariamente do Interpretador
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temp {
    pub a: usize,
}

// Estrutura para armazenar uma linha indexada
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedLine {
    label: String,
    line: String,
}

impl IndexedLine {
    pub fn new(label: String, line: String) -> Self {
        Self { label, line }
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
        Self { lines, interpreter }
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
*/
