use std::convert::From;

pub struct VirtualMachine {
    ip: i64,
    stack: Vec<i64>,
    memory: Vec<i64>,
}

pub enum OpCode {
    Halt = 0x00,
    Push = 0x01,
    Pop = 0x02,
    Add = 0x03,
    Sub = 0x04,
    Mul = 0x05,
    Div = 0x06,
    Jump = 0x07,
    JumpIfEqual = 0x08,
    JumpIfNotEqual = 0x09,
    JumpIfLessThan = 0x0A,
    JumpIfGreaterThan = 0x0B,
    JumpIfLessThanOrEqual = 0x0C,
    JumpIfGreaterThanOrEqual = 0x0D,
    Call = 0x0E,
    Return = 0x0F,
    Print = 0x10,
    Store = 0x11,
    Load = 0x12,
}

impl From<i64> for OpCode {
    fn from(value: i64) -> Self {
        match value {
            0x00 => OpCode::Halt,
            0x01 => OpCode::Push,
            0x02 => OpCode::Pop,
            0x03 => OpCode::Add,
            0x04 => OpCode::Sub,
            0x05 => OpCode::Mul,
            0x06 => OpCode::Div,
            0x07 => OpCode::Jump,
            0x08 => OpCode::JumpIfEqual,
            0x09 => OpCode::JumpIfNotEqual,
            0x0A => OpCode::JumpIfLessThan,
            0x0B => OpCode::JumpIfGreaterThan,
            0x0C => OpCode::JumpIfLessThanOrEqual,
            0x0D => OpCode::JumpIfGreaterThanOrEqual,
            0x0E => OpCode::Call,
            0x0F => OpCode::Return,
            0x10 => OpCode::Print,
            0x11 => OpCode::Store,
            0x12 => OpCode::Load,
            _ => panic!("Unknown opcode: {}", value),
        }
    }
}

pub enum Instruction {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Jump(i64),
    JumpIfEqual(i64),
    JumpIfNotEqual(i64),
    JumpIfLessThan(i64),
    JumpIfGreaterThan(i64),
    JumpIfLessThanOrEqual(i64),
    JumpIfGreaterThanOrEqual(i64),
    Call(i64),
    Return,
    Print,
    Store(i64),
    Load(i64),
    Halt,
}

impl VirtualMachine {
    pub fn new(stack_size: i64, memory_size: i64) -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            stack: vec![0; usize::try_from(stack_size).unwrap().to_owned()],
            memory: vec![0; usize::try_from(memory_size).unwrap().to_owned()],
        }
    }

    pub fn load_program(&mut self, program: &[i64]) {
        assert!(program.len() <= self.memory.len());
        for (i, &instruction) in program.iter().enumerate() {
            self.memory[i] = instruction;
        }
    }

    pub fn to_usize(&self, value: i64) -> usize {
        usize::try_from(value).unwrap().to_owned()
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory[self.ip as usize];
            match opcode.into() {
                OpCode::Halt => {
                    break;
                }
                OpCode::Push => {
                    let value = self.memory[(self.ip + 1) as usize];
                    self.stack.push(value);
                    self.ip += 2;
                }
                OpCode::Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
                OpCode::Add => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op1 + op2;
                    self.stack.push(result);
                    self.ip += 1;
                }
                OpCode::Sub => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op2 - op1;
                    self.stack.push(result);
                    self.ip += 1;
                }
                OpCode::Mul => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op1 * op2;
                    self.stack.push(result);
                    self.ip += 1;
                }
                OpCode::Div => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op2 / op1;
                    self.stack.push(result);
                    self.ip += 1;
                }
                OpCode::Jump => {
                    let address = self.memory[(self.ip + 1) as usize];
                    self.ip = address;
                }
                OpCode::JumpIfEqual => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 == op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                OpCode::JumpIfNotEqual => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 != op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                OpCode::JumpIfLessThan => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 < op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                OpCode::JumpIfGreaterThan => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 > op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                OpCode::JumpIfLessThanOrEqual => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 <= op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                OpCode::JumpIfGreaterThanOrEqual => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 >= op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                OpCode::Call => {
                    let address = self.memory[(self.ip + 1) as usize];
                    self.stack.push(self.ip + 2);
                    self.ip = address;
                }
                OpCode::Return => {
                    let address = self.stack.pop().unwrap();
                    self.ip = address;
                }
                OpCode::Print => {
                    let value = self.stack.last().unwrap();
                    println!("{}", value);
                    self.ip += 1;
                }
                OpCode::Store => {
                    let address = self.memory[(self.ip + 1) as usize];
                    let value = self.stack.pop().unwrap();
                    self.memory[address as usize] = value;
                    self.ip += 2;
                }
                OpCode::Load => {
                    let address = self.memory[(self.ip + 1) as usize];
                    let value = self.memory[address as usize];
                    self.stack.push(value);
                    self.ip += 2;
                }
            }
        }
    }
}

