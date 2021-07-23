pub enum OperationType {
    Inc,
    Dec,
    AddConst,
    SubConst,
    CmpConst,
    AddRegs,
    SubRegs,
    CmpRegs,
    Macro,
}

pub enum TestType{
    Zero,
    Macro
}

pub enum InstructionType {
    Operation(OperationType),
    Test(TestType)
}

pub struct Label {
    label: String
}

impl Label {
    pub fn new(label: String) -> Label{
        Label {
            label
        }
    }
}

pub struct Instruction {
    instruction_type: InstructionType,
    registers: Vec<String>,
    next_instruction_true: Label,
    next_instruction_false: Option<Label>,
    macro_name: Option<String>,
}

impl Instruction {

    // criar um enum para inicialização ou inicializar com Inc mesmo?
    pub fn new() -> Self {
        Instruction {
            instruction_type: InstructionType::Operation(OperationType::Inc),
            registers: Vec::<String>::new(),
            next_instruction_true: Label::new(String::new()),
            next_instruction_false: None,
            macro_name: None,
        }
    }

    pub fn set_type(&mut self, instruction_type: InstructionType) {
        self.instruction_type = instruction_type;
    }

    pub fn add_registers(&mut self, register_name: String) {
        self.registers.push(register_name);
    }

    pub fn set_next_instructions(&mut self, next_true: Label, next_false: Option<Label>) {
        self.next_instruction_true = next_true;
        self.next_instruction_false = next_false;
    }

    pub fn set_macro(&mut self, macro_name: Option<String>) {
        self.macro_name = macro_name;
    }
}