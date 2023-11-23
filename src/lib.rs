#![allow(dead_code)]

use bitflags::bitflags;

const MEM_SIZE: usize = 0x10000;
const STACK_OFFSET: usize = 0x0100;
const RESET_VECTOR: usize = 0xFFFC;

bitflags! {
    pub struct StatusFlags: u8 {
        const N = 1 << 7;   // Negative
        const V = 1 << 6;   // Overflow
        const E = 1 << 5;   // Extension (unused, but initialized to 1)
        const B = 1 << 4;   // Break
        const D = 1 << 3;   // Decimal
        const I = 1 << 2;   // Interrupt Disable
        const Z = 1 << 1;   // Zero
        const C = 1 << 0;   // Carry
    }
}

pub struct Registers {
    pub pc: u16,        // Program counter
    pub s: u8,          // Stack pointer
    pub a: u8,          // Accumulator
    pub x: u8,          // X register
    pub y: u8,          // Y register
    pub p: StatusFlags  // Status
}

pub struct Cpu6502 {
    pub registers: Registers,
    pub ram: [u8; MEM_SIZE]
}

impl Cpu6502 {
    pub fn new() -> Self {
        Cpu6502 {
            registers: Registers {
                pc: 0,
                a: 0,
                x: 0,
                y: 0,
                s: 0,
                p: StatusFlags::empty()
            },

            ram: [0; MEM_SIZE]
        }
    }

    pub fn reset(&mut self) {
        // Set the PC to point to address stored in reset vector
        let lsb = self.ram[RESET_VECTOR];
        let msb = self.ram[RESET_VECTOR + 1];
        self.registers.pc = (msb as u16) << 8 | (lsb as u16);

        // Disable interrupts flag and extension bit should be set
        self.registers.p = StatusFlags::E | StatusFlags::I;
    }

    pub fn tick(&mut self) {
        // TODO: Implement giant switch over opcodes
    }
}