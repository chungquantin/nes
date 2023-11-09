use std::fmt::Debug;

use crate::constant::NEGATIVE_FLAG;
use crate::cpu::Cpu6502;
use crate::mem::MemoryManage;
use crate::util::get_bit;
use crate::{address::*, opcode::Operation};
use anyhow::{Ok, Result};

pub type CycleCount = u8;

#[derive(Copy, Clone)]
pub struct CpuInstruction {
    pub opcode: Operation,
    pub cycle: CycleCount,
    pub address_mode: AddressingMode,
    pub extra_cycle: CycleCount,
    pub mode_args: u16,
    pub write_target: Option<u16>,
}

impl Debug for CpuInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuInstruction")
            .field("Opcode", &self.opcode)
            .field("Cycle", &self.cycle)
            .field("Address Mode", &self.address_mode)
            .field("Extra cycle", &self.extra_cycle)
            .field("Target value", &format!("0x{:0x?}", self.mode_args))
            .field(
                "Target address",
                &format!("0x{:0x?}", self.write_target.unwrap_or_default()),
            )
            .finish()
    }
}

impl Cpu6502 {
    fn is_negative(&self, result: u8) -> bool {
        (result & 0x80) == NEGATIVE_FLAG
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.zero = result == 0x0;
        // bit masking and get the first bit
        self.registers.negative = self.is_negative(result);
    }

    fn update_accumulator_flags(&mut self) {
        self.update_zero_and_negative_flags(self.registers.a);
    }

    #[allow(dead_code)]
    pub fn print_register_status(&self) {
        println!("Carry: {:?}", self.registers.carry);
        println!("Decimal: {:?}", self.registers.decimal);
        println!("Negative: {:?}", self.registers.negative);
        println!("Overflow: {:?}", self.registers.overflow);
        println!(
            "interrupt_disabled: {:?}",
            self.registers.interrupt_disabled
        );
        println!("Zero: {:?}", self.registers.zero);
    }

    /// This instruction adds the contents of a memory location to the accumulator together with the carry bit
    #[inline]
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
    pub fn AND(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.a &= instr.mode_args as u8;
        self.update_accumulator_flags();
        Ok(())
    }

    #[inline]
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

    fn execute_branch(&mut self) {
        let instr = self.instr.unwrap();

        let v = instr.mode_args;
        self.registers.pc = self.registers.pc.wrapping_add((v as i8) as u16);
        // +1 if branch success
        self.clocks_to_pause += 1;
        // TODO +2 if crossed page
    }

    #[inline]
    pub fn BCC(&mut self) -> Result<()> {
        // If the carry flag is clear then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.carry {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    pub fn BCS(&mut self) -> Result<()> {
        // If the carry flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.carry {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    pub fn BEQ(&mut self) -> Result<()> {
        // If the zero flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.zero {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
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
    pub fn BMI(&mut self) -> Result<()> {
        // If the negative flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.negative {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    pub fn BNE(&mut self) -> Result<()> {
        // If the zero flag is clear then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.zero {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
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
    pub fn BRK(&mut self) -> Result<()> {
        // BRK is executed in the main thread of CPU, don't need to implement anything
        unimplemented!()
    }

    #[inline]
    pub fn BVC(&mut self) -> Result<()> {
        // If the overflow flag is clear then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if !self.registers.overflow {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    pub fn BVS(&mut self) -> Result<()> {
        // If the overflow flag is set then add the relative displacement to the program counter
        // to cause a branch to a new location.
        if self.registers.overflow {
            self.execute_branch();
        }
        Ok(())
    }

    #[inline]
    pub fn CLC(&mut self) -> Result<()> {
        self.registers.carry = false;
        Ok(())
    }

    #[inline]
    pub fn CLD(&mut self) -> Result<()> {
        self.registers.decimal = false;
        Ok(())
    }

    #[inline]
    pub fn CLI(&mut self) -> Result<()> {
        // Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
        self.registers.interrupt_disabled = false;
        Ok(())
    }

    #[inline]
    pub fn CLV(&mut self) -> Result<()> {
        // Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
        self.registers.overflow = false;
        Ok(())
    }

    /// LDA: Load byte memory into the accumulator
    #[inline]
    pub fn LDA(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.a = instr.mode_args as u8;
        self.update_zero_and_negative_flags(self.registers.a);
        Ok(())
    }

    /// LDY: Load byte memory into the register y
    #[inline]
    pub fn LDY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.y = instr.mode_args as u8;
        self.update_zero_and_negative_flags(self.registers.y);
        Ok(())
    }

    /// LDX: Load byte memory into the register x
    #[inline]
    pub fn LDX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        self.registers.x = instr.mode_args as u8;
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
        let addr = instr.write_target.unwrap();
        self.mem_write(addr, self.registers.a)?;
        Ok(())
    }

    /// Store the contents of the X register into memory
    #[inline]
    pub fn STX(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.write_target.unwrap();
        self.mem_write(addr, self.registers.x)?;
        Ok(())
    }

    /// Store the contents of the Y register into memory
    #[inline]
    pub fn STY(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        let addr = instr.write_target.unwrap();
        self.mem_write(addr, self.registers.y)?;
        Ok(())
    }
}
