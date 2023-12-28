use bitflags::bitflags;

bitflags! {

   // 7  bit  0
   // ---- ----
   // VPHB SINN
   // |||| ||||
   // |||| ||++- Base nametable address
   // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
   // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
   // |||| |     (0: add 1, going across; 1: add 32, going down)
   // |||| +---- Sprite pattern table address for 8x8 sprites
   // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
   // |||+------ Background pattern table address (0: $0000; 1: $1000)
   // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
   // |+-------- PPU master/slave select
   // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
   // +--------- Generate an NMI at the start of the
   //            vertical blanking interval (0: off; 1: on)
   #[derive(Debug, Clone)]
   pub struct PpuControlRegister: u8 {
       const NAMETABLE1              = 0b00000001;
       const NAMETABLE2              = 0b00000010;
       const VRAM_ADD_INCREMENT      = 0b00000100;
       const SPRITE_PATTERN_ADDR     = 0b00001000;
       const BACKROUND_PATTERN_ADDR  = 0b00010000;
       const SPRITE_SIZE             = 0b00100000;
       const MASTER_SLAVE_SELECT     = 0b01000000;
       const GENERATE_NMI            = 0b10000000;
   }
}

impl PpuControlRegister {
    pub const fn new() -> Self {
        Self::from_bits_truncate(0x00)
    }

    #[inline]
    pub fn write(&mut self, val: u8) {
        *self = Self::from_bits_truncate(val);
    }

    #[inline]
    #[must_use]
    /// Base nametable address
    /// (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    pub fn nametable_addr(&self) -> u16 {
        match self.bits() & 0b11 {
            0b00 => 0x2000,
            0b01 => 0x2400,
            0b10 => 0x2800,
            0b11 => 0x2C00,
            _ => unreachable!("impossible nametable_addr"),
        }
    }

    #[inline]
    #[must_use]
    /// VRAM address increment per CPU read/write of PPUDATA
    /// (0: add 1, going across; 1: add 32, going down)
    pub const fn vram_increment(&self) -> u16 {
        if self.contains(Self::VRAM_ADD_INCREMENT) {
            32
        } else {
            1
        }
    }

    #[inline]
    #[must_use]
    pub const fn spr_select(&self) -> u16 {
        if self.contains(Self::SPRITE_PATTERN_ADDR) {
            0x1000
        } else {
            0x0000
        }
    }

    #[inline]
    #[must_use]
    pub const fn bg_select(&self) -> u16 {
        if self.contains(Self::BACKROUND_PATTERN_ADDR) {
            0x1000
        } else {
            0x0000
        }
    }

    #[inline]
    #[must_use]
    pub const fn spr_height(&self) -> u32 {
        if self.contains(Self::SPRITE_SIZE) {
            16
        } else {
            8
        }
    }

    #[inline]
    #[must_use]
    pub const fn master_slave(&self) -> u8 {
        if self.contains(Self::MASTER_SLAVE_SELECT) {
            1
        } else {
            0
        }
    }

    #[inline]
    #[must_use]
    pub const fn nmi_enabled(&self) -> bool {
        self.contains(Self::GENERATE_NMI)
    }
}
