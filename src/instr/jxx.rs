use anyhow::Result;

use crate::{cpu::Cpu6502, stack::Stacked};

impl Cpu6502 {
    #[inline]
    #[allow(non_snake_case)]
    pub fn JMP(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.pc = instr.write_target.unwrap();
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn JSR(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.push_stack16(self.registers.pc.wrapping_sub(1))?;
        self.registers.pc = instr.write_target.unwrap();
        Ok(())
    }
}
