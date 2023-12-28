use anyhow::Result;

use crate::{cpu::Cpu6502, util::get_bit};

impl Cpu6502 {
    /// LDA: Load byte memory into the accumulator
    #[inline]
    #[allow(non_snake_case)]
    pub fn LDA(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.a = instr.mode_args as u8;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// LDY: Load byte memory into the register y
    #[inline]
    #[allow(non_snake_case)]
    pub fn LDY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.y = instr.mode_args as u8;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// LDX: Load byte memory into the register x
    #[inline]
    #[allow(non_snake_case)]
    pub fn LDX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.x = instr.mode_args as u8;
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// LSR: Shift Right One Bit (M or A)
    #[inline]
    #[allow(non_snake_case)]
    pub fn LSR(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        // M or A
        let target_value = self.read_write_target(instr.write_target)?;
        // The bit that was in bit 0 is shifted into the carry flag
        self.registers.carry = get_bit(target_value, 0) > 0;

        let result = target_value.wrapping_shr(1);
        self.update_zero_and_negative_flags(result);
        Ok(())
    }
}
