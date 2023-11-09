use crate::{address::*, instruction::CycleCount};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    // "Extra" opcodes
    KIL,
    ISC,
    DCP,
    AXS,
    LAS,
    LAX,
    AHX,
    SAX,
    XAA,
    SHX,
    RRA,
    TAS,
    SHY,
    ARR,
    SRE,
    ALR,
    RLA,
    ANC,
    SLO,
}

use Operation::*;

pub const OPCODE_TABLE: [(Operation, AddressingMode, CycleCount, CycleCount); 256] =
    // TODO Audit each record to see that it was input correctly
    // (Operation, addressing mode, clock cycles, extra clock cycles if page boundary crossed)
    [
        // 0x
        (BRK, IMP, 7, 0), // x0
        (ORA, IZX, 6, 0), // x1
        (KIL, IMP, 0, 0), // x2
        (SLO, IZX, 8, 0), // x3
        (NOP, ZP, 3, 0),  // x4
        (ORA, ZP, 3, 0),  // x5
        (ASL, ZP, 5, 0),  // x6
        (SLO, ZP, 5, 0),  // x7
        (PHP, IMP, 3, 0), // x8
        (ORA, IMM, 2, 0), // x9
        (ASL, ACC, 2, 0), // xA
        (ANC, IMM, 2, 0), // xB
        (NOP, ABS, 4, 0), // xC
        (ORA, ABS, 4, 0), // xD
        (ASL, ABS, 6, 0), // xE
        (SLO, ABS, 6, 0), // xF
        // 1x
        (BPL, REL, 2, 1), // x0
        (ORA, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (SLO, IZY, 8, 0), // x3
        (NOP, ZPX, 4, 0), // x4
        (ORA, ZPX, 4, 0), // x5
        (ASL, ZPX, 6, 0), // x6
        (SLO, ZPX, 6, 0), // x7
        (CLC, IMP, 2, 0), // x8
        (ORA, ABY, 4, 1), // x9
        (NOP, IMP, 2, 0), // xA
        (SLO, ABY, 7, 0), // xB
        (NOP, ABX, 4, 1), // xC
        (ORA, ABX, 4, 1), // xD
        (ASL, ABX, 7, 0), // xE
        (SLO, ABX, 7, 0), // xF
        // 2x
        (JSR, ABS, 6, 0), // x0
        (AND, IZX, 6, 0), // x1
        (KIL, IMP, 0, 0), // x2
        (RLA, IZX, 8, 0), // x3
        (BIT, ZP, 3, 0),  // x4
        (AND, ZP, 3, 0),  // x5
        (ROL, ZP, 5, 0),  // x6
        (RLA, ZP, 5, 0),  // x7
        (PLP, IMP, 4, 0), // x8
        (AND, IMM, 2, 0), // x9
        (ROL, ACC, 2, 0), // xA
        (ANC, IMM, 2, 0), // xB
        (BIT, ABS, 4, 0), // xC
        (AND, ABS, 4, 0), // xD
        (ROL, ABS, 6, 0), // xE
        (RLA, ABS, 6, 0), // xF
        // 3x
        (BMI, REL, 2, 1), // x0
        (AND, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (RLA, IZY, 8, 0), // x3
        (NOP, ZPX, 4, 0), // x4
        (AND, ZPX, 4, 0), // x5
        (ROL, ZPX, 6, 0), // x6
        (RLA, ZPX, 6, 0), // x7
        (SEC, IMP, 2, 0), // x8
        (AND, ABY, 4, 1), // x9
        (NOP, IMP, 2, 0), // xA
        (RLA, ABY, 7, 0), // xB
        (NOP, ABX, 4, 1), // xC
        (AND, ABX, 4, 1), // xD
        (ROL, ABX, 7, 0), // xE
        (RLA, ABX, 7, 0), // xF
        // 4x
        (RTI, IMP, 6, 0), // x0
        (EOR, IZX, 6, 0), // x1
        (KIL, IMP, 0, 0), // x2
        (SRE, IZX, 8, 0), // x3
        (NOP, ZP, 3, 0),  // x4
        (EOR, ZP, 3, 0),  // x5
        (LSR, ZP, 5, 0),  // x6
        (SRE, ZP, 5, 0),  // x7
        (PHA, IMP, 3, 0), // x8
        (EOR, IMM, 2, 0), // x9
        (LSR, IMP, 2, 0), // xA
        (ALR, IMM, 2, 0), // xB
        (JMP, ABS, 3, 0), // xC
        (EOR, ABS, 4, 0), // xD
        (LSR, ABS, 6, 0), // xE
        (SRE, ABS, 6, 0), // xF
        // 5x
        (BVC, REL, 2, 1), // x0
        (EOR, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (SRE, IZY, 8, 0), // x3
        (NOP, ZPX, 4, 0), // x4
        (EOR, ZPX, 4, 0), // x5
        (LSR, ZPX, 6, 0), // x6
        (SRE, ZPX, 6, 0), // x7
        (CLI, IMP, 2, 0), // x8
        (EOR, ABY, 4, 1), // x9
        (NOP, IMP, 2, 0), // xA
        (SRE, ABY, 7, 0), // xB
        (NOP, ABX, 4, 1), // xC
        (EOR, ABX, 4, 1), // xD
        (LSR, ABX, 7, 0), // xE
        (SRE, ABX, 7, 0), // xF
        // 6x
        (RTS, IMP, 6, 0), // x0
        (ADC, IZX, 6, 0), // x1
        (KIL, IMP, 0, 0), // x2
        (RRA, IZX, 8, 0), // x3
        (NOP, ZP, 3, 0),  // x4
        (ADC, ZP, 3, 0),  // x5
        (ROR, ZP, 5, 0),  // x6
        (RRA, ZP, 5, 0),  // x7
        (PLA, IMP, 4, 0), // x8
        (ADC, IMM, 2, 0), // x9
        (ROR, IMP, 2, 0), // xA
        (ARR, IMM, 2, 0), // xB
        (JMP, IND, 5, 0), // xC
        (ADC, ABS, 4, 0), // xD
        (ROR, ABS, 6, 0), // xE
        (RRA, ABS, 6, 0), // xF
        // 7x
        (BVS, REL, 2, 1), // x0
        (ADC, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (RRA, IZY, 8, 0), // x3
        (NOP, ZPX, 4, 0), // x4
        (ADC, ZPX, 4, 0), // x5
        (ROR, ZPX, 6, 0), // x6
        (RRA, ZPX, 6, 0), // x7
        (SEI, IMP, 2, 0), // x8
        (ADC, ABY, 4, 1), // x9
        (NOP, IMP, 2, 0), // xA
        (RRA, ABY, 7, 0), // xB
        (NOP, ABX, 4, 1), // xC
        (ADC, ABX, 4, 1), // xD
        (ROR, ABX, 7, 0), // xE
        (RRA, ABX, 7, 0), // xF
        // 8x
        (NOP, IMM, 2, 0), // x0
        (STA, IZX, 6, 0), // x1
        (NOP, IMM, 2, 0), // x2
        (SAX, IZX, 6, 0), // x3
        (STY, ZP, 3, 0),  // x4
        (STA, ZP, 3, 0),  // x5
        (STX, ZP, 3, 0),  // x6
        (SAX, ZP, 3, 0),  // x7
        (DEY, IMP, 2, 0), // x8
        (NOP, IMM, 2, 0), // x9
        (TXA, IMP, 2, 0), // xA
        (XAA, IMM, 2, 1), // xB
        (STY, ABS, 4, 0), // xC
        (STA, ABS, 4, 0), // xD
        (STX, ABS, 4, 0), // xE
        (SAX, ABS, 4, 0), // xF
        // 9x
        (BCC, REL, 2, 1), // x0
        (STA, IZY, 6, 0), // x1
        (KIL, IMP, 0, 0), // x2
        (AHX, IZY, 6, 0), // x3
        (STY, ZPX, 4, 0), // x4
        (STA, ZPX, 4, 0), // x5
        (STX, ZPY, 4, 0), // x6
        (SAX, ZPY, 4, 0), // x7
        (TYA, IMP, 2, 0), // x8
        (STA, ABY, 5, 0), // x9
        (TXS, IMP, 2, 0), // xA
        (TAS, ABY, 5, 0), // xB
        (SHY, ABX, 5, 0), // xC
        (STA, ABX, 5, 0), // xD
        (SHX, ABY, 5, 0), // xE
        (AHX, ABY, 5, 0), // xF
        // Ax
        (LDY, IMM, 2, 0), // x0
        (LDA, IZX, 6, 0), // x1
        (LDX, IMM, 2, 0), // x2
        (LAX, IZX, 6, 0), // x3
        (LDY, ZP, 3, 0),  // x4
        (LDA, ZP, 3, 0),  // x5
        (LDX, ZP, 3, 0),  // x6
        (LAX, ZP, 3, 0),  // x7
        (TAY, IMP, 2, 0), // x8
        (LDA, IMM, 2, 0), // x9
        (TAX, IMP, 2, 0), // xA
        (LAX, IMM, 2, 0), // xB
        (LDY, ABS, 4, 0), // xC
        (LDA, ABS, 4, 0), // xD
        (LDX, ABS, 4, 0), // xE
        (LAX, ABS, 4, 0), // xF
        // Bx
        (BCS, REL, 2, 1), // x0
        (LDA, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (LAX, IZY, 5, 1), // x3
        (LDY, ZPX, 4, 0), // x4
        (LDA, ZPX, 4, 0), // x5
        (LDX, ZPY, 4, 0), // x6
        (LAX, ZPY, 4, 0), // x7
        (CLV, IMP, 2, 0), // x8
        (LDA, ABY, 4, 1), // x9
        (TSX, IMP, 2, 0), // xA
        (LAS, ABY, 4, 1), // xB
        (LDY, ABX, 4, 1), // xC
        (LDA, ABX, 4, 1), // xD
        (LDX, ABY, 4, 1), // xE
        (LAX, ABY, 4, 1), // xF
        // Cx
        (CPY, IMM, 2, 0), // x0
        (CMP, IZX, 6, 0), // x1
        (NOP, IMM, 2, 0), // x2
        (DCP, IZX, 8, 0), // x3
        (CPY, ZP, 3, 0),  // x4
        (CMP, ZP, 3, 0),  // x5
        (DEC, ZP, 5, 0),  // x6
        (DCP, ZP, 5, 0),  // x7
        (INY, IMP, 2, 0), // x8
        (CMP, IMM, 2, 0), // x9
        (DEX, IMP, 2, 0), // xA
        (AXS, IMM, 2, 0), // xB
        (CPY, ABS, 4, 0), // xC
        (CMP, ABS, 4, 0), // xD
        (DEC, ABS, 6, 0), // xE
        (DCP, ABS, 6, 0), // xF
        // Dx
        (BNE, REL, 2, 1), // x0
        (CMP, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (DCP, IZY, 8, 0), // x3
        (NOP, ZPX, 4, 0), // x4
        (CMP, ZPX, 4, 0), // x5
        (DEC, ZPX, 6, 0), // x6
        (DCP, ZPX, 6, 0), // x7
        (CLD, IMP, 2, 0), // x8
        (CMP, ABY, 4, 1), // x9
        (NOP, IMP, 2, 0), // xA
        (DCP, ABY, 7, 0), // xB
        (NOP, ABX, 4, 1), // xC
        (CMP, ABX, 4, 1), // xD
        (DEC, ABX, 7, 0), // xE
        (DCP, ABX, 7, 0), // xF
        // Ex
        (CPX, IMM, 2, 0), // x0
        (SBC, IZX, 6, 0), // x1
        (NOP, IMM, 2, 0), // x2
        (ISC, IZX, 8, 0), // x3
        (CPX, ZP, 3, 0),  // x4
        (SBC, ZP, 3, 0),  // x5
        (INC, ZP, 5, 0),  // x6
        (ISC, ZP, 5, 0),  // x7
        (INX, IMP, 2, 0), // x8
        (SBC, IMM, 2, 0), // x9
        (NOP, IMP, 2, 0), // xA
        (SBC, IMM, 2, 0), // xB
        (CPX, ABS, 4, 0), // xC
        (SBC, ABS, 4, 0), // xD
        (INC, ABS, 6, 0), // xE
        (ISC, ABS, 6, 0), // xF
        // Fx
        (BEQ, REL, 2, 1), // x0
        (SBC, IZY, 5, 1), // x1
        (KIL, IMP, 0, 0), // x2
        (ISC, IZY, 8, 0), // x3
        (NOP, ZPX, 4, 0), // x4
        (SBC, ZPX, 4, 0), // x5
        (INC, ZPX, 6, 0), // x6
        (ISC, ZPX, 6, 0), // x7
        (SED, IMP, 2, 0), // x8
        (SBC, ABY, 4, 1), // x9
        (NOP, IMP, 2, 0), // xA
        (ISC, ABY, 7, 0), // xB
        (NOP, ABX, 4, 1), // xC
        (SBC, ABX, 4, 1), // xD
        (INC, ABX, 7, 0), // xE
        (ISC, ABX, 7, 0), // xF
    ];
