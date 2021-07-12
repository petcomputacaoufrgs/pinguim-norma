use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use communication::*;
use norma::Machine;
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};

// Core da máquina norma
mod norma;
// Módulo de comunicação com o frontend
mod communication;

/*
    Simulador de máquina Norma
    Criado pelo grupo PET Computação, baseado no modelo do
    professor Rodrigo Machado, para a matéria de
    Teoria da Computação
*/

/*
    TODO: aprender a atualizar a estrutura no front
*/

#[wasm_bindgen(js_name = compileText)]
pub fn compile(text: String) -> DataExporter {
    //Importa texto
    //Tokeniza
    //Parseia
    //[...]
    //Retorna (Por enquanto retorna um Mock)
    let line = IndexedLine::from("1.a", "do inc A goto 1.b");
    let lines = IndexedLineList::new(vec!{line});
    DataExporter::from(lines, Machine::new(BigUint::one()))
}

#[wasm_bindgen]
pub fn run_all() {

}

#[wasm_bindgen]
pub fn run_step() {

}