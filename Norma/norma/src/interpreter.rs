//! Define itens relacionados ao interpretador da Norma.

pub mod instruction;

use instruction::{Instruction, Operation, OperationKind, Test, TestKind};

use crate::machine::Machine;
use indexmap::IndexMap;
use num_bigint::BigUint;
use num_traits::Zero;
use std::{cmp::Ordering, ops::AddAssign};

// ("1.add.2", "do inc X goto 1.add.3")

/// Executa um dado programa uma única vez, a partir da entrada (AKA registrador
/// X), mapa de rótulos para instruções, e iterável (e.g. lista) de nomes de
/// registradores usados no programa (não precisa passar os nomes "X" nem "Y").
/// Retorna a saída do programa (AKA registrador Y).
pub fn run_once<'regs, I>(
    input: BigUint,
    instructions: IndexMap<String, Instruction>,
    aux_registers: I,
) -> BigUint
where
    I: IntoIterator<Item = &'regs str>,
{
    let mut interpreter = Interpreter::new(instructions, aux_registers);
    interpreter.input(input);
    interpreter.run_all();
    interpreter.output()
}

/// O interpretador da Norma.
#[derive(Debug, Clone)]
pub struct Interpreter {
    /// Rótulo da instrução atual.
    current: String,
    /// Mapeamento de rótulos para instruções.
    instructions: IndexMap<String, Instruction>,
    /// Máquina sendo operada.
    machine: Machine,
    /// Passos dados.
    steps: BigUint,
}

