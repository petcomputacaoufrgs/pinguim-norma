use std::{fmt, str::FromStr};

use norma::{
    compiler::{
        self,
        error::{Diagnostics, Error},
        position::Span,
    },
    interpreter::{program::Program, Interpreter},
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

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
pub struct NumberParseError {
    pub message: String,
}

impl fmt::Display for NumberParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

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
    JsValue::from_serde(&errors).unwrap()
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
pub struct InterpreterStatus {
    pub registers: Vec<ExportableRegister>,
    #[serde(rename(serialize = "currentLabel", deserialize = "currentLabel"))]
    pub current_label: String,
    pub steps: String,
    pub running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterData {
    pub instructions: Vec<ExportableInstruction>,
    pub status: InterpreterStatus,
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

    fn running(&self) -> bool {
        let current_label = self.interpreter.current_label();
        self.interpreter.program().is_label_valid(current_label)
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
    #[wasm_bindgen(js_name = "data")]
    pub fn js_data(&self) -> JsValue {
        let data = InterpreterData {
            instructions: self.export_instructions(),
            status: self.export_status(self.running()),
        };
        JsValue::from_serde(&data).unwrap()
    }

    #[wasm_bindgen(js_name = "instructions")]
    pub fn js_instructions(&self) -> JsValue {
        JsValue::from_serde(&self.export_instructions()).unwrap()
    }

    #[wasm_bindgen(js_name = "input")]
    pub fn js_input(&mut self, value_text: &str) -> Result<(), JsValue> {
        match BigUint::from_str(&value_text) {
            Ok(value) => {
                self.interpreter.input(value);
                Ok(())
            },

            Err(error) => {
                let message = error.to_string();
                let exported_error = NumberParseError { message };
                Err(JsValue::from_serde(&exported_error).unwrap())
            },
        }
    }

    #[wasm_bindgen(js_name = "reset")]
    pub fn js_reset(&mut self) {
        self.interpreter.reset();
    }

    #[wasm_bindgen(js_name = "status")]
    pub fn js_status(&self) -> JsValue {
        let running = self.running();
        JsValue::from_serde(&self.export_status(running)).unwrap()
    }

    #[wasm_bindgen(js_name = "runStep")]
    pub fn js_run_step(&mut self) -> JsValue {
        let running = self.interpreter.run_step();
        JsValue::from_serde(&self.export_status(running)).unwrap()
    }

    #[wasm_bindgen(js_name = "runSteps")]
    pub fn js_run_steps(&mut self, max_steps: u32) -> JsValue {
        let running = self.interpreter.run_steps(max_steps);
        JsValue::from_serde(&self.export_status(running)).unwrap()
    }
}
