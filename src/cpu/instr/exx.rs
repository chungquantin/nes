use anyhow::Result;

use crate::cpu::Cpu6502;

impl Cpu6502 {
    #[inline]
    #[allow(non_snake_case)]
    pub fn EOR(&mut self) -> Result<()> {
        let instr = self.instr.unwrap();
        // An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
        self.registers.a ^= instr.mode_args as u8;

        self.update_accumulator_flags();

        Ok(())
    }
}
