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

    fn execute_cmp(&mut self, a: u16, b: u16) {
        let (result, _) = a.overflowing_sub(b);
        self.registers.carry = a >= b;
        self.update_zero_and_negative_flags(result as u8);
    }

    /// This instruction compares the contents of the A register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    #[inline]
    #[allow(non_snake_case)]
    pub fn CMP(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.execute_cmp(self.registers.a as u16, instr.mode_args);
        Ok(())
    }

    /// This instruction compares the contents of the X register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    #[inline]
    #[allow(non_snake_case)]
    pub fn CPX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.execute_cmp(self.registers.x as u16, instr.mode_args);
        Ok(())
    }

    /// This instruction compares the contents of the Y register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    #[inline]
    #[allow(non_snake_case)]
    pub fn CPY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.execute_cmp(self.registers.y as u16, instr.mode_args);
        Ok(())
    }
}
