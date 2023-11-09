use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    /// TAX: Copies the current contents of the X register into the accumulator
    #[inline]
    #[allow(non_snake_case)]
    pub fn TAX(&mut self) -> Result<()> {
        self.registers.x = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// TXA: Copies the current contents of the accumulator into the X register
    #[inline]
    #[allow(non_snake_case)]
    pub fn TXA(&mut self) -> Result<()> {
        self.registers.a = self.registers.x;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// TYA: Copies the current contents of the X register into the accumulator
    #[inline]
    #[allow(non_snake_case)]
    pub fn TAY(&mut self) -> Result<()> {
        self.registers.y = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// TYA: Copies the current contents of the X register into the accumulator
    #[inline]
    #[allow(non_snake_case)]
    pub fn TYA(&mut self) -> Result<()> {
        self.registers.a = self.registers.y;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// TXS: Copies the current contents of the X register into the stack register.
    #[inline]
    #[allow(non_snake_case)]
    pub fn TXS(&mut self) -> Result<()> {
        self.registers.s = self.registers.x;
        Ok(())
    }
}
