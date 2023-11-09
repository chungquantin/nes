mod address;
mod cli;
mod constant;
mod cpu;
mod debugger;
mod instruction;
mod mem;
mod opcode;
mod register;

use crate::{cpu::Cpu6502, mem::MemoryManage};

fn main() {}

mod tests {

    use super::*;

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = Cpu6502::default();
        cpu.mem_write(0x10, 0x55).unwrap();
        cpu.load_program(vec![0xa5, 0x10, 0x00]).run().unwrap();
        assert_eq!(cpu.registers.a, 0x55);
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0x05, 0x00]).run().unwrap();

        assert_eq!(cpu.registers.a, 0x05);
        assert!(!cpu.registers.zero);
        assert!(!cpu.registers.negative);
    }

    #[test]
    fn test_adc() {
        let mut cpu = crate::cpu::Cpu6502::default();
        let program: Vec<u8> = vec![
            0xA9, 0x00, // LDA #$00
            0x69, 0x69, // ADC #$69
        ];
        // 0x6E = 1101110
        cpu.set_status_register_from_byte(0x6E);

        assert!(cpu.registers.zero);
        assert!(cpu.registers.interrupted);
        assert!(cpu.registers.overflow);
        assert!(cpu.registers.decimal);

        cpu.load_program(program).run().unwrap();
        cpu.print_register_status();
        assert_eq!(cpu.status_register_byte(true), 0x2c);
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
