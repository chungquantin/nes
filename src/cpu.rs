use std::fs::File;
use std::io::BufReader;

use anyhow::{Error, Result};
use structopt::StructOpt;

use crate::cli::Cli;
use crate::constant::ADDRESS_BRK;
use crate::constant::MEMORY_MAX;
use crate::constant::PC_ADDRESS_RESET;
use crate::constant::PRG_ROM_ADDRESS;
use crate::instruction::CpuInstruction;
use crate::mem::MemoryManage;
use crate::opcode::OPCODE_TABLE;
use crate::opcode::*;
use crate::register::*;

#[derive(Debug)]
pub struct Cpu6502 {
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
        Self {
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
                instr.target_val = addr_value;
                instr.target_addr = addr;

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
    fn reset_status(&mut self) {
        self.registers.carry = false;
        self.registers.zero = false;
        self.registers.interrupted = false;
        self.registers.decimal = false;
        self.registers.overflow = false;
        self.registers.negative = false;
    }

    pub fn reset(&mut self) -> Result<()> {
        self.instr = None;

        self.registers.a = 0;
        self.registers.x = 0;

        self.reset_status();
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
            target_addr: None,
            target_val: 0,
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
            LDA, LDX, LDY, BRK, TAX, TXA, TAY, TYA, TXS, AND, INX, INY, STA, STX, STY
        );
    }

    /// Read image from a provided input path
    fn load_image(self: &mut Self) {
        let cli = Cli::from_args();

        let f = File::open(cli.path).expect("couldn't open file");
        let mut f = BufReader::new(f);
        println!("{}", f.capacity());
    }
}
