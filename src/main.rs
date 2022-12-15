use rsvm::vm::{Assembler, Instruction, VirtualMachine};

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
