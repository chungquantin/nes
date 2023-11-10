use anyhow::Result;

use crate::{cpu::Cpu6502, stack::Stacked};

impl Cpu6502 {
    /// PHA - Push Accumulator
    #[inline]
    #[allow(non_snake_case)]
    pub fn PHA(&mut self) -> Result<()> {
        self.push_stack(self.registers.a)?;
        Ok(())
    }

    /// PHP - Push Processor Status
    #[inline]
    #[allow(non_snake_case)]
    pub fn PHP(&mut self) -> Result<()> {
        self.push_stack(self.status_register_byte(true))?;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn PLA(&mut self) -> Result<()> {
        self.registers.a = self.pop_stack()?;
        self.update_accumulator_flags();
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn PLP(&mut self) -> Result<()> {
        let val = self.pop_stack()?;
        self.set_status_register_from_byte(val);
        Ok(())
    }
}
