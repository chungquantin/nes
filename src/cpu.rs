use anyhow::{Error, Result};

use crate::constant::MEMORY_MAX;
use crate::instruction::CpuInstruction;
use crate::opcode::OPCODE_TABLE;
use crate::opcode::*;
use crate::register::*;

#[derive(Debug)]
pub struct Cpu6502 {
    pub program_data: Vec<u8>,
    pub clocks_to_pause: u8,
    pub registers: CpuRegister,
    /// NES memory uses 16-bit for memory addressing
    /// The stack address space is hardwired to memory page $01, i.e. the address range $0100–$01FF (256–511)
    pub memory: [u8; MEMORY_MAX],
}

impl Default for Cpu6502 {
    fn default() -> Self {
        Self {
            program_data: Default::default(),
            clocks_to_pause: 0,
            registers: CpuRegister::default(),
            memory: [0u8; MEMORY_MAX],
        }
    }
}

impl Cpu6502 {
    pub fn load_program(self: &mut Self, data: Vec<u8>) -> &mut Self {
        self.program_data = data.to_vec();
        return self;
    }

    fn decode_instruction(self: &Self, opcode: u8) -> Result<CpuInstruction> {
        let (opcode, address_mode, cycle, extra_cycle) = &OPCODE_TABLE[opcode as usize];
        Ok(CpuInstruction {
            opcode: *opcode,
            cycle: *cycle,
            address_mode: *address_mode,
            extra_cycle: *extra_cycle,
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
        return execute_opcode!(LDA, LDX, LDY, BRK, TAX, TXA, TAY, TYA, TXS, AND, INX, INY);
    }

    pub fn fetch(self: &mut Self) -> Result<()> {
        self.registers.pc = 0;

        while let Some(opcode) = self.program_data.get(self.registers.pc) {
            if self.clocks_to_pause > 0 {
                self.clocks_to_pause -= 1;
                continue;
            }
            self.registers.pc += 1;
            let instr = self.decode_instruction(*opcode).unwrap();
            self.execute_instruction(&instr)?;
            self.clocks_to_pause += instr.cycle - 1;
            println!("PC: {} | INSTRUCTION: {:?}", self.registers.pc, instr);
        }
        Ok(())
    }
}
