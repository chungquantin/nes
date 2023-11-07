mod address;
mod cli;
mod constant;
mod cpu;
mod instruction;
mod mem;
mod opcode;
mod register;
use crate::cpu::Clocked;

fn main() {}

mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0x05, 0x00]).run().unwrap();

        assert_eq!(cpu.registers.a, 0x05);
        assert!(!cpu.registers.zero);
        assert!(cpu.registers.negative);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0x00, 0x00]).run().unwrap();

        assert!(cpu.registers.zero);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0x10, 0xaa, 0x00])
            .run()
            .unwrap();

        assert_eq!(cpu.registers.x, 0x10)
    }

    #[test]
    fn test_0x8a_txa_move_x_to_a() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa2, 0x10, 0x8a, 0x00])
            .run()
            .unwrap();

        assert_eq!(cpu.registers.a, 0x10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00])
            .run()
            .unwrap();

        assert_eq!(cpu.registers.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00])
            .run()
            .unwrap();

        assert_eq!(cpu.registers.x, 1)
    }
}
