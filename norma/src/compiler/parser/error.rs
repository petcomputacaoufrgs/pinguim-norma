use crate::compiler::lexer::token::TokenType;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
/// Erro em que a função principal main é declarada mais de uma vez no código
pub struct MainAlreadyDeclared;

impl fmt::Display for MainAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Main já foi declarada neste programa")
    }
}

impl Error for MainAlreadyDeclared {}

#[derive(Clone, Debug)]
/// Erro em que a função principal main não é declarada nenhuma vez no código
pub struct MainNotDeclared;

impl fmt::Display for MainNotDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Main não foi declarada neste programa")
    }
}

impl Error for MainNotDeclared {}

#[derive(Clone, Debug)]
/// Erro em que determinada macro já foi declarada (com mesmo nome)
pub struct MacroAlreadyDeclared {
    ///
    /// - `macro_name`: nome da macro que foi encontrado mais de uma vez no código como declaração
    pub macro_name: String,
}

impl fmt::Display for MacroAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" já foi declarada neste programa",
            self.macro_name
        )
    }
}

impl Error for MacroAlreadyDeclared {}

#[derive(Clone, Debug)]
/// Erro em que dois ou mais rótulos tem o mesmo nome
pub struct LabelAlreadyDeclared {
    ///
    /// - `label_name`: nome do rótulo repetido
    pub label_name: String,
}

impl fmt::Display for LabelAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" já foi declarada neste programa",
            self.label_name
        )
    }
}

impl Error for LabelAlreadyDeclared {}

#[derive(Clone, Debug)]
/// Erro em que o token lido não é de um tipo esperado
pub struct UnexpectedToken {
    ///
    /// - `expected_types`: vetor com todos os tipos de tokens que poderiam aparecer nessa posição
    pub expected_types: Vec<TokenType>,
}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Token inesperado encontrado, esperava-se um ")?;

        let count = self.expected_types.len();

        if count > 1 {
            for expected_type in &self.expected_types[.. count - 2] {
                write!(formatter, "\"{}\", ", expected_type)?;
            }

            write!(formatter, "\"{}\" ou ", self.expected_types[count - 2])?;
        }

        if count > 0 {
            write!(formatter, "\"{}\"", self.expected_types[count - 1])?;
        }

        Ok(())
    }
}

impl Error for UnexpectedToken {}

#[derive(Clone, Debug)]
/// Erro em que esperava-se algum token, porém o vetor de tokens chegou ao fim
pub struct UnexpectedEndOfInput;

impl fmt::Display for UnexpectedEndOfInput {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Fim inesperado do código")
    }
}

impl Error for UnexpectedEndOfInput {}

#[derive(Clone, Debug)]
/// Erro em que o nome do label é inválido
pub struct InvalidLabel;

impl fmt::Display for InvalidLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Nome de label não pode ser \"true\" nem \"false\"")
    }
}

impl Error for InvalidLabel {}
