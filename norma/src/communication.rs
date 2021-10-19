use crate::{
    compiler::{
        self,
        error::{Diagnostics, Error},
        position::Span,
    },
    interpreter::{program::Program, Interpreter},
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableSpan {
    pub rendered: String,
    pub start: usize,
    pub end: usize,
}

impl ExportableSpan {
    pub fn new(span: Span) -> Self {
        Self {
            rendered: span.to_string(),
            start: span.start.utf16_index,
            end: span.end.utf16_index,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableError {
    pub message: String,
    pub span: Option<ExportableSpan>,
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
    JsValue::from_serde(&errors).unwrap_or(JsValue::null())
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
        Ok(program) => Ok(InterpreterHandle::new(program)),

        Err(diagnostics) => Err(export_diagnostics(&diagnostics)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableRegister {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableInstruction {
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableProgram {
    pub instructions: Vec<ExportableInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterStatus {
    pub registers: Vec<ExportableRegister>,
    pub current_label: String,
    pub steps: String,
    pub running: bool,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct InterpreterHandle {
    interpreter: Interpreter,
}

impl InterpreterHandle {
    pub fn new(program: Program) -> Self {
        Self { interpreter: Interpreter::new(program) }
    }

    fn export_status(&self, running: bool) -> InterpreterStatus {
        InterpreterStatus {
            registers: self.export_registers(),
            current_label: self.interpreter.current_label().to_string(),
            steps: self.interpreter.steps().to_string(),
            running,
        }
    }

    fn export_registers(&self) -> Vec<ExportableRegister> {
        let mut registers = Vec::new();
        for name in self.interpreter.machine().register_names() {
            registers.push(ExportableRegister {
                name: name.to_owned(),
                value: self.interpreter.machine().get_value(name).to_string(),
            });
        }
        registers
    }

    fn export_instructions(&self) -> Vec<ExportableInstruction> {
        let mut instructions = Vec::new();
        for instruction in self.interpreter.program().instructions() {
            instructions.push(ExportableInstruction {
                label: instruction.label().to_owned(),
                kind: instruction.kind.to_string(),
            });
        }
        instructions
    }
}

#[wasm_bindgen]
impl InterpreterHandle {
    #[wasm_bindgen(js_name = "instructions")]
    pub fn js_instructions() -> JsValue {
        JsValue::from_serde(&self.export_instructions()).unwrap()
    }

    #[wasm_bindgen(js_name = "status")]
    pub fn js_status() -> JsValue {
        JsValue::from_serde(&self.export_status(false)).unwrap()
    }

    #[wasm_bindgen(js_name = "runStep")]
    pub fn js_run_step(&mut self) -> JsValue {
        let running = self.interpreter.run_step();
        JsValue::from_serde(&self.export_status(running)).unwrap()
    }

    #[wasm_bindgen(js_name = "runSteps")]
    pub fn js_run_steps(&mut self, max_steps: u64) -> JsValue {
        let running = self.interpreter.run_steps(max_steps);
        JsValue::from_serde(&self.export_status(running)).unwrap()
    }
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
