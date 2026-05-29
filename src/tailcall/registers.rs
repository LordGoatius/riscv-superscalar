use crate::instr::decoding::Reg;

pub struct Registers {
    regs: [u32; 31],
}

impl Registers {
    pub fn get(&self, reg: Reg) -> u32 {
        if reg == 0 {
            0
        } else {
            self.regs[reg as usize + 1]
        }
    }

    pub fn set(&mut self, reg: Reg, val: u32) {
        if reg == 0 {
            return
        } else {
            self.regs[reg as usize + 1] = val;
        }
    }
}
