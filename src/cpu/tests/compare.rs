use crate::cpu::Cpu;
use crate::cpu::opcodes::*;

// CMP
#[test]
fn test_cmp_equal() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x42, CMP_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry (A >= M)
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero (A == M)
    assert_eq!(cpu.register_a, 0x42); // A unchanged
}

#[test]
fn test_cmp_greater() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x50, CMP_IMMEDIATE, 0x10, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry (A >= M)
    assert_eq!(cpu.processor_status & 0b0000_0010, 0); // not Zero
}

#[test]
fn test_cmp_less() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x10, CMP_IMMEDIATE, 0x50, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry (A < M)
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_cmp_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x10, 0x42);
    cpu.run(&[LDA_IMMEDIATE, 0x42, CMP_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

// CPX
#[test]
fn test_cpx_equal() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDX_IMMEDIATE, 0x42, CPX_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
    assert_eq!(cpu.register_x, 0x42); // X unchanged
}

#[test]
fn test_cpx_less() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDX_IMMEDIATE, 0x10, CPX_IMMEDIATE, 0x50, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// CPY
#[test]
fn test_cpy_equal() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDY_IMMEDIATE, 0x42, CPY_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
    assert_eq!(cpu.register_y, 0x42); // Y unchanged
}

#[test]
fn test_cpy_less() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDY_IMMEDIATE, 0x10, CPY_IMMEDIATE, 0x50, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}
