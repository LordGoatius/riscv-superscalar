use crate::{
    instr::RV32I,
    tailcall::{Machine, Pc},
};

pub type Address = u32;

pub enum ExceptionType {
    IllegalInstr,
}

pub struct Exception(Address, ExceptionType);

type DispatchFn = fn(&mut Machine, instr: RV32I, pc: Pc) -> Result<Pc, Exception>;

const TABLE_SIZE: usize = 40;

#[used]
static TABLE: [DispatchFn; TABLE_SIZE] = [
    add,
    sub,
    xor,
    or,
    and,
    sll,
    srl,
    sra,
    slt,
    sltu,
    addi,
    subi,
    xori,
    ori,
    andi,
    slli,
    srli,
    srai,
    slti,
    sltiu,
    lb,
    lh,
    lw,
    lbu,
    lhu,
    sb,
    sh,
    sw,
    beq,
    bne,
    blt,
    bge,
    bltu,
    bgeu,
    jal,
    jalr,
    lui,
    aupic,
    ecall,
    ebreak,
];

macro_rules! dispatch_fn {
    (RInstr {$(($name:ident, $enum:ident),)*}) => {
        $(fn $name (machine: &mut Machine, instr: RV32I, pc: Pc) -> Result<Pc, Exception> {
            let RV32I::$enum(rd, rs1, rs2) = instr else {
                return Err(Exception(pc, ExceptionType::IllegalInstr))
            };
            let pc = machine.$name(rd, rs1, rs2);
            let next = machine.next(pc).map_err(|err| Exception(pc, err))?;
            become TABLE[next.disc() as usize](machine, next, pc)
        })*
    };
    (IInstr {$(($name:ident, $enum:ident),)*}) => {
        $(fn $name (machine: &mut Machine, instr: RV32I, pc: Pc) -> Result<Pc, Exception> {
            let RV32I::$enum(rd, rs1, imm) = instr else {
                return Err(Exception(pc, ExceptionType::IllegalInstr))
            };
            let pc = machine.$name(rd, rs1, imm);
            let next = machine.next(pc).map_err(|err| Exception(pc, err))?;
            become TABLE[next.disc() as usize](machine, next, pc)
        })*
    };
    (SBInstr {$(($name:ident, $enum:ident),)*}) => {
        $(fn $name (machine: &mut Machine, instr: RV32I, pc: Pc) -> Result<Pc, Exception> {
            let RV32I::$enum(rs1, rs2, imm) = instr else {
                return Err(Exception(pc, ExceptionType::IllegalInstr))
            };
            let pc = machine.$name(rs1, rs2, imm);
            let next = machine.next(pc).map_err(|err| Exception(pc, err))?;
            become TABLE[next.disc() as usize](machine, next, pc)
        })*
    };
    (UJInstr {$(($name:ident, $enum:ident),)*}) => {
        $(fn $name (machine: &mut Machine, instr: RV32I, pc: Pc) -> Result<Pc, Exception> {
            let RV32I::$enum(reg, imm) = instr else {
                return Err(Exception(pc, ExceptionType::IllegalInstr))
            };
            let pc = machine.$name(reg, imm);
            let next = machine.next(pc).map_err(|err| Exception(pc, err))?;
            become TABLE[next.disc() as usize](machine, next, pc)
        })*
    };
    (SysInstr {$(($name:ident, $enum:ident),)*}) => {
        $(fn $name (machine: &mut Machine, instr: RV32I, pc: Pc) -> Result<Pc, Exception> {
            let RV32I::$enum = instr else {
                return Err(Exception(pc, ExceptionType::IllegalInstr))
            };
            let pc = machine.$name();
            let next = machine.next(pc).map_err(|err| Exception(pc, err))?;
            become TABLE[next.disc() as usize](machine, next, pc)
        })*
    };
    ($($instr:ident { $(($name:ident, $enum:ident),)* }, )*) => {
        $(dispatch_fn!(
            $instr { $(($name, $enum),)* }
        );)*
    }
}

dispatch_fn!(
    // RInstr
    RInstr {
        (add,  Add ),
        (sub,  Sub ),
        (xor,  Xor ),
        (or,   Or  ),
        (and,  And ),
        (sll,  Sll ),
        (srl,  Srl ),
        (sra,  Sra ),
        (slt,  Slt ),
        (sltu, Sltu),
    },
    IInstr {
        (addi,   Addi ),
        (subi,   Subi ),
        (xori,   Xori ),
        (ori,    Ori  ),
        (andi,   Andi ),
        (slli,   Slli ),
        (srli,   Srli ),
        (srai,   Srai ),
        (slti,   Slti ),
        (sltiu,  Sltiu),
        (jalr,   Jalr ),
        (lb,     Lb   ),
        (lh,     Lh   ),
        (lw,     Lw   ),
        (lbu,    Lbu  ),
        (lhu,    Lhu  ),
    },
    SBInstr {
        (sb,   Sb  ),
        (sh,   Sh  ),
        (sw,   Sw  ),
        (beq,  Beq ),
        (bne,  Bne ),
        (blt,  Blt ),
        (bge,  Bge ),
        (bltu, Bltu),
        (bgeu, Bgeu),
    },
    UJInstr {
        (jal  , Jal  ),
        (lui  , Lui  ),
        (aupic, Aupic),
    },
    SysInstr {
        (ecall , Ecall ),
        (ebreak, Ebreak),
    },
);
