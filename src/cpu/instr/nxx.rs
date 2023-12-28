use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    /// NOP - No Operation
    #[inline]
    #[allow(non_snake_case)]
    pub fn NOP(&mut self) -> Result<()> {
        // The NOP instruction causes no changes to the processor other than the
        // normal incrementing of the program counter to the next instruction.
        Ok(())
    }
}
