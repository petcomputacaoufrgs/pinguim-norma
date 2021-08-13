#[derive(Clone, Debug)]
pub enum OperationType {
    Inc,
    Dec,
    AddConst,
    SubConst,
    CmpConst,
    AddRegs,
    SubRegs,
    CmpRegs
}

#[derive(Clone, Debug)]
pub enum TestType {
    Zero
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
