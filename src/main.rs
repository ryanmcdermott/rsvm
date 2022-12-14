pub struct VirtualMachine {
    ip: usize,
    stack: Vec<usize>,
    memory: Vec<usize>,
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
    Push(usize),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Jump(usize),
    JumpIfEqual(usize),
    JumpIfNotEqual(usize),
    JumpIfLessThan(usize),
    JumpIfGreaterThan(usize),
    JumpIfLessThanOrEqual(usize),
    JumpIfGreaterThanOrEqual(usize),
    Call(usize),
    Return,
    Print,
    Store(usize),
    Load(usize),
    Halt,
}

impl VirtualMachine {
    pub fn new(stack_size: usize, memory_size: usize) -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            stack: vec![0; stack_size],
            memory: vec![0; memory_size],
        }
    }

    pub fn load_program(&mut self, program: &[usize]) {
        assert!(program.len() <= self.memory.len());
        for (i, &instruction) in program.iter().enumerate() {
            self.memory[i] = instruction;
        }
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory[self.ip];
            match opcode {
                // Halt
                0x00 => {
                    break;
                }
                // Push
                0x01 => {
                    let value = self.memory[self.ip + 1];
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
                    let address = self.memory[self.ip + 1];
                    self.ip = address;
                }
                // JumpIfEqual
                0x08 => {
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    let address = self.memory[self.ip + 1];
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
                    let address = self.memory[self.ip + 1];
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
                    let address = self.memory[self.ip + 1];
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
                    let address = self.memory[self.ip + 1];
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
                    let address = self.memory[self.ip + 1];
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
                    let address = self.memory[self.ip + 1];
                    if op1 >= op2 {
                        self.ip = address;
                    } else {
                        self.ip += 2;
                    }
                }
                // Call
                0x0E => {
                    let address = self.memory[self.ip + 1];
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
                    let value = self.stack.pop().unwrap();
                    println!("{}", value);
                    self.ip += 1;
                }
                // Store
                0x11 => {
                    let address = self.memory[self.ip + 1];
                    let value = self.stack.pop().unwrap();
                    self.memory[address] = value;
                    self.ip += 2;
                }
                // Load
                0x12 => {
                    let address = self.memory[self.ip + 1];
                    let value = self.memory[address];
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
    pub machine_code: Vec<usize>,
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
                    self.machine_code.push(OpCode::Halt as usize);
                }
                Instruction::Push(value) => {
                    self.machine_code.push(OpCode::Push as usize);
                    self.machine_code.push(value);
                }
                Instruction::Pop => {
                    self.machine_code.push(OpCode::Pop as usize);
                }
                Instruction::Add => {
                    self.machine_code.push(OpCode::Add as usize);
                }
                Instruction::Sub => {
                    self.machine_code.push(OpCode::Sub as usize);
                }
                Instruction::Mul => {
                    self.machine_code.push(OpCode::Mul as usize);
                }
                Instruction::Div => {
                    self.machine_code.push(OpCode::Div as usize);
                }
                Instruction::Jump(address) => {
                    self.machine_code.push(OpCode::Jump as usize);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfEqual(address) => {
                    self.machine_code.push(OpCode::JumpIfEqual as usize);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfNotEqual(address) => {
                    self.machine_code.push(OpCode::JumpIfNotEqual as usize);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfLessThan(address) => {
                    self.machine_code.push(OpCode::JumpIfLessThan as usize);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfGreaterThan(address) => {
                    self.machine_code.push(OpCode::JumpIfGreaterThan as usize);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfLessThanOrEqual(address) => {
                    self.machine_code
                        .push(OpCode::JumpIfLessThanOrEqual as usize);
                    self.machine_code.push(address);
                }
                Instruction::JumpIfGreaterThanOrEqual(address) => {
                    self.machine_code
                        .push(OpCode::JumpIfGreaterThanOrEqual as usize);
                    self.machine_code.push(address);
                }
                Instruction::Call(address) => {
                    self.machine_code.push(OpCode::Call as usize);
                    self.machine_code.push(address);
                }
                Instruction::Return => {
                    self.machine_code.push(OpCode::Return as usize);
                }
                Instruction::Print => {
                    self.machine_code.push(OpCode::Print as usize);
                }
                Instruction::Store(address) => {
                    self.machine_code.push(OpCode::Store as usize);
                    self.machine_code.push(address);
                }
                Instruction::Load(address) => {
                    self.machine_code.push(OpCode::Load as usize);
                    self.machine_code.push(address);
                }
            }
        }
    }
}

fn main() {
    // Program that calculates (6 * 5 + 4 - 3 - 1) / 2
    // Result: 15
    let instructions = vec![
        Instruction::Push(6),
        Instruction::Push(5),
        Instruction::Mul,
        Instruction::Push(4),
        Instruction::Add,
        Instruction::Push(3),
        Instruction::Sub,
        Instruction::Push(1),
        Instruction::Sub,
        Instruction::Push(2),
        Instruction::Div,
        Instruction::Print,
        Instruction::Halt,
    ];

    let mut assembler = Assembler::new();
    assembler.assemble(instructions);
    let mut vm = VirtualMachine::new(1024, 1024);
    vm.load_program(&assembler.machine_code);
    vm.execute();
}
