use crate::instr::{BInstr, IInstr, JInstr, RInstr, SInstr, UInstr, encoding::{Encoding, OPCODE_MASK}, imm::Imm};

/// Reg should never be > 0b11111
pub type Reg = u32;

/// Resulting type from a pipelined instruction decode
/// Uses tuple type variants for constructor syntax.
/// Worth it for the documentation necessary to access values.
/// Arith: rd, rs1, rs2
/// Immediate: rd, rs1, imm
/// Loads: rd, rs1, imm
/// Stores: rs1, rs2, imm
/// Branches: rs1, rs2, imm
/// Jumps w/ Link:
///   - rd, imm
///   - rd, rs1, imm
/// Upper Imm: rd, imm
/// Env
pub enum RV32I {
    // Arith: rd, rs1, rs2
    Add    (Reg, Reg, Reg),
    Sub    (Reg, Reg, Reg),
    Xor    (Reg, Reg, Reg),
    Or     (Reg, Reg, Reg),
    And    (Reg, Reg, Reg),
    Sll    (Reg, Reg, Reg),
    Srl    (Reg, Reg, Reg),
    Sra    (Reg, Reg, Reg),
    Slt    (Reg, Reg, Reg),
    Sltu   (Reg, Reg, Reg),
    // Immediate: rd, rs1, imm
    Addi   (Reg, Reg, u32),
    Subi   (Reg, Reg, u32),
    Xori   (Reg, Reg, u32),
    Ori    (Reg, Reg, u32),
    Andi   (Reg, Reg, u32),
    Slli   (Reg, Reg, u32),
    Srli   (Reg, Reg, u32),
    Srai   (Reg, Reg, u32),
    Slti   (Reg, Reg, u32),
    Sltiu  (Reg, Reg, u32),
    // Loads: rd, rs1, imm
    Lb     (Reg, Reg, u32),
    Lh     (Reg, Reg, u32),
    Lw     (Reg, Reg, u32),
    Lbu    (Reg, Reg, u32),
    Lhu    (Reg, Reg, u32),
    // Stores: rs1, rs2, imm
    Sb     (Reg, Reg, u32),
    Sh     (Reg, Reg, u32),
    Sw     (Reg, Reg, u32),
    // Branches: rs1, rs2, imm
    Beq    (Reg, Reg, u32),
    Bne    (Reg, Reg, u32),
    Blt    (Reg, Reg, u32),
    Bge    (Reg, Reg, u32),
    Bltu   (Reg, Reg, u32),
    Bgeu   (Reg, Reg, u32),
    // Jumps w/ Link
    // rd, imm
    Jal    (Reg, u32),
    // rd, rs1, imm
    Jalr   (Reg, Reg, u32),
    // Upper Imm: rd, imm
    Lui    (Reg, u32),
    Aupic  (Reg, u32),
    // Env
    Ecall,
    Ebreak
}

const ARITH_OP:  u32 = 0b0110011;
const IMM_OP:    u32 = 0b0010011;
const LOAD_OP:   u32 = 0b0000011;
const STORE_OP:  u32 = 0b0100011;
const BRANCH_OP: u32 = 0b1100011;
const JUMP_OP:   u32 = 0b1101111;
const JUMPL_OP:  u32 = 0b1100111;
const LOADUI_OP: u32 = 0b0110111;
const AUIPC_OP:  u32 = 0b0010111;
const ENV_OP:    u32 = 0b1110011;

