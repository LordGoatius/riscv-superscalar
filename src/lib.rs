pub struct RInstr(u32);
pub struct IInstr(u32);
pub struct SInstr(u32);
/// B is a type of S encoding for Sign Extended imm
pub struct BInstr(u32);
pub struct UInstr(u32);
/// J is a type of U encoding for Sign Extended imm
pub struct JInstr(u32);

pub trait Encoding {
    fn get_funct7(self) -> Option<u32>;
    fn get_funct3(self) -> Option<u32>;
    fn get_rd(self) -> Option<u32>;
    fn get_rs1(self) -> Option<u32>;
    fn get_rs2(self) -> Option<u32>;
    fn get_opcode(self) -> u32;
}

