/// Because the CPU and the PPU are on separate buses, neither has direct access to the other's memory.
/// The CPU writes to VRAM through a pair of registers on the PPU by first loading an address into PPUADDR and then it writing data repeatedly to PPUDATA.
/// The 16-bit address is written to PPUADDR one byte at a time, upper byte first. Whether this is the first or second write is tracked internally by the w register, which is shared with PPUSCROLL.
#[derive(Debug, Clone)]
pub struct PpuAddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl PpuAddrRegister {
    pub fn new() -> Self {
        PpuAddrRegister {
            value: (0, 0), // high byte first, lo byte second
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }

    #[inline]
    // Valid addresses are $0000–$3FFF; higher addresses will be mirrored down.
    fn mirror_down(&mut self) {
        if self.get() > 0x3fff {
            //mirror down addr above 0x3fff
            self.set(self.get() & 0b11111111111111);
        }
    }

    #[inline]
    pub fn write(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }
        self.mirror_down();
        self.hi_ptr = !self.hi_ptr;
    }

    #[inline]
    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        self.mirror_down();
    }

    #[inline]
    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}
