// reference: https://www.nesdev.org/wiki/CPU_registers
#[derive(Clone, Debug, Default)]
pub struct CpuRegister {
    /// accumulator (8-bit)
    pub a: u8,
    /// indexes (8-bit)
    pub x: u8,
    pub y: u8,
    /// program counter (16-bit)
    pub pc: usize,
    /// stack pointer (8-bit)
    pub s: u8,
    /// status register (8-bit wide, used 6-bit)
    pub carry: bool, // 0
    pub zero: bool,        // 10
    pub interrupted: bool, // 100
    pub decimal: bool,     // 1000
    pub overflow: bool,    // 10000
    pub negative: bool,    // 100000
}
