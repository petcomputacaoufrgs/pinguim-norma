//! Este módulo exporta itens necessários para construir um programa da norma.

use indexmap::{map, IndexMap};
use num_bigint::BigUint;
use std::fmt;

/// Um programa da Norma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    instructions: IndexMap<String, Instruction>,
}

impl Program {
    /// Cria um programa vazio.
    pub fn empty() -> Self {
        Self { instructions: IndexMap::new() }
    }

    /// Retorna se o programa está vazio, isto é, não tem instrução alguma.
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Retorna o tamanho do programa, ou seja, o número de instruções.
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Retorna o primeiro rótulo, se houver ao menos uma instrução no programa.
    pub fn first_label(&self) -> &str {
        match self.instructions.first() {
            Some((label, _)) => label,
            None => "0",
        }
    }

    /// Insere uma dada instrução no programa.
    ///
    /// # Panics
    ///
    /// Invoca `panic!()` caso o rótulo esteja duplicado.
    pub fn insert(&mut self, instruction: Instruction) {
        match self.instructions.entry(instruction.label().to_owned()) {
            map::Entry::Vacant(entry) => {
                entry.insert(instruction);
            }
            map::Entry::Occupied(_) => {
                panic!("Duplicated label {}", instruction.label)
            }
        }
    }

    /// Testa se um dado rótulo é válido, i.e. existe uma instrução para o qual
    /// esse rótulo mapeia.
    pub fn is_label_valid(&self, label: &str) -> bool {
        self.instructions.contains_key(label)
    }

    /// Busca a instrução associada com o dado rótulo. Retorna `None` caso o
    /// rótulo seja inválido (fora do programa).
    pub fn instruction(&self, label: &str) -> Option<&Instruction> {
        self.instructions.get(label)
    }

    /// Busca a instrução associada com o dado rótulo, e retorna uma referência
    /// mutável para ela. Retorna `None` caso o rótulo seja inválido (fora
    /// do programa).
    pub fn instruction_mut(&mut self, label: &str) -> Option<&mut Instruction> {
        self.instructions.get_mut(label)
    }

    /// Constrói um iterador sobre referências de instruções. Pode ser usado no
    /// `for`.
    pub fn instructions(&self) -> Instructions {
        Instructions { inner: self.instructions.values() }
    }

    /// Constrói um iterador sobre referências mutáveis de instruções. Pode ser
    /// usado no `for`.
    pub fn instructions_mut(&mut self) -> InstructionsMut {
        InstructionsMut { inner: self.instructions.values_mut() }
    }

    /// Coleta todos os rótulos usados nessa instrução, usando uma função
    /// genérica para lidar com cada um e coletar.
    pub fn collect_labels<F>(&self, mut collector: F)
    where
        F: FnMut(&str),
    {
        for instruction in self.instructions() {
            instruction.collect_labels(&mut collector);
        }
    }

    /// Coleta todos os nomes de registradores usados no programa, usando uma
    /// função genérica para lidar com cada um e coletar.
    pub fn collect_registers<F>(&self, mut collector: F)
    where
        F: FnMut(&str),
    {
        for instruction in self.instructions() {
            instruction.collect_registers(&mut collector);
        }
    }

    /// Exporta todas as instruções do programa para serem usadas com JS, no
    /// formato `(label, instruction-data)`. TODO: substituir tuplas por um tipo
    /// próprio da comunicação.
    pub fn export(&self) -> Vec<(String, String)> {
        self.instructions().map(Instruction::export).collect()
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

impl<'prog> IntoIterator for &'prog Program {
    type Item = &'prog Instruction;
    type IntoIter = Instructions<'prog>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions()
    }
}

impl<'prog> IntoIterator for &'prog mut Program {
    type Item = &'prog mut Instruction;
    type IntoIter = InstructionsMut<'prog>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions_mut()
    }
}

/// Iterador sobre instruções de um programa.
#[derive(Debug, Clone)]
pub struct Instructions<'prog> {
    inner: map::Values<'prog, String, Instruction>,
}

impl<'prog> Iterator for Instructions<'prog> {
    type Item = &'prog Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'prog> DoubleEndedIterator for Instructions<'prog> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'prog> ExactSizeIterator for Instructions<'prog> {}

/// Iterador sobre instruções mutáveis de um programa.
pub struct InstructionsMut<'prog> {
    inner: map::ValuesMut<'prog, String, Instruction>,
}

impl<'prog> Iterator for InstructionsMut<'prog> {
    type Item = &'prog mut Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'prog> DoubleEndedIterator for InstructionsMut<'prog> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'prog> ExactSizeIterator for InstructionsMut<'prog> {}

/// Uma instrução genérica da Norma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// O rótulo identificado essa instrução.
    label: String,
    /// O tipo específico dessa instrução.
    pub kind: InstructionKind,
}

