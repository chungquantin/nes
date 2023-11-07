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

pub const abs: AddressingMode = Absolute;
pub const acc: AddressingMode = Accumulator;
pub const imm: AddressingMode = Immediate;
pub const imp: AddressingMode = Implicit;
pub const izx: AddressingMode = IndirectX;
pub const izy: AddressingMode = IndirectY;
pub const zp: AddressingMode = ZeroPage;
pub const zpx: AddressingMode = ZeroPageX;
pub const zpy: AddressingMode = ZeroPageY;
pub const rel: AddressingMode = Relative;
pub const abx: AddressingMode = AbsoluteX;
pub const aby: AddressingMode = AbsoluteY;
pub const ind: AddressingMode = Indirect;

impl Cpu6502 {
    // reference: https://skilldrick.github.io/easy6502/#addressing
    pub fn get_operand_address(&self, mode: &AddressingMode) -> Result<u16> {
        Ok(match mode {
            AddressingMode::Immediate => self.registers.pc,
            AddressingMode::ZeroPage => self.mem_read(self.registers.pc)?.into(),
            AddressingMode::Absolute => self.mem_read_u16(self.registers.pc)?.into(),
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.registers.pc)?;
                let addr = pos.wrapping_add(self.registers.x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.registers.pc)?;
                let addr = pos.wrapping_add(self.registers.y) as u16;
                addr
            }
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.registers.pc)?;
                let addr = base.wrapping_add(self.registers.x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.registers.pc)?;
                let addr = base.wrapping_add(self.registers.y as u16);
                addr
            }
            AddressingMode::Accumulator => self.registers.a as u16,
            // Indexed Indirect: Read base value from program counter, before dreferencing
            // add x and use value of register x as the address
            AddressingMode::IndirectX => {
                let ptr = self.get_operand_address(&AddressingMode::ZeroPageX)?;
                self.mem_read_u16(ptr)?
            }
            // Indirect Indexed: read base value from program counter, dereferenece,
            // add y and return
            AddressingMode::IndirectY => {
                let base = self.get_operand_address(&AddressingMode::ZeroPage)?;
                let deref_base = self.mem_read_u16(base)?;
                let deref = deref_base.wrapping_add(self.registers.y as u16);
                deref
            }

            _ => unimplemented!(),
        })
    }
}
