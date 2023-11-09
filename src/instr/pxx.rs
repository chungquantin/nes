use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    #[inline]
    #[allow(non_snake_case)]
    pub fn PHA(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn PHP(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn PLA(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn PLP(&mut self) -> Result<()> {
        Ok(())
    }
}
