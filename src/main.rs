mod address;
mod constant;
mod cpu;
mod instruction;
mod opcode;
mod register;

fn main() {}

mod tests {
    #[test]
    pub fn test_basic() {
        let mut cpu = crate::cpu::Cpu6502::default();
        let program_data: Vec<u8> = [0xa9, 0xc0, 0xaa, 0xe8, 0x00].to_vec();
        cpu.load_program(program_data).fetch().unwrap();
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0x05, 0x00]).fetch().unwrap();

        assert_eq!(cpu.registers.a, 0x05);
        assert!(!cpu.registers.zero);
        assert!(cpu.registers.negative);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0x00, 0x00]).fetch().unwrap();

        assert!(cpu.registers.zero);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.registers.a = 10;
        cpu.load_program(vec![0xaa, 0x00]).fetch().unwrap();

        assert_eq!(cpu.registers.x, 10)
    }

    #[test]
    fn test_0x8a_txa_move_x_to_a() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.registers.x = 10;
        cpu.load_program(vec![0x8a, 0x00]).fetch().unwrap();

        assert_eq!(cpu.registers.a, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00])
            .fetch()
            .unwrap();

        assert_eq!(cpu.registers.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = crate::cpu::Cpu6502::default();
        cpu.load_program(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00])
            .fetch()
            .unwrap();

        assert_eq!(cpu.registers.x, 1)
    }
}
