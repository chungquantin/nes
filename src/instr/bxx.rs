use anyhow::Result;

use crate::{cpu::Cpu6502, util::get_bit};

impl Cpu6502 {
    fn execute_branch(&mut self) {
        let instr = self.instr.unwrap();

        let v = instr.mode_args;
        self.registers.pc = self.registers.pc.wrapping_add((v as i8) as u16);
        // +1 if branch success
        self.clocks_to_pause += 1;
        // TODO +2 if crossed page
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BCC(&mut self) -> Result<()> {
        // If the carry flag is clear then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.carry {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BCS(&mut self) -> Result<()> {
        // If the carry flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.carry {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BEQ(&mut self) -> Result<()> {
        // If the zero flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.zero {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BIT(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let v = instr.mode_args;
        let x = v & self.registers.a as u16;
        self.registers.negative = get_bit(v as u8, 7) > 0;
        self.registers.overflow = get_bit(v as u8, 6) > 0;
        self.registers.zero = x == 0;
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BMI(&mut self) -> Result<()> {
        // If the negative flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.negative {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BNE(&mut self) -> Result<()> {
        // If the zero flag is clear then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.zero {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BPL(&mut self) -> Result<()> {
        // If the negative flag is clear (positive) then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.negative {
            self.execute_branch();
        }
        Ok(())
    }

    /// Break the program
    #[inline]
    #[allow(non_snake_case)]
    pub fn BRK(&mut self) -> Result<()> {
        // BRK is executed in the main thread of CPU, don't need to implement anything
        unimplemented!()
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BVC(&mut self) -> Result<()> {
        // If the overflow flag is clear then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.overflow {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn BVS(&mut self) -> Result<()> {
        // If the overflow flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.overflow {
            self.execute_branch();
        }
        Ok(())
    }
}
