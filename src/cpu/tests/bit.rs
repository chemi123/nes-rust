use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

// BIT ZeroPage
#[test]
fn test_bit_zero_flag_set() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xF0, BIT_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x0F);
    // A=0xF0, BIT $10 -> A & 0x0F = 0x00, Zero=1
    cpu.run();
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_bit_zero_flag_clear() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, BIT_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0xFF);
    // A=0x01, BIT $10 -> A & 0xFF = 0x01, Zero=0
    cpu.run();
    assert_eq!(cpu.processor_status & 0b0000_0010, 0); // no Zero
}

#[test]
fn test_bit_negative_from_memory() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, BIT_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x80); // bit7=1
    // A=0x00, BIT $10 -> Negative = value bit7 = 1
    cpu.run();
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_bit_overflow_from_memory() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, BIT_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x40); // bit6=1
    // A=0x00, BIT $10 -> Overflow = value bit6 = 1
    cpu.run();
    assert_eq!(cpu.processor_status & 0b0100_0000, 0b0100_0000); // Overflow
}

#[test]
fn test_bit_does_not_modify_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x42, BIT_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0xFF);
    cpu.run();
    assert_eq!(cpu.register_a, 0x42); // unchanged
}

#[test]
fn test_bit_all_flags() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xC0, BIT_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0xC0); // bit7=1, bit6=1
    // A=0xC0, BIT $10 -> A & 0xC0 = 0xC0 (not zero), Negative=1, Overflow=1
    cpu.run();
    assert_eq!(cpu.processor_status & 0b0000_0010, 0); // no Zero
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
    assert_eq!(cpu.processor_status & 0b0100_0000, 0b0100_0000); // Overflow
}

// BIT Absolute
#[test]
fn test_bit_absolute() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, BIT_ABSOLUTE, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0200, 0xC0);
    cpu.run();
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
    assert_eq!(cpu.processor_status & 0b0100_0000, 0b0100_0000); // Overflow
}
