pub mod instr;
pub mod fivestage;
pub mod common;

#[cfg(feature = "xlen_64")]
const XLEN: usize = 64;
#[cfg(feature = "xlen_64")]
type XTYPE = u64;

#[cfg(not(feature = "xlen_64"))]
const XLEN: usize = 32;
#[cfg(not(feature = "xlen_64"))]
type XTYPE = u32;

#[cfg(feature = "ext_c")]
const IALIGN: usize = 32;
#[cfg(not(feature = "ext_c"))]
const IALIGN: usize = 16;
