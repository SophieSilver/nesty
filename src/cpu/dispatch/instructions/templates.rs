//! Template functions for implementing instructions
//!
//! Most instructions have commonalities, such as memory access patterns
//!
//! This module defines template instructions that implement the common functionality between instructions
//! by taking a closure as an argument that performs the actual work after the addressing is done

use std::ops::ControlFlow;

use crate::{
    cpu::{CpuState, StatusFlags},
    memory::MemoryMapping,
};

use super::fetch_from_pc;

pub fn read_immediate<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            let value = fetch_from_pc(cpu_state, memory);
            f(cpu_state, value);
            ControlFlow::Break(())
        }
        _ => unreachable!(),
    }
}

pub fn read_zeropage<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);

            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_zeropage_indexed<F, I>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    get_index: I,
    f: F,
) -> ControlFlow<()>
where
    F: FnOnce(&mut CpuState, u8),
    I: FnOnce(&CpuState) -> u8,
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            // dummy read coz every cycle is a read or a write
            let _ = memory.load(cpu_state.effective_address);
            cpu_state.effective_address += get_index(cpu_state) as u16;
            // upper byte is always 0, page overflow is ignored
            cpu_state.effective_address &= 0xFF;
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);

            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_absolute<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            cpu_state.effective_address |= (fetch_from_pc(cpu_state, memory) as u16) << 8;
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_absolute_indexed<F, I>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    get_index: I,
    f: F,
) -> ControlFlow<()>
where
    F: FnOnce(&mut CpuState, u8),
    I: FnOnce(&CpuState) -> u8,
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            let address_high_byte = fetch_from_pc(cpu_state, memory);

            let (address_low_byte, carry) =
                (cpu_state.effective_address as u8).overflowing_add(get_index(cpu_state));

            cpu_state.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
            // store information about the carry.
            // the 6502 will first read from the page-wrapped address, and then,
            // if the carry was set, will fix up the address and do a second read.
            // Most importantly, we can't use the CARRY flag, since that
            // is documented not to change, the real CPU uses the ALU output, we don't
            // simulate an ALU, so we use the IGNORED_FLAG flag to persist a single bit of information
            cpu_state.flags.set(StatusFlags::IGNORED_FLAG, carry);
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);

            if cpu_state.flags.contains(StatusFlags::IGNORED_FLAG) {
                // oops, blown through a page, fix up the effective address
                cpu_state.effective_address = cpu_state.effective_address.wrapping_add(1 << 8);
            } else {
                f(cpu_state, value);
                return ControlFlow::Break(());
            }
        }
        4 => {
            // do another read if the previous address was wrong
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}
