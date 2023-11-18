mod address;
mod cli;
mod constant;
mod cpu;
mod debugger;
mod instr;
mod instruction;
mod mem;
mod opcode;
mod register;
mod stack;
mod util;

use crate::cpu::Cpu6502;

fn main() {
    let mut cpu = Cpu6502::new();
    cpu.load_test_program(vec![0x00]).unwrap();
    cpu.run().unwrap();
}

mod tests {
    use crate::{constant::ADDRESS_TEST_PROGRAM, cpu::Cpu6502};

    #[allow(unused)]
    fn create_test_cpu(program: Vec<u8>) -> Cpu6502 {
        let mut cpu = Cpu6502::new();
        let rom_address = ADDRESS_TEST_PROGRAM as usize;
        cpu.memory[rom_address..(rom_address + program.len())].copy_from_slice(&program[..]);
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
    fn test_ror() {
        let mut cpu = self::create_test_cpu(vec![
            0xa9, 0x55, // LDA #$55
            0x6a,
        ]);
        cpu.set_status_register_from_byte(0x24);
        cpu.run().unwrap();
        assert_eq!(cpu.registers.a, 0x2A);
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
}
