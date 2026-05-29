//! Tailcall interpreter for RISCV

pub mod dispatch;
pub mod registers;
pub mod ram;

use crate::{instr::{decoding::Reg, *}, tailcall::{ram::Ram, registers::Registers}};

struct Machine {
    ram: Ram,
    reg: Registers,
}

pub type Address = u32;

pub enum ExceptionType {
    IllegalInstr,
    ECall,
    EBreak,
}

type MachineResult = Result<Pc, Exception>;

pub struct Exception(Address, ExceptionType);

pub type Pc = u32;

impl Machine {
    fn next(&mut self, pc: Pc) -> Result<RV32I, Exception> {
        todo!()
    }
    // Arith: rd, rs1, rs2
    fn add   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1).wrapping_add(self.reg.get(rs2))); Ok(pc + 4) }
    fn sub   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1).wrapping_sub(self.reg.get(rs2))); Ok(pc + 4) }
    fn xor   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) ^ self.reg.get(rs2)); Ok(pc + 4) }
    fn or    (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) | self.reg.get(rs2)); Ok(pc + 4) }
    fn and   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) & self.reg.get(rs2)); Ok(pc + 4) }
    fn sll   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) << self.reg.get(rs2)); Ok(pc + 4) }
    fn srl   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) >> self.reg.get(rs2)); Ok(pc + 4) }
    fn sra   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult { self.reg.set(rd, ((self.reg.get(rs1) as i32) << self.reg.get(rs2)) as u32); Ok(pc + 4) }
    fn slt   (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult {
        let v = if (self.reg.get(rs1) as i32) < (self.reg.get(rs2) as i32) {
            1
        } else {
            0
        };
        self.reg.set(rd, v);
        Ok(pc + 4)
    }
    fn sltu  (&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Pc) -> MachineResult {
        let v = if self.reg.get(rs1) < self.reg.get(rs2) {
            1
        } else {
            0
        };
        self.reg.set(rd, v);
        Ok(pc + 4)
    }

    // Immediate: rd, rs1, imm
    fn addi  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1).wrapping_add(imm)); Ok(pc + 4) }
    fn subi  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1).wrapping_sub(imm)); Ok(pc + 4) }
    fn xori  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) ^ imm); Ok(pc + 4) }
    fn ori   (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) | imm); Ok(pc + 4) }
    fn andi  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) & imm); Ok(pc + 4) }
    fn slli  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) << imm); Ok(pc + 4) }
    fn srli  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, self.reg.get(rs1) >> imm); Ok(pc + 4) }
    fn srai  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult { self.reg.set(rd, ((self.reg.get(rs1) as i32) << imm) as u32); Ok(pc + 4) }
    fn slti  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let v = if (self.reg.get(rs1) as i32) < (imm as i32) {
            1
        } else {
            0
        };
        self.reg.set(rd, v);
        Ok(pc + 4)
    }
    fn sltiu (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let v = if self.reg.get(rs1) < imm {
            1
        } else {
            0
        };
        self.reg.set(rd, v);
        Ok(pc + 4)
    }

    // Loads: rd, rs1, imm
    // TODO: This has *got* to be fixed at some point.
    fn lb    (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        let val = &raw const self.ram[addr as usize];
        unsafe {
            self.reg.set(rd, *val as u32);
        }
        Ok(pc + 4)
    }
    fn lh    (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        let val = &raw const self.ram[addr as usize] as *const u16;
        unsafe {
            self.reg.set(rd, val.read_unaligned() as u32);
        }
        Ok(pc + 4)
    }
    fn lw    (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        let val = &raw const self.ram[addr as usize] as *const u32;
        unsafe {
            self.reg.set(rd, *val as u32);
        }
        Ok(pc + 4)
    }
    fn lbu   (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        let val = &raw const self.ram[addr as usize];
        unsafe {
            self.reg.set(rd, val.read_unaligned() as u32);
        }
        Ok(pc + 4)
    }
    fn lhu   (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        let val = &raw const self.ram[addr as usize] as *const u16;
        unsafe {
            self.reg.set(rd, val.read_unaligned() as u32);
        }
        Ok(pc + 4)
    }

    // Stores: rs1, rs2, imm
    fn sb    (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        unsafe {
            (&raw mut self.ram[addr as usize]).write_unaligned(self.reg.get(rs2) as u8);
        }
        Ok(pc + 4)
    }
    fn sh    (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        unsafe {
            (&raw mut self.ram[addr as usize] as *mut u16).write_unaligned(self.reg.get(rs2) as u16);
        }
        Ok(pc + 4)
    }
    fn sw    (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        let addr = self.reg.get(rs1) + imm;
        unsafe {
            (&raw mut self.ram[addr as usize]).write_unaligned(self.reg.get(rs2) as u8);
        }
        Ok(pc + 4)
    }

    // Branches: rs1, rs2, imm
    fn beq   (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        if self.reg.get(rs1) == self.reg.get(rs2) {
            Ok(pc + imm)
        } else {
            Ok(pc + 4)
        }
    }
    fn bne   (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        if self.reg.get(rs1) == self.reg.get(rs2) {
            Ok(pc + imm)
        } else {
            Ok(pc + 4)
        }
    }
    fn blt   (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        if self.reg.get(rs1) == self.reg.get(rs2) {
            Ok(pc + imm)
        } else {
            Ok(pc + 4)
        }
    }
    fn bge   (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        if self.reg.get(rs1) == self.reg.get(rs2) {
            Ok(pc + imm)
        } else {
            Ok(pc + 4)
        }
    }
    fn bltu  (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        if self.reg.get(rs1) == self.reg.get(rs2) {
            Ok(pc + imm)
        } else {
            Ok(pc + 4)
        }
    }
    fn bgeu  (&mut self, rs1: Reg, rs2: Reg, imm: u32, pc: Pc) -> MachineResult {
        if self.reg.get(rs1) == self.reg.get(rs2) {
            Ok(pc + imm)
        } else {
            Ok(pc + 4)
        }
    }

    // Jumps w/ Link
    // rd, imm
    fn jal   (&mut self, rd: Reg, imm: u32, pc: Pc) -> MachineResult {
        self.reg.set(rd, pc + 4);
        Ok(pc + imm)
    }

    // rd, rs1, imm
    fn jalr  (&mut self, rd: Reg, rs1: Reg, imm: u32, pc: Pc) -> MachineResult {
        self.reg.set(rd, pc + 4);
        Ok(imm + self.reg.get(rs1))
    }

    // Upper Imm: rd, imm
    fn lui   (&mut self, rd: Reg, imm: u32, pc: Pc) -> MachineResult {
        self.reg.set(rd, imm << 12);
        Ok(pc + 4)
    }
    fn aupic (&mut self, rd: Reg, imm: u32, pc: Pc) -> MachineResult {
        self.reg.set(rd, pc + (imm << 12));
        Ok(4)
    }

    // Env
    fn ecall (&mut self, pc: Pc) -> MachineResult { todo!() }
    fn ebreak(&mut self, pc: Pc) -> MachineResult { todo!() }
}
