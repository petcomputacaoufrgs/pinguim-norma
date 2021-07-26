#[cfg(test)]
mod test;

use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::{cmp::Ordering, collections::HashMap, ops::AddAssign};

/// Declaração da estrutura de um registrador
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Register {
    value: BigUint,
}

impl Register {
    // Cria um novo registrador com o valor desejado
    fn new(number: BigUint) -> Register {
        Register { value: number }
    }

    // Cria um novo registrador com valor zero
    fn new_empty() -> Register {
        Register { value: BigUint::zero() }
    }

    // Incrementa o valor do registrador
    fn inc(&mut self) {
        self.value += 1u8
    }

    // Decrementa o valor do registrador (caso seja maior que 0)
    fn dec(&mut self) {
        if !self.is_zero() {
            self.value -= 1u8
        }
    }

    // Verifica se o valor do registrador é 0
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    fn add_const(&mut self, constant: &BigUint) {
        self.value += constant;
    }

    fn sub_const(&mut self, constant: &BigUint) {
        if self.value <= *constant {
            self.value.set_zero();
        } else {
            self.value -= constant;
        }
    }

    fn cmp_const(&self, constant: &BigUint) -> Ordering {
        self.value.cmp(constant)
    }

    // Retorna o valor do registrador
    fn get_value(&self) -> BigUint {
        self.value.clone()
    }

    // Atualiza valor do registrador
    fn update_value(&mut self, new_value: BigUint) {
        self.value = new_value
    }
}

/// Declaração da estrutura do banco de Registradores
pub struct Machine {
    registers: HashMap<String, Register>,
    steps_counter: BigUint,
}

impl Machine {
    // Inicia um novo banco de regitradores com 2 registradores básicos (X e Y)
    // e inicia contador: X: Registrador de entrada, receberá o valor
    // desejado Y: Registrador de saída, armazenará o valor retornado ao fim
    // da execução
    pub fn new(input: BigUint) -> Machine {
        let mut register_bank: HashMap<String, Register> = HashMap::new();
        register_bank.insert("X".to_string(), Register::new(input));
        register_bank.insert("Y".to_string(), Register::new_empty());
        Machine { registers: register_bank, steps_counter: BigUint::zero() }
    }

    // Insere um novo registrador no banco
    // key: nome do registrador
    pub fn insert(&mut self, key: &str) {
        self.registers.insert(key.to_string(), Register::new_empty());
    }

    // Insere um novo registrador com valor diferente de 0
    // key: nome do registrador
    // value: valor do registrador
    pub fn insert_with_value(&mut self, key: &str, value: BigUint) {
        self.registers.insert(key.to_string(), Register::new(value));
    }

    // Incrementa o valor de um registrador existente, panicking caso o
    // registrador não exista. key: nome do registrador
    pub fn inc(&mut self, key: &str) {
        self.count_steps(1u8);
        self.get_register_mut(key).inc();
    }

    // Decrementa o valor de um registrador existente, panicking caso o
    // registrador não exista. key: nome do registrador
    pub fn dec(&mut self, key: &str) {
        self.count_steps(1u8);
        self.get_register_mut(key).dec();
    }

    // Soma valor constante a um registrador, caso o registrador exista
    // key: nome do registrador
    // cons: constante a ser somada ao valor já existente do registrador
    pub fn add_const(&mut self, key: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.get_register_mut(key).add_const(constant);
    }

    // Subtrai valor constante de um dado registrador, caso o registrador exista
    // key: nome registrador
    // cons: constante a ser subtraída do valor do registrador
    pub fn sub_const(&mut self, key: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.get_register_mut(key).sub_const(constant);
    }

    // Compara se o valor de um registrador é igual a uma dada constante
    // key: nome do registrador
    // cons: constante com a qual o valor do registrador será comparado
    pub fn cmp_const(&mut self, key: &str, constant: &BigUint) -> Ordering {
        let register = self.get_register(key);
        let cmp_result = register.cmp_const(constant);

        let steps = if cmp_result <= Ordering::Equal {
            constant * 3u8 + 1u8
        } else {
            register.get_value() * 3u8 + 1u8
        };
        self.count_steps(steps);

        cmp_result
    }

    // Testa se o valor de um registrador existente é zero, panicking caso o
    // registrador não exista. key: nome do registrador
    pub fn is_zero(&mut self, key: &str) -> bool {
        self.count_steps(1u8);
        self.get_register_mut(key).is_zero()
    }

    // Retorna o valor de um registrador pela sua chave
    pub fn get_value(&self, key: &str) -> BigUint {
        self.get_register(key).get_value()
    }

    // Retorna valor do contador
    pub fn get_counted_steps(&self) -> BigUint {
        self.steps_counter.clone()
    }

    // Retorna HashMap de registradores
    pub fn export_registers(&mut self) -> HashMap<String, String> {
        let mut exported: HashMap<String, String> = HashMap::new();
        for (reg_name, reg_obj) in &self.registers {
            exported
                .insert(reg_name.to_string(), reg_obj.value.to_str_radix(10));
        }
        exported
    }

    fn count_steps<T>(&mut self, value: T)
    where
        BigUint: AddAssign<T>,
    {
        self.steps_counter += value;
    }

    fn get_register(&self, key: &str) -> &Register {
        match self.registers.get(key) {
            Some(register) => register,
            None => panic!("Register {} does not exist", key),
        }
    }

    // Busca registrador, retornando none caso não exista
    // key: nome do registrador
    fn get_register_mut(&mut self, key: &str) -> &mut Register {
        match self.registers.get_mut(key) {
            Some(register) => register,
            None => panic!("Register {} does not exist", key),
        }
    }

    fn update_register(&mut self, key: &str, new_value: BigUint) {
        self.get_register_mut(key).update_value(new_value)
    }
}