impl Instruction {
    /// Cria uma instrução a partir do seu rótulo e de seu tipo específico.
    pub fn new(label: String, kind: InstructionKind) -> Self {
        Self { label, kind }
    }

    /// O rótulo da instrução.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Exporta essa instrução para ser usada com JS, no formato `(label,
    /// instruction-data)`. TODO: substituir tuplas por um tipo próprio da
    /// comunicação.
    pub fn export(&self) -> (String, String) {
        (self.label.clone(), self.kind.to_string())
    }

    /// Coleta todos os rótulos usados nessa instrução, usando uma função
    /// genérica para lidar com cada um e coletar.
    pub fn collect_labels<F>(&self, collector: F)
    where
        F: FnMut(&str),
    {
        self.kind.collect_labels(collector);
    }

    /// Coleta todos os nomes de registradores usados na instrução, usando uma
    /// função genérica para lidar com cada um e coletar.
    pub fn collect_registers<F>(&self, collector: F)
    where
        F: FnMut(&str),
    {
        self.kind.collect_registers(collector);
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}: {}", self.label, self.kind)
    }
}

/// Um tipo específico de instrução.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl InstructionKind {
    /// Renomeia todos os rótulos desse tipo de instrução a partir de uma função
    /// de renomeamento. Pelo fato de aplicarmos a renomeação no tipo da
    /// instrução e não na instrução toda, isso significa que o rótulo que
    /// identifica a instrução em si não é renomeado.
    pub fn rename_labels<F>(&mut self, renamer: F)
    where
        F: FnMut(&mut String),
    {
        match self {
            InstructionKind::Operation(oper) => oper.rename_labels(renamer),
            InstructionKind::Test(test) => test.rename_labels(renamer),
        }
    }

    /// Coleta todos os rótulos usados nesse tipo de instrução, usando uma
    /// função genérica para lidar com cada um e coletar.
    pub fn collect_labels<F>(&self, collector: F)
    where
        F: FnMut(&str),
    {
        match self {
            InstructionKind::Operation(oper) => oper.collect_labels(collector),
            InstructionKind::Test(test) => test.collect_labels(collector),
        }
    }

    /// Coleta todos os nomes de registradores usados nesse tipo específico de
    /// instrução, usando uma função genérica para lidar com cada um e
    /// coletar.
    pub fn collect_registers<F>(&self, collector: F)
    where
        F: FnMut(&str),
    {
        match self {
            InstructionKind::Operation(oper) => {
                oper.collect_registers(collector);
            }
            InstructionKind::Test(test) => {
                test.collect_registers(collector);
            }
        }
    }
}

/// Dados de uma instrução de operação.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Operation {
    /// A partir de uma função de renomeamento, renomeia todos os rótulos da
    /// operação, que na verdade é simplesmente o rótulo de `next`.
    pub fn rename_labels<F>(&mut self, mut renamer: F)
    where
        F: FnMut(&mut String),
    {
        renamer(&mut self.next);
    }

    /// Coleta todos os rótulos usados nessa operação, usando uma função
    /// genérica para lidar com cada um e coletar.
    pub fn collect_labels<F>(&self, mut collector: F)
    where
        F: FnMut(&str),
    {
        collector(&self.next);
    }

    /// Coleta todos os nomes de registradores usados nessa operação, usando
    /// uma função genérica para lidar com cada um e coletar.
    pub fn collect_registers<F>(&self, collector: F)
    where
        F: FnMut(&str),
    {
        self.kind.collect_registers(collector);
    }
}

