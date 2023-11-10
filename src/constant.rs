/**
Addressable memory: 16-bit memory
Spec: 6502 has 2^16 = 65,536 memory locations
=> Memory max is around 128KB

How to calculate the maximum memory?

1. 65536 * 16 bit-memory = 1048576 total bits required
2. 1048576 /  8 = 131072 / 1024 = 128 KB
**/
pub const MEMORY_MAX: usize = 1 << 16; // 0xFFFF

#[allow(unused)]
pub const ADDRESS_NMI: u16 = 0xFFFA;
pub const PC_ADDRESS_RESET: u16 = 0xFFFC;
pub const PRG_ROM_ADDRESS: u16 = 0x8000;
pub const ADDRESS_BRK: u16 = 0xFFFE;
#[allow(unused)]
pub const ADDRESS_TEST_PROGRAM: u16 = 0xC000;
pub const NEGATIVE_FLAG: u8 = 0x80;
// $0100–$01FF: The page containing the stack, which can be located anywhere here,
// but typically starts at $01FF
pub const SP_BASE_ADDRESS: u16 = 0x0100;
