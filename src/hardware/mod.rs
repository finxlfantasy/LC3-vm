pub mod instruction;
pub mod register;
pub mod vm;

use vm::VM;

pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

pub fn execute_program(vm: &mut VM) {
    while vm.registers.pc < MEMORY_SIZE as u16 {
        // Reads Instructions
        let instruction = vm.read_memory(vm.registers.pc);
        
        // Increment program counter
        vm.registers.pc += 1;

        // Extracting op_code and execute operation
        instruction::execute_instruction(instruction, vm)
    }
} 