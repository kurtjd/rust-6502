#![allow(dead_code)]

use std::path::PathBuf;
use rust_6502::*;
use serde::Deserialize;
use rstest::*;

#[derive(Deserialize)]
struct TestRam {
    address: u16,
    value: u8
}

#[derive(Deserialize)]
struct TestCycles {
    address: usize,
    value: u8,
    ctype: String
}

#[derive(Deserialize)]
struct TestState {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<TestRam>
}

#[derive(Deserialize)]
struct Test {
    name: String,
    #[serde(rename = "initial")]
    initial_state: TestState,
    #[serde(rename = "final")]
    final_state: TestState,
    cycles: Vec<TestCycles>
}

fn parse_test(path: &PathBuf) -> Vec<Test> {
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

fn opcode_test(path: &PathBuf) {
    let mut cpu = Cpu6502::new();
    let tests = parse_test(path);

    for t in &tests {
        // Set the initial state of the CPU
        cpu.clear_cycles();
        cpu.registers.pc = t.initial_state.pc;
        cpu.registers.s = t.initial_state.s;
        cpu.registers.a = t.initial_state.a;
        cpu.registers.x = t.initial_state.x;
        cpu.registers.y = t.initial_state.y;
        cpu.registers.p = StatusFlags::from_bits(t.initial_state.p).unwrap();

        // Set the initial state of RAM
        for m in &t.initial_state.ram {
            cpu.ram[m.address as usize] = m.value;
        }

        // Execute opcode
        cpu.tick();

        // Check the final state of the CPU
        assert_eq!(cpu.registers.pc, t.final_state.pc, "Test ({}): Incorrect program counter!", t.name);
        assert_eq!(cpu.registers.s, t.final_state.s, "Test ({}): Incorrect stack pointer!", t.name);
        assert_eq!(cpu.registers.a, t.final_state.a, "Test ({}): Incorrect accumulator!", t.name);
        assert_eq!(cpu.registers.x, t.final_state.x, "test ({}): Incorrect X register!", t.name);
        assert_eq!(cpu.registers.y, t.final_state.y, "Test ({}): Incorrect Y register!", t.name);
        assert_eq!(cpu.registers.p.bits(), t.final_state.p, "Test ({}): Incorrect status register!", t.name);

        // Check the final state of RAM
        for m in &t.final_state.ram {
            assert_eq!(cpu.ram[m.address as usize], m.value, "Test ({}): Incorrect RAM @ {}!", t.name, m.address);
        }

        // Check cycles
        assert_eq!(cpu.cycles.len(), t.cycles.len());
        for (cpu_cycle, test_cycle) in cpu.cycles.iter().zip(t.cycles.iter()) {
            assert_eq!(cpu_cycle.address, test_cycle.address, "{}", t.name);
            assert_eq!(cpu_cycle.value, test_cycle.value);
            assert_eq!(cpu_cycle.ctype, test_cycle.ctype);
        }
    }
}

#[rstest]
fn cpu_test(#[files("tests/test_cases/*.json")] path: PathBuf) {
    opcode_test(&path);
}