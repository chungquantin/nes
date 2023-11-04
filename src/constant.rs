/**
Addressable memory: 16-bit memory
Spec: 6502 has 2^16 = 65,536 memory locations
=> Memory max is around 128KB

How to calculate the maximum memory?

1. 65536 * 16 bit-memory = 1048576 total bits required
2. 1048576 /  8 = 131072 / 1024 = 128 KB
**/
pub const MEMORY_MAX: usize = 1 << 16;
pub enum MemoryRamLayout {
    Start = 0x0000,
    End = 0x2000, // 2KiB for address space
}

/// memory space [0x0100 .. 0x1FF] is used for stack
pub enum MemoryStackLayout {
    Start = 0x100,
    End = 0x1FF,
}

pub const ADDRESS_NMI: u16 = 0xFFFA;
pub const ADDRESS_RESET: u16 = 0xFFFC;
pub const ADDRESS_BRK: u16 = 0xFFFE;
pub const ADDRESS_TEST_PROGRAM: u16 = 0xC000;
pub const NEGATIVE_FLAG: u8 = 0;
