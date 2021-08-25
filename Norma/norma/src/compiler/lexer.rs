#[cfg(test)]
mod test;

use super::{
    error::{BadCommentStart, BadRegister, Diagnostics, Error, InvalidChar},
    token::{Span, Token, TokenType},
};
use regex::Regex;
use std::{error::Error as StdError, mem};

#[derive(Debug, Clone)]
struct Lexer {
    tokens: Vec<Token>,
    is_curr_error: bool,
    curr_token: Token,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            is_curr_error: false,
            curr_token: Token {
                span: Span::default(),
                token_type: TokenType::None,
                content: String::new(),
            },
            tokens: Vec::new(),
        }
    }
}

impl Lexer {
    fn handle_char(&mut self, character: char, diagnostics: &mut Diagnostics) {
        if self.curr_token.token_type == TokenType::None {
            self.curr_token.span.finish();
        }

        let mut char_span = self.curr_token.span;
        char_span.finish();
        char_span.update(character);

        if self.curr_token.token_type != TokenType::Comment {
            if Self::check_comment(character) {
                self.handle_comment();
            } else if Self::check_punctuation(character) {
                self.handle_punctuation(character, char_span);
            } else {
                self.handle_default(character, char_span, diagnostics);
            }
        }

        self.curr_token.span.update(character);
        if Self::check_newline(character) {
            self.handle_newline();
        }
    }

    fn end_token(&mut self) {
        if self.curr_token.token_type == TokenType::String {
            self.handle_string();
        }

        if self.curr_token.token_type != TokenType::None {
            self.finish_token();
        }
    }

    fn handle_newline(&mut self) {
        if self.curr_token.token_type == TokenType::Comment {
            self.curr_token.token_type = TokenType::None;
        }

        if self.curr_token.token_type == TokenType::None {
            self.curr_token.span.finish();
        }
    }

    fn handle_comment(&mut self) {
        if self.curr_token.token_type == TokenType::None {
            self.curr_token.token_type = TokenType::SingleSlash;
        } else if self.curr_token.token_type == TokenType::SingleSlash {
            self.curr_token.token_type = TokenType::Comment;
        }
    }

    fn handle_punctuation(&mut self, character: char, char_span: Span) {
        // termina o token de antes (string/número)
        self.end_token();

        if let Some(typ) = Self::match_punctuation(character) {
            let prev_span = mem::replace(&mut self.curr_token.span, char_span);
            self.curr_token.content.push(character);
            self.curr_token.token_type = typ;
            self.finish_token();
            self.curr_token.span = prev_span;
        }

        self.curr_token.span.finish();
    }

    fn handle_string(&mut self) {
        if Self::check_keyword(&self.curr_token.content) {
            self.handle_keyword();
        } else if Self::check_builtin_func(&self.curr_token.content) {
            self.handle_builtin_func();
        }
    }

    fn handle_keyword(&mut self) {
        self.curr_token.token_type =
            Self::match_keyword(&self.curr_token.content).unwrap();
    }

    fn handle_builtin_func(&mut self) {
        self.curr_token.token_type =
            Self::match_builtin_func(&self.curr_token.content).unwrap();
    }

    fn handle_default(
        &mut self,
        character: char,
        char_span: Span,
        diagnostics: &mut Diagnostics,
    ) {
        if character.is_ascii_digit() {
            self.curr_token.content.push(character);
            self.handle_digit();
        } else if character.is_uppercase() {
            self.curr_token.content.push(character);
            self.handle_register();
        } else if character.is_alphabetic() {
            self.curr_token.content.push(character);
            self.handle_string_start(char_span, diagnostics);
        } else {
            self.handle_punctuation(character, char_span);
            self.raise(diagnostics, InvalidChar { character }, char_span);
        }
    }

    fn handle_digit(&mut self) {
        if self.curr_token.token_type == TokenType::None {
            self.curr_token.token_type = TokenType::Number;
        }
    }

    fn handle_register(&mut self) {
        if self.curr_token.token_type == TokenType::None {
            self.curr_token.token_type = TokenType::Register;
        }
    }

    fn handle_string_start(
        &mut self,
        char_span: Span,
        diagnostics: &mut Diagnostics,
    ) {
        match self.curr_token.token_type {
            TokenType::None => self.curr_token.token_type = TokenType::String,
            TokenType::Number => self.curr_token.token_type = TokenType::String,
            TokenType::SingleSlash => {
                self.curr_token.content.clear();
                self.raise(diagnostics, BadCommentStart, char_span)
            },
            TokenType::Register if !self.is_curr_error => {
                let reg_name = self.curr_token.content.clone();
                let span = Span {
                    start: self.curr_token.span.start,
                    end: char_span.end,
                };
                self.raise(diagnostics, BadRegister { reg_name }, span);
            },
            _ => (),
        }
    }

    fn finish_token(&mut self) {
        let new_token = Token {
            token_type: TokenType::None,
            content: String::new(),
            span: self.curr_token.span,
        };
        let token = mem::replace(&mut self.curr_token, new_token);
        self.tokens.push(token);
        self.curr_token.span.finish();
        self.is_curr_error = false;
    }

    fn raise<E>(&mut self, diagnostics: &mut Diagnostics, cause: E, span: Span)
    where
        E: StdError + Send + Sync + 'static,
    {
        diagnostics.raise(Error::new(cause, span));
        self.is_curr_error = true;
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

    fn check_keyword(word: &str) -> bool {
        let keywords =
            ["do", "if", "then", "else", "goto", "main", "operation", "test"];
        keywords.contains(&word)
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

    fn match_keyword(word: &str) -> Option<TokenType> {
        match word {
            "do" => Some(TokenType::Do),
            "else" => Some(TokenType::Else),
            "goto" => Some(TokenType::Goto),
            "if" => Some(TokenType::If),
            "main" => Some(TokenType::Main),
            "operation" => Some(TokenType::Operation),
            "test" => Some(TokenType::Test),
            "then" => Some(TokenType::Then),
            _ => None,
        }
    }

    fn check_builtin_func(func: &str) -> bool {
        let builtin_func = ["inc", "dec", "add", "sub", "cmp", "zero"];
        builtin_func.contains(&func)
    }

    fn match_builtin_func(func: &str) -> Option<TokenType> {
        match func {
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

pub fn generate_tokens(
    source: &str,
    diagnostics: &mut Diagnostics,
) -> Vec<Token> {
    let mut lexer = Lexer::default();
    for character in source.chars() {
        lexer.handle_char(character, diagnostics);
    }
    lexer.end_token();
    lexer.tokens
}
