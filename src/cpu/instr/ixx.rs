use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    /// Adds one to the X register setting the zero and negative flags as appropriate.
    #[inline]
    #[allow(non_snake_case)]
    pub fn INX(&mut self) -> Result<()> {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    #[inline]
    #[allow(non_snake_case)]
    pub fn INY(&mut self) -> Result<()> {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn INC(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let value = instr.mode_args.wrapping_add(1) as u8;
        let addr = instr.write_target;
        // Rewrite the new value to the memory location
        self.store_write_target(value, addr)?;
        self.update_zero_and_negative_flags(self.read_write_target(addr)?);
        Ok(())
    }
}
