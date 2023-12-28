use bitflags::bitflags;

bitflags! {

   // 7  bit  0
   // ---- ----
   // BGRs bMmG
   // |||| ||||
   // |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
   // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
   // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
   // |||| +---- 1: Show background
   // |||+------ 1: Show sprites
   // ||+------- Emphasize red (green on PAL/Dendy)
   // |+-------- Emphasize green (red on PAL/Dendy)
   // +--------- Emphasize blue
   #[derive(Debug, Clone)]
   pub struct PpuMaskRegister: u8 {
       const GREYSCALE = 0b00000001;
       const SHOW_BG_LEFTMOST = 0b00000010;
       const SHOW_SPIRTE_LEFTMOST = 0b00000100;
       const SHOW_BG = 0b00001000;
       const SHOW_SPIRTES = 0b00010000;
       const EMPHASIZE_RED = 0b00100000;
       const EMPHASIZE_GREEN = 0b01000000;
       const EMPHASIZE_BLUE = 0b10000000;
   }
}

impl PpuMaskRegister {
    pub const fn new() -> Self {
        Self::from_bits_truncate(0x00)
    }

    #[inline]
    pub fn write(&mut self, val: u8) {
        *self = Self::from_bits_truncate(val);
    }
}

// TODO: implement the logic for PpuMaskRegister
