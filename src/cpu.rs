use std::fs::File;
use std::io::BufReader;

use anyhow::{Error, Result};
use structopt::StructOpt;

use crate::cli::Cli;
use crate::constant::ADDRESS_BRK;
use crate::constant::MEMORY_MAX;
use crate::constant::PC_ADDRESS_RESET;
use crate::constant::PRG_ROM_ADDRESS;
use crate::debugger::CpuDebugger;
use crate::instruction::CpuInstruction;
use crate::mem::MemoryManage;
use crate::opcode::OPCODE_TABLE;
use crate::opcode::*;
use crate::register::*;

#[derive(Debug)]
pub struct Cpu6502 {
    pub debugger: CpuDebugger<u8>,
    pub running: bool,
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
            running: false,
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
        while self.running && self.registers.pc != ADDRESS_BRK {
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
    pub fn set_status_register_from_byte(&mut self, v: u8) {
        self.registers.carry = v & 0b00000001 > 0;
        self.registers.zero = v & 0b00000010 > 0;
        self.registers.interrupted = v & 0b00000100 > 0;
        self.registers.decimal = v & 0b00001000 > 0;
        // Break isn't a real register
        // Bit 5 is unused
        self.registers.overflow = v & 0b01000000 > 0;
        self.registers.negative = v & 0b10000000 > 0;
    }

    pub fn status_register_byte(&self, is_instruction: bool) -> u8 {
        let result = ((self.registers.carry      as u8) << 0) |
            ((self.registers.zero       as u8) << 1) |
            ((self.registers.interrupted as u8) << 2) |
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

    pub fn load_program(self: &mut Self, program: Vec<u8>) -> &mut Self {
        // $8000–$FFFF: ROM and mapper registers ((see MMC1 and UxROM for examples))
        let program_rom_address = PRG_ROM_ADDRESS as usize;
        self.memory[program_rom_address..(program_rom_address + program.len())]
            .copy_from_slice(&program[..]);

        // Write the value of program counter as the start address of PRG ROM
        self.mem_write_u16(PC_ADDRESS_RESET, PRG_ROM_ADDRESS)
            .unwrap();
        return self;
    }

    pub fn run(self: &mut Self) -> Result<()> {
        self.reset()?;
        self.running = true;
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
            ADC, ASL, LDA, LDX, LDY, BRK, TAX, TXA, TAY, TYA, TXS, AND, INX, INY, STA, STX, STY
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
