use super::*;

/// all instructions have the opcode in the same location, and it is always
/// 7 bits long.
const OPCODE_MASK: u32 = 0b1111111;
/// All operands are always at the same location in all encodings.
const REG_MASK: u32 = 0b11111;
const RD_OFFSET: u32 = 7;
const RD_MASK: u32 = REG_MASK << RD_OFFSET;
const FUNCT3_OFFSET: u32 = 12;
const FUNCT3_MASK: u32 = 0b111 << FUNCT3_OFFSET;
const RS1_OFFSET: u32 = 15;
const RS1_MASK: u32 = REG_MASK << RS1_OFFSET;
const RS2_OFFSET: u32 = 20;
const RS2_MASK: u32 = REG_MASK << RS2_OFFSET;
const FUNCT7_OFFSET: u32 = 25;
const FUNCT7_MASK: u32 = 0b1111111 << FUNCT7_OFFSET;

// I offer no defense for my behavior using this trait. I have bent and mutilated
// Rust to my will.
pub unsafe trait Encoding: Sized {
    fn get_funct7(self) -> Option<u32> {
        Some((self.get_num() & FUNCT7_MASK) >> FUNCT7_OFFSET)
    }

    fn get_funct3(self) -> Option<u32> {
        Some((self.get_num() & FUNCT3_MASK) >> FUNCT3_OFFSET)
    }

    fn get_rd(self) -> Option<u32> {
        Some((self.get_num() & RD_MASK) >> RD_OFFSET)
    }

    fn get_rs1(self) -> Option<u32> {
        Some((self.get_num() & RS1_MASK) >> RS1_OFFSET)
    }

    fn get_rs2(self) -> Option<u32> {
        Some((self.get_num() & RS2_MASK) >> RS2_OFFSET)
    }

    fn get_opcode(self) -> u32 {
        self.get_num() & OPCODE_MASK
    }

    #[inline]
    fn get_num(self) -> u32 {
        unsafe {
            (&raw const self).cast::<u32>().read()
        }
    }
}

// TODO? Macro this at some point. Not that it matters now.
unsafe impl Encoding for RInstr {}
unsafe impl Encoding for IInstr {
    fn get_funct7(self) -> Option<u32> {
        None
    }
    fn get_rs2(self) -> Option<u32> {
        None
    }
}
unsafe impl Encoding for SInstr {
    fn get_rd(self) -> Option<u32> {
        None
    }
    fn get_funct7(self) -> Option<u32> {
        None
    }
}
unsafe impl Encoding for BInstr {
    fn get_rd(self) -> Option<u32> {
        None
    }
    fn get_funct7(self) -> Option<u32> {
        None
    }
}
unsafe impl Encoding for UInstr {
    fn get_funct7(self) -> Option<u32> {
        None
    }
    fn get_funct3(self) -> Option<u32> {
        None
    }
    fn get_rs1(self) -> Option<u32> {
        None
    }
    fn get_rs2(self) -> Option<u32> {
        None
    }
}
unsafe impl Encoding for JInstr {
    fn get_funct7(self) -> Option<u32> {
        None
    }
    fn get_funct3(self) -> Option<u32> {
        None
    }
    fn get_rs1(self) -> Option<u32> {
        None
    }
    fn get_rs2(self) -> Option<u32> {
        None
    }
}
