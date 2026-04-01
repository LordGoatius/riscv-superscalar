pub mod encoding;
pub mod decoding;
pub mod imm;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RInstr(u32);
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IInstr(u32);
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SInstr(u32);
/// B is a type of S encoding for different imm encoding
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BInstr(SInstr);
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct UInstr(u32);
/// J is a type of U encoding for different imm encoding
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct JInstr(UInstr);

