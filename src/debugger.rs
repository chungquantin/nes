use std::{
    fmt::{Binary, Debug},
    marker::PhantomData,
};

use crate::{cpu::Cpu6502, instruction::CpuInstruction};

#[derive(Copy, Clone, Debug, Default)]
pub struct CpuDebugger<T: Binary + Debug> {
    _marker_data: PhantomData<T>,
}

impl<T> CpuDebugger<T>
where
    T: Binary + Debug,
{
    #[allow(unused)]
    pub fn bin(self, value: T) {
        let x = value;
        println!("{x:#b}")
    }

    #[allow(unused)]
    pub fn hex(self, value: T) {
        let x = value;
        println!("{:0x?}", x)
    }

    pub fn debug_instr(self, cpu: &Cpu6502, instr: CpuInstruction) {
        println!(
            "PC: ${:0x?} | OPCODE: 0x{:0x?} | INSTRUCTION: {:?}",
            cpu.registers.pc, instr.opcode, instr
        );
    }
}
