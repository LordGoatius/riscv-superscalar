use super::*;

/// all instructions have the opcode in the same location, and it is always
/// 7 bits long.
pub const OPCODE_MASK: u32 = 0b1111111;
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

// # Safety
// I offer no defense for my behavior with this trait. I have bent and mutilated
// Rust to my will. (Don't use on non-u32 transparent types for intended behavior.
// Don't use on anything that can't be safely dereferenced as a u32 to avoid UB).
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

// The DRY that OOP fears.

macro_rules! define_encoding {
    ($($func:ident),*) => {
        $(
            fn $func(self) -> Option<u32> {
                None
            }
        )*
    };
    (($name:ident, $($func:ident),*)) => {
        unsafe impl Encoding for $name {
            define_encoding!($($func),*);
        }
    };
    ($($tt:tt),+ $(,)?) => {
        $(
            define_encoding!($tt);
        )+
    };
}

define_encoding!(
    (RInstr, ),
    (IInstr, get_funct7, get_rs2),
    (SInstr, get_funct7, get_rd),
    (BInstr, get_funct7, get_rd),
    (UInstr, get_funct7, get_funct3, get_rs1, get_rs2),
    (JInstr, get_funct7, get_funct3, get_rs1, get_rs2),
);