pub(crate) mod funct_consts {
    pub const FUNCT3_ADD:  u32 = 0x0;
    pub const FUNCT7_ADD:  u32 = 0x00;
    pub const FUNCT3_SUB:  u32 = 0x0;
    pub const FUNCT7_SUB:  u32 = 0x20;
    pub const FUNCT3_XOR:  u32 = 0x4;
    pub const FUNCT7_XOR:  u32 = 0x00;
    pub const FUNCT3_OR:   u32 = 0x6;
    pub const FUNCT7_OR:   u32 = 0x00;
    pub const FUNCT3_AND:  u32 = 0x7;
    pub const FUNCT7_AND:  u32 = 0x00;
    pub const FUNCT3_SLL:  u32 = 0x1;
    pub const FUNCT7_SLL:  u32 = 0x00;
    pub const FUNCT3_SRL:  u32 = 0x5;
    pub const FUNCT7_SRL:  u32 = 0x00;
    pub const FUNCT3_SRA:  u32 = 0x5;
    pub const FUNCT7_SRA:  u32 = 0x20;
    pub const FUNCT3_SLT:  u32 = 0x2;
    pub const FUNCT7_SLT:  u32 = 0x00;
    pub const FUNCT3_SLTU: u32 = 0x3;
    pub const FUNCT7_SLTU: u32 = 0x00;

    pub const FUNCT3_ADDI:  u32 = 0x0;
    pub const FUNCT3_XORI:  u32 = 0x4;
    pub const FUNCT3_ORI:   u32 = 0x6;
    pub const FUNCT3_ANDI:  u32 = 0x7;
    pub const FUNCT3_SLLI:  u32 = 0x1;
    pub const FUNCT3_SRLI:  u32 = 0x5;
    pub const FUNCT3_SRAI:  u32 = 0x5;
    pub const FUNCT3_SLTI:  u32 = 0x2;
    pub const FUNCT3_SLTIU: u32 = 0x3;

    pub const FUNCT3_LB:  u32 = 0x0;
    pub const FUNCT3_LH:  u32 = 0x1;
    pub const FUNCT3_LW:  u32 = 0x2;
    pub const FUNCT3_LBU: u32 = 0x4;
    pub const FUNCT3_LHU: u32 = 0x5;

    pub const FUNCT3_SB:  u32 = 0x0;
    pub const FUNCT3_SH:  u32 = 0x1;
    pub const FUNCT3_SW:  u32 = 0x2;

    pub const FUNCT3_BEQ:  u32 = 0x0;
    pub const FUNCT3_BNE:  u32 = 0x1;
    pub const FUNCT3_BLT:  u32 = 0x4;
    pub const FUNCT3_BGE:  u32 = 0x5;
    pub const FUNCT3_BLTU: u32 = 0x6;
    pub const FUNCT3_BGEU: u32 = 0x7;
}
use funct_consts::*;

