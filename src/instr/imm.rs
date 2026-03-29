use super::{encoding::Encoding, BInstr, IInstr, JInstr, SInstr, UInstr};

const SIGN_EXT_OFFSET: u32 = 31;
const SIGN_EXT_MASK: u32 = 0b1 << SIGN_EXT_OFFSET;

pub trait Imm: Encoding {
    fn get_imm(self) -> u32;
}

impl Imm for IInstr {
    fn get_imm(self) -> u32 {
        let num = self.get_num();
        let sign_ext = (num & SIGN_EXT_MASK) >> SIGN_EXT_OFFSET;
        let base = if sign_ext == 1 { 0b111111111111111111111 << 11 } else { 0b0 };
        let low = (num & (0b11111111111 << 19)) >> 19;

        base | low
    }
}

impl Imm for SInstr {
    fn get_imm(self) -> u32 {
        let num = self.get_num();
        let sign_ext = (num & SIGN_EXT_MASK) >> SIGN_EXT_OFFSET;
        let base = if sign_ext == 1 { 0b111111111111111111111 << 11 } else { 0b0 };

        let mid = (num & (0b111111 << 25)) >> (25 - 5);

        let low = (num & (0b11111 << 7)) >> 7;

        base | mid | low
    }
}

impl Imm for BInstr {
    fn get_imm(self) -> u32 {
        let num = self.get_num();
        let sign_ext = (num & SIGN_EXT_MASK) >> SIGN_EXT_OFFSET;
        let base = if sign_ext == 1 { 0b11111111111111111111 << 12 } else { 0b0 };

        let mid_high = (num & (0b1 << 7)) << 4;

        let mid_low = (num & (0b111111 << 25)) >> (25 - 5);

        let low = (num & (0b1111 << 8)) >> 7;

        base | mid_high | mid_low | low
    }
}

impl Imm for UInstr {
   fn get_imm(self) -> u32 {
        let num = self.get_num();

        num & (0b11111111111111111111 << 12) >> 12
   } 
}

impl Imm for JInstr {
   fn get_imm(self) -> u32 {
        let num = self.get_num();
        let sign_ext = (num & SIGN_EXT_MASK) >> SIGN_EXT_OFFSET;
        let base = if sign_ext == 1 { 0b11111111111111111111 << 12 } else { 0b0 };

        

        todo!()
   } 
}
