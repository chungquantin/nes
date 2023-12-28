mod cli;
mod constant;
mod cpu;
mod mem;
mod nes;
mod ppu;
mod stack;
mod util;

use crate::cpu::Cpu6502;

fn main() {
    let mut cpu = Cpu6502::default();
    cpu.load_test_program(vec![0x00]).unwrap();
    cpu.run().unwrap();
}

mod tests {
    use crate::{constant::ADDRESS_TEST_PROGRAM, cpu::Cpu6502, mem::Mem};

    #[allow(unused)]
    fn create_test_cpu(program: Vec<u8>) -> Cpu6502 {
        let mut cpu = Cpu6502::default();
        let rom_address = ADDRESS_TEST_PROGRAM as usize;
        cpu.mapper[rom_address..(rom_address + program.len())].copy_from_slice(&program[..]);
        cpu.registers.pc = ADDRESS_TEST_PROGRAM;
        return cpu;
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = self::create_test_cpu(vec![0xa9, 0x05, 0x00]);
        cpu.run().unwrap();

        assert_eq!(cpu.registers.a, 0x05);
        assert!(!cpu.registers.zero);
        assert!(!cpu.registers.negative);
    }

    #[test]
    fn test_adc() {
        let mut cpu = self::create_test_cpu(vec![
            0xA9, 0x00, // LDA #$00
            0x69, 0x69, // ADC #$69
        ]);
        // 0x6E = 1101110
        cpu.set_status_register_from_byte(0x6E);

        assert!(cpu.registers.zero);
        assert!(cpu.registers.interrupt_disabled);
        assert!(cpu.registers.overflow);
        assert!(cpu.registers.decimal);

        cpu.run().unwrap();
        cpu.print_register_status();
        assert_eq!(cpu.status_register_byte(true), 0x2c);
    }

    #[test]
    fn test_asl() {
        let mut cpu = self::create_test_cpu(vec![
            0xa9, 0x80, // LDA #$80
            0xa,
        ]);
        cpu.set_status_register_from_byte(0xe5);
        cpu.run().unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.status_register_byte(true), 0x67);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = self::create_test_cpu(vec![0xa9, 0x00, 0x00]);
        cpu.run().unwrap();

        assert!(cpu.registers.zero);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = self::create_test_cpu(vec![0xa9, 0x10, 0xaa, 0x00]);
        cpu.run().unwrap();

        assert_eq!(cpu.registers.x, 0x10)
    }

    #[test]
    fn test_0x8a_txa_move_x_to_a() {
        let mut cpu = self::create_test_cpu(vec![0xa2, 0x10, 0x8a, 0x00]);
        cpu.run().unwrap();

        assert_eq!(cpu.registers.a, 0x10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = self::create_test_cpu(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        cpu.run().unwrap();

        assert_eq!(cpu.registers.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = self::create_test_cpu(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        cpu.run().unwrap();

        assert_eq!(cpu.registers.x, 1)
    }

    #[test]
    fn test_bcc() {
        let mut cpu = self::create_test_cpu(vec![0x90, 0x09]);
        cpu.set_status_register_from_byte(0xf9);
        assert_eq!(cpu.registers.carry, true);

        let pc = cpu.registers.pc;

        cpu.run().unwrap();

        assert_eq!(cpu.registers.pc - pc, 2); // Branch not taken
    }

    #[test]
    fn test_bit() {
        let program: Vec<u8> = vec![
            0xa9, 0xff, // LDA #255
            0x85, 0x01, // STA $01
            0x24, 0x01, // BIT $01
        ];
        let mut cpu = create_test_cpu(program);
        cpu.bounded_run(3).unwrap();
        assert!(cpu.registers.overflow);
    }

    #[test]
    fn test_flags() {
        let program: Vec<u8> = vec![
            0xA9, 0xFF, // LDA #$FF
            0x85, 0x01, // STA $01 = 00
            0x24, 0x01, // BIT $01 = FF
            0xa9, 0x00, // LDA #$00
            0x38, // SEC
            0x78, // SEI
            0xf8, // SED
            0x08, // PHP
            0x68, // PLA
        ];
        let mut cpu = create_test_cpu(program);
        cpu.bounded_run(9).unwrap();
        assert_eq!(cpu.registers.a, 111);
    }

    #[test]
    fn test_ror() {
        let mut cpu = self::create_test_cpu(vec![
            0xa9, 0x55, // LDA #$55
            0x6a, // ROR
        ]);
        cpu.set_status_register_from_byte(0x24);
        cpu.bounded_run(2).unwrap();
        assert_eq!(cpu.registers.a, 0x2A);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.status_register_byte(true), 0x25);
    }

    #[test]
    fn test_jsr() {
        let mut cpu = self::create_test_cpu(vec![
            0x20, 0x03, 0xc0, // JSR $c003
            0x68, // PLA
        ]);
        cpu.run().unwrap();
        assert_eq!(cpu.registers.a, 0x02);
    }

    #[test]
    fn test_subroutine() {
        let mut cpu = self::create_test_cpu(vec![
            0x20, 0x03, 0xC0, // JSR $C003
            0x60, // RTS
        ]);
        cpu.bounded_run(2).unwrap();
        assert_eq!(cpu.registers.pc, 0xC003);
    }

    #[test]
    fn test_lsr() {
        let mut cpu = self::create_test_cpu(vec![
            0xa9, 0x01, // LDA #$01
            0x4a,
        ]);
        cpu.set_status_register_from_byte(0x65);
        cpu.run().unwrap();
        assert_eq!(cpu.status_register_byte(true), 0x67);
    }

    #[test]
    fn test_cmp() {
        let mut cpu = self::create_test_cpu(vec![0xc9, 0x4d, 0x0]);
        cpu.registers.a = 0b1001101;
        cpu.set_status_register_from_byte(0b100111);
        cpu.run().unwrap();
        assert_eq!(cpu.status_register_byte(true), 0b100111);
    }

    #[test]
    fn test_ram_mirror() {
        let mut cpu = self::create_test_cpu(vec![0xa9, 0x01, 0x00]);
        let sum_ram: u8 = cpu.mapper[0x0000..0x2000].iter().sum();
        // Make sure that the RAM is zeroed out
        assert_eq!(sum_ram, 0);

        cpu.mem_write(0x07FF, 21).unwrap();
        let tmp_data = cpu.mem_read(0x07FF).unwrap();
        assert_eq!(tmp_data, 21);

        let data_at_0fff = cpu.mem_read(0x0FFF).unwrap();
        let data_at_17ff = cpu.mem_read(0x17FF).unwrap();
        let data_ata_1fff = cpu.mem_read(0x1FFF).unwrap();

        assert_eq!(data_at_0fff, 21);
        assert_eq!(data_at_17ff, 21);
        assert_eq!(data_ata_1fff, 21);
    }
}
