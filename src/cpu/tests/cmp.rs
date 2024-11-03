use crate::cpu::{Cpu, StatusFlags};
use utils::possible_byte_pairs;

use super::*;

fn verify(a: u8, b: u8) -> impl Fn(&mut Cpu, &mut TestMemory) {
    let z = a == b;
    let n = (a.wrapping_sub(b) as i8) < 0;
    let c = a >= b;

    move |cpu, _memory| {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            c,
            "CARRY flag set incorrectly, a = {a}, b = {b}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            n,
            "NEGATIVE flag set incorrectly, a = {a}, b = {b}"
        );
        assert_eq!(cpu.flags.contains(StatusFlags::ZERO), z);
    }
}

#[test]
fn immediate() {
    for (a, b) in possible_byte_pairs() {
        TestOpcodeOptions::new(OpCode::CmpImmediate, 2, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
            })
            .with_arguments(&[b])
            .test();
    }
}

#[test]
fn zeropage() {
    for (a, b) in possible_byte_pairs() {
        let addr = 0x25;

        TestOpcodeOptions::new(OpCode::CmpZeroPage, 3, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
            })
            .with_arguments(&[addr])
            .with_additional_values(&[(addr as u16, b)])
            .test();
    }
}

#[test]
fn zeropage_x() {
    for (a, b) in possible_byte_pairs() {
        let base_addr = 0x25;
        let offset = 0x20;

        TestOpcodeOptions::new(OpCode::CmpZeroPageX, 4, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&[base_addr])
            .with_additional_values(&[(base_addr.wrapping_add(offset) as u16, b)])
            .test();
    }
}

#[test]
fn zeropage_x_overflow() {
    for (a, b) in possible_byte_pairs() {
        let base_addr = 0x85;
        let offset = 0xD0;

        TestOpcodeOptions::new(OpCode::CmpZeroPageX, 4, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&[base_addr])
            .with_additional_values(&[(base_addr.wrapping_add(offset) as u16, b)])
            .test();
    }
}

#[test]
fn absolute() {
    for (a, b) in possible_byte_pairs() {
        let addr: u16 = 0x0425;

        TestOpcodeOptions::new(OpCode::CmpAbsolute, 4, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr, b)])
            .test();
    }
}

#[test]
fn absolute_x() {
    for (a, b) in possible_byte_pairs() {
        let addr: u16 = 0x0425;
        let offset: u8 = 0x5A;

        TestOpcodeOptions::new(OpCode::CmpAbsoluteX, 4, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn absolute_y() {
    for (a, b) in possible_byte_pairs() {
        let addr: u16 = 0x0425;
        let offset: u8 = 0x5A;

        TestOpcodeOptions::new(OpCode::CmpAbsoluteY, 4, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn absolute_x_overflow() {
    for (a, b) in possible_byte_pairs() {
        let addr: u16 = 0x04A5;
        let offset: u8 = 0x6A;

        TestOpcodeOptions::new(OpCode::CmpAbsoluteX, 5, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn absolute_y_overflow() {
    for (a, b) in possible_byte_pairs() {
        let addr: u16 = 0x04A5;
        let offset: u8 = 0x6A;

        TestOpcodeOptions::new(OpCode::CmpAbsoluteY, 5, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn indirect_x() {
    for (a, b) in possible_byte_pairs() {
        let ptr_base: u8 = 0x3F;
        let offset: u8 = 0x5A;
        let final_ptr = ptr_base.wrapping_add(offset) as u16;
        let addr: u16 = 0x0458;

        TestOpcodeOptions::new(OpCode::CmpIndirectX, 6, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&[ptr_base])
            .with_additional_values(&[
                (final_ptr, addr.to_le_bytes()[0]),
                (final_ptr.wrapping_add(1), addr.to_le_bytes()[1]),
                (addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_y() {
    for (a, b) in possible_byte_pairs() {
        let ptr: u8 = 0x3F;
        let offset: u8 = 0x5A;
        let base_addr: u16 = 0x0458;
        let final_addr = base_addr.wrapping_add(offset as u16);

        TestOpcodeOptions::new(OpCode::CmpIndirectY, 5, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
            })
            .with_arguments(&[ptr])
            .with_additional_values(&[
                (ptr as u16, base_addr.to_le_bytes()[0]),
                (ptr.wrapping_add(1) as u16, base_addr.to_le_bytes()[1]),
                (final_addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_x_overflow() {
    for (a, b) in possible_byte_pairs() {
        let ptr_base: u8 = 0x3F;
        let offset: u8 = 0xFA;
        let final_ptr = ptr_base.wrapping_add(offset) as u16;
        let addr: u16 = 0x0458;

        TestOpcodeOptions::new(OpCode::CmpIndirectX, 6, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&[ptr_base])
            .with_additional_values(&[
                (final_ptr, addr.to_le_bytes()[0]),
                (final_ptr.wrapping_add(1), addr.to_le_bytes()[1]),
                (addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_y_overflow() {
    for (a, b) in possible_byte_pairs() {
        let ptr: u8 = 0x3F;
        let offset: u8 = 0xFA;
        let base_addr: u16 = 0x0458;
        let final_addr = base_addr.wrapping_add(offset as u16);

        TestOpcodeOptions::new(OpCode::CmpIndirectY, 6, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
            })
            .with_arguments(&[ptr])
            .with_additional_values(&[
                (ptr as u16, base_addr.to_le_bytes()[0]),
                (ptr.wrapping_add(1) as u16, base_addr.to_le_bytes()[1]),
                (final_addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_x_page_split() {
    for (a, b) in possible_byte_pairs() {
        let ptr_base: u8 = 0xFF;
        let offset: u8 = 0x0;
        let final_ptr = ptr_base.wrapping_add(offset);
        let addr: u16 = 0x0458;

        TestOpcodeOptions::new(OpCode::CmpIndirectX, 6, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
            })
            .with_arguments(&[ptr_base])
            .with_additional_values(&[
                (final_ptr as u16, addr.to_le_bytes()[0]),
                (final_ptr.wrapping_add(1) as u16, addr.to_le_bytes()[1]),
                (addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_y_page_split() {
    for (a, b) in possible_byte_pairs() {
        let ptr: u8 = 0xFF;
        let offset: u8 = 0x5A;
        let base_addr: u16 = 0x0458;
        let final_addr = base_addr.wrapping_add(offset as u16);

        TestOpcodeOptions::new(OpCode::CmpIndirectY, 5, verify(a, b))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
            })
            .with_arguments(&[ptr])
            .with_additional_values(&[
                (ptr as u16, base_addr.to_le_bytes()[0]),
                (ptr.wrapping_add(1) as u16, base_addr.to_le_bytes()[1]),
                (final_addr, b),
            ])
            .test();
    }
}
