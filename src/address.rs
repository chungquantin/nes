/// NES CPU uses Little-Endian addressing rather than Big-Endian.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,
    Implicit,
}

use anyhow::Result;
use AddressingMode::*;

use crate::{cpu::Cpu6502, mem::MemoryManage};

pub const ABS: AddressingMode = Absolute;
pub const ACC: AddressingMode = Accumulator;
pub const IMM: AddressingMode = Immediate;
pub const IMP: AddressingMode = Implicit;
pub const IZX: AddressingMode = IndirectX;
pub const IZY: AddressingMode = IndirectY;
pub const ZP: AddressingMode = ZeroPage;
pub const ZPX: AddressingMode = ZeroPageX;
pub const ZPY: AddressingMode = ZeroPageY;
pub const REL: AddressingMode = Relative;
pub const ABX: AddressingMode = AbsoluteX;
pub const ABY: AddressingMode = AbsoluteY;
pub const IND: AddressingMode = Indirect;

impl Cpu6502 {
    // reference: https://skilldrick.github.io/easy6502/#addressing
    // return value: (address, value, program counter step)
    pub fn decode_addressing_mode(&self, mode: AddressingMode) -> Result<(Option<u16>, u16, u16)> {
        let ptr = self.registers.pc.wrapping_add(1);
        Ok(match mode {
            IMM | REL => {
                let v = self.mem_read(ptr)?;
                (Some(ptr), v as u16, 1)
            }
            ZP => {
                let addr = self.mem_read(ptr)?.into();
                let v = self.mem_read(addr)?;
                (Some(addr), v as u16, 1)
            }
            ZPX => {
                let pos = self.mem_read(ptr)?;
                let addr = pos.wrapping_add(self.registers.x) as u16;
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            ZPY => {
                let pos = self.mem_read(ptr)?;
                let addr = pos.wrapping_add(self.registers.y) as u16;
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            ABS => {
                let addr = self.mem_read_u16(ptr)?;
                let v = self.mem_read(addr)?;
                (Some(addr), v as u16, 2)
            }
            ABX => {
                let base = self.mem_read_u16(ptr)?;
                let addr = base.wrapping_add(self.registers.x as u16);
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 2)
            }
            ABY => {
                let base = self.mem_read_u16(ptr)?;
                let addr = base.wrapping_add(self.registers.y as u16);
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 2)
            }
            IND => {
                let addr = self.mem_read_u16(ptr)?;
                let jmp_ptr = self.mem_read_u16(addr)?;
                (Some(jmp_ptr), 0xDEAD, 2)
            }
            // Indexed Indirect: Read base value from program counter, before dreferencing
            // add x and use value of register x as the address
            IZX => {
                let pos = self.mem_read(ptr)?;
                let ptr = pos.wrapping_add(self.registers.x) as u16;
                let addr = self.mem_read_u16(ptr)?;
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            // Indirect Indexed: read base value from program counter, dereferenece,
            // add y and return
            IZY => {
                let base = self.mem_read(ptr)?.into();
                let deref_base = self.mem_read_u16(base)?;
                let addr = deref_base.wrapping_add(self.registers.y as u16);
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            ACC => (None, self.registers.a as u16, 1),
            IMP => (None, 0xDEAD, 0),
        })
    }
}
