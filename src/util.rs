pub fn get_bit(x: u8, i: u8) -> u8 {
    return (x >> i) & 1;
}
