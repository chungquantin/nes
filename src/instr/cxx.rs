use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    #[inline]
    #[allow(non_snake_case)]
    pub fn CLC(&mut self) -> Result<()> {
        self.registers.carry = false;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn CLD(&mut self) -> Result<()> {
        self.registers.decimal = false;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn CLI(&mut self) -> Result<()> {
        // Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
        self.registers.interrupt_disabled = false;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn CLV(&mut self) -> Result<()> {
        // Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
        self.registers.overflow = false;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn CMP(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn CPX(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn CPY(&mut self) -> Result<()> {
        Ok(())
    }
}
