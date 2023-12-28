use crate::cpu::Cpu6502;
use crate::ppu::Ppu;

// Main entry point for the NES emulator
struct NesEmulator {
    pub cpu: Box<Cpu6502>,
    pub ppu: Box<Ppu>,
}

impl Default for NesEmulator {
    fn default() -> Self {
        Self {
            cpu: Box::new(Cpu6502::default()),
            ppu: Box::new(Ppu::default()),
        }
    }
}
