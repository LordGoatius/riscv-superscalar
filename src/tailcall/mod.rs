//! Tailcall interpreter for RISCV

pub mod dispatch;

use crate::{instr::{decoding::Reg, encoding::Encoding, imm::Imm, *}, tailcall::dispatch::{Exception, ExceptionType}};

struct Machine {
    ram: Vec<u8>,
}

// NOTE: SO much of this could be macro'd out.

// Arith: rd, rs1, rs2
fn r_ops(instr: RInstr) -> (Reg, Reg, Reg) {
    let rd = instr.get_rd().unwrap();
    let rs1 = instr.get_rs1().unwrap();
    let rs2 = instr.get_rs2().unwrap();
    (rd, rs1, rs2)
}

// Immediate: rd, rs1, imm
fn i_ops(instr: IInstr) -> (Reg, Reg, u32) {
    let rd = instr.get_rd().unwrap();
    let rs1 = instr.get_rs1().unwrap();
    let imm = instr.get_imm();
    (rd, rs1, imm)
}

// Stores: rs1, rs2, imm
fn s_ops(instr: SInstr) -> (Reg, Reg, u32) {
    let rs1 = instr.get_rs1().unwrap();
    let rs2 = instr.get_rs2().unwrap();
    let imm = instr.get_imm();
    (rs1, rs2, imm)
}

// Branches: rs1, rs2, imm
fn b_ops(instr: BInstr) -> (Reg, Reg, u32) {
    let rs1 = instr.get_rs1().unwrap();
    let rs2 = instr.get_rs2().unwrap();
    let imm = instr.get_imm();
    (rs1, rs2, imm)
}

// Upper Imm: rd, imm
fn u_ops(instr: UInstr) -> (Reg, u32) {
    let rd = instr.get_rd().unwrap();
    let imm = instr.get_imm();
    (rd, imm)
}

// Jump: rd, imm
fn j_ops(instr: JInstr) -> (Reg, u32) {
    let rd = instr.get_rd().unwrap();
    let imm = instr.get_imm();
    (rd, imm)
}

pub type Pc = u32;

impl Machine {
    fn next(&mut self, pc: Pc) -> Result<RV32I, ExceptionType> {
        todo!()
    }
    // Arith: rd, rs1, rs2
    fn add   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn sub   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn xor   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn or    (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn and   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn sll   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn srl   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn sra   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn slt   (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }
    fn sltu  (&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Pc { todo!() }

    // Immediate: rd, rs1, imm
    fn addi  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn subi  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn xori  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn ori   (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn andi  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn slli  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn srli  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn srai  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn slti  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn sltiu (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }

    // Loads: rd, rs1, imm
    fn lb    (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn lh    (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn lw    (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn lbu   (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }
    fn lhu   (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }

    // Stores: rs1, rs2, imm
    fn sb    (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn sh    (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn sw    (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }

    // Branches: rs1, rs2, imm
    fn beq   (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn bne   (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn blt   (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn bge   (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn bltu  (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }
    fn bgeu  (&mut self, rs1: Reg, rs2: Reg, imm: u32) -> Pc { todo!() }

    // Jumps w/ Link
    // rd, imm
    fn jal   (&mut self, rd: Reg, imm: u32) -> Pc { todo!() }

    // rd, rs1, imm
    fn jalr  (&mut self, rd: Reg, rs1: Reg, imm: u32) -> Pc { todo!() }

    // Upper Imm: rd, imm
    fn lui   (&mut self, rd: Reg, imm: u32) -> Pc { todo!() }
    fn aupic (&mut self, rd: Reg, imm: u32) -> Pc { todo!() }

    // Env
    fn ecall (&mut self, ) -> Pc { todo!() }
    fn ebreak(&mut self, ) -> Pc { todo!() }
}
