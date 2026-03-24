use crate::cpu::Cpu;
use crate::cpu::constants::*;

// ADC
#[test]
fn test_adc_immediate() {
    let mut cpu = Cpu::new();
    // A=0x10, ADC #$20 -> A=0x30
    cpu.run(&[LDA_IMMEDIATE, 0x10, ADC_IMMEDIATE, 0x20, BRK]);
    assert_eq!(cpu.register_a, 0x30);
}

#[test]
fn test_adc_carry_flag() {
    let mut cpu = Cpu::new();
    // A=0xFF, ADC #$01 -> A=0x00, Carry=1
    cpu.run(&[LDA_IMMEDIATE, 0xFF, ADC_IMMEDIATE, 0x01, BRK]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_adc_carry_input() {
    let mut cpu = Cpu::new();
    // A=0xFF, ADC #$01 -> Carry=1, then A=0x00, ADC #$01 -> A=0x02 (with carry)
    cpu.run(&[
        LDA_IMMEDIATE, 0xFF,
        ADC_IMMEDIATE, 0x01,
        LDA_IMMEDIATE, 0x00,
        ADC_IMMEDIATE, 0x01,
        BRK,
    ]);
    assert_eq!(cpu.register_a, 0x02); // 0x00 + 0x01 + carry(1)
}

#[test]
fn test_adc_overflow_positive() {
    let mut cpu = Cpu::new();
    // 0x50 + 0x50 = 0xA0 (80 + 80 = 160, signed: -96) -> Overflow
    cpu.run(&[LDA_IMMEDIATE, 0x50, ADC_IMMEDIATE, 0x50, BRK]);
    assert_eq!(cpu.register_a, 0xA0);
    assert_eq!(cpu.processor_status & 0b0100_0000, 0b0100_0000); // Overflow
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_adc_overflow_negative() {
    let mut cpu = Cpu::new();
    // 0x80 + 0x80 = 0x100 -> A=0x00, Overflow=1, Carry=1
    // (-128) + (-128) = (-256), wraps to 0 -> overflow
    cpu.run(&[LDA_IMMEDIATE, 0x80, ADC_IMMEDIATE, 0x80, BRK]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0100_0000, 0b0100_0000); // Overflow
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b0000_0001); // Carry
}

#[test]
fn test_adc_no_overflow() {
    let mut cpu = Cpu::new();
    // 0x50 + 0x10 = 0x60 (positive + positive = positive) -> no overflow
    cpu.run(&[LDA_IMMEDIATE, 0x50, ADC_IMMEDIATE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x60);
    assert_eq!(cpu.processor_status & 0b0100_0000, 0); // no Overflow
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
}

#[test]
fn test_adc_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x10, 0x05);
    cpu.run(&[LDA_IMMEDIATE, 0x03, ADC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x08);
}

#[test]
fn test_adc_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x0200, 0x05);
    cpu.run(&[LDA_IMMEDIATE, 0x03, ADC_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_a, 0x08);
}

// SBC
#[test]
fn test_sbc_immediate() {
    let mut cpu = Cpu::new();
    // SEC, LDA 0x50, SBC 0x10 -> 0x50 - 0x10 = 0x40
    cpu.run(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x50, SBC_IMMEDIATE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x40);
}

#[test]
fn test_sbc_carry_clear() {
    let mut cpu = Cpu::new();
    // Carry starts at 0, so SBC borrows: 0x50 - 0x10 - 1 = 0x3F
    cpu.run(&[LDA_IMMEDIATE, 0x50, SBC_IMMEDIATE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x3F);
}

#[test]
fn test_sbc_underflow() {
    let mut cpu = Cpu::new();
    // Carry=0: 0x00 - 0x01 - 1 = -2 = 0xFE, Carry=0 (borrow)
    cpu.run(&[LDA_IMMEDIATE, 0x00, SBC_IMMEDIATE, 0x01, BRK]);
    assert_eq!(cpu.register_a, 0xFE);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry (borrow occurred)
}

#[test]
fn test_sbc_overflow() {
    let mut cpu = Cpu::new();
    // 0x50 - 0xB0 = 80 - (-80) = 160 -> overflow
    cpu.run(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x50, SBC_IMMEDIATE, 0xB0, BRK]);
    assert_eq!(cpu.register_a, 0xA0);
    assert_eq!(cpu.processor_status & 0b0100_0000, 0b0100_0000); // Overflow
}

#[test]
fn test_sbc_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x10, 0x05);
    // 0x08 - 0x05 = 0x03
    cpu.run(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x08, SBC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x03);
}

#[test]
fn test_sbc_with_sec() {
    let mut cpu = Cpu::new();
    // Now we can use SEC properly: 0x50 - 0x10 = 0x40
    cpu.run(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x50, SBC_IMMEDIATE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x40);
}
