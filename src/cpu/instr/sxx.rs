use anyhow::Result;

use crate::{cpu::Cpu6502, mem::Mem};

impl Cpu6502 {
    /// Store the contents of the accummulator into memory
    #[inline]
    #[allow(non_snake_case)]
    pub fn STA(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.write_target.unwrap();
        self.mem_write(addr, self.registers.a)?;
        Ok(())
    }

    /// Store the contents of the X register into memory
    #[inline]
    #[allow(non_snake_case)]
    pub fn STX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.write_target.unwrap();
        self.mem_write(addr, self.registers.x)?;
        Ok(())
    }

    /// Store the contents of the Y register into memory
    #[inline]
    #[allow(non_snake_case)]
    pub fn STY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.write_target.unwrap();
        self.mem_write(addr, self.registers.y)?;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SBC(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let mem = instr.mode_args;
        let (v1, o1) = self.registers.a.overflowing_sub(mem as u8);

        let neg_carry = !self.registers.carry;
        let (v2, o2) = v1.overflowing_sub(neg_carry as u8);

        self.registers.carry = !(o1 | o2);
        // Set if sign bit is incorrect
        let signed_sum =
            (mem as i8 as i16) + (self.registers.a as i8 as i16) + (self.registers.carry as i16);
        self.registers.a = v2;
        self.registers.overflow = (signed_sum < -128) || (signed_sum > 127);
        self.update_accumulator_flags();

        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SEC(&mut self) -> Result<()> {
        self.registers.carry = true;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SED(&mut self) -> Result<()> {
        self.registers.decimal = true;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SEI(&mut self) -> Result<()> {
        self.registers.interrupt_disabled = true;
        Ok(())
    }
}
