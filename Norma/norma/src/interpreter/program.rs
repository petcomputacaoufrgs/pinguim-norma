use indexmap::IndexMap;
use num_bigint::BigUint;

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
}

/// Uma instrução genérica da Norma.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// O rótulo identificado essa instrução.
    pub label: String,
    /// O tipo específico dessa instrução.
    pub kind: InstructionKind,
}

/// Um tipo específico de instrução.
#[derive(Debug, Clone)]
pub enum InstructionKind {
    /// Uma instrução de operação.
    Operation(Operation),
    /// Um instrução de teste.
    Test(Test),
}

/// Dados de uma instrução de operação.
#[derive(Debug, Clone)]
pub struct Operation {
    /// O "core" da operação em si, o tipo específico de operação.
    pub kind: OperationKind,
    /// O rótulo da pŕoxima instrução.
    pub next: String,
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
