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
pub fn add(instruction: u16, vm: &mut VM) {
    let dr = (instruction  >> 9) & 0x7;

    let sr1 = (instruction >> 6) &0x7;

    let imm_flag = (instruction >> 5) & 0x1;
    if imm_flag == 1 {
        let im5 = sign_extend(instruction & 0x1F, 5);

        let val: u32 = im5 as u32 + vm.registers.get(sr1) as u32;
        vm.registers.update(dr, val as u16);
    } else {
        let s42 = instruction & 0x7;
        let val: u32 = vm.registers.get(sr1) as u32 + vm.registers.get(sr2) as u32;
        vm.registers.update(dr, val as u16);
    }
    vm.registers.update_r_cond_register(dr);
}

pub fn not(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7;
    vm.registers.update(dr, !vm.registers.get(sr1));

    vm.registers.update_r_cond_register(dr);
}

pub fn trap(instruction: u16, vm: &mut VM) {
    println!("trap instruction: {:#018b}\n", instruction);

    match instruction & 0xFF {
        0x20 => {}
        0x21 => {}
        0x22 => {
            //Puts 
            let mut index = vm.registers.r0;

            let mut c = vm.read_memory(index);

            while c != 0x0000 {
                print!("{}", as u8 as char);
                index += 1;
                c = vm.read_memory(index);
        }
        io::stdout().flush().unwrap();
    }
        0x23 => {}
        0x24 => {}
        0x25 => {
            println!("HALT detected");
            io::stdout().flush().expect("Failed to flush");
            process::exit(1);
        }
        _ => {
            process::exit(1);
            
        }
    }
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