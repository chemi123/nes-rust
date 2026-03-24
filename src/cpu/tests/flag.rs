use crate::cpu::Cpu;
use crate::cpu::constants::*;

#[test]
fn test_sec_clc() {
    let mut cpu = Cpu::new();
    cpu.run(&[SEC_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry set
    cpu.run(&[CLC_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // Carry clear
}

#[test]
fn test_sed_cld() {
    let mut cpu = Cpu::new();
    cpu.run(&[SED_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_1000, 0b1000); // Decimal set
    cpu.run(&[CLD_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_1000, 0); // Decimal clear
}

#[test]
fn test_sei_cli() {
    let mut cpu = Cpu::new();
    cpu.run(&[SEI_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0100, 0b100); // Interrupt set
    cpu.run(&[CLI_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0100, 0); // Interrupt clear
}

#[test]
fn test_clv() {
    let mut cpu = Cpu::new();
    // Trigger overflow: 0x50 + 0x50 = 0xA0
    cpu.run(&[LDA_IMMEDIATE, 0x50, ADC_IMMEDIATE, 0x50, CLV_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0100_0000, 0); // Overflow cleared
}
