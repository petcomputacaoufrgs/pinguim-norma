#[cfg(test)]
mod test;

use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::{cmp::Ordering, collections::HashMap, ops::AddAssign};

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

    /// Adiciona uma constante ao registrador.
    fn add_const(&mut self, constant: &BigUint) {
        self.value += constant;
    }

    /// Subtrai uma constante do registrador. A subtração satura no zero, caso a
    /// constante seja maior que o valor armazenado.
    fn sub_const(&mut self, constant: &BigUint) {
        if self.value <= *constant {
            self.value.set_zero();
        } else {
            self.value -= constant;
        }
    }

    /// Compara o registrador a uma constante e retorna se o valor armazenado é
    /// menor, igual ou maior à ela.
    fn cmp_const(&self, constant: &BigUint) -> Ordering {
        self.value.cmp(constant)
    }

    /// Retorna o valor do registrador.
    fn get_value(&self) -> BigUint {
        self.value.clone()
    }

    /// Atualiza valor do registrador.
    fn update_value(&mut self, new_value: BigUint) {
        self.value = new_value
    }
}

/// Banco de registradores da Norma.
pub struct Machine {
    /// Mapa de nomes de registradores para seus valores.
    registers: HashMap<String, Register>,
    /// Contador de passos.
    /// (LEMBRETE: rever se isso fica aqui ou vai pro interpretador).
    steps_counter: BigUint,
}

impl Machine {
    /// Inicia um novo banco de regitradores com 2 registradores básicos (X e Y)
    /// e inicia contador: X: Registrador de entrada, receberá o valor
    /// desejado Y: Registrador de saída, armazenará o valor retornado ao fim
    /// da execução
    pub fn new(input: BigUint) -> Machine {
        let mut register_bank: HashMap<String, Register> = HashMap::new();
        register_bank.insert("X".to_string(), Register::new(input));
        register_bank.insert("Y".to_string(), Register::new_empty());
        Machine { registers: register_bank, steps_counter: BigUint::zero() }
    }

    /// Insere um novo registrador no banco de nome `key`.
    pub fn insert(&mut self, key: &str) {
        self.registers.insert(key.to_string(), Register::new_empty());
    }

    /// Insere um novo registrador com valor arbitrário (i.e. possibilita
    /// valores diferentes de zero), onde `key` é o nome do registrador e
    /// `value` é o valor inicial do registrador.
    pub fn insert_with_value(&mut self, key: &str, value: BigUint) {
        self.registers.insert(key.to_string(), Register::new(value));
    }

    /// Incrementa o valor de um registrador existente com nome `key`.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn inc(&mut self, key: &str) {
        self.count_steps(1u8);
        self.get_register_mut(key).inc();
    }

    /// Decrementa o valor de um registrador existente com nome `key`. Satura em
    /// zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn dec(&mut self, key: &str) {
        self.count_steps(1u8);
        self.get_register_mut(key).dec();
    }

    /// Soma uma constante `constant` ao valor de um registrador existente com
    /// nome `key`. Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn add_const(&mut self, key: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.get_register_mut(key).add_const(constant);
    }

    /// Subtrai uma constante `constant` do valor de um registrador existente
    /// com nome `key`. Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn sub_const(&mut self, key: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.get_register_mut(key).sub_const(constant);
    }

    /// Compara o valor do registrador existente de nome `key` a uma constante
    /// `constant` com nome `key`. Retorna se é menor, igual ou maior à
    /// constante.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
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

    /// Testa se o valor do registrador existente de nome `key` é zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn is_zero(&mut self, key: &str) -> bool {
        self.count_steps(1u8);
        self.get_register_mut(key).is_zero()
    }

    /// Retorna o valor de um registrador existente pela sua chave.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn get_value(&self, key: &str) -> BigUint {
        self.get_register(key).get_value()
    }

    /// Retorna valor do contador de passos.
    pub fn get_counted_steps(&self) -> BigUint {
        self.steps_counter.clone()
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

    /// Conta a quantidade informada de passos dados, tal que a quantidade seja
    /// de qualquer tipo somável a um `BigUint`, tal como `u8`, `u64`,
    /// `BigUint`, `&BigUint`, etc.
    fn count_steps<T>(&mut self, value: T)
    where
        BigUint: AddAssign<T>,
    {
        self.steps_counter += value;
    }

    /// Pesquisa um registrador existente de nome `key` e retorna uma referência
    /// imutável a ele.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    fn get_register(&self, key: &str) -> &Register {
        match self.registers.get(key) {
            Some(register) => register,
            None => panic!("Register {} does not exist", key),
        }
    }

    /// Pesquisa um registrador existente de nome `key` e retorna uma referência
    /// Mutável a ele, ou seja, possibilita modificação.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    fn get_register_mut(&mut self, key: &str) -> &mut Register {
        match self.registers.get_mut(key) {
            Some(register) => register,
            None => panic!("Register {} does not exist", key),
        }
    }

    /// Atualiza o valor de um registrador existente.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    ///
    /// LEMBRETE: ver se vamos acabar usando, é um método interno de Machine e
    /// agora isso não tá sendo usado pra nada.
    fn update_register(&mut self, key: &str, new_value: BigUint) {
        self.get_register_mut(key).update_value(new_value)
    }
}
