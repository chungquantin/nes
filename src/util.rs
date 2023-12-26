/// ## WARNING
/// 0 index is the lost significant bit(lsb)<br />
/// 7 index is the most significant bit(msb)<br />
/// u8 are decoded from right to left<br />
/// Normally bit sequences are decoded from left to right.
pub fn get_bit(x: u8, i: u8) -> u8 {
    return (x >> i) & 1;
}

pub fn get_bits(x: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[i] = get_bit(x, i as u8) == 1;
    }

    return bits;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        assert_eq!(0x41, 0b01000001);
        let msb = get_bit(0x41, 0);
        assert_eq!(msb, 1);
        let lsb = get_bit(0x41, 7);
        assert_eq!(lsb, 0);
        let six_bit = get_bit(0x41, 6);
        assert_eq!(six_bit, 1);
    }

    #[test]
    fn test_get_bits() {
        let bits = get_bits(0x41);
        assert_eq!(bits, [true, false, false, false, false, false, true, false]);
    }
}
