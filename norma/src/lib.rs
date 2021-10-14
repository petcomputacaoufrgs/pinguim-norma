use crate::interpreter::program::*;
use communication::*;
use wasm_bindgen::prelude::*;

// Core da máquina norma
mod machine;
// Módulo de comunicação com o frontend
mod communication;
// Módulo do compilador
mod compiler;
mod interpreter;

/*
    Simulador de máquina Norma
    Criado pelo grupo PET Computação, baseado no modelo do
    professor Rodrigo Machado, para a matéria de
    Teoria da Computação
*/

/*
#[wasm_bindgen(js_name = compileText)]
pub fn compile(text: String) -> DataExporter {
    //Importa texto
    //Tokeniza
    //Parseia
    //[...]
    //Retorna (Por enquanto retorna um Mock)
    DataExporter::new(lines_mock(), Temp { a: 0 })
}

//Chama a função run_all do interpretador
#[wasm_bindgen]
pub fn run_all() {}

//Chama a função run_step do interpretador
#[wasm_bindgen]
pub fn run_step() {}

/**
 * TODO: Atualizar para o novo formato de instrução
 * baseado no novo interpretador;
 */
fn lines_mock() -> Vec<IndexedLine> {
    let mut i = Instruction::new();
    i.set_type(InstructionType::Test(TestType::CmpConst));
    i.set_registers(vec![String::from("A")]);
    i.set_constant(5);
    i.set_next_instructions(
        String::from("1.add.fim"),
        Some(String::from("1.add.fim")),
    );

    let mut j = Instruction::new();
    j.set_type(InstructionType::Operation(OperationType::AddRegs));
    j.set_registers(vec![String::from("Y"), String::from("B")]);
    j.set_next_instructions(String::from("1.add.2"), None);

    let line1 = IndexedLine::from_instruction(String::from("1.add.1."), i);
    let line2 = IndexedLine::from_instruction(String::from("1.add.2."), j);

    vec![line1, line2]
}
*/
