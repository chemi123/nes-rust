use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

// AND
#[test]
fn test_and_immediate() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xFF, AND_IMMEDIATE, 0x0F, BRK]));
    // 0xFF & 0x0F = 0x0F
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x0F);
}

#[test]
fn test_and_zero_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xF0, AND_IMMEDIATE, 0x0F, BRK]));
    // 0xF0 & 0x0F = 0x00
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_and_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xFF, AND_IMMEDIATE, 0x80, BRK]));
    // 0xFF & 0x80 = 0x80
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_and_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xFF, AND_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x0F);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x0F);
}

// ORA
#[test]
fn test_ora_immediate() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xF0, ORA_IMMEDIATE, 0x0F, BRK]));
    // 0xF0 | 0x0F = 0xFF
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_ora_zero_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, ORA_IMMEDIATE, 0x00, BRK]));
    // 0x00 | 0x00 = 0x00
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_ora_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, ORA_IMMEDIATE, 0x80, BRK]));
    // 0x00 | 0x80 = 0x80
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_ora_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xF0, ORA_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x0F);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0xFF);
}

// EOR
#[test]
fn test_eor_immediate() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xFF, EOR_IMMEDIATE, 0x0F, BRK]));
    // 0xFF ^ 0x0F = 0xF0
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0xF0);
}

#[test]
fn test_eor_zero_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xFF, EOR_IMMEDIATE, 0xFF, BRK]));
    // 0xFF ^ 0xFF = 0x00
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_eor_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, EOR_IMMEDIATE, 0x80, BRK]));
    // 0x00 ^ 0x80 = 0x80
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_eor_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xFF, EOR_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x0F);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0xF0);
}
