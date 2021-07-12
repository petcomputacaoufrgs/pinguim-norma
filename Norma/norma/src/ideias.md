# Estrutura Final

**Nome**: Data Exporter
*deve ter*: Getter e Setter de todas os elementos

instruções: JsValue (aka.: JSON)
Machine: Temp (só um ponteiro para o Javascript)


``` rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use communication::*;

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

#[wasm_bindgen(js_name = compileText)]
pub fn compile(text: String) -> DataExporter {
    //Importa texto
    //Tokeniza
    //Parseia
    //[...]
    //Retorna (Por enquanto retorna um Mock)
    let line = IndexedLine::from("1:", "do inc A goto 2");
    let lines = IndexedLineList::new(vec!{line});
    let interpreter = Temp {a: 2021};
    //DataExporter::from(lines, interpreter)
}

#[wasm_bindgen]
pub fn run_all() {

}

#[wasm_bindgen]
pub fn run_step() {

}
```

```rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use communication::*;

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

#[wasm_bindgen(js_name = compileText)]
pub fn compile(text: String) -> DataExporter {
    //Importa texto
    //Tokeniza
    //Parseia
    //[...]
    //Retorna (Por enquanto retorna um Mock)
    let line = IndexedLine::from("1:", "do inc A goto 2");
    let lines = IndexedLineList::new(vec!{line});
    let interpreter = Temp {a: 2021};
    //DataExporter::from(lines, interpreter)
}

#[wasm_bindgen]
pub fn run_all() {

}

#[wasm_bindgen]
pub fn run_step() {

}
```