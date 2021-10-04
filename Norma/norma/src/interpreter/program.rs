use indexmap::IndexMap;
use num_bigint::BigUint;
use std::fmt;

/// Um programa da Norma.
#[derive(Debug, Clone)]
pub struct Program {
    instructions: IndexMap<String, Instruction>,
}

impl Program {
    /// Cria um programa vazio. TODO: vamos permitir mesmo criar programas
    /// vazios? Ou vamos exigir uma instrução inicial na criação do programa?
    pub fn empty() -> Self {
        Self { instructions: IndexMap::new() }
    }

    /// Retorna o primeiro rótulo, se houver ao menos uma instrução no programa.
    pub fn first_label(&self) -> Option<String> {
        self.instructions.first().map(|(label, _)| label).cloned()
    }

    /// Insere uma dada instrução no programa. Rótulos repetidos sobreescrevem o
    /// antigo (TODO: vai ser assim mesmo?).
    pub fn insert(&mut self, instruction: Instruction) {
        self.instructions.insert(instruction.label.clone(), instruction);
    }

    /// Busca a instrução associada com o dado rótulo. Retorna `None` caso o
    /// rótulo seja inválido (fora do programa).
    pub fn get_instruction(&self, label: &str) -> Option<Instruction> {
        self.instructions.get(label).cloned()
    }

    /// Exports all program instructions to be used with JS, in
    /// `(label, instruction-data)` format. TODO: replace tuple by a proper
    /// communication struct.
    pub fn export(&self) -> Vec<(String, String)> {
        self.instructions.values().map(Instruction::export).collect()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        for instruction in self.instructions.values() {
            write!(fmtr, "{}\n", instruction)?;
        }
        Ok(())
    }
}

/// Uma instrução genérica da Norma.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// O rótulo identificado essa instrução.
    pub label: String,
    /// O tipo específico dessa instrução.
    pub kind: InstructionKind,
}

impl Instruction {
    /// Exports this instruction to be used with JS, in
    /// `(label, instruction-data)` format. TODO: replace tuple by a proper
    /// communication struct.
    pub fn export(&self) -> (String, String) {
        (self.label.clone(), self.kind.to_string())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}: {}", self.label, self.kind)
    }
}

/// Um tipo específico de instrução.
#[derive(Debug, Clone)]
pub enum InstructionKind {
    /// Uma instrução de operação.
    Operation(Operation),
    /// Um instrução de teste.
    Test(Test),
}

impl fmt::Display for InstructionKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionKind::Operation(oper) => write!(fmtr, "{}", oper),
            InstructionKind::Test(test) => write!(fmtr, "{}", test),
        }
    }
}

/// Dados de uma instrução de operação.
#[derive(Debug, Clone)]
pub struct Operation {
    /// O "core" da operação em si, o tipo específico de operação.
    pub kind: OperationKind,
    /// O rótulo da pŕoxima instrução.
    pub next: String,
}

impl fmt::Display for Operation {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "do {} goto {}", self.kind, self.next)
    }
}

/// O tipo específico do "core" da operação.
#[derive(Debug, Clone)]
pub enum OperationKind {
    /// Incrementa o registrador do primeiro parâmetro.
    Inc(String),
    /// Decrementa o registrador do primeiro parâmetro.
    Dec(String),
    /// Limpa o registrador do primeiro parâmetro.
    Clear(String),
    /// Carrega uma constante (segundo parâmetro) no registrador do primeiro
    /// parâmetro.
    Load(String, BigUint),
    /// Adiciona uma constante (segundo parâmetro) ao registrador do primeiro
    /// parâmetro.
    AddConst(String, BigUint),
    /// Adiciona os dois primeiros registradores no primeiro, usando o terceiro
    /// registrador como temporário, que será zerado.
    Add(String, String, String),
    /// Subtrai uma constante (segundo parâmetro) do registrador do primeiro
    /// parâmetro.
    SubConst(String, BigUint),
    /// Subtraí o segundo registrador do primeiro e atualiza o primeiro, usando
    /// o terceiro registrador como temporário, que será zerado.
    Sub(String, String, String),
}

