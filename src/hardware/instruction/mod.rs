pub enum OpCode {
    BR = 0, // branch
    ADD, // add
    LD,  // load
    KSR, // jump register
    AND, // bitwise and
    LDR, // load register
    STR, // store register 
    RTI, // unused
    NOT, // bitwise not
    LDI, // load indirect
    STI, // store indirect
    JMP, // jump
    RES, // reserved (unused)
    LEA, // load effective address
    TRAP, // excecute trap
}

pub fn get_op_code(instruction: &u16) -> Option<OpCode> {
    match instruction >> 12 {
        0 => Some(OpCode::BR),
        1 => Some(OpCode::ADD),
        2 => Some(OpCode::LD),
        3 => Some(OpCode::ST),
        4 => Some(OpCode::JSR),
        5 => Some(OpCode::AND),
        6 => Some(OpCode::LDR),
        7 => Some(OpCode::STR),
        8 => Some(OpCode::RTI),
        9 => Some(OpCode::NOT),
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::RES),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        15 => Some(OpCode::TRAP),
        _ => None,
    }
}

pub fn execute_instruction(instr: u16, vm: &mut VM) {
    let op_code = get_op_code(&instr);
    match op_code {
        // op_code and execute instruction
        Some(OpCode::ADD) => add(instr, vm),
        Some(OpCode::AND) => and(instr, vm),
        Some(OpCode::NOT) => not(instr, vm),
        Some(OpCode::BR) => br(instr, vm),
        Some(OpCode::JMP) => jmp(instr, vm),
        Some(OpCode::JSR) => jsr(instr, vm),
        Some(OpCode::LD) => ld(instr, vm),
        Some(OpCode::LDI) => ldi(instr, vm),
        Some(OpCode::LDR) => ldr(instr, vm),
        Some(OpCode::LEA) => lea(instr, vm),
        Some(OpCode::ST) => st(instr, vm),
        Some(OpCode::STI) => sti(instr, vm),
        Some(OpCode::STR) => str(instr, vm),
        Some(OpCode::TRAP) => trap(instr, vm),
        _ => {}
    }


    pub fn ldi(instruction: u16, vm: &mut VM) {
        let dr = (instruction >> 9) & 0x7;

        // Fetch PC offset and sign it to become 16 bits
        let pc_offset = sign_extend(instuction & 0x1ff, 9);

    }


    pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
        if (x >> (bit_count -1)) & 1 != 0 {
            x |= 0xFFFF << bit_count;
        }
        x
    }
}