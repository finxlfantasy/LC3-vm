
pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers, 
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }

    pub fn read_memory(&mut self, address: u16) -> u16 {
        if address == MemoryMappedReg::kbsr as u16 {
            self.handle_keyboard();
        }
        self.memory[address as usize]
    }

    fn handle_keyboard(&mut self) {
        let mut buffer = [0; 1];
        std::io_stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write_memory(MemoryMappedReg::kbsr as usize, 1 << 15);
            self.write_memory(MemoryMappedReg::kbdr as usize, buffer[0] as u16);
        } else {
            self.write_memory(MemoryMappedReg::kbsr as uszie, 0)
        }
    }

    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub enum MemoryMappedReg {
        kbsr = 0xFE00,

        kbdr = 0xFE02,
    }

}