impl OperationKind {
    pub fn map_registers<F>(&self, mut mapper: F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            OperationKind::Inc(register) => {
                OperationKind::Inc(mapper(register))
            },
            OperationKind::Dec(register) => {
                OperationKind::Dec(mapper(register))
            },
            OperationKind::Clear(register) => {
                OperationKind::Clear(mapper(register))
            },
            OperationKind::Load(register, constant) => {
                OperationKind::Load(mapper(register), constant.clone())
            },
            OperationKind::AddConst(register, constant) => {
                OperationKind::AddConst(mapper(register), constant.clone())
            },
            OperationKind::Add(left, right, temp) => {
                OperationKind::Add(mapper(left), mapper(right), mapper(temp))
            },
            OperationKind::SubConst(register, constant) => {
                OperationKind::SubConst(mapper(register), constant.clone())
            },
            OperationKind::Sub(left, right, temp) => {
                OperationKind::Sub(mapper(left), mapper(right), mapper(temp))
            },
        }
    }
}

impl fmt::Display for OperationKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationKind::Inc(register) => write!(fmtr, "inc {}", register),
            OperationKind::Dec(register) => write!(fmtr, "dec {}", register),
            OperationKind::Clear(register) => {
                write!(fmtr, "clear ({})", register)
            },
            OperationKind::Load(register, constant) => {
                write!(fmtr, "load ({}, {})", register, constant)
            },
            OperationKind::AddConst(register, constant) => {
                write!(fmtr, "add ({}, {})", register, constant)
            },
            OperationKind::Add(reg_src, reg_dest, reg_tmp) => {
                write!(fmtr, "add ({}, {}, {})", reg_src, reg_dest, reg_tmp)
            },
            OperationKind::SubConst(register, constant) => {
                write!(fmtr, "sub ({}, {})", register, constant)
            },
            OperationKind::Sub(reg_src, reg_dest, reg_tmp) => {
                write!(fmtr, "sub ({}, {}, {})", reg_src, reg_dest, reg_tmp)
            },
        }
    }
}

/// Dados de uma instrução de teste.
#[derive(Debug, Clone)]
pub struct Test {
    /// O "core" do teste, o tipo específico do teste.
    pub kind: TestKind,
    /// O rótulo da próxima instrução caso seja verdadeiro.
    pub next_then: String,
    /// O rótulo da próxima instrução caso seja falso.
    pub next_else: String,
}

impl fmt::Display for Test {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "if {} then goto {} else goto {}",
            self.kind, self.next_then, self.next_else
        )
    }
}

/// O tipo específico do "core" do teste.
#[derive(Debug, Clone)]
pub enum TestKind {
    /// Testa se o primeiro parâmetro (registrador) é zero.
    Zero(String),
    /// Testa se o dado registrador (primeiro parâmetro) é igual a dada
    /// constante (segundo parâmetro).
    EqualsConst(String, BigUint),
    /// Teste se os dois primeiros registradores são iguais, usando o terceiro
    /// registrador como temporário, que será zerado.
    Equals(String, String, String),
    /// Testa se o dado registrador (primeiro parâmetro) é menor que a dada
    /// constante (segundo parâmetro).
    LessThanConst(String, BigUint),
    /// Teste se o primeiro registrador é menor que o segundo, usando o
    /// terceiro registrador como temporário, que será zerado.
    LessThan(String, String, String),
}

impl TestKind {
    pub fn map_registers<F>(&self, mut mapper: F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            TestKind::Zero(register) => TestKind::Zero(mapper(register)),
            TestKind::EqualsConst(register, constant) => {
                TestKind::EqualsConst(mapper(register), constant.clone())
            },
            TestKind::Equals(left, right, temp) => {
                TestKind::Equals(mapper(left), mapper(right), mapper(temp))
            },
            TestKind::LessThanConst(register, constant) => {
                TestKind::LessThanConst(mapper(register), constant.clone())
            },
            TestKind::LessThan(left, right, temp) => {
                TestKind::LessThan(mapper(left), mapper(right), mapper(temp))
            },
        }
    }
}

impl fmt::Display for TestKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestKind::Zero(register) => write!(fmtr, "zero {}", register),
            TestKind::EqualsConst(register, constant) => {
                write!(fmtr, "equals ({}, {})", register, constant)
            },
            TestKind::Equals(reg_left, reg_right, reg_tmp) => {
                write!(
                    fmtr,
                    "equals ({}, {}, {})",
                    reg_left, reg_right, reg_tmp
                )
            },
            TestKind::LessThanConst(register, constant) => {
                write!(fmtr, "lessThan ({}, {})", register, constant)
            },
            TestKind::LessThan(reg_left, reg_right, reg_tmp) => {
                write!(
                    fmtr,
                    "lessThan ({}, {}, {})",
                    reg_left, reg_right, reg_tmp
                )
            },
        }
    }
}
