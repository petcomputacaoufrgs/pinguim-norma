#[cfg(test)]
mod test;

use super::token::*;
use regex::Regex;

pub fn generate_tokens(text: String) -> Vec<Token> {
    // vetor final de tokens
    let mut tokens = Vec::<Token>::new();

    // localização inicial
    let mut curr_location = Position { line: 1, column: 1 };
    let mut next_location = curr_location;

    // conteúdo do token, se for maior que um caracter
    let mut token_content = Vec::<char>::new();
    let mut token_type = TokenType::None;

    for character in text.chars() {

        if check_newline(character) {
            next_location.update_for_newline();

            // newline marca o fim de um comentário
            if token_type == TokenType::Comment {
                token_type = TokenType::None;
            }

            if token_type == TokenType::None {
                curr_location = next_location;
            }
        } else {
            if token_type == TokenType::None {
                curr_location = next_location;
            }
            let punct_location = next_location;
            
            next_location.update_column();

            // ignorar tudo que for lido se for um comentário
            if token_type == TokenType::Comment {
                continue;
            } else if check_comment(character) {
                if token_type == TokenType::None {
                    token_type = TokenType::SingleSlash;
                } else if token_type == TokenType::SingleSlash {
                    token_type = TokenType::Comment;
                }
            } else if check_punctuation(character) {
                // acho que da pra fazer melhor esse bloco
                if token_type == TokenType::String {
                    if check_keyword(token_content.iter().collect()) {
                        token_type =
                            match_keyword(token_content.iter().collect())
                                .unwrap();
                    } else if check_builtin_func(token_content.iter().collect())
                    {
                        token_type =
                            match_builtin_func(token_content.iter().collect())
                                .unwrap();
                    }
                }

                // termina o token de antes (string/número)
                if token_type != TokenType::None {
                    tokens.push(Token {
                        token_type,
                        content: token_content.iter().collect(),
                        position: curr_location,
                    });
                }

                token_content = Vec::<char>::new();
                token_type = TokenType::None;

                if let Some(t) = match_punctuation(character) {
                    tokens.push(Token {
                        token_type: t,
                        content: String::from(character),
                        position: punct_location,
                    });
                }

                curr_location = next_location;
            } else {
                token_content.push(character);

                if character.is_ascii_digit() {
                    if token_type == TokenType::None {
                        token_type = TokenType::Number;
                    }
                } else if character.is_uppercase() {
                    if token_type == TokenType::None {
                        token_type = TokenType::Register;
                    }
                } else if character.is_alphabetic() {
                    match token_type {
                        TokenType::None => token_type = TokenType::String,
                        TokenType::Number => token_type = TokenType::String,
                        TokenType::SingleSlash => {
                            panic!(
                                "Comment: Sintax error at {:?}",
                                curr_location
                            )
                        },
                        TokenType::Register => {
                            panic!(
                                "Register: Sintax error at {:?}",
                                curr_location
                            )
                        },
                        _ => {
                            continue;
                        },
                    }
                } else {
                    panic!(
                        "Invalid Character: Sintax error at {:?}",
                        curr_location
                    );
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
    let mut keywords =
        vec!["do", "if", "then", "else", "goto", "main", "operation", "test"];
    keywords.sort();

    for kw in keywords {
        if word == kw {
            return true;
        }
    }

    return false;
}

fn match_keyword(word: String) -> Option<TokenType> {
    match word.as_str() {
        "do" => return Some(TokenType::Do),
        "else" => return Some(TokenType::Else),
        "goto" => return Some(TokenType::Goto),
        "if" => return Some(TokenType::If),
        "main" => return Some(TokenType::Main),
        "operation" => return Some(TokenType::Operation),
        "test" => return Some(TokenType::Test),
        "then" => return Some(TokenType::Then),
        _ => return None,
    }
}

fn check_builtin_func(func: String) -> bool {
    let mut builtin_func = vec!["inc", "dec", "add", "sub", "cmp", "zero"];
    builtin_func.sort();

    for bf in builtin_func {
        if func == bf {
            return true;
        }
    }

    return false;
}

fn match_builtin_func(func: String) -> Option<TokenType> {
    match func.as_str() {
        "add" => return Some(TokenType::Add),
        "sub" => return Some(TokenType::Sub),
        "cmp" => return Some(TokenType::Cmp),
        "zero" => return Some(TokenType::Zero),
        "inc" => return Some(TokenType::Inc),
        "dec" => return Some(TokenType::Dec),
        _ => return None,
    }
}

fn check_punctuation(c: char) -> bool {
    let rgx = Regex::new(r"[\s:;,\{\}\(\)]").unwrap();
    return rgx.is_match(&c.to_string());
}

fn match_punctuation(c: char) -> Option<TokenType> {
    match c {
        ' ' => {return None},
        ':' => {return Some(TokenType::Colon);},
        ',' => {return Some(TokenType::Comma)},
        '{' => {return Some(TokenType::OpenCurly)},
        '}' => {return Some(TokenType::CloseCurly)},
        '(' => {return Some(TokenType::OpenParen)},
        ')' => {return Some(TokenType::CloseParen)},
        _ => {return None}
    }
}

fn check_comment(c: char) -> bool {
    if c == '/' {
        return true;
    } else {
        return false;
    }
}

