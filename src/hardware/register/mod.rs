pub struct Register {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
 
}

impl Register {
    pub fn new() -> Self {
        Register {
            r0: 0,              // general purpose register
            r1: 0,              // general purpose register
            r2: 0,              // general purpose register
            r3: 0,              // general purpose register  
            r4: 0,              // general purpose register
            r5: 0,              // general purpose register
            r6: 0,              // general purpose register
            r7: 0,              // general purpose register
            pc: PC_START,       // program counter
            cond: 0,            // condition flag
        }
    }

    pub fn get(&self, index: u8) -> Option<u16> {
        match index {
            0 => Some(self.r0),
            1 => Some(self.r1),
            2 => Some(self.r2),
            3 => Some(self.r3),
            4 => Some(self.r4),
            5 => Some(self.r5),
            6 => Some(self.r6),
            7 => Some(self.r7),
            _ => None,
        }
    }

    pub fn update(&mut self, index: u8,  usize: u16) -> bool {
        match index {
            0 => { self.r0 = value;  true },
            1 => { self.r1 = value;  true },
            2 => { self.r2 = value;  true },
            3 => { self.r3 = value;  true },
            4 => { self.r4 = value;  true },
            5 => { self.r5 = value;  true },
            6 => { self.r6 = value;  true },
            7 => { self.r7 = value;  true },
            8 => { self.pc = value; true },
            9 => { self.cond = value; true },
            _ => false,
        }
    }
}


enum Conditionflags {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}