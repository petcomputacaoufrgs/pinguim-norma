mod lexer;
mod token;
mod instruction;
mod parser;

use crate::lexer::*;
use crate::token::*;
use crate::instruction::*;
use crate::parser::*;

fn main() {
    let code = String::from("// A:=B (atribuição destrutiva)
    operation load(A,B,C){
      1: do clear(A) goto 2
      A2A: do soma(A,B,C) goto 0
    }
    main {
      1: do inc A goto 4
      2: do load(A, B, Y) goto 3
      4: 
    }");

    let tokens = generate_tokens(code);    

    // println!("{:?}", separe_macros(tokens));

}

// não guarda os registradores da macro, apenas o nome delas
// tem que achar um jeito de guardar os rotulos das instrucoes (indice da lista ou guardar na propria struct)
// label da main + soma numero + proprio nome da macro
// retorna um hasmap pro front com Instruction com metodo de to_string()