use crate::token::*;
use crate::instruction::*;
use std::ops::Range;
use std::collections::HashMap;

pub fn parse(tokens: Vec<Token>) {

    let mut last_token = TokenType::None;               // token anterior ao token atual do loop
    let mut instr: Instruction = Instruction::new();    // instrução a ser formada no loop por alguns tokens
    let mut key: String = String::new();                // chave da hashmap de macros, é sempre o nome da macro

    let mut is_op = false;      // flag para sinalizar que os tokens lidos dizem respeito a uma Operation
    let mut is_test = false;    // flag para sinalizar que os tokens lidos dizem respeito a um Test

    let mut open_curly_counter = 0;     // contador de "{" para balanceamento
    let mut close_curly_counter = 0;    // contador de "}" para balanceamento
    let mut open_paren_counter = 0;     // contador de "(" para balanceamento
    let mut close_paren_counter = 0;    // contador de ")" para balanceamento

    let mut operations_instr_aux = Vec::<Instruction>::new();   // instruções de uma determinada macro Operation (valor da hashmap)
    let mut tests_instr_aux = Vec::<Instruction>::new();        // instruções de uma determinada macro Test (valor da hashmap)

    let mut aux_tokens = tokens.clone();            // clone do array de tokens para não perder o original
    let macros_tokens = separe_macros(aux_tokens);  // lista de tokens relativos a parte das macros apenas


    for token in macros_tokens {

        if token.token_type == TokenType::Operation || token.token_type == TokenType::Test {
            last_token = token.token_type;
            is_op = token.token_type == TokenType::Operation; 
            is_test = token.token_type == TokenType::Test;
            key = String::new();
            
        } else {

            // read tokens

            if is_op {
                operations_instr_aux.push(instr);
                instr = Instruction::new();
            }

            if is_test {
                tests_instr_aux.push(instr);
                instr = Instruction::new();
            }

        }

    }  

    let mut main_instr = HashMap::<String, Instruction>::new();     // hashmap das instruções da main <rótulo, instrução>
    let mut main_counter = 0;                                       // contador para garantir que exista apenas uma main
    
    open_curly_counter = 0;     // contador de "{" para balanceamento
    close_curly_counter = 0;    // contador de "}" para balanceamento
    open_paren_counter = 0;     // contador de "(" para balanceamento
    close_paren_counter = 0;    // contador de ")" para balanceamento

    last_token = TokenType::None;   // token anterior ao token atual do loop
    instr = Instruction::new();     // instrução a ser formada no loop por alguns tokens
    key = String::new();            // chave da hashmap de main, é sempre o rótulo da instrução

    let mut label_true: String;                     // rótulo da próxima instrução (default e caso true) 
    let mut label_false: Option<String> = None;     // rótulo da próxima instrução (option e caso false)

    let main_tokens = separe_main(tokens);  // lista de tokens relativos a parte da main apenas

    for token in main_tokens {

        match token.token_type {

            TokenType::Main => {
                if main_counter == 0 && last_token == TokenType::None {
                    main_counter = main_counter + 1;
                    last_token = TokenType::Main;
                } else {
                    //panic: Não pode haver uma quantidade diferente de main!
                }
            },

            TokenType::OpenCurly => {
                if last_token == TokenType::Main {
                    if open_curly_counter == 0 {
                        open_curly_counter = open_curly_counter + 1;
                        last_token = TokenType::OpenCurly;
                    } else {
                        // panic: Parenteses desbalanceados!  
                    }
                } else {
                    //panic: Par de chaves usados em outro lugar que não ao redor da main function!
                }
            },

            TokenType::CloseCurly => {
                if close_curly_counter == 0 {
                    close_curly_counter = close_curly_counter + 1;
    
                    if open_curly_counter == 1 {
                        if last_token == TokenType::String || last_token == TokenType::Number {
                            last_token = TokenType::CloseCurly;
                            break;
                        }
                    } else {
                        // panic: Parenteses desbalanceados!
                    }
                } else {
                    //panic: Por enquanto só pode haver 1 par de chaves!
                }
            },

            TokenType::String => {
                match last_token {
                    TokenType::Goto => {
                        // string é um label
                        // não sei se é label_true ou se label_false
                    },
                    TokenType::Do => {
                        // string é uma operation ou built-in
                        // procurar a string na hashmap de macros (operations),
                        // se não encontrar, conferir funções built-in
                        // se não existir, panic: Macro não declarada
                    },
                    TokenType::If => {
                        // string é um teste ou built-in
                        // procurar a string na hashmap de macros (tests),
                        // se não encontrar, conferir funções built-in
                        // se não existir, panic: Macro não declarada
                    }, 
                    TokenType::Number => {
                        // então veio depois do rótulo da próxima linha da instrução anterior
                        // logo, é o label da nova instrução
                        // guardar e confirmar no próximo loop com ":"
                    },
                    TokenType::String => {
                        // então veio depois do rótulo da próxima linha da instrução anterior
                        // logo, é o label da nova instrução
                        // guardar e confirmar no próximo loop com ":"
                    }
                    _ => {
                        // panic: não faz sentido
                    }
                }

                last_token = TokenType::String;
                
            },

            TokenType::Number => {
                match last_token {
                    TokenType::Goto => {
                        // então é label!! mesmo caso de antes
                        // não sei se é label_true ou se label_false
                    },
                    TokenType::Register => {
                        // então é constante e pertence a uma operação
                        // adicionar constrante no campo correto da instancia de Instruction
                    },
                    TokenType::Number => {
                        // então veio depois do rótulo da próxima linha da instrução anterior
                        // logo, é o label da nova instrução
                        // guardar token.content em key e confirmar no próximo loop com ":"
                    },
                    TokenType::String => {
                        // então veio depois do rótulo da próxima linha da instrução anterior
                        // logo, é o label da nova instrução
                        // guardar token.content em key e confirmar no próximo loop com ":"
                    },
                    _ => {
                        // panic: Não faz sentido
                    }
                }

                last_token = TokenType::Number;
            },

            TokenType::Colon => {
                if last_token != TokenType::String || last_token != TokenType::Number {
                    // panic: Rótulo mal escrito
                }
            },

            TokenType::OpenParen => { 
                if last_token == TokenType::String {
                    if open_paren_counter == 0 {
                        open_paren_counter = open_paren_counter + 1;
                        last_token = TokenType::OpenParen;
                        
                        // conferir se está na hashmap de macros, se sim pegar as linhas relativas aquela macro
                        
                    } else {
                        //panic: Por enquanto só pode haver 1 par de chaves!
                    }
                }
            },

            TokenType::CloseParen => {
                if last_token == TokenType::String {
                    if open_paren_counter == 0 {
                        open_paren_counter = open_paren_counter + 1;
                        last_token = TokenType::OpenParen;
                        
                        // conferir se está na hashmap de macros, se sim pegar as linhas relativas aquela macro
                        // e fazer append na main, corrigindo os labels
                        // problema: correção dos labels :(
                        
                    } else {
                        //panic: Por enquanto só pode haver 1 par de chaves!
                    }
                }
            },

            TokenType::Register => {
                if last_token == TokenType::OpenParen || last_token == TokenType::Comma ||  // função com ()
                    last_token == TokenType::String || last_token == TokenType::Register {  // função sem ()
                    
                    // adiciona o registrador no vetor de registradores
                    // instr.add_register(token.content);
                    last_token = TokenType::Register;

                } else {    
                    // panic: Registrador em lugar errado
                }  
            },

            TokenType::Comma => {
                if last_token == TokenType::Register {
                    last_token = TokenType::Comma;
                } else {
                    // panic: Virgula no lugar errado
                }
            },

            TokenType::Do => {
                if last_token == TokenType::Colon {
                    last_token = TokenType::Do;
                } else {
                    // panic: Keyword no lugar errado
                }
            },

            TokenType::If => {
                if last_token == TokenType::Colon {
                    last_token = TokenType::If;
                } else {
                    // panic: Keyword no lugar errado
                }
            },

            TokenType::Then => {
                // testar se o que vem antes é um Test
            },

            TokenType::Else => {
                // testar se o que vem antes é uma String/Number
            },

            TokenType::Goto => {
                // 1. testar se o que vem antes é uma String/Register/Number -> endereço único
                // 2. testar se o que vem antes é um Then -> endereço verdadeiro
                // 3. testar se o que vem antes é um Else -> endereço falso
            }
            
            _ => {}
        }
    }

} 

fn separe_main(mut tokens: Vec<Token>) -> Vec<Token> {

    let range_tuple = get_range_main(&tokens);

    let range_main = Range {
        start: range_tuple.0,
        end: range_tuple.1 + 1,
    };

    return tokens.drain(range_main).collect();
}

fn separe_macros(mut tokens: Vec<Token>) -> Vec<Token> {

    let range_tuple = get_range_main(&tokens);

    let range_macros = Range {
        start: 0,
        end: range_tuple.0,
    };

    return tokens.drain(range_macros).collect();
}

fn get_range_main(tokens: &Vec<Token>) -> (usize, usize) {
    let mut index_start = 0;
    let mut index_end = 0;

    for i in 0..tokens.len() {
        if tokens[i].token_type == TokenType::Main {
            index_start = i;
        }
        if tokens[i].token_type == TokenType::CloseCurly {
            index_end = i;
        }
    }

    let tuple = (index_start, index_end);
    return tuple;
}
