#[cfg(test)]
mod test;

use super::token::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct Lexer {
    curr_position: Position,
    next_position: Position,
    tokens: Vec<Token>,
    token_type: TokenType,
    token_content: String,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            curr_position: Position { line: 1, column: 1 },
            next_position: Position { line: 1, column: 1 },
            tokens: Vec::new(),
            token_type: TokenType::None,
            token_content: String::new(),
        }
    }
}

impl Lexer {
    fn handle_char(&mut self, character: char) {
        if Self::check_newline(character) {
            self.handle_newline();
        } else {
            if self.token_type == TokenType::None {
                self.curr_position = self.next_position;
            }
            let punct_position = self.next_position;
            self.next_position.update_column();

            if self.token_type != TokenType::Comment {
                if Self::check_comment(character) {
                    self.handle_comment();
                } else if Self::check_punctuation(character) {
                    self.handle_punctuation(character, punct_position);
                } else {
                    self.handle_default(character);
                }
            }
        }
    }

    fn handle_newline(&mut self) {
        self.next_position.update_for_newline();

        if self.token_type == TokenType::Comment {
            self.token_type = TokenType::None;
        }

        if self.token_type == TokenType::None {
            self.curr_position = self.next_position;
        }
    }

    fn handle_comment(&mut self) {
        if self.token_type == TokenType::None {
            self.token_type = TokenType::SingleSlash;
        } else if self.token_type == TokenType::SingleSlash {
            self.token_type = TokenType::Comment;
        }
    }

    fn handle_punctuation(
        &mut self,
        character: char,
        punct_position: Position,
    ) {
        if self.token_type == TokenType::String {
            self.handle_string();
        }

        // termina o token de antes (string/número)
        if self.token_type != TokenType::None {
            self.finish_token();
        }

        if let Some(typ) = Self::match_punctuation(character) {
            self.curr_position = punct_position;
            self.token_content.push(character);
            self.token_type = typ;
            self.finish_token();
        }

        self.curr_position = self.next_position;
    }

    fn handle_string(&mut self) {
        if Self::check_keyword(self.token_content.clone()) {
            self.handle_keyword();
        } else if Self::check_builtin_func(self.token_content.clone()) {
            self.handle_builtin_func();
        }
    }

    fn handle_keyword(&mut self) {
        self.token_type =
            Self::match_keyword(self.token_content.clone()).unwrap();
    }

    fn handle_builtin_func(&mut self) {
        self.token_type =
            Self::match_builtin_func(self.token_content.clone()).unwrap();
    }

    fn handle_default(&mut self, character: char) {
        self.token_content.push(character);

        if character.is_ascii_digit() {
            self.handle_digit();
        } else if character.is_uppercase() {
            self.handle_register();
        } else if character.is_alphabetic() {
            self.handle_string_start();
        } else {
            panic!(
                "Invalid Character: Sintax error at {:?}",
                self.curr_position
            )
        }
    }

    fn handle_digit(&mut self) {
        if self.token_type == TokenType::None {
            self.token_type = TokenType::Number;
        }
    }

    fn handle_register(&mut self) {
        if self.token_type == TokenType::None {
            self.token_type = TokenType::Register;
        }
    }

    fn handle_string_start(&mut self) {
        match self.token_type {
            TokenType::None => self.token_type = TokenType::String,
            TokenType::Number => self.token_type = TokenType::String,
            TokenType::SingleSlash => {
                panic!("Comment: Sintax error at {:?}", self.curr_position)
            },
            TokenType::Register => {
                panic!("Register: Sintax error at {:?}", self.curr_position)
            },
            _ => (),
        }
    }

    fn finish_token(&mut self) {
        let token = Token {
            token_type: self.token_type,
            content: self.token_content.clone(),
            position: self.curr_position,
        };
        self.tokens.push(token);
        self.token_content.clear();
        self.token_type = TokenType::None;
    }

    fn check_newline(character: char) -> bool {
        character == '\n'
    }

    fn check_comment(character: char) -> bool {
        character == '/'
    }

    fn check_punctuation(character: char) -> bool {
        let rgx = Regex::new(r"[\s:;,\{\}\(\)]").unwrap();
        rgx.is_match(&character.to_string())
    }

    fn check_keyword(word: String) -> bool {
        let keywords =
            ["do", "if", "then", "else", "goto", "main", "operation", "test"];
        keywords.contains(&word.as_str())
    }

    fn match_punctuation(character: char) -> Option<TokenType> {
        match character {
            ' ' => None,
            ':' => Some(TokenType::Colon),
            ',' => Some(TokenType::Comma),
            '{' => Some(TokenType::OpenCurly),
            '}' => Some(TokenType::CloseCurly),
            '(' => Some(TokenType::OpenParen),
            ')' => Some(TokenType::CloseParen),
            _ => None,
        }
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
        let builtin_func = ["inc", "dec", "add", "sub", "cmp", "zero"];
        builtin_func.contains(&func.as_str())
    }

    fn match_builtin_func(func: String) -> Option<TokenType> {
        match func.as_str() {
            "add" => Some(TokenType::Add),
            "sub" => Some(TokenType::Sub),
            "cmp" => Some(TokenType::Cmp),
            "zero" => Some(TokenType::Zero),
            "inc" => Some(TokenType::Inc),
            "dec" => Some(TokenType::Dec),
            _ => None,
        }
    }
}

pub fn generate_tokens(text: String) -> Vec<Token> {
    let mut lexer = Lexer::default();
    for character in text.chars() {
        lexer.handle_char(character);
    }
    lexer.tokens
}
