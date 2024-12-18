use super::Cpu;
use crate::memory::Memory;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::ops::ControlFlow;
use utils::fetch_from_pc;

pub(in crate::cpu) mod instructions;
use instructions::*;

#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    // ADC
    AdcImmediate = 0x69,
    AdcZeroPage = 0x65,
    AdcZeroPageX = 0x75,
    AdcAbsolute = 0x6D,
    AdcAbsoluteX = 0x7D,
    AdcAbsoluteY = 0x79,
    AdcIndirectX = 0x61,
    AdcIndirectY = 0x71,

    // AND
    AndImmediate = 0x29,
    AndZeroPage = 0x25,
    AndZeroPageX = 0x35,
    AndAbsolute = 0x2D,
    AndAbsoluteX = 0x3D,
    AndAbsoluteY = 0x39,
    AndIndirectX = 0x21,
    AndIndirectY = 0x31,

    // BIT
    BitZeroPage = 0x24,
    BitAbsolute = 0x2C,

    Clc = 0x18,
    Cld = 0xD8,
    Cli = 0x58,
    Clv = 0xB8,

    // CMP
    CmpImmediate = 0xC9,
    CmpZeroPage = 0xC5,
    CmpZeroPageX = 0xD5,
    CmpAbsolute = 0xCD,
    CmpAbsoluteX = 0xDD,
    CmpAbsoluteY = 0xD9,
    CmpIndirectX = 0xC1,
    CmpIndirectY = 0xD1,

    // DEX/Y
    Dex = 0xCA,
    Dey = 0x88,

    // EOR
    EorImmediate = 0x49,
    EorZeroPage = 0x45,
    EorZeroPageX = 0x55,
    EorAbsolute = 0x4D,
    EorAbsoluteX = 0x5D,
    EorAbsoluteY = 0x59,
    EorIndirectX = 0x41,
    EorIndirectY = 0x51,

    // INX/Y
    Inx = 0xE8,
    Iny = 0xC8,

    // LDA
    LdaImmediate = 0xA9,
    LdaZeroPage = 0xA5,
    LdaZeroPageX = 0xB5,
    LdaAbsolute = 0xAD,
    LdaAbsoluteX = 0xBD,
    LdaAbsoluteY = 0xB9,
    LdaIndirectX = 0xA1,
    LdaIndirectY = 0xB1,

    // LDX
    LdxImmediate = 0xA2,
    LdxZeroPage = 0xA6,
    LdxZeroPageY = 0xB6,
    LdxAbsolute = 0xAE,
    LdxAbsoluteY = 0xBE,

    // LDY
    LdyImmediate = 0xA0,
    LdyZeroPage = 0xA4,
    LdyZeroPageX = 0xB4,
    LdyAbsolute = 0xAC,
    LdyAbsoluteX = 0xBC,

    Nop = 0xEA,

    // ORA
    OraImmediate = 0x09,
    OraZeroPage = 0x05,
    OraZeroPageX = 0x15,
    OraAbsolute = 0x0D,
    OraAbsoluteX = 0x1D,
    OraAbsoluteY = 0x19,
    OraIndirectX = 0x01,
    OraIndirectY = 0x11,

    // SBC
    SbcImmediate = 0xE9,
    SbcZeroPage = 0xE5,
    SbcZeroPageX = 0xF5,
    SbcAbsolute = 0xED,
    SbcAbsoluteX = 0xFD,
    SbcAbsoluteY = 0xF9,
    SbcIndirectX = 0xE1,
    SbcIndirectY = 0xF1,

    // SE*
    Sec = 0x38,
    Sed = 0xF8,
    Sei = 0x78,

    // STA
    StaZeroPage = 0x85,
    StaZeroPageX = 0x95,
    StaAbsolute = 0x8D,
    StaAbsoluteX = 0x9D,
    StaAbsoluteY = 0x99,
    StaIndirectX = 0x81,
    StaIndirectY = 0x91,

    // STX
    StxZeroPage = 0x86,
    StxZeroPageY = 0x96,
    StxAbsolute = 0x8E,

    // STY
    StyZeroPage = 0x84,
    StyZeroPageX = 0x94,
    StyAbsolute = 0x8C,

    // T??
    Tax = 0xAA,
    Tay = 0xA8,
    Tsx = 0xBA,
    Txa = 0x8A,
    Txs = 0x9A,
    Tya = 0x98,

    #[default]
    Unimplemented = 0x0,
}

