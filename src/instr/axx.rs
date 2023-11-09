use anyhow::Result;

use crate::{cpu::Cpu6502, mem::MemoryManage, util::get_bit};

impl Cpu6502 {
    /// This instruction adds the contents of a memory location to the accumulator together with the carry bit
    #[inline]
    #[allow(non_snake_case)]
    pub fn ADC(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let v = instr.mode_args as u8;
        let (x1, o1) = v.overflowing_add(self.registers.a);
        let (x2, o2) = x1.overflowing_add(self.registers.carry as u8);

        self.registers.carry = o1 | o2;
        let signed_sum =
            (v as i8 as i16) + (self.registers.a as i8 as i16) + (self.registers.carry as i16);
        self.registers.a = x2;
        self.registers.overflow = (signed_sum < -128) || (signed_sum > 127);
        self.update_accumulator_flags();
        Ok(())
    }

    /// AND: A logical AND is performed, bit by bit
    /// on the accumulator contents using the contents of a byte of memory.
    #[inline]
    #[allow(non_snake_case)]
    pub fn AND(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.a &= instr.mode_args as u8;
        self.update_accumulator_flags();
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn ASL(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        // This operation shifts all the bits of the accumulator or memory contents one bit left
        let r = self.read_write_target(instr.write_target)?;
        let (x, _) = r.overflowing_mul(2);
        // Bit 0 is set to 0 and bit 7 is placed in the carry flag
        self.registers.carry = get_bit(r, 7) != 0;
        self.store_write_target(x, instr.write_target)?;
        self.update_accumulator_flags();
        Ok(())
    }
}
