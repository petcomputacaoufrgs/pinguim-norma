use std::collections::HashMap;
use num_bigint::{BigUint};
use num_traits::identities::{Zero, One};

/// Declaração da estrutura de um registrador
pub struct Register {
    value: BigUint,
}

impl Register {
    // Cria um novo registrador com o valor desejado
    pub fn new(number: BigUint) -> Register {
        Register {
            value: number
        }
    }

    // Cria um novo registrador com valor zero
    pub fn new_empty() -> Register {
        Register {
            value: BigUint::from(0 as u8)
        }
    }

    // Incrementa o valor do registrador
    pub fn inc(&mut self) {
        self.value += 1 as u8
    }

    // Decrementa o valor do registrador (caso seja maior que 0)
    pub fn dec(&mut self) {
        if !self.is_zero() {
            self.value -= 1 as u8
        }
    }

    // Verifica se o valor do registrador é 0
    pub fn is_zero(&mut self) -> bool {
        self.value == BigUint::from(0 as u8)
    }

    // Retorna o valor do registrador
    pub fn get_value(&mut self) -> BigUint {
        self.value.clone()
    }
}


/// Declaração da estrutura do banco de Registradores
pub struct RegisterBank {
    registers: HashMap<String, Register>
}

impl RegisterBank {
    // Inicia um novo banco de regitradores, com 2 registradores:
    // X: Registrador de entrada, receberá o valor desejado
    // Y: Registrador de saída, armazenará o valor retornado ao fim da execução
    pub fn new(input: BigUint) -> RegisterBank {
        let mut register_bank: HashMap<String, Register> = HashMap::new();
        register_bank.insert("X".to_string(), Register::new(input));
        register_bank.insert("Y".to_string(), Register::new_empty());
        RegisterBank {
            registers: register_bank
        }
    }

    // Insere um novo registrador no banco
    // key: nome do registrador
    pub fn insert(&mut self, key: &str) {
        self.registers.insert(key.to_string(), Register::new_empty());
    }

    // Incrementa o valor de um registrador existente, criando um caso não exista
    // key: nome do registrador
    pub fn inc(&mut self, key: &str) {
        match self.get_register(key) {
            Some(register) => {
                register.inc();
            },
            None => {
                self.insert_with_value(key, BigUint::one())
            }
        }
    }

    // Decrementa o valor de um registrador existente, verdadeiro caso não exista
    // key: nome do registrador
    pub fn dec(&mut self, key: &str) {
        match self.get_register(key) {
            Some(register) => {
                register.dec();
            },
            None => {
                self.insert(key)
            }
        }
    }

    // Decrementa o valor de um registrador existente, criando um caso não exista
    // key: nome do registrador
    pub fn is_zero(&mut self, key: &str) -> bool {
        match self.get_register(key) {
            Some(register) => {
                register.is_zero()
            },
            None => {
                true
            }
        }
    }

    // Retorna o valor de um registrador pela sua chave
    pub fn get_value(&mut self, key: &str) -> BigUint {
        match self.get_register(key) {
            Some(register) => register.get_value(),
            None => BigUint::zero()
        }
    }

    // Aplica uma função a um registrador caso encontrado
    pub fn apply(&mut self, key: &str, f: fn(&mut Register)) {
        match self.get_register(key) {
            Some(register) => f(register),
            None => {}
        }
    }

    // Insere um novo registrador com valor diferente de 0
    // key: nome do registrador
    // value: valor do registrador
    fn insert_with_value(&mut self, key: &str, value: BigUint) {
        self.registers.insert(key.to_string(), Register::new(value));
    }

    // Busca registrador, retornando none caso não exista
    // key: nome do registrador
    fn get_register(&mut self, key: &str) -> Option<&mut Register> {
        match self.registers.get_mut(key) {
            Some(register) => return Some(register),
            None => return None
        }
    }

    // Printa em tela os valores atuais do banco de registradores (desordenado)
    pub fn print(&mut self) {
        for (register_label, register) in &self.registers {
             println!("{}: {}", register_label, register.value);
        }
    }
}