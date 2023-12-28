use anyhow::Result;
mod registers;

use crate::{constant::MEMORY_MAX, mem::Mem};

use self::registers::{PpuAddrRegister, PpuControlRegister, PpuMaskRegister};

/// # PPU Registers
/// The PPU exposes eight memory-mapped registers to the CPU. These nominally sit at $2000 through $2007 in the CPU's address space, but because their addresses are incompletely decoded, they're mirrored in every 8 bytes from $2008 through $3FFF. For example, a write to $3456 is the same as a write to $2006.
/// After power-on and reset, many of the PPU's registers are not immediately usable until enough time has passed. See PPU power up state and Init code for details.
#[derive(Debug, Clone)]
struct PpuRegister {
    // $2000 - PPUCTRL controller
    ppuctrl: PpuControlRegister,
    // $2001 - PPUMASK mask register
    ppumask: PpuMaskRegister,
    // $2002 - PPUSTATUS status
    ppustatus: u8,
    // $2003 - OAM data read/write
    odmadata: u8,
    // $2004 - OAM data read/write
    oamdata: u8,
    // $2005 - fine scroll position
    ppuscroll: u8,
    // $2006 - PPU read/write address (two writes: most significant byte, least significant byte)
    ppuaddr: PpuAddrRegister,
    // $2007 - PPU data read/write
    ppudata: u8,
}

impl Default for PpuRegister {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Ppu {
    pub registers: PpuRegister,
    pub mapper: [u8; MEMORY_MAX], // 64KB
}

impl Ppu {
    fn write_to_ppuaddr(&mut self, value: u8) {
        self.registers.ppuaddr.write(value);
    }

    fn write_to_ppuctrl(&mut self, value: u8) {
        self.registers.ppuctrl.write(value);
    }

    fn write_to_ppumask(&mut self, value: u8) {
        self.registers.ppumask.write(value);
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            registers: PpuRegister::default(),
            mapper: [0u8; MEMORY_MAX],
        }
    }
}

impl Mem for Ppu {
    // TODO @dromaz help to confirm if I can apply the same mirroring method for the PPU
    fn mem_read(&self, addr: u16) -> Result<u8> {
        match addr {
            0x0000..=0x3fff => {
                // Mask to zero out the highest two bits in a 16-bit address
                let mirror_down_addr = addr & 0b00000111_11111111;
                Ok(self.mapper[mirror_down_addr as usize])
            }
            _ => Ok(self.mapper[addr as usize]),
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.mapper[addr as usize] = data;
        Ok(())
    }
}
