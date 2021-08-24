use std::fmt;

#[derive(Clone, Debug)]
pub enum OperationType {
    Inc,
    Dec,
    AddConst,
    SubConst,
    AddRegs,
    SubRegs
}

#[derive(Clone, Debug)]
pub enum TestType {
    Zero,
    CmpConst,
    CmpRegs
}

#[derive(Clone, Debug)]
pub enum InstructionType {
    Operation(OperationType),
    Test(TestType)
}

#[derive(Clone, Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    registers: Vec<String>,
    constant: Option<usize>,
    next_label_true: String,
    next_label_false: Option<String>,
}

impl Instruction {

    // criar um enum para inicialização ou inicializar com Inc mesmo?
    pub fn new() -> Self {
        Instruction {
            instruction_type: InstructionType::Operation(OperationType::Inc),
            registers: Vec::<String>::new(),
            constant: None,
            next_label_true: String::new(),
            next_label_false: None,
        }
    }

    pub fn set_type(&mut self, instruction_type: InstructionType) {
        self.instruction_type = instruction_type;
    }

    pub fn add_register(&mut self, register_name: String) {
        self.registers.push(register_name);
    }

    pub fn set_registers(&mut self, registers: Vec<String>) {
        self.registers = registers.clone();
    }

    pub fn set_constant(&mut self, constant: usize) {
        self.constant = Some(constant);
    }

    pub fn set_next_instructions(&mut self, next_true: String, next_false: Option<String>) {
        self.next_label_true = next_true;
        self.next_label_false = next_false;
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operation = self.get_instruction_string();
        let registers = self.get_registers_string();
        let constant = self.get_constant_string();
        let next_instruction = self.get_next_instruction_string();

        write!(f, "{}({}{}) {}", operation, registers, constant, next_instruction)
    }
}

// Funções auxiliares para display (todas fechadas)
impl Instruction {
    fn get_instruction_string(&self) -> String {
        match self.instruction_type.clone() {
            InstructionType::Operation(o) => self.get_operation_string(o),
            InstructionType::Test(t) => self.get_test_string(t)
        }
    }

    fn get_operation_string(&self, operation: OperationType) -> String {
        match operation {
            OperationType::Inc => String::from("do inc"),
            OperationType::Dec => String::from("do dec"),
            OperationType::AddConst => String::from("do add"),
            OperationType::SubConst => String::from("do sub"),
            OperationType::AddRegs => String::from("do add"),
            OperationType::SubRegs => String::from("do sub"),
        }
    }

    fn get_test_string(&self, test: TestType) -> String {
        match test {
            TestType::Zero => String::from("if zero"),
            TestType::CmpConst => String::from("if cmp"),
            TestType::CmpRegs => String::from("if cmp")
        }
    }

    fn get_registers_string(&self) -> String {
        let mut registers_list = String::new();

        for reg in self.registers.iter() {
            registers_list += reg;
            registers_list.push(',');
        }
        registers_list.pop();
        registers_list
    }

    fn get_constant_string(&self) -> String {
        match self.constant {
            Some(cons) => format!(",{}",cons.to_string()),
            None => String::new()
        }
    }

    fn get_next_instruction_string(&self) -> String {
        match self.instruction_type.clone() {
            InstructionType::Operation(_) => self.get_next_operation_string(),
            InstructionType::Test(_) => self.get_next_test_string()
        }
    }

    fn get_next_operation_string(&self) -> String {
        format!{"goto {}", self.next_label_true}
    }

    fn get_next_test_string(&self) -> String {
        format!{"then goto {} else goto {}",
                self.next_label_true.clone(), 
                self.next_label_false.clone().unwrap()}
    }
}