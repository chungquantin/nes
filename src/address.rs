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
    // return value: (address, value, program counter step)
    pub fn decode_addressing_mode(&self, mode: AddressingMode) -> Result<(Option<u16>, u16, u16)> {
        let ptr = self.registers.pc.wrapping_add(1);
        Ok(match mode {
            imm | rel => {
                let v = self.mem_read(ptr)?;
                (Some(ptr), v as u16, 1)
            }
            zp => {
                let addr = self.mem_read(ptr)?.into();
                let v = self.mem_read(addr)?;
                (Some(addr), v as u16, 1)
            }
            zpx => {
                let pos = self.mem_read(ptr)?;
                let addr = pos.wrapping_add(self.registers.x) as u16;
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            zpy => {
                let pos = self.mem_read(ptr)?;
                let addr = pos.wrapping_add(self.registers.y) as u16;
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            abs => {
                let addr = self.mem_read_u16(ptr)?;
                let v = self.mem_read(addr)?;
                (Some(addr), v as u16, 2)
            }
            abx => {
                let base = self.mem_read_u16(ptr)?;
                let addr = base.wrapping_add(self.registers.x as u16);
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 2)
            }
            aby => {
                let base = self.mem_read_u16(ptr)?;
                let addr = base.wrapping_add(self.registers.y as u16);
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 2)
            }
            ind => {
                let addr = self.mem_read_u16(ptr)?;
                let jmp_ptr = self.mem_read_u16(addr)?;
                (Some(jmp_ptr), 0xDEAD, 2)
            }
            // Indexed Indirect: Read base value from program counter, before dreferencing
            // add x and use value of register x as the address
            izx => {
                let pos = self.mem_read(ptr)?;
                let ptr = pos.wrapping_add(self.registers.x) as u16;
                let addr = self.mem_read_u16(ptr)?;
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            // Indirect Indexed: read base value from program counter, dereferenece,
            // add y and return
            izy => {
                let base = self.mem_read(ptr)?.into();
                let deref_base = self.mem_read_u16(base)?;
                let addr = deref_base.wrapping_add(self.registers.y as u16);
                let v = self.mem_read(ptr)?;
                (Some(addr), v as u16, 1)
            }
            acc => (None, self.registers.a as u16, 1),
            imp => (None, 0xDEAD, 0),
        })
    }
}
