use num_bigint::BigUint;

/// Uma instrução genérica da Norma.
#[derive(Debug, Clone)]
pub enum Instruction {
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
