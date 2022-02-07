use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    ///
    /// - `utf8_index`: índice do caractere na string em utf8 para rust
    pub utf8_index: usize,
    ///
    /// - `utf16_index`: índice do caractere na string em utf16 para JS
    pub utf16_index: usize,
    ///
    /// - `line`: linha do caractere
    pub line: u64,
    ///
    /// - `column`: coluna do caractere
    pub column: u64,
}

/// Implementa trait Default para criar uma estrutura Position com os atributos como se o caractere estivesse na primeira posição
impl Default for Position {
    fn default() -> Self {
        Position { utf8_index: 0, utf16_index: 0, line: 1, column: 1 }
    }
}

impl Position {
    /// Atualiza nova linha
    fn update_newline(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    /// Atualiza nova coluna
    fn update_column(&mut self) {
        self.column += 1;
    }

    /// Atualiza índices conforme caractere
    ///
    /// - `character`: novo caractere lido
    fn update_indices(&mut self, character: char) {
        self.utf8_index += character.len_utf8();
        self.utf16_index += character.len_utf16();
    }

    /// Atualiza nova linha e nova coluna de acordo com o caractere lido
    ///
    /// - `character`: novo caractere lido
    pub fn update(&mut self, character: char) {
        self.update_indices(character);
        if character == '\n' {
            self.update_newline();
        } else {
            self.update_column()
        }
    }
}

/// Implementa trait display para a parte de posição em mensagens de erro
impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "linha {} e coluna {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    ///
    /// - `start`: posição inicial do span (inclusivo)
    pub start: Position,
    ///
    /// - `end`: posição final do span (exclusivo)
    pub end: Position,
}

/// Implementa trait Default para criar um Span com começo e fim no mesmo lugar
impl Default for Span {
    fn default() -> Self {
        Self::from_start(Position::default())
    }
}

impl Span {
    /// Cria uma estrutura Span com começo e fim no mesmo lugar
    ///
    /// - `start`: posição inicial do caractere
    pub fn from_start(start: Position) -> Self {
        Self { start, end: start }
    }

    /// Atualiza posição final do símbolo
    ///
    /// - `character`:  novo caractere lido
    pub fn update(&mut self, character: char) {
        self.end.update(character);
    }

    /// Finaliza Span voltando para o default
    pub fn finish(&mut self) {
        self.start = self.end;
    }
}

// Implementa trait display para a parte de span em mensagens de erro
impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let end = Position { column: self.end.column - 1, ..self.end };

        if self.start.line != self.end.line {
            write!(formatter, "de {}, até {}", self.start, end)
        } else if self.start.column + 1 == self.end.column {
            write!(formatter, "na {}", self.start)
        } else {
            write!(formatter, "de {}, até coluna {}", self.start, end.column)
        }
    }
}
