#![allow(dead_code)]

use rstest::*;
use rust_6502::*;
use serde::Deserialize;
use std::path::PathBuf;

use std::cell::RefCell;
use std::rc::Rc;

const ILLEGAL_OPCODES: [u8; 105] = [
    0x1A, 0x3A, 0x5A, 0x7A, 0xDA, 0xFA, 0x80, 0x82, 0x89, 0xC2, 0xE2, 0x04, 0x44, 0x64, 0x14, 0x34,
    0x54, 0x74, 0xD4, 0xF4, 0x0C, 0x1C, 0x3C, 0x5C, 0x7C, 0xDC, 0xFC, 0x4B, 0x0B, 0x2B, 0x8B, 0x6B,
    0xC7, 0xD7, 0xCF, 0xDF, 0xDB, 0xC3, 0xD3, 0xE7, 0xF7, 0xEF, 0xFF, 0xFB, 0xE3, 0xF3, 0xBB, 0xA7,
    0xB7, 0xAF, 0xBF, 0xA3, 0xB3, 0xAB, 0x27, 0x37, 0x2F, 0x3F, 0x3B, 0x23, 0x33, 0x67, 0x77, 0x6F,
    0x7F, 0x7B, 0x63, 0x73, 0x87, 0x97, 0x8F, 0x83, 0xCB, 0x9F, 0x93, 0x9E, 0x9C, 0x07, 0x17, 0x0F,
    0x1F, 0x1B, 0x03, 0x13, 0x47, 0x57, 0x4F, 0x5F, 0x5B, 0x43, 0x53, 0x9B, 0xEB, 0x02, 0x12, 0x22,
    0x32, 0x42, 0x52, 0x62, 0x72, 0x92, 0xB2, 0xD2, 0xF2,
];

const MEM_SIZE: usize = 0x10000;

#[derive(Deserialize)]
struct TestRam {
    address: u16,
    value: u8,
}

#[derive(Deserialize)]
struct TestCycles {
    address: usize,
    value: u8,
    ctype: String,
}

#[derive(Deserialize)]
struct TestState {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<TestRam>,
}

#[derive(Deserialize)]
struct Test {
    name: String,
    #[serde(rename = "initial")]
    initial_state: TestState,
    #[serde(rename = "final")]
    final_state: TestState,
    cycles: Vec<TestCycles>,
}

struct Cycle {
    address: usize,
    value: u8,
    ctype: String,
}

// Since the CPU requires external memory management, setup a simple memory manager
struct MemManager {
    ram: [u8; MEM_SIZE],
    cycles: Vec<Cycle>,
}

impl MemManager {
    pub fn new() -> Self {
        MemManager {
            ram: [0; MEM_SIZE],
            cycles: Vec::new(),
        }
    }

    pub fn mem_read(&mut self, address: usize) -> u8 {
        let value = self.ram[address];

        // So we can check read activity
        self.cycles.push(Cycle {
            address,
            value,
            ctype: "read".to_string(),
        });

        value
    }

    pub fn mem_write(&mut self, address: usize, value: u8) {
        self.ram[address] = value;

        // So we can check write activity
        self.cycles.push(Cycle {
            address,
            value,
            ctype: "write".to_string(),
        })
    }
}

fn parse_test(path: &PathBuf) -> Vec<Test> {
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

fn opcode_test(path: &PathBuf) {
    // The memory manager
    let mem_man = Rc::new(RefCell::new(MemManager::new()));

    // Create closures for memory manager's read/write methods
    let mem_read = |address: usize| -> u8 { mem_man.clone().borrow_mut().mem_read(address) };
    let mem_write = |address: usize, value: u8| {
        mem_man.clone().borrow_mut().mem_write(address, value);
    };

    // Pass memory management closures to CPU
    let mut cpu = Cpu6502::new(Box::new(mem_read), Box::new(mem_write));

    let tests = parse_test(path);

    for t in &tests {
        // Set the initial state of the CPU
        cpu.registers.pc = t.initial_state.pc;
        cpu.registers.s = t.initial_state.s;
        cpu.registers.a = t.initial_state.a;
        cpu.registers.x = t.initial_state.x;
        cpu.registers.y = t.initial_state.y;
        cpu.registers.p = StatusFlags::from_bits(t.initial_state.p).unwrap();

        // Set the initial state of RAM
        for m in &t.initial_state.ram {
            mem_man.borrow_mut().ram[m.address as usize] = m.value;
        }

        // Execute opcode
        let num_cycles = cpu.tick();
        let mut mem_man = mem_man.borrow_mut();

        // Check the final state of the CPU
        assert_eq!(
            cpu.registers.pc, t.final_state.pc,
            "Test ({}): Incorrect program counter!",
            t.name
        );
        assert_eq!(
            cpu.registers.s, t.final_state.s,
            "Test ({}): Incorrect stack pointer!",
            t.name
        );
        assert_eq!(
            cpu.registers.a, t.final_state.a,
            "Test ({}): Incorrect accumulator!",
            t.name
        );
        assert_eq!(
            cpu.registers.x, t.final_state.x,
            "test ({}): Incorrect X register!",
            t.name
        );
        assert_eq!(
            cpu.registers.y, t.final_state.y,
            "Test ({}): Incorrect Y register!",
            t.name
        );
        assert_eq!(
            cpu.registers.p.bits(),
            t.final_state.p,
            "Test ({}): Incorrect status register!",
            t.name
        );

        // Check the final state of RAM
        for m in &t.final_state.ram {
            assert_eq!(
                mem_man.ram[m.address as usize], m.value,
                "Test ({}): Incorrect RAM @ {}!",
                t.name, m.address
            );
        }

        // Don't have cycle accuracy for illegal opcodes yet, so don't test
        let opcode = &u8::from_str_radix(&t.name[0..2], 16).unwrap();
        if ILLEGAL_OPCODES.contains(opcode) {
            continue;
        }

        // Check cycles
        assert_eq!(num_cycles as usize, t.cycles.len());
        for (cpu_cycle, test_cycle) in mem_man.cycles.iter().zip(t.cycles.iter()) {
            assert_eq!(cpu_cycle.address, test_cycle.address, "{}", t.name);
            assert_eq!(cpu_cycle.value, test_cycle.value);
            assert_eq!(cpu_cycle.ctype, test_cycle.ctype);
        }
        mem_man.cycles.clear();
    }
}

#[rstest]
fn cpu_test(#[files("tests/test_cases/*.json")] path: PathBuf) {
    opcode_test(&path);
}
