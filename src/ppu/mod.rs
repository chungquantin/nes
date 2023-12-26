mod registers;

trait PpuClock {
    fn tick(&mut self);
}

// NTSC
pub struct Ricoh2C02 {
    vram: [u8; 0x800], // 2kb
}

impl Ricoh2C02 {
    pub fn new_test() -> Self {
        Self { vram: [0; 0x800] }
    }
}

// https://www.nesdev.org/wiki/PPU_pattern_tables
mod graphics {
    use crate::util::get_bits;
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ColorPaletteIndex {
        Transparent, // 0b00
        One,         // 0b01
        Two,         // 0b10
        Three,       // 0b11
    }

    /**
     * A tile is a 8x8 pixel block.
     */
    pub struct Tile {
        pixels: [u8; 16],
    }

    impl Tile {
        pub fn new(pixels: [u8; 16]) -> Self {
            Self { pixels }
        }

        /// Decodes the tile data into a `[ColorPaletteIndex; 64]` array. <br />
        /// The array is decoded from right to left, from top to bottom. <br />
        /// Example:  <br />
        /// [NES Development Wiki](https://www.nesdev.org/w/images/default/a/a4/One_half_fraction_CHR.png) <br />
        /// ```
        /// // From the first row
        ///  assert_eq!(decoded_tile[0], ColorPaletteIndex::Three); // 0b11
        ///  assert_eq!(decoded_tile[7], ColorPaletteIndex::Transparent); // 0b00
        /// ```
        pub fn decode_tile(&self) -> [ColorPaletteIndex; 64] {
            let block_color_palette = self.pixels.iter().as_slice().chunks_exact(2).flat_map(|x| {
                let [low, high] = [x[0], x[1]];
                let low_bits = get_bits(low);
                let high_bits = get_bits(high);
                let mut row_palette = [ColorPaletteIndex::Transparent; 8];
                for i in 0..8 {
                    let [zero_plane_bit, one_plane_bit] = [low_bits[i], high_bits[i]];
                    let color_pallete_index = match (one_plane_bit, zero_plane_bit) {
                        (false, false) => ColorPaletteIndex::Transparent,
                        (false, true) => ColorPaletteIndex::One,
                        (true, false) => ColorPaletteIndex::Two,
                        (true, true) => ColorPaletteIndex::Three,
                    };
                    row_palette[i] = color_pallete_index;
                }
                row_palette
            });

            return block_color_palette.collect::<Vec<_>>().try_into().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphics::{ColorPaletteIndex, Tile};

    /// Tests the `decode_tile` function.
    ///
    /// The tile data used in this test can be referenced from:
    /// [NES Development Wiki](https://www.nesdev.org/w/images/default/a/a4/One_half_fraction_CHR.png)
    #[test]
    fn test_decode_tile() {
        let tile = Tile::new([
            0x41, 0x01, 0xc2, 0x02, 0x44, 0x04, 0x48, 0x08, 0x10, 0x16, 0x20, 0x21, 0x40, 0x42,
            0x80, 0x87,
        ]);

        let decoded_tile = tile.decode_tile();
        assert_eq!(decoded_tile[0], ColorPaletteIndex::Three);
        assert_eq!(decoded_tile[6], ColorPaletteIndex::One);
        assert_eq!(decoded_tile[14], ColorPaletteIndex::One);
        assert_eq!(decoded_tile[15], ColorPaletteIndex::One);
        assert_eq!(
            decoded_tile
                .iter()
                .filter(|x| x.eq(&&ColorPaletteIndex::One))
                .count(),
            5
        );
        assert_eq!(
            decoded_tile
                .iter()
                .filter(|x| x.eq(&&ColorPaletteIndex::Two))
                .count(),
            7
        );
    }
}
/*
- Tile, are the basic unit of graphics in NES.
- Tile is 8x8 pixels.
_ _ _ _ _ _ _ _
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
|1|2|3|4|5|6|7|8|
|_|_|_|_|_|_|_|_|
*/

/*
Nametables
- nametables is a 2KiB are of memory in the VRAM.
- nametables store the indice of tiles from the pattern tables.
- nametables specify which tile from the chosen pattern is to be displayed at each tile position on the screen.
*/

/*
Pattern tables
- pattern tables contains the tile data.
*/
