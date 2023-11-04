use crate::{address::*, opcode::Operation};
use crate::{constant::NEGATIVE_FLAG, cpu::Cpu6502};
use anyhow::Result;

pub type CycleCount = u8;

#[derive(Copy, Clone, Debug)]
pub struct CpuInstruction {
    pub opcode: Operation,
    pub cycle: CycleCount,
    pub address_mode: AddressingMode,
    pub extra_cycle: CycleCount,
}

impl Cpu6502 {
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.zero = result == 0x0;
        self.registers.negative = (result & 0x80) == NEGATIVE_FLAG;
    }

    /// LDA: Load byte memory into the accumulator
    pub fn LDA(&mut self) -> Result<()> {
        let param = self.program_data[self.registers.pc as usize];
        self.registers.pc += 1;
        self.registers.a = param;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// LDY: Load byte memory into the register y
    pub fn LDY(&mut self) -> Result<()> {
        let param = self.program_data[self.registers.pc as usize];
        self.registers.pc += 1;
        self.registers.y = param;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// LDX: Load byte memory into the register x
    pub fn LDX(&mut self) -> Result<()> {
        let param = self.program_data[self.registers.pc as usize];
        self.registers.pc += 1;
        self.registers.x = param;
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// TAX: Copies the current contents of the X register into the accumulator
    pub fn TAX(&mut self) -> Result<()> {
        self.registers.x = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// TXA: Copies the current contents of the accumulator into the X register
    pub fn TXA(&mut self) -> Result<()> {
        self.registers.a = self.registers.x;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// TYA: Copies the current contents of the X register into the accumulator
    pub fn TAY(&mut self) -> Result<()> {
        self.registers.y = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// TYA: Copies the current contents of the X register into the accumulator
    pub fn TYA(&mut self) -> Result<()> {
        self.registers.a = self.registers.y;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// TXS: Copies the current contents of the X register into the stack register.
    pub fn TXS(&mut self) -> Result<()> {
        self.registers.s = self.registers.x;
        Ok(())
    }

    /// AND: A logical AND is performed, bit by bit
    /// on the accumulator contents using the contents of a byte of memory.
    pub fn AND(&mut self) -> Result<()> {
        let param = self.program_data[self.registers.pc as usize];
        self.registers.pc += 1;
        self.registers.a = self.registers.a & param;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn INX(&mut self) -> Result<()> {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub fn INY(&mut self) -> Result<()> {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// Break the program
    pub fn BRK(&mut self) -> Result<()> {
        return Ok(());
    }
}
