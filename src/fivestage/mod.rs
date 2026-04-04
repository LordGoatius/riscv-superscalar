use crate::{common::*, instr::decoding::{RV32I, Reg}};

// 5 stage RISC pipeline
// 
// 1. Fetch
// 2. Decode
// 3. Execution
// 4. Memory
// 5. Writeback

pub struct Machine<const MEM_SIZE: usize> {
    registers: [Register; 32],
    memory:    Box<[u8; MEM_SIZE]>,
    pc: usize,
    pipleline: Pipeline
}

// Kinda weird way of trying to do this, but maybe it'll work?

struct Pipeline {
    // Represents a list of current registers in the pipeline, and at what spot.
    // This allows us to add hazards to avoid incoherence.
    dependencies: Vec<Reg>,
    // During decode, these are set 
    vals1: u32,
    vals2: u32,
    // set during execute. For branches, this is set to the next PC. Branches add hazards and stop the pipeline until they can
    // "retire".
    valres: u32,
    // set during memory access. res1 is used for the WB stage
    valmem: u32,
    valres1: u32,

    fetch:     Vec<Option<u32>>,
    decode:    Vec<Option<RV32I>>,
    // u32 for reg WB, opt for PC writeback
    value:     Vec<Option<(u32, Option<u32>)>>,
    memory:    Vec<Option<MemoryAccess>>,
    writeback: Vec<Option<WriteBack>>,
}

enum MemoryAccess {
    Read(Reg),
    Write(Reg),
    None,
}

struct WriteBack(Reg);
