use num::{BigUint, Zero};
use std::{collections::HashMap, fmt};

/// Type alias for register indices. Defined for clarity.
pub type RegisterIndex = usize;

/// A register's data in the register bank, such as symbol and stored value.
#[derive(Debug, Clone)]
pub struct Register {
    symbol: String,
    value: BigUint,
}

impl Register {
    /// Initializes a register with the given symbol and zeroed stored content.
    pub fn zeroed(symbol: &str) -> Self {
        Self::with_value(symbol, BigUint::zero())
    }

    /// Initializes a register with the given symbol and initial stored content.
    pub fn with_value(symbol: &str, value: BigUint) -> Self {
        Self { symbol: symbol.to_string(), value }
    }

    /// Returns the symbol associated with this register.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the value stored in this register.
    pub fn value(&self) -> &BigUint {
        &self.value
    }

    /// Returns whether the register is zero.
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// Increments the stored value of the register without upper bounds.
    pub fn inc(&mut self) {
        self.value += 1u8;
    }

    /// Decrements the stored value of the register but saturates at zero.
    pub fn dec(&mut self) {
        if !self.is_zero() {
            self.value -= 1u8;
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.symbol(), self.value())
    }
}

/// The register bank of Norma machine, i.e. a collection of register names and
/// stored values. Registers are primarily accessed by an index.
#[derive(Debug, Clone)]
pub struct RegisterBank {
    registers: Vec<Register>,
    symbol_table: HashMap<String, RegisterIndex>,
}

impl RegisterBank {
    /// Initializes the register bank from a given list of registers, with their
    /// symbols and initial stored values. Registers are associated with an
    /// index corresponding to their position in the given input list.
    pub fn new(registers: Vec<Register>) -> Self {
        let symbol_table = registers
            .iter()
            .enumerate()
            .map(|(index, register)| (register.symbol().to_string(), index))
            .collect();
        Self { registers, symbol_table }
    }

    /// Queries for the index of the register associated with the given symbol.
    /// Returns `None` if not found.
    pub fn symbol_to_index(&self, symbol: &str) -> Option<RegisterIndex> {
        self.symbol_table.get(symbol).copied()
    }

    /// Gets an immutable reference to the register associated with the given
    /// index.
    ///
    /// # Panic
    /// Panics if the index is invalid.
    pub fn register(&self, index: RegisterIndex) -> &Register {
        &self.registers[index]
    }

    /// Gets a mutable reference to the register associated with the given
    /// index, i.e. you can modify the register.
    ///
    /// # Panic
    /// Panics if the index is invalid.
    pub fn register_mut(&mut self, index: RegisterIndex) -> &mut Register {
        &mut self.registers[index]
    }
}

impl fmt::Display for RegisterBank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for register in &self.registers {
            write!(fmt, "{}\n", register)?;
        }
        Ok(())
    }
}
