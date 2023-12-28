use crate::constant::{ADDRESS_TEST_PROGRAM, SP_ADDRESS_RESET};

#[derive(Clone, Debug)]
pub struct CpuRegister {
    /// accumulator (8-bit)
    pub a: u8,
    /// indexes (8-bit)
    pub x: u8,
    pub y: u8,
    /// program counter (16-bit)
    pub pc: u16,
    /// stack pointer (8-bit)
    pub sp: u8,
    /// status register (8-bit wide, used 6-bit)
    pub carry: bool, // 0
    pub zero: bool,               // 10
    pub interrupt_disabled: bool, // 100
    pub decimal: bool,            // 1000
    pub overflow: bool,           // 10000
    pub negative: bool,           // 100000
}

impl Default for CpuRegister {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: ADDRESS_TEST_PROGRAM,
            sp: SP_ADDRESS_RESET,
            carry: false,
            zero: false,
            interrupt_disabled: false,
            decimal: false,
            overflow: false,
            negative: false,
        }
    }
}
