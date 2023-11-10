use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    // Decrement memory
    #[inline]
    #[allow(non_snake_case)]
    pub fn DEC(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let value = instr.mode_args.wrapping_sub(1) as u8;
        let addr = instr.write_target;
        // Rewrite the new value to the memory location
        self.store_write_target(value, addr)?;
        self.update_zero_and_negative_flags(self.read_write_target(addr)?);
        Ok(())
    }

    // Decrement X Register
    #[inline]
    #[allow(non_snake_case)]
    pub fn DEX(&mut self) -> Result<()> {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    // Decrement Y Register
    #[inline]
    #[allow(non_snake_case)]
    pub fn DEY(&mut self) -> Result<()> {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }
}