/// O tipo específico do "core" da operação.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Mapeia todos os registradores para novos registradores, usando a dada
    /// função. Clona outros dados.
    pub fn map_registers<F>(&self, mut mapper: F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            OperationKind::Inc(register) => {
                OperationKind::Inc(mapper(register))
            }
            OperationKind::Dec(register) => {
                OperationKind::Dec(mapper(register))
            }
            OperationKind::Clear(register) => {
                OperationKind::Clear(mapper(register))
            }
            OperationKind::Load(register, constant) => {
                OperationKind::Load(mapper(register), constant.clone())
            }
            OperationKind::AddConst(register, constant) => {
                OperationKind::AddConst(mapper(register), constant.clone())
            }
            OperationKind::Add(left, right, temp) => {
                OperationKind::Add(mapper(left), mapper(right), mapper(temp))
            }
            OperationKind::SubConst(register, constant) => {
                OperationKind::SubConst(mapper(register), constant.clone())
            }
            OperationKind::Sub(left, right, temp) => {
                OperationKind::Sub(mapper(left), mapper(right), mapper(temp))
            }
        }
    }

    /// Coleta todos os nomes de registradores usados nesse tipo específico de
    /// operação, usando uma função genérica para lidar com cada um e coletar.
    pub fn collect_registers<F>(&self, mut collector: F)
    where
        F: FnMut(&str),
    {
        match self {
            OperationKind::Inc(register) => {
                collector(register);
            }
            OperationKind::Dec(register) => {
                collector(register);
            }
            OperationKind::Clear(register) => {
                collector(register);
            }
            OperationKind::Load(register, _constant) => {
                collector(register);
            }
            OperationKind::AddConst(register, _constant) => {
                collector(register);
            }
            OperationKind::Add(left, right, temp) => {
                collector(left);
                collector(right);
                collector(temp);
            }
            OperationKind::SubConst(register, _constant) => {
                collector(register);
            }
            OperationKind::Sub(left, right, temp) => {
                collector(left);
                collector(right);
                collector(temp);
            }
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
            }
            OperationKind::Load(register, constant) => {
                write!(fmtr, "load ({}, {})", register, constant)
            }
            OperationKind::AddConst(register, constant) => {
                write!(fmtr, "add ({}, {})", register, constant)
            }
            OperationKind::Add(reg_src, reg_dest, reg_tmp) => {
                write!(fmtr, "add ({}, {}, {})", reg_src, reg_dest, reg_tmp)
            }
            OperationKind::SubConst(register, constant) => {
                write!(fmtr, "sub ({}, {})", register, constant)
            }
            OperationKind::Sub(reg_src, reg_dest, reg_tmp) => {
                write!(fmtr, "sub ({}, {}, {})", reg_src, reg_dest, reg_tmp)
            }
        }
    }
}

/// Dados de uma instrução de teste.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Test {
    /// A partir de uma função de renomeamento, renomeia todos os rótulos do
    /// teste, que na verdade são os rótulo de `then` e de `else`.
    pub fn rename_labels<F>(&mut self, mut renamer: F)
    where
        F: FnMut(&mut String),
    {
        renamer(&mut self.next_then);
        renamer(&mut self.next_else);
    }

    /// Coleta todos os rótulos usados nesse teste, usando uma função
    /// genérica para lidar com cada um e coletar.
    pub fn collect_labels<F>(&self, mut collector: F)
    where
        F: FnMut(&str),
    {
        collector(&self.next_then);
        collector(&self.next_else);
    }

    /// Coleta todos os nomes de registradores usados nesse teste, usando
    /// uma função genérica para lidar com cada um e coletar.
    pub fn collect_registers<F>(&self, collector: F)
    where
        F: FnMut(&str),
    {
        self.kind.collect_registers(collector);
    }
}

/// O tipo específico do "core" do teste.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Mapeia todos os registradores para novos registradores, usando a dada
    /// função. Clona outros dados.
    pub fn map_registers<F>(&self, mut mapper: F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            TestKind::Zero(register) => TestKind::Zero(mapper(register)),
            TestKind::EqualsConst(register, constant) => {
                TestKind::EqualsConst(mapper(register), constant.clone())
            }
            TestKind::Equals(left, right, temp) => {
                TestKind::Equals(mapper(left), mapper(right), mapper(temp))
            }
            TestKind::LessThanConst(register, constant) => {
                TestKind::LessThanConst(mapper(register), constant.clone())
            }
            TestKind::LessThan(left, right, temp) => {
                TestKind::LessThan(mapper(left), mapper(right), mapper(temp))
            }
        }
    }

    /// Coleta todos os nomes de registradores usados nesse tipo específico de
    /// teste, usando uma função genérica para lidar com cada um e coletar.
    pub fn collect_registers<F>(&self, mut collector: F)
    where
        F: FnMut(&str),
    {
        match self {
            TestKind::Zero(register) => {
                collector(register);
            }
            TestKind::EqualsConst(register, _constant) => {
                collector(register);
            }
            TestKind::Equals(left, right, temp) => {
                collector(left);
                collector(right);
                collector(temp);
            }
            TestKind::LessThanConst(register, _constant) => {
                collector(register);
            }
            TestKind::LessThan(left, right, temp) => {
                collector(left);
                collector(right);
                collector(temp);
            }
        }
    }
}

impl fmt::Display for TestKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestKind::Zero(register) => write!(fmtr, "zero {}", register),
            TestKind::EqualsConst(register, constant) => {
                write!(fmtr, "equals ({}, {})", register, constant)
            }
            TestKind::Equals(reg_left, reg_right, reg_tmp) => {
                write!(
                    fmtr,
                    "equals ({}, {}, {})",
                    reg_left, reg_right, reg_tmp
                )
            }
            TestKind::LessThanConst(register, constant) => {
                write!(fmtr, "lessThan ({}, {})", register, constant)
            }
            TestKind::LessThan(reg_left, reg_right, reg_tmp) => {
                write!(
                    fmtr,
                    "lessThan ({}, {}, {})",
                    reg_left, reg_right, reg_tmp
                )
            }
        }
    }
}
