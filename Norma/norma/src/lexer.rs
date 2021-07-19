use crate::token::*;
use regex::Regex;

pub fn generate_tokens(text: String) -> Vec<Token>{

    // vetor final de tokens
    let mut tokens = Vec::<Token>::new(); 
   
    // localização inicial
    let curr_location = Position { line: 1, column: 1 };

    // conteúdo do token, se for maior que um caracter
    let mut token_content = Vec::<char>::new();
    let mut token_type = TokenType::None;
    
    for character in text.chars() {

       if check_newline(character) {

           if token_type == TokenType::Comment || token_type == TokenType::SingleSlash {
                tokens.push(Token{
                    token_type: token_type,
                    content: token_content.iter().collect(),
                    position: Position {
                    line: curr_location.line,
                    column: curr_location.column
                    }
                });

                curr_location.update_for_newline();
           }

           token_type = TokenType::None;
           token_content = Vec::<char>::new();

       } else {

           curr_location.update_column();

           if token_type == TokenType::Comment || token_type == TokenType::SingleSlash {
                token_content.push(character);
           }

           if check_punctuation(character) {

                // confere se é keyword
                if token_type == TokenType::String {
                    if check_keyword(token_content.iter().collect()) {
                        token_type = match_keyword(token_content.iter().collect()).unwrap();
                    }
                }
               
                // termina o token de antes (string/número)
                tokens.push(Token{
                    token_type: token_type,
                    content: token_content.iter().collect(),
                    position: Position {
                       line: curr_location.line,
                       column: curr_location.column - 1
                    }
                });
              
                token_type = TokenType::None;
                token_content = Vec::<char>::new();

                match match_punctuation(character) {
                   Some(t) => { tokens.push(Token{
                       token_type: t,
                       content: String::from(character),
                       position: curr_location,
                   })},
                   None => { 
                       // não cria token, pois é caractere de ignorar
                   }
                }

           } else {

                token_content.push(character);

                if character.is_ascii_digit() {
                
                    if token_type == TokenType::None {
                        token_type = TokenType::Number;
                    }
    
                } else if character.is_alphabetic() {
    
                    if token_type == TokenType::Number || token_type == TokenType::None {
                        token_type = TokenType::String;
                    }
    
                } else if character.is_uppercase() {
    
                    if token_type == TokenType::None {
                        token_type = TokenType::Register;
                    }
    
                } else if check_comment(character) {
    
                    if token_type == TokenType::None {
                        token_type = TokenType::SingleSlash;
    
                    } else if token_type == TokenType::SingleSlash {
                        token_type = TokenType::Comment;
                    }

                } else {
                    
                    if character.is_ascii_whitespace() {
                        tokens.push(Token{
                            token_type: token_type,
                            content: token_content.iter().collect(),
                            position: curr_location
                        });
                    } else {

                    // o que fazer quando não é nenhum caractere previsto?
                    }
                }
           }
       }
    }

    return tokens;
}

fn check_newline(c: char) -> bool {
    return c == '\n';
}

fn check_keyword(word: String) -> bool {
    let mut keywords = vec!["do", "if", "then", "else", "goto", "main", "operation", "test"];
    keywords.sort();

    for kw in keywords {
        if word == kw {
            return true;
        }
    }

    return false;
}

fn match_keyword(word: String) -> Option<TokenType>{

    match word.as_str() {
        "do" => {return Some(TokenType::Do)},
        "else" => {return Some(TokenType::Else)},
        "goto" => {return Some(TokenType::Goto)},
        "if" => {return Some(TokenType::If)},
        "main" => {return Some(TokenType::Main)},
        "operation" => {return Some(TokenType::Operation)},
        "test" => {return Some(TokenType::Test)},
        "then" => {return Some(TokenType::Then)},
        _ => {return None}
    }
}

fn check_punctuation(c: char) -> bool {
    let rgx = Regex::new(r"[\s:;,\{\}\(\)]").unwrap();
    return rgx.is_match(&c.to_string());
}

fn match_punctuation(c: char) -> Option<TokenType> {

    match c {
        ':' => {return Some(TokenType::Colon);},
        ';' => {return Some(TokenType::Semicolon)},
        ',' => {return Some(TokenType::Comma)},
        '{' => {return Some(TokenType::OpenCurly)},
        '}' => {return Some(TokenType::CloseCurly)},
        '(' => {return Some(TokenType::OpenParen)},
        ')' => {return Some(TokenType::CloseParen)},
        _ => {return None}
    }
}

fn check_comment(c: char) -> bool {
    if c == '/' { return true; } else { return false; }
}