pub struct Assembler {
    pub machine_code: Vec<i64>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            machine_code: vec![],
        }
    }

    pub fn assemble(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::Halt => {
                    self.machine_code.push(OpCode::Halt as i64);
                }
                Instruction::Push(value) => {
                    self.machine_code.push(OpCode::Push as i64);
                    self.machine_code.push(value);
                }
                Instruction::Pop => {
                    self.machine_code.push(OpCode::Pop as i64);
                }
                Instruction::Add => {
                    self.machine_code.push(OpCode::Add as i64);
                }
                Instruction::Sub => {
                    self.machine_code.push(OpCode::Sub as i64);
                }
                Instruction::Mul => {
                    self.machine_code.push(OpCode::Mul as i64);
                }
                Instruction::Div => {
                    self.machine_code.push(OpCode::Div as i64);
                }
                Instruction::Jump(address) => {
                    self.machine_code.push(OpCode::Jump as i64);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfEqual(address) => {
                    self.machine_code.push(OpCode::JumpIfEqual as i64);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfNotEqual(address) => {
                    self.machine_code.push(OpCode::JumpIfNotEqual as i64);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfLessThan(address) => {
                    self.machine_code.push(OpCode::JumpIfLessThan as i64);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfGreaterThan(address) => {
                    self.machine_code.push(OpCode::JumpIfGreaterThan as i64);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfLessThanOrEqual(address) => {
                    self.machine_code.push(OpCode::JumpIfLessThanOrEqual as i64);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfGreaterThanOrEqual(address) => {
                    self.machine_code
                        .push(OpCode::JumpIfGreaterThanOrEqual as i64);
                    self.machine_code.push(address);
                }
                Instruction::Call(address) => {
                    self.machine_code.push(OpCode::Call as i64);
                    self.machine_code.push(address);
                }
                Instruction::Return => {
                    self.machine_code.push(OpCode::Return as i64);
                }
                Instruction::Print => {
                    self.machine_code.push(OpCode::Print as i64);
                }
                Instruction::Store(address) => {
                    self.machine_code.push(OpCode::Store as i64);
                    self.machine_code.push(address);
                }
                Instruction::Load(address) => {
                    self.machine_code.push(OpCode::Load as i64);
                    self.machine_code.push(address);
                }
            }
        }
    }
}

mod tests {
    use super::{Assembler, Instruction, OpCode, VirtualMachine};

    #[test]
    fn test_assembler() {
        let mut assembler = Assembler::new();
        assembler.assemble(vec![
            Instruction::Push(1),
            Instruction::Push(2),
            Instruction::Add,
            Instruction::Print,
            Instruction::Halt,
        ]);
        assert_eq!(
            assembler.machine_code,
            vec![
                OpCode::Push as i64,
                1,
                OpCode::Push as i64,
                2,
                OpCode::Add as i64,
                OpCode::Print as i64,
                OpCode::Halt as i64
            ]
        );
    }

    #[test]
    fn test_virtual_machine() {
        let mut vm = VirtualMachine::new(1024, 1024);
        vm.memory = vec![
            OpCode::Push as i64,
            1,
            OpCode::Push as i64,
            2,
            OpCode::Add as i64,
            OpCode::Print as i64,
            OpCode::Halt as i64,
        ];

        vm.execute();
        assert_eq!(
            vm.stack.last().unwrap().to_owned(),
            i64::try_from(3).unwrap()
        );
    }
}
