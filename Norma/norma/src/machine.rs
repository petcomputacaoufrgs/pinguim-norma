#[cfg(test)]
mod test;

use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::{cmp::Ordering, collections::HashMap};

/// Um registrador da norma (sendo um  número natural arbitrário).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Register {
    /// Valor do registrador em número natural (tradicional da Norma).
    value: BigUint,
}

impl Register {
    /// Cria um novo registrador com o valor desejado
    fn new(number: BigUint) -> Register {
        Register { value: number }
    }

    /// Cria um novo registrador com valor zero.
    fn new_empty() -> Register {
        Register { value: BigUint::zero() }
    }

    /// Incrementa o valor do registrador.
    fn inc(&mut self) {
        self.value += 1u8
    }

    /// Decrementa o valor do registrador (caso seja maior que 0).
    fn dec(&mut self) {
        if !self.is_zero() {
            self.value -= 1u8
        }
    }

    /// Verifica se o valor do registrador é zero.
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    fn clear(&mut self) {
        self.value.set_zero();
    }

    /// Adiciona uma constante ao registrador.
    fn add(&mut self, constant: &BigUint) {
        self.value += constant;
    }

    /// Subtrai uma constante do registrador. A subtração satura no zero, caso a
    /// constante seja maior que o valor armazenado.
    fn sub(&mut self, constant: &BigUint) {
        if self.value <= *constant {
            self.value.set_zero();
        } else {
            self.value -= constant;
        }
    }

    /// Compara o registrador a uma constante e retorna se o valor armazenado é
    /// menor, igual ou maior à ela.
    fn cmp(&self, constant: &BigUint) -> Ordering {
        self.value.cmp(constant)
    }

    /// Retorna o valor do registrador.
    fn get_value(&self) -> BigUint {
        self.value.clone()
    }
}

/// Banco de registradores da Norma.
#[derive(Debug, Clone)]
pub struct Machine {
    /// Mapa de nomes de registradores para seus valores.
    registers: HashMap<String, Register>,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new(BigUint::zero())
    }
}

impl Machine {
    /// Inicia um novo banco de regitradores com 2 registradores básicos (X e Y)
    /// e inicia contador: X: Registrador de entrada, receberá o valor
    /// desejado Y: Registrador de saída, armazenará o valor retornado ao fim
    /// da execução
    pub fn new(input: BigUint) -> Machine {
        let mut this = Self { registers: HashMap::new() };
        this.insert_with_value("X", input);
        this.insert("Y");
        this
    }

    pub fn output(&self) -> BigUint {
        self.get_value("Y")
    }

    /// Insere um novo registrador no banco de nome `reg_name`.
    pub fn insert(&mut self, reg_name: &str) {
        self.registers.insert(reg_name.to_string(), Register::new_empty());
    }

    /// Insere um novo registrador com valor arbitrário (i.e. possibilita
    /// valores diferentes de zero), onde `reg_name` é o nome do registrador e
    /// `value` é o valor inicial do registrador.
    pub fn insert_with_value(&mut self, reg_name: &str, value: BigUint) {
        self.registers.insert(reg_name.to_string(), Register::new(value));
    }

    pub fn clear(&mut self, reg_name: &str) {
        self.get_register_mut(reg_name).clear();
    }

    pub fn clear_all(&mut self) {
        for register in self.registers.values_mut() {
            register.clear();
        }
    }

    /// Incrementa o valor de um registrador existente com nome `reg_name`.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn inc(&mut self, reg_name: &str) {
        self.get_register_mut(reg_name).inc();
    }

    /// Decrementa o valor de um registrador existente com nome `reg_name`.
    /// Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn dec(&mut self, reg_name: &str) {
        self.get_register_mut(reg_name).dec();
    }

    pub fn add(&mut self, dest: &str, src: &str, tmp: &str) {
        let operand = self.get_value(src);
        self.get_register_mut(dest).add(&operand);
        self.get_register_mut(tmp).clear();
    }

    /// Soma uma constante `constant` ao valor de um registrador existente com
    /// nome `reg_name`. Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn add_const(&mut self, reg_name: &str, constant: &BigUint) {
        self.get_register_mut(reg_name).add(constant);
    }

    pub fn sub(&mut self, dest: &str, src: &str, tmp: &str) {
        let operand = self.get_value(src);
        self.get_register_mut(dest).sub(&operand);
        self.get_register_mut(tmp).clear();
    }

    /// Subtrai uma constante `constant` do valor de um registrador existente
    /// com nome `reg_name`. Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn sub_const(&mut self, reg_name: &str, constant: &BigUint) {
        self.get_register_mut(reg_name).sub(constant);
    }

    pub fn cmp(
        &mut self,
        reg_left: &str,
        reg_right: &str,
        reg_tmp: &str,
    ) -> Ordering {
        self.get_register_mut(reg_tmp).clear();
        self.get_register(reg_left).cmp(&self.get_register(&reg_right).value)
    }

    /// Compara o valor do registrador existente de nome `reg_name` a uma
    /// constante `constant` com nome `reg_name`. Retorna se é menor, igual
    /// ou maior à constante.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn cmp_const(
        &mut self,
        reg_name: &str,
        constant: &BigUint,
    ) -> Ordering {
        self.get_register(reg_name).cmp(constant)
    }

    /// Testa se o valor do registrador existente de nome `reg_name` é zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn is_zero(&self, reg_name: &str) -> bool {
        self.get_register(reg_name).is_zero()
    }

    /// Retorna o valor de um registrador existente pela sua chave.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn get_value(&self, reg_name: &str) -> BigUint {
        self.get_register(reg_name).get_value()
    }

    /// Exporta os registradores em um mapa de
    /// `nome do registrador -> valor do registrador`, com valor renderizado em
    /// string, para ser exibido em front-end.
    pub fn export_registers(&mut self) -> HashMap<String, String> {
        let mut exported: HashMap<String, String> = HashMap::new();
        for (reg_name, reg_obj) in &self.registers {
            exported
                .insert(reg_name.to_string(), reg_obj.value.to_str_radix(10));
        }
        exported
    }

    /// Pesquisa um registrador existente de nome `reg_name` e retorna uma
    /// referência imutável a ele.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    fn get_register(&self, reg_name: &str) -> &Register {
        match self.registers.get(reg_name) {
            Some(register) => register,
            None => panic!("Register {} does not exist", reg_name),
        }
    }

    /// Pesquisa um registrador existente de nome `reg_name` e retorna uma
    /// referência Mutável a ele, ou seja, possibilita modificação.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    fn get_register_mut(&mut self, reg_name: &str) -> &mut Register {
        match self.registers.get_mut(reg_name) {
            Some(register) => register,
            None => panic!("Register {} does not exist", reg_name),
        }
    }
}
