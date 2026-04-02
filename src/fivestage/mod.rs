use crate::{common::*, instr::decoding::{RV32I, Reg}};

// 5 stage RISC pipeline
// 1. Fetch
// 2. Decode
// 3. Execution
// 4. Memory
// 5. Writeback

pub struct Machine<const MEM_SIZE: usize> {
    registers: [Register; 32],
    memory: Box<[u8; MEM_SIZE]>,

    // Pipeline specific values
    fetched: Option<u32>,
    decoded: Option<RV32I>,
    // Computational value from execute stage
    value: Option<u32>,
    mem: Option<MemoryAccess>,
    writeback: Option<(u32, RV32I)>,
}

enum AccType {
    Read,
    Write,
}

struct MemoryAccess(AccType, Reg, u32);

struct WriteBack(Reg, u32);