pub fn dispatch_current_opcode<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    // First cycle is always fetching the opcode
    if cpu.current_cycle == 0 {
        cpu.current_opcode = OpCode::from(fetch_from_pc(cpu, memory));
        return ControlFlow::Continue(());
    }
    match cpu.current_opcode {
        // ADC
        OpCode::AdcImmediate => adc::immediate(cpu, memory),
        OpCode::AdcZeroPage => adc::zeropage(cpu, memory),
        OpCode::AdcZeroPageX => adc::zeropage_x(cpu, memory),
        OpCode::AdcAbsolute => adc::absolute(cpu, memory),
        OpCode::AdcAbsoluteX => adc::absolute_x(cpu, memory),
        OpCode::AdcAbsoluteY => adc::absolute_y(cpu, memory),
        OpCode::AdcIndirectX => adc::indirect_x(cpu, memory),
        OpCode::AdcIndirectY => adc::indirect_y(cpu, memory),

        //AND
        OpCode::AndImmediate => and::immediate(cpu, memory),
        OpCode::AndZeroPage => and::zeropage(cpu, memory),
        OpCode::AndZeroPageX => and::zeropage_x(cpu, memory),
        OpCode::AndAbsolute => and::absolute(cpu, memory),
        OpCode::AndAbsoluteX => and::absolute_x(cpu, memory),
        OpCode::AndAbsoluteY => and::absolute_y(cpu, memory),
        OpCode::AndIndirectX => and::indirect_x(cpu, memory),
        OpCode::AndIndirectY => and::indirect_y(cpu, memory),

        // BIT
        OpCode::BitZeroPage => bit::zeropage(cpu, memory),
        OpCode::BitAbsolute => bit::absolute(cpu, memory),

        // CL*
        OpCode::Clc => clc(cpu, memory),
        OpCode::Cld => cld(cpu, memory),
        OpCode::Cli => cli(cpu, memory),
        OpCode::Clv => clv(cpu, memory),

        // CMP
        OpCode::CmpImmediate => cmp::immediate(cpu, memory),
        OpCode::CmpZeroPage => cmp::zeropage(cpu, memory),
        OpCode::CmpZeroPageX => cmp::zeropage_x(cpu, memory),
        OpCode::CmpAbsolute => cmp::absolute(cpu, memory),
        OpCode::CmpAbsoluteX => cmp::absolute_x(cpu, memory),
        OpCode::CmpAbsoluteY => cmp::absolute_y(cpu, memory),
        OpCode::CmpIndirectX => cmp::indirect_x(cpu, memory),
        OpCode::CmpIndirectY => cmp::indirect_y(cpu, memory),

        // DEX/Y
        OpCode::Dex => dex(cpu, memory),
        OpCode::Dey => dey(cpu, memory),

        // EOR
        OpCode::EorImmediate => eor::immediate(cpu, memory),
        OpCode::EorZeroPage => eor::zeropage(cpu, memory),
        OpCode::EorZeroPageX => eor::zeropage_x(cpu, memory),
        OpCode::EorAbsolute => eor::absolute(cpu, memory),
        OpCode::EorAbsoluteX => eor::absolute_x(cpu, memory),
        OpCode::EorAbsoluteY => eor::absolute_y(cpu, memory),
        OpCode::EorIndirectX => eor::indirect_x(cpu, memory),
        OpCode::EorIndirectY => eor::indirect_y(cpu, memory),

        // INX/Y
        OpCode::Inx => inx(cpu, memory),
        OpCode::Iny => iny(cpu, memory),

        // LDA
        OpCode::LdaImmediate => lda::immediate(cpu, memory),
        OpCode::LdaZeroPage => lda::zeropage(cpu, memory),
        OpCode::LdaZeroPageX => lda::zeropage_x(cpu, memory),
        OpCode::LdaAbsolute => lda::absolute(cpu, memory),
        OpCode::LdaAbsoluteX => lda::absolute_x(cpu, memory),
        OpCode::LdaAbsoluteY => lda::absolute_y(cpu, memory),
        OpCode::LdaIndirectX => lda::indirect_x(cpu, memory),
        OpCode::LdaIndirectY => lda::indirect_y(cpu, memory),

        // LDX
        OpCode::LdxImmediate => ldx::immediate(cpu, memory),
        OpCode::LdxZeroPage => ldx::zeropage(cpu, memory),
        OpCode::LdxZeroPageY => ldx::zeropage_y(cpu, memory),
        OpCode::LdxAbsolute => ldx::absolute(cpu, memory),
        OpCode::LdxAbsoluteY => ldx::absolute_y(cpu, memory),

        // LDY
        OpCode::LdyImmediate => ldy::immediate(cpu, memory),
        OpCode::LdyZeroPage => ldy::zeropage(cpu, memory),
        OpCode::LdyZeroPageX => ldy::zeropage_x(cpu, memory),
        OpCode::LdyAbsolute => ldy::absolute(cpu, memory),
        OpCode::LdyAbsoluteX => ldy::absolute_x(cpu, memory),

        // NOP
        OpCode::Nop => nop(cpu, memory),

        // ORA
        OpCode::OraImmediate => ora::immediate(cpu, memory),
        OpCode::OraZeroPage => ora::zeropage(cpu, memory),
        OpCode::OraZeroPageX => ora::zeropage_x(cpu, memory),
        OpCode::OraAbsolute => ora::absolute(cpu, memory),
        OpCode::OraAbsoluteX => ora::absolute_x(cpu, memory),
        OpCode::OraAbsoluteY => ora::absolute_y(cpu, memory),
        OpCode::OraIndirectX => ora::indirect_x(cpu, memory),
        OpCode::OraIndirectY => ora::indirect_y(cpu, memory),

        // SBC
        OpCode::SbcImmediate => sbc::immediate(cpu, memory),
        OpCode::SbcZeroPage => sbc::zeropage(cpu, memory),
        OpCode::SbcZeroPageX => sbc::zeropage_x(cpu, memory),
        OpCode::SbcAbsolute => sbc::absolute(cpu, memory),
        OpCode::SbcAbsoluteX => sbc::absolute_x(cpu, memory),
        OpCode::SbcAbsoluteY => sbc::absolute_y(cpu, memory),
        OpCode::SbcIndirectX => sbc::indirect_x(cpu, memory),
        OpCode::SbcIndirectY => sbc::indirect_y(cpu, memory),

        // SE*
        OpCode::Sec => sec(cpu, memory),
        OpCode::Sed => sed(cpu, memory),
        OpCode::Sei => sei(cpu, memory),

        // STA
        OpCode::StaZeroPage => sta::zeropage(cpu, memory),
        OpCode::StaZeroPageX => sta::zeropage_x(cpu, memory),
        OpCode::StaAbsolute => sta::absolute(cpu, memory),
        OpCode::StaAbsoluteX => sta::absolute_x(cpu, memory),
        OpCode::StaAbsoluteY => sta::absolute_y(cpu, memory),
        OpCode::StaIndirectX => sta::indirect_x(cpu, memory),
        OpCode::StaIndirectY => sta::indirect_y(cpu, memory),

        // STX
        OpCode::StxZeroPage => stx::zeropage(cpu, memory),
        OpCode::StxZeroPageY => stx::zeropage_y(cpu, memory),
        OpCode::StxAbsolute => stx::absolute(cpu, memory),

        // STY
        OpCode::StyZeroPage => sty::zeropage(cpu, memory),
        OpCode::StyZeroPageX => sty::zeropage_x(cpu, memory),
        OpCode::StyAbsolute => sty::absolute(cpu, memory),

        // T??
        OpCode::Tax => tax(cpu, memory),
        OpCode::Tay => tay(cpu, memory),
        OpCode::Tsx => tsx(cpu, memory),
        OpCode::Txa => txa(cpu, memory),
        OpCode::Txs => txs(cpu, memory),
        OpCode::Tya => tya(cpu, memory),

        _ => unimplemented!(),
    }
}
