use anyhow::Result;

use crate::{cpu::Cpu6502, stack::Stacked, util::get_bit};

impl Cpu6502 {
    #[inline]
    #[allow(non_snake_case)]
    pub fn ROL(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let old_carry = self.registers.carry as u8;
        let v = self.read_write_target(instr.write_target)?;

        // Bit 0 is filled with the current value of the carry flag
        let result = v.wrapping_shl(1) | old_carry;
        // whilst the old bit 7 becomes the new carry flag value.
        self.registers.carry = get_bit(v, 7) > 0;
        self.store_write_target(result, instr.write_target)?;
        self.update_zero_and_negative_flags(result);
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn ROR(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let v = self.read_write_target(instr.write_target)?;
        // Bit 0 is filled with the current value of the carry flag
        let mut result = v.rotate_right(1);
        if self.registers.carry {
            result |= 1 << 7
        } else {
            result &= !(1 << 7)
        }
        // whilst the old bit 7 becomes the new carry flag value.
        self.registers.carry = get_bit(v, 0) > 0;
        self.store_write_target(result, instr.write_target)?;
        self.update_zero_and_negative_flags(result);
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn RTI(&mut self) -> Result<()> {
        let flags = self.pop_stack()?;
        self.set_status_register_from_byte(flags);
        self.registers.pc = self.pop_stack16()?;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn RTS(&mut self) -> Result<()> {
        let sv = self.pop_stack16()?;
        self.registers.pc = sv.wrapping_add(1);
        Ok(())
    }
}
