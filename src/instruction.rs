use std::fmt::Debug;

use crate::constant::NEGATIVE_FLAG;
use crate::cpu::Cpu6502;
use crate::mem::MemoryManage;
use crate::{address::*, opcode::Operation};
use anyhow::{Ok, Result};

pub type CycleCount = u8;

#[derive(Copy, Clone)]
pub struct CpuInstruction {
    pub opcode: Operation,
    pub cycle: CycleCount,
    pub address_mode: AddressingMode,
    pub extra_cycle: CycleCount,
    pub target_val: u16,
    pub target_addr: Option<u16>,
}

impl Debug for CpuInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuInstruction")
            .field("Opcode", &self.opcode)
            .field("Cycle", &self.cycle)
            .field("Address Mode", &self.address_mode)
            .field("Extra cycle", &self.extra_cycle)
            .field("Target value", &format!("0x{:0x?}", self.target_val))
            .field(
                "Target address",
                &format!("0x{:0x?}", self.target_addr.unwrap_or_default()),
            )
            .finish()
    }
}

impl Cpu6502 {
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.zero = result == 0x0;
        // bit masking and get the first bit
        self.registers.negative = (result & 0x80) == NEGATIVE_FLAG;
    }

    /// LDA: Load byte memory into the accumulator
    #[inline]
    pub fn LDA(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.a = instr.target_val as u8;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// LDY: Load byte memory into the register y
    #[inline]
    pub fn LDY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.y = instr.target_val as u8;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// LDX: Load byte memory into the register x
    #[inline]
    pub fn LDX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.x = instr.target_val as u8;
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// TAX: Copies the current contents of the X register into the accumulator
    #[inline]
    pub fn TAX(&mut self) -> Result<()> {
        self.registers.x = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// TXA: Copies the current contents of the accumulator into the X register
    #[inline]
    pub fn TXA(&mut self) -> Result<()> {
        self.registers.a = self.registers.x;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// TYA: Copies the current contents of the X register into the accumulator
    #[inline]
    pub fn TAY(&mut self) -> Result<()> {
        self.registers.y = self.registers.a;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// TYA: Copies the current contents of the X register into the accumulator
    #[inline]
    pub fn TYA(&mut self) -> Result<()> {
        self.registers.a = self.registers.y;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// TXS: Copies the current contents of the X register into the stack register.
    #[inline]
    pub fn TXS(&mut self) -> Result<()> {
        self.registers.s = self.registers.x;
        Ok(())
    }

    /// AND: A logical AND is performed, bit by bit
    /// on the accumulator contents using the contents of a byte of memory.
    #[inline]
    pub fn AND(&mut self) -> Result<()> {
        let param = self.mem_read(self.registers.pc)?;
        self.registers.pc += 1;
        self.registers.a = self.registers.a & param;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// Adds one to the X register setting the zero and negative flags as appropriate.
    #[inline]
    pub fn INX(&mut self) -> Result<()> {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.x);
        Ok(())
    }

    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    #[inline]
    pub fn INY(&mut self) -> Result<()> {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// Store the contents of the accummulator into memory
    #[inline]
    pub fn STA(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.target_addr.unwrap();
        self.mem_write(addr, self.registers.a)?;
        Ok(())
    }

    /// Store the contents of the X register into memory
    #[inline]
    pub fn STX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.target_addr.unwrap();
        self.mem_write(addr, self.registers.x)?;
        Ok(())
    }

    /// Store the contents of the Y register into memory
    #[inline]
    pub fn STY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.target_addr.unwrap();
        self.mem_write(addr, self.registers.y)?;
        Ok(())
    }

    /// Break the program
    #[inline]
    pub fn BRK(&mut self) -> Result<()> {
        self.running = false;
        return Ok(());
    }
}
