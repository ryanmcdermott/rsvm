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
            match opcode {
                // Halt
                0x00 => {
                    break;
                }
                // Push
                0x01 => {
                    let value = self.memory[(self.ip + 1) as usize];
                    self.stack.push(value);
                    self.ip += 2;
                }
                // Pop
                0x02 => {
                    self.stack.pop();
                    self.ip += 1;
                }
                // Add
                0x03 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op1 + op2;
                    self.stack.push(result);
                    self.ip += 1;
                }
                // Sub
                0x04 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op2 - op1;
                    self.stack.push(result);
                    self.ip += 1;
                }
                // Mul
                0x05 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op1 * op2;
                    self.stack.push(result);
                    self.ip += 1;
                }
                // Div
                0x06 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let result = op2 / op1;
                    self.stack.push(result);
                    self.ip += 1;
                }
                // Jump
                0x07 => {
                    let address = self.memory[(self.ip + 1) as usize];
                    self.ip = address;
                }
                // JumpIfEqual
                0x08 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 == op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // JumpIfNotEqual
                0x09 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 != op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // JumpIfLessThan
                0x0A => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 < op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // JumpIfGreaterThan
                0x0B => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 > op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // JumpIfLessThanOrEqual
                0x0C => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 <= op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // JumpIfGreaterThanOrEqual
                0x0D => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[(self.ip + 1) as usize];
                    if op1 >= op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // Call
                0x0E => {
                    let address = self.memory[(self.ip + 1) as usize];
                    self.stack.push(self.ip + 2);
                    self.ip = address;
                }
                // Return
                0x0F => {
                    let address = self.stack.pop().unwrap();
                    self.ip = address;
                }
                // Print
                0x10 => {
                    let value = self.stack.last().unwrap();
                    println!("{}", value);
                    self.ip += 1;
                }
                // Store
                0x11 => {
                    let address = self.memory[(self.ip + 1) as usize];
                    let value = self.stack.pop().unwrap();
                    self.memory[address as usize] = value;
                    self.ip += 2;
                }
                // Load
                0x12 => {
                    let address = self.memory[(self.ip + 1) as usize];
                    let value = self.memory[address as usize];
                    self.stack.push(value);
                    self.ip += 2;
                }
                _ => {
                    panic!("unknown opcode: {}", opcode);
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
    use super::{Assembler, Instruction, VirtualMachine};

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
            vec![0x01, 1, 0x01, 2, 0x03, 0x10, 0x00]
        );
    }

    #[test]
    fn test_virtual_machine() {
        let mut vm = VirtualMachine::new(1024, 1024);
        vm.memory = vec![0x01, 1, 0x01, 2, 0x03, 0x10, 0x00];
        vm.execute();
        assert_eq!(
            vm.stack.last().unwrap().to_owned(),
            i64::try_from(3).unwrap()
        );
    }
}
