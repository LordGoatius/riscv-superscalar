use crate::instr::encoding::OPCODE_MASK;

/// Reg should never be > 0b11111
pub type Reg = u32;

// Resulting type from a pipelined instruction decode
pub enum RV32I {
    Add    { rd: Reg, rs1: Reg, rs2: Reg },
    Sub    { rd: Reg, rs1: Reg, rs2: Reg },
    Xor    { rd: Reg, rs1: Reg, rs2: Reg },
    Or     { rd: Reg, rs1: Reg, rs2: Reg },
    And    { rd: Reg, rs1: Reg, rs2: Reg },
    Sll    { rd: Reg, rs1: Reg, rs2: Reg },
    Srl    { rd: Reg, rs1: Reg, rs2: Reg },
    Sra    { rd: Reg, rs1: Reg, rs2: Reg },
    Slt    { rd: Reg, rs1: Reg, rs2: Reg },
    Sltu   { rd: Reg, rs1: Reg, rs2: Reg },
    // Immediate
    Addi   { rd: Reg, rs1: Reg, imm: u32 },
    Subi   { rd: Reg, rs1: Reg, imm: u32 },
    Xori   { rd: Reg, rs1: Reg, imm: u32 },
    Ori    { rd: Reg, rs1: Reg, imm: u32 },
    Andi   { rd: Reg, rs1: Reg, imm: u32 },
    Slli   { rd: Reg, rs1: Reg, imm: u32 },
    Srli   { rd: Reg, rs1: Reg, imm: u32 },
    Srai   { rd: Reg, rs1: Reg, imm: u32 },
    Slti   { rd: Reg, rs1: Reg, imm: u32 },
    Sltui  { rd: Reg, rs1: Reg, imm: u32 },
    // Loads
    Lb     { rd: Reg, rsi: Reg, imm: u32 },
    Lh     { rd: Reg, rsi: Reg, imm: u32 },
    Lw     { rd: Reg, rsi: Reg, imm: u32 },
    Lbu    { rd: Reg, rsi: Reg, imm: u32 },
    Lhu    { rd: Reg, rsi: Reg, imm: u32 },
    // Stores
    Sb     { rs1: Reg, rs2: Reg, imm: u32 },
    Sh     { rs1: Reg, rs2: Reg, imm: u32 },
    Sw     { rs1: Reg, rs2: Reg, imm: u32 },
    // Branches
    Beq    { rs1: Reg, rs2: Reg, imm: u32 },
    Bne    { rs1: Reg, rs2: Reg, imm: u32 },
    Blt    { rs1: Reg, rs2: Reg, imm: u32 },
    Bge    { rs1: Reg, rs2: Reg, imm: u32 },
    Bltu   { rs1: Reg, rs2: Reg, imm: u32 },
    Bgeu   { rs1: Reg, rs2: Reg, imm: u32 },
    // Jumps w/ Link
    Jal    { rd: Reg, imm: u32 },
    Jalr   { rd: Reg, rs1: Reg, imm: u32 },
    // Upper Imm
    Lui    { rd: Reg, imm: u32 },
    Aupic  { rd: Reg, imm: u32 },
    // Env
    Ecall  {},
    Ebreak {}
}

const ARITH_OP:  u32 = 0b0110011;
const IMM_OP:    u32 = 0b0010011;
const LOAD_OP:   u32 = 0b0000011;
const STORE_OP:  u32 = 0b0100011;
const BRANCH_OP: u32 = 0b1100011;
const JUMP_OP:   u32 = 0b1101111;
const JUMPL_OP:  u32 = 0b1101111;
const LOADUI_OP: u32 = 0b0110111;
const AUIPC_OP:  u32 = 0b0010111;
const ENV_OP:    u32 = 0b1110011;

pub fn decode(instr: u32) -> RV32I {
    match instr & OPCODE_MASK {
        ARITH_OP => {
            
        }
        IMM_OP => {
            
        }
        LOAD_OP => {
            
        }
        STORE_OP => {
            
        }
        BRANCH_OP => {
            
        }
        JUMP_OP => {
            
        }
        JUMPL_OP => {
            
        }
        LOADUI_OP => {
            
        }
        AUIPC_OP => {
            
        }
        ENV_OP => {
            
        }
        _ => { panic!("Illegal Instruction") }
    }

    todo!()
}