impl Interpreter {
    /// Inicia o interpretador com o estado inicial do programa, a partir das
    /// instruções do programa, e de um iterável (e.g. lista) de nomes de
    /// registradores usados (não precisa passar os nomes "X" nem "Y").
    pub fn new<'regs, I>(
        instructions: IndexMap<String, Instruction>,
        aux_registers: I,
    ) -> Self
    where
        I: IntoIterator<Item = &'regs str>,
    {
        let (start, _) =
            instructions.first().expect("No mínimo uma instrução esperada");

        let mut machine = Machine::default();
        for register in aux_registers {
            machine.insert(register);
        }

        Self::from_state(start.clone(), instructions, machine, BigUint::zero())
    }

    /// Cria um novo interpretador a partir de um dado estado: o rótulo da
    /// instrução atual, as instruções, a máquina sendo operada, e os
    /// passos dados.
    pub fn from_state(
        current: String,
        instructions: IndexMap<String, Instruction>,
        machine: Machine,
        steps: BigUint,
    ) -> Self {
        Self { current, instructions, machine, steps }
    }

    /// Define o valor de entrada (AKA valor do registrador X).
    pub fn input(&mut self, data: BigUint) {
        self.machine.input(data);
    }

    /// Pega o valor de saída (AKA valor do registrador Y).
    pub fn output(&self) -> BigUint {
        self.machine.output()
    }

    /// Reseta o estado do programa, limpando todos os registradores, e
    /// redefinindo o rótulo da instrução atual como o rótulo da primeira
    /// instrução do mapa de instruções.
    pub fn reset(&mut self) {
        let (start, _) = self
            .instructions
            .first()
            .expect("No mínimo uma instrução esperada");

        self.machine.clear_all();
        self.current = start.clone();
    }

    /// Roda a instrução atual, mas somente essa, caso o rótulo da instrução
    /// atual seja válido (caso contrário, chegamos ao final do programa). O
    /// rótulo da instrução atual é atualizado de acordo com a instrução
    /// específica. Retorna `true` se o rótulo é válido e a instrução foi de
    /// fato executada.
    pub fn run_step(&mut self) -> bool {
        let entry = self.instructions.get(&self.current).cloned();
        match entry {
            Some(instruction) => {
                self.run_instruction(instruction);
                true
            },
            None => false,
        }
    }

    /// Roda no máximo `max_steps` instrções, a partir do rótulo atual (que é
    /// redefinido após cada instrução). Se chegarmos ao final do programa antes
    /// de `max_steps`, a execução para. Retorna `true` se ainda restam
    /// instruções para serem executadas.
    pub fn run_steps(&mut self, max_steps: u64) -> bool {
        for _ in 0 .. max_steps {
            if !self.run_step() {
                return false;
            }
        }
        true
    }

    /// Executa todas as instruções, partindo do rótulo atual, até chegar no
    /// final do programa (potencial loop infinito).
    pub fn run_all(&mut self) {
        while self.run_step() {}
    }

    /// Retorna quantos passos foram dados.
    pub fn steps(&self) -> BigUint {
        self.steps.clone()
    }

    /// Conta a dada quantidade de passos dados em uma determinada instrução,
    /// junto com os passos anteriores.
    fn count_steps<T>(&mut self, amount: T)
    where
        BigUint: AddAssign<T>,
    {
        self.steps += amount;
    }

    /// Performa a execução de uma dada instrução (no parâmetro), de acordo com
    /// o tipo específico de instrução (cada tipo também atualiza o rótulo atual
    /// de uma forma própria).
    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Test(test) => self.run_test(test),
            Instruction::Operation(operation) => self.run_operation(operation),
        }
    }

    /// Executa uma dada instrução do tipo operação. Além de executar a operação
    /// em si, atualiza o rótulo para aquele indicado no `goto` da operação.
    fn run_operation(&mut self, operation: Operation) {
        match operation.kind {
            OperationKind::Inc(register) => self.run_inc(&register),
            OperationKind::Dec(register) => self.run_dec(&register),
            OperationKind::Clear(register) => self.run_clear(&register),
            OperationKind::AddConst(register, constant) => {
                self.run_add_const(&register, &constant)
            },
            OperationKind::Add(reg_dest, reg_src, reg_tmp) => {
                self.run_add(&reg_dest, &reg_src, &reg_tmp)
            },
            OperationKind::SubConst(register, constant) => {
                self.run_sub_const(&register, &constant)
            },
            OperationKind::Sub(reg_dest, reg_src, reg_tmp) => {
                self.run_sub(&reg_dest, &reg_src, &reg_tmp)
            },
        }
        self.current = operation.next;
    }

    /// Executa uma dada instrução do tipo teste. Além de executar o teste em
    /// si, atualiza o rótulo para aquele indicado no `then goto` ou `else goto`
    /// do teste, de acordo com o resultado do teste.
    fn run_test(&mut self, test: Test) {
        let success = match test.kind {
            TestKind::Zero(register) => self.test_zero(&register),
            TestKind::EqualsConst(register, constant) => {
                self.test_equals_const(&register, &constant)
            },
            TestKind::Equals(reg_left, reg_right, reg_tmp) => {
                self.test_equals(&reg_left, &reg_right, &reg_tmp)
            },
            TestKind::LessThanConst(register, constant) => {
                self.test_less_than_const(&register, &constant)
            },
            TestKind::LessThan(reg_left, reg_right, reg_tmp) => {
                self.test_less_than(&reg_left, &reg_right, &reg_tmp)
            },
        };
        self.current = if success { test.next_then } else { test.next_else };
    }

    /// `inc A`
    ///
    /// `1` step
    fn run_inc(&mut self, reg_name: &str) {
        self.count_steps(1u8);
        self.machine.inc(reg_name);
    }

    /// `dec A`
    ///
    /// `1` step
    fn run_dec(&mut self, reg_name: &str) {
        self.count_steps(1u8);
        self.machine.dec(reg_name);
    }

    /// ```pre
    /// operation clear (Tmp) {
    ///     is_done: if zero Tmp then goto done else goto next_Tmp
    ///     next_Tmp: do dec Tmp goto is_done
    /// }
    /// ```
    ///
    /// `Tmp * 2 + 1` steps
    fn run_clear(&mut self, reg_name: &str) {
        let mut steps = self.machine.get_value(reg_name);
        steps *= 2u8;
        steps += 1u8;
        self.count_steps(steps);
        self.machine.clear(reg_name);
    }

    /// ```pre
    /// operation add (A) N {
    ///     1: do inc A goto 2
    ///     2: do inc A goto 3
    ///     ...
    ///     N: do inc A goto done
    /// }
    /// ```
    ///
    /// `N` steps
    fn run_add_const(&mut self, reg_name: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.machine.add_const(reg_name, constant);
    }

    /// ```pre
    /// operation add (Dest, Src, Tmp) {
    ///     // Tmp * 2 + 1
    ///     cleanup: do clear (Tmp) goto start
    ///
    ///     // Src * 4 + 1
    ///     start: if zero Src then goto restore else goto next_Dest
    ///     next_Dest: do inc Dest goto next_Src
    ///     next_Src: do dec Src goto save_Tmp
    ///     save_Tmp: do inc Tmp goto start
    ///
    ///     // Src * 3 + 1
    ///     restore: if zero Tmp then goto done else goto undo_Src
    ///     undo_Src: do inc Src goto undo_Tmp
    ///     undo_Tmp: do dec Tmp goto restore
    /// }
    /// ```
    ///
    /// `Tmp * 2 + Src * 7 + 3` steps
    fn run_add(&mut self, reg_dest: &str, reg_src: &str, reg_tmp: &str) {
        let mut tmp_steps = self.machine.get_value(reg_src);
        tmp_steps *= 2u8;

        let mut steps = self.machine.get_value(reg_src);
        steps *= 7u8;
        steps += tmp_steps;
        steps += 3u8;
        self.count_steps(steps);

        self.machine.add(reg_dest, reg_src, reg_tmp);
    }

    /// ```pre
    /// operation sub (A) N {
    ///     1: do dec A goto 2
    ///     2: do dec A goto 3
    ///     ...
    ///     N: do dec A goto done
    /// }
    /// ```
    ///
    /// `N` steps
    fn run_sub_const(&mut self, reg_name: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.machine.sub_const(reg_name, constant);
    }

    /// ```pre
    /// operation sub (Dest, Src, Tmp) {
    ///     // Tmp * 2 + 1
    ///     cleanup: do clear (Tmp) goto start
    ///
    ///     // Src * 4 + 1
    ///     start: if zero Src then goto restore else goto next_Dest
    ///     next_Dest: do dec Dest goto next_Src
    ///     next_Src: do dec Src goto save_Tmp
    ///     save_Tmp: do inc Tmp goto start
    ///     
    ///     // Src * 3 + 1
    ///     restore: if zero Tmp then goto done else goto undo_Src
    ///     undo_Src: do inc Src goto undo_Tmp
    ///     undo_Tmp: do dec Tmp goto restore
    /// }
    /// ```
    ///
    /// `Tmp * 2 + Src * 7 + 3` steps
    fn run_sub(&mut self, reg_dest: &str, reg_src: &str, reg_tmp: &str) {
        let mut tmp_steps = self.machine.get_value(reg_src);
        tmp_steps *= 2u8;

        let mut steps = self.machine.get_value(reg_src);
        steps *= 7u8;
        steps += tmp_steps;
        steps += 3u8;
        self.count_steps(steps);

        self.machine.sub(reg_dest, reg_src, reg_tmp);
    }

    /// `zero A`
    ///
    /// `1` step
    fn test_zero(&mut self, reg_name: &str) -> bool {
        self.count_steps(1u8);
        self.machine.is_zero(reg_name)
    }

    /// ```pre
    /// test equals (A) N {
    ///     1: if zero A then goto false else goto 1_dec
    ///     1_dec: do dec A goto 2
    ///     2: if zero A then goto N_restore_false else goto 2_dec
    ///     2_dec: do dec A goto 3
    ///     ...
    ///     Nplus1: if zero A then goto 1_restore_true else goto 1_restore_false
    ///
    ///     1_restore_true: do inc A goto 2_restore_true
    ///     2_restore_true: do inc A goto 3_restore_true
    ///     ...
    ///     N_restore_true: do inc A goto true
    ///
    ///     1_restore_false: do inc A goto 2_restore_false
    ///     2_restore_false: do inc A goto 3_restore_false
    ///     ...
    ///     N_restore_false: do inc A goto false
    /// }
    /// ```
    ///
    /// `min(A, N) * 3 + 1` steps
    fn test_equals_const(
        &mut self,
        register: &str,
        constant: &BigUint,
    ) -> bool {
        let ordering = self.machine.cmp_const(register, constant);

        let mut steps = if ordering <= Ordering::Equal {
            self.machine.get_value(register)
        } else {
            constant.clone()
        };
        steps *= 3u8;
        steps += 1u8;
        self.count_steps(steps);

        ordering == Ordering::Equal
    }

    /// ```pre
    /// test equals (L, R, Tmp) {
    ///     // Tmp * 2 + 1
    ///     cleanup: do clean (Tmp) goto check_L
    ///
    ///     // min(L, R) * 5 + 2
    ///     check_L: if zero L then goto check_LR else goto check_R
    ///     check_LR: if zero R then goto restore_true else goto restore_false
    ///     check_R: if zero R then goto restore_false else goto next_L
    ///     next_L: do dec L goto next_R
    ///     next_R: do dec R goto save_Tmp
    ///     save_Tmp: do inc Tmp goto check_L
    ///
    ///     // min(L, R) * 4 + 1
    ///     restore: if zero Tmp then goto true else goto restore_true_Tmp
    ///     restore_true_Tmp: do dec Tmp goto restore_true_L
    ///     restore_true_L: do inc L goto restore_true_R
    ///     restore_true_R: do inc R goto restore_true
    ///
    ///     // min(L, R) * 4 + 1
    ///     restore: if zero Tmp then goto false else goto restore_false_Tmp
    ///     restore_false_Tmp: do dec Tmp goto restore_false_L
    ///     restore_false_L: do inc L goto restore_false_R
    ///     restore_false_R: do inc R goto restore_false
    /// }
    /// ```
    ///
    /// `Tmp * 2 + min(L, R) * 9 + 4` steps
    fn test_equals(
        &mut self,
        reg_left: &str,
        reg_right: &str,
        reg_tmp: &str,
    ) -> bool {
        let ordering = self.machine.cmp(reg_left, reg_right, reg_tmp);

        let mut tmp_steps = self.machine.get_value(reg_tmp);
        tmp_steps *= 2u8;
        let minimum =
            if ordering <= Ordering::Equal { reg_left } else { reg_right };
        let mut steps = self.machine.get_value(minimum);

        steps *= 9u8;
        steps += tmp_steps;
        steps += 4u8;

        self.count_steps(steps);
        ordering == Ordering::Equal
    }

    /// ```pre
    /// test lessThan (A) N {
    ///     1: if zero A then goto true else goto 1_dec
    ///     1_dec: do dec A goto 2
    ///     2: if zero A then goto Nminus1_restore_true else goto 2_dec
    ///     2_dec: do dec A goto 3
    ///     ...
    ///     N: if zero A then goto 1_restore_true else goto 1_restore_false
    ///
    ///     1_restore_true: do inc A goto 2_restore_true
    ///     2_restore_true: do inc A goto 3_restore_true
    ///     ...
    ///     Nminus1_restore_true: do inc A goto true
    ///
    ///     1_restore_false: do inc A goto 2_restore_false
    ///     2_restore_false: do inc A goto 3_restore_false
    ///     ...
    ///     Nminus1_restore_false: do inc A goto false
    /// }
    /// ```
    ///
    /// `min(A, N - 1) * 3 + 1` steps
    fn test_less_than_const(
        &mut self,
        register: &str,
        constant: &BigUint,
    ) -> bool {
        let ordering = self.machine.cmp_const(register, constant);

        let mut steps = if ordering < Ordering::Equal {
            self.machine.get_value(register)
        } else {
            constant - 1u8
        };
        steps *= 3u8;
        steps += 1u8;
        self.count_steps(steps);

        ordering == Ordering::Less
    }

    /// ```pre
    /// test lessThan (L, R, Tmp) {
    ///     // Tmp * 2 + 1
    ///     cleanup: do clean (Tmp) goto check_L
    ///
    ///     // min(L, R) * 5 + 2
    ///     check_L: if zero L then goto check_LR else goto check_R
    ///     check_LR: if zero R then goto restore_false else goto restore_true
    ///     check_R: if zero R then goto restore_false else goto next_L
    ///     next_L: do dec L goto next_R
    ///     next_R: do dec R goto save_Tmp
    ///     save_Tmp: do inc Tmp goto check_L
    ///
    ///     // min(L, R) * 4 + 1
    ///     restore: if zero Tmp then goto true else goto restore_true_Tmp
    ///     restore_true_Tmp: do dec Tmp goto restore_true_L
    ///     restore_true_L: do inc L goto restore_true_R
    ///     restore_true_R: do inc R goto restore_true
    ///
    ///     // min(L, R) * 4 + 1
    ///     restore: if zero Tmp then goto false else goto restore_false_Tmp
    ///     restore_false_Tmp: do dec Tmp goto restore_false_L
    ///     restore_false_L: do inc L goto restore_false_R
    ///     restore_false_R: do inc R goto restore_false
    /// }
    /// ```
    ///
    /// `Tmp * 2 + min(L, R) * 9 + 4` steps
    fn test_less_than(
        &mut self,
        reg_left: &str,
        reg_right: &str,
        reg_tmp: &str,
    ) -> bool {
        let ordering = self.machine.cmp(reg_left, reg_right, reg_tmp);

        let mut tmp_steps = self.machine.get_value(reg_tmp);
        tmp_steps *= 2u8;
        let minimum =
            if ordering <= Ordering::Equal { reg_left } else { reg_right };
        let mut steps = self.machine.get_value(minimum);

        steps *= 9u8;
        steps += tmp_steps;
        steps += 4u8;

        self.count_steps(steps);
        ordering == Ordering::Less
    }
}
