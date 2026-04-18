use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::opcodes::*;

#[test]
fn test_sec_clc() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, CLC_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // Carry cleared after SEC then CLC
}

#[test]
fn test_sed_cld() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SED_IMPLIED, CLD_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.processor_status & 0b0000_1000, 0); // Decimal cleared after SED then CLD
}

#[test]
fn test_sei_cli() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEI_IMPLIED, CLI_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0); // Interrupt cleared after SEI then CLI
}

#[test]
fn test_clv() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x50, ADC_IMMEDIATE, 0x50, CLV_IMPLIED, BRK]));
    // Trigger overflow: 0x50 + 0x50 = 0xA0
    cpu.run().unwrap();
    assert_eq!(cpu.processor_status & 0b0100_0000, 0); // Overflow cleared
}
