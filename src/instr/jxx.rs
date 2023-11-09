use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    #[inline]
    #[allow(non_snake_case)]
    pub fn JMP(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn JSR(&mut self) -> Result<()> {
        Ok(())
    }
}
