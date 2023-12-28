use std::fmt::Debug;

use crate::cpu::address::*;
use crate::cpu::opcode::Operation;

pub type CycleCount = u8;

#[derive(Copy, Clone)]
pub struct CpuInstruction {
    pub opcode: Operation,
    pub cycle: CycleCount,
    pub address_mode: AddressingMode,
    pub extra_cycle: CycleCount,
    pub mode_args: u16,
    pub write_target: Option<u16>,
}

impl Debug for CpuInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuInstruction")
            .field("Opcode", &self.opcode)
            .field("Cycle", &self.cycle)
            .field("Address Mode", &self.address_mode)
            .field("Extra cycle", &self.extra_cycle)
            .field("Target value", &format!("0x{:0x?}", self.mode_args))
            .field(
                "Target address",
                &format!("0x{:0x?}", self.write_target.unwrap_or_default()),
            )
            .finish()
    }
}