#[rustfmt::skip]
fn decode(instr: u32) -> RV32I {
    match instr & OPCODE_MASK {
        ARITH_OP => {
            let op = RInstr(instr);
            let funct3 = op.get_funct3().unwrap();
            let funct7 = op.get_funct7().unwrap();
            let rd     = op.get_rd().unwrap();
            let rs1    = op.get_rs1().unwrap();
            let rs2    = op.get_rs2().unwrap();

            (match (funct3, funct7) {
                (FUNCT3_ADD, FUNCT7_ADD) => RV32I::Add,
                (FUNCT3_SUB, FUNCT7_SUB) => RV32I::Sub,
                (FUNCT3_XOR, FUNCT7_XOR) => RV32I::Xor,
                (FUNCT3_OR,  FUNCT7_OR ) => RV32I::Or ,
                (FUNCT3_AND, FUNCT7_AND) => RV32I::And,
                (FUNCT3_SLL, FUNCT7_SLL) => RV32I::Sll,
                (FUNCT3_SRL, FUNCT7_SRL) => RV32I::Srl,
                (FUNCT3_SRA, FUNCT7_SRA) => RV32I::Sra,
                (FUNCT3_SLT, FUNCT7_SLT) => RV32I::Slt,
                _ => { panic!("Illegal Instruction") }
            })(
                rd,
                rs1,
                rs2,
            )
        }
        IMM_OP => {
            let op = IInstr(instr);
            let funct3 = op.get_funct3().unwrap();
            let rd     = op.get_rd().unwrap();
            let rs1    = op.get_rs1().unwrap();
            let imm = op.get_imm();

            (match funct3 {
                FUNCT3_ADDI  => RV32I::Addi,
                FUNCT3_XORI  => RV32I::Xori,
                FUNCT3_ORI   => RV32I::Ori,
                FUNCT3_ANDI  => RV32I::Andi,
                FUNCT3_SLLI  => RV32I::Slli,
                FUNCT3_SRLI  => RV32I::Srli,
                FUNCT3_SRAI  => RV32I::Srai,
                FUNCT3_SLTI  => RV32I::Slti,
                FUNCT3_SLTIU => RV32I::Sltiu,
                _ => { panic!("Illegal Instruction") }
            })(
                rd,
                rs1,
                imm,
            )
        }
        LOAD_OP => {
            let op = IInstr(instr);
            let funct3 = op.get_funct3().unwrap();
            let rd     = op.get_rd().unwrap();
            let rs1    = op.get_rs1().unwrap();
            let imm    = op.get_imm();

            (match funct3 {
                FUNCT3_LB  => RV32I::Lb,
                FUNCT3_LH  => RV32I::Lh,
                FUNCT3_LW  => RV32I::Lw,
                FUNCT3_LBU => RV32I::Lbu,
                FUNCT3_LHU => RV32I::Lhu,
                _ => { panic!("Illegal Instruction") }
            })(
                rd,
                rs1,
                imm,
            )
        }
        STORE_OP => {
            let op = SInstr(instr);
            let funct3 = op.get_funct3().unwrap();
            let rs1    = op.get_rs1().unwrap();
            let rs2    = op.get_rs2().unwrap();
            let imm    = op.get_imm();

            (match funct3 {
                FUNCT3_LB  => RV32I::Lb,
                FUNCT3_LH  => RV32I::Lh,
                FUNCT3_LW  => RV32I::Lw,
                FUNCT3_LBU => RV32I::Lbu,
                FUNCT3_LHU => RV32I::Lhu,
                _ => { panic!("Illegal Instruction") }
            })(
                rs1,
                rs2,
                imm,
            )
        }
        BRANCH_OP => {
            let op = BInstr(SInstr(instr));
            let funct3 = op.get_funct3().unwrap();
            let rs1    = op.get_rs1().unwrap();
            let rs2    = op.get_rs2().unwrap();
            let imm    = op.get_imm();

            (match funct3 {
                FUNCT3_BEQ  => RV32I::Beq,
                FUNCT3_BNE  => RV32I::Bne,
                FUNCT3_BLT  => RV32I::Blt,
                FUNCT3_BGE  => RV32I::Bge,
                FUNCT3_BLTU => RV32I::Bltu,
                FUNCT3_BGEU => RV32I::Bgeu,
                _ => { panic!("Illegal Instruction") }
            })(
                rs1,
                rs2,
                imm,
            )
        }
        JUMP_OP => {
            let op = JInstr(UInstr(instr));
            let rd  = op.get_rd().unwrap();
            let imm = op.get_imm();

            RV32I::Jal(rd, imm)
        }
        JUMPL_OP => {
            let op = UInstr(instr);
            let rd  = op.get_rd().unwrap();
            let rs1 = op.get_rs1().unwrap();
            let imm = op.get_imm();

            RV32I::Jalr(rd, rs1, imm)
        }
        funct @ (LOADUI_OP | AUIPC_OP) => {
            let op = UInstr(instr);
            let rd  = op.get_rd().unwrap();
            let imm = op.get_imm();

            (match funct {
                LOADUI_OP => RV32I::Lui,
                AUIPC_OP  => RV32I::Aupic,
                _ => { panic!("Impossible Branch") }
            })(
                rd,
                imm
            )
        }
        ENV_OP => {
            let op = IInstr(instr);
            let funct3 = op.get_funct3().unwrap();
            let imm = op.get_imm();

            match (funct3, imm) {
                (0x0, 0x0) => RV32I::Ecall,
                (0x0, 0x1) => RV32I::Ebreak,
                _ => { panic!("Illegal Instruction") }
            }
        }
        _ => { panic!("Illegal Instruction") }
    }
}
