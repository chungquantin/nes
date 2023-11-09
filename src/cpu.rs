use std::fs::File;
use std::io::BufReader;

use anyhow::{Error, Result};
use structopt::StructOpt;

use crate::cli::Cli;
use crate::constant::ADDRESS_BRK;
use crate::constant::MEMORY_MAX;
use crate::constant::NEGATIVE_FLAG;
use crate::constant::PC_ADDRESS_RESET;
use crate::constant::PRG_ROM_ADDRESS;
use crate::debugger::CpuDebugger;
use crate::instruction::CpuInstruction;
use crate::mem::MemoryManage;
use crate::opcode::{Operation, OPCODE_TABLE};
use crate::register::*;

#[derive(Debug)]
pub struct Cpu6502 {
    pub debugger: CpuDebugger<u8>,
    pub clocks_to_pause: u8,
    pub registers: CpuRegister,
    /// NES memory uses 16-bit for memory addressing
    /// The stack address space is hardwired to memory page $01, i.e. the address range $0100–$01FF (256–511)
    pub memory: [u8; MEMORY_MAX],
    pub instr: Option<CpuInstruction>, // The currently executing instruction
}

impl Default for Cpu6502 {
    fn default() -> Self {
        let debugger = CpuDebugger::default();
        Self {
            debugger,
            clocks_to_pause: 0,
            registers: CpuRegister::default(),
            memory: [0u8; MEMORY_MAX],
            instr: None,
        }
    }
}

pub trait Clocked {
    fn clocked(self: &mut Self) -> Result<()>;
}

impl Clocked for Cpu6502 {
    fn clocked(self: &mut Self) -> Result<()> {
        // // load cpu program counter register at $8000
        while self.registers.pc != ADDRESS_BRK {
            if let Ok(opcode) = self.mem_read(self.registers.pc) {
                let mut instr = self.decode_instruction(opcode as u8).unwrap();
                let (addr, addr_value, num_bytes) =
                    self.decode_addressing_mode(instr.address_mode)?;
                instr.mode_args = addr_value;
                instr.write_target = addr;

                println!(
                    "PC: ${:0x?} | OPCODE: 0x{:0x?} | INSTRUCTION: {:?}",
                    self.registers.pc, opcode, instr
                );

                self.instr = Some(instr);
                if self.clocks_to_pause > 0 {
                    self.clocks_to_pause -= 1;
                    continue;
                }
                if instr.opcode == Operation::BRK {
                    break;
                }

                self.execute_instruction(&instr)?;

                self.registers.pc += num_bytes + 1;
                self.clocks_to_pause += instr.cycle - 1;
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl Cpu6502 {
    pub fn is_negative(&self, result: u8) -> bool {
        (result & 0x80) == NEGATIVE_FLAG
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.zero = result == 0x0;
        // bit masking and get the first bit
        self.registers.negative = self.is_negative(result);
    }

    pub fn update_accumulator_flags(&mut self) {
        self.update_zero_and_negative_flags(self.registers.a);
    }

    #[allow(dead_code)]
    pub fn print_register_status(&self) {
        println!("Carry: {:?}", self.registers.carry);
        println!("Decimal: {:?}", self.registers.decimal);
        println!("Negative: {:?}", self.registers.negative);
        println!("Overflow: {:?}", self.registers.overflow);
        println!(
            "interrupt_disabled: {:?}",
            self.registers.interrupt_disabled
        );
        println!("Zero: {:?}", self.registers.zero);
    }

    #[allow(unused)]
    pub fn set_status_register_from_byte(&mut self, v: u8) {
        self.registers.carry = v & 0b00000001 > 0;
        self.registers.zero = v & 0b00000010 > 0;
        self.registers.interrupt_disabled = v & 0b00000100 > 0;
        self.registers.decimal = v & 0b00001000 > 0;
        // Break isn't a real register
        // Bit 5 is unused
        self.registers.overflow = v & 0b01000000 > 0;
        self.registers.negative = v & 0b10000000 > 0;
    }

    #[allow(unused)]
    pub fn status_register_byte(&self, is_instruction: bool) -> u8 {
        let result = ((self.registers.carry      as u8) << 0) |
            ((self.registers.zero       as u8) << 1) |
            ((self.registers.interrupt_disabled as u8) << 2) |
            ((self.registers.decimal    as u8) << 3) |
            (0                       << 4) | // Break flag
            ((if is_instruction {1} else {0}) << 5) |
            ((self.registers.overflow   as u8) << 6) |
            ((self.registers.negative   as u8) << 7);
        return result;
    }

    pub fn reset(&mut self) -> Result<()> {
        self.instr = None;

        self.registers.a = 0;
        self.registers.x = 0;
        // Reset the address of program counter
        self.registers.pc = self.mem_read_u16(PC_ADDRESS_RESET).unwrap();
        Ok(())
    }

    pub fn load_program(self: &mut Self, program: Vec<u8>) -> Result<()> {
        // $8000–$FFFF: ROM and mapper registers ((see MMC1 and UxROM for examples))
        let program_rom_address = PRG_ROM_ADDRESS as usize;
        self.memory[program_rom_address..(program_rom_address + program.len())]
            .copy_from_slice(&program[..]);

        // Write the value of program counter as the start address of PRG ROM
        self.mem_write_u16(PC_ADDRESS_RESET, PRG_ROM_ADDRESS)
            .unwrap();

        // Reset the cpu after loading the program
        self.reset()?;

        Ok(())
    }

    pub fn run(self: &mut Self) -> Result<()> {
        self.clocked()?;
        Ok(())
    }

    fn decode_instruction(self: &Self, opcode: u8) -> Result<CpuInstruction> {
        let (opcode, address_mode, cycle, extra_cycle) = &OPCODE_TABLE[opcode as usize];
        Ok(CpuInstruction {
            opcode: *opcode,
            cycle: *cycle,
            address_mode: *address_mode,
            extra_cycle: *extra_cycle,
            write_target: None,
            mode_args: 0,
        })
    }

    fn execute_instruction(self: &mut Self, instruction: &CpuInstruction) -> Result<(), Error> {
        macro_rules! execute_opcode {
            ($($opcode:ident),*) => {
                match instruction.opcode {
                    $(
                        Operation::$opcode => self.$opcode(),
                    )*
                    _ => unimplemented!()
                }
            };
        }
        return execute_opcode!(
            ADC, AND, ASL, // Axx
            BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, // Bxx
            CLC, CLD, CLI, CLV, CMP, CPX, CPY, // Cxx
            DEC, DEX, DEY, // Dxx
            EOR, // Exx
            INC, INX, INY, // Ixx
            JMP, JSR, // Jxx
            LDA, LDX, LDY, LSR, // Lxx
            NOP, // Nxx
            ORA, // Oxx
            PHA, PHP, PLA, PLP, // Pxx
            ROL, ROR, RTI, RTS, // Rxx
            SBC, SEC, SED, SEI, STA, STX, STY, // Sxx
            TAX, TXA, TAY, TYA, TXS // Txx
        );
    }

    /// Read image from a provided input path
    #[allow(dead_code)]
    fn load_image(self: &mut Self) {
        let cli = Cli::from_args();

        let f = File::open(cli.path).expect("couldn't open file");
        let f = BufReader::new(f);
        println!("{}", f.capacity());
    }
}
