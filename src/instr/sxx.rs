use anyhow::Result;

use crate::{cpu::Cpu6502, mem::MemoryManage};

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
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SEC(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SED(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn SEI(&mut self) -> Result<()> {
        Ok(())
    }
}
