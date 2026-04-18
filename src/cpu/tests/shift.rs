use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

// ASL Accumulator
#[test]
fn test_asl_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, ASL_ACCUMULATOR, BRK]));
    // A=0x01, ASL A -> A=0x02
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x02);
}

#[test]
fn test_asl_accumulator_sets_carry() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x80, ASL_ACCUMULATOR, BRK]));
    // A=0x80 (bit7=1), ASL A -> A=0x00, Carry=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_asl_accumulator_clears_carry() {
    let mut cpu = Cpu::new(NESBus::with_program(&[
        LDA_IMMEDIATE, 0x80,
        ASL_ACCUMULATOR,
        LDA_IMMEDIATE, 0x01,
        ASL_ACCUMULATOR,
        BRK,
    ]));
    // Set carry first via 0x80 shift, then shift 0x01 (bit7=0) -> Carry=0
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x02);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
}

#[test]
fn test_asl_accumulator_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x40, ASL_ACCUMULATOR, BRK]));
    // A=0x40 (0100_0000), ASL A -> A=0x80 (1000_0000) -> Negative=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// ASL ZeroPage
#[test]
fn test_asl_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ASL_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x05);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0x0A);
}

#[test]
fn test_asl_zero_page_carry() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ASL_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0xFF);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0xFE);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// ASL ZeroPage,X
#[test]
fn test_asl_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, ASL_ZERO_PAGE_X, 0x10, BRK]));
    cpu.bus.write(0x14, 0x03);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x14), 0x06);
}

// ASL Absolute
#[test]
fn test_asl_absolute() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ASL_ABSOLUTE, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0200, 0x05);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0200), 0x0A);
}

// ASL Absolute,X
#[test]
fn test_asl_absolute_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, ASL_ABSOLUTE_X, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0204, 0x05);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0204), 0x0A);
}

#[test]
fn test_asl_zero_result() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ASL_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x80);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

// LSR Accumulator
#[test]
fn test_lsr_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x04, LSR_ACCUMULATOR, BRK]));
    // A=0x04, LSR A -> A=0x02
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x02);
}

#[test]
fn test_lsr_accumulator_sets_carry() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, LSR_ACCUMULATOR, BRK]));
    // A=0x01 (bit0=1), LSR A -> A=0x00, Carry=1, Zero=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_lsr_accumulator_clears_carry() {
    let mut cpu = Cpu::new(NESBus::with_program(&[
        LDA_IMMEDIATE, 0x01,
        LSR_ACCUMULATOR,
        LDA_IMMEDIATE, 0x04,
        LSR_ACCUMULATOR,
        BRK,
    ]));
    // Set carry first via 0x01 shift, then shift 0x04 (bit0=0) -> Carry=0
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x02);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
}

#[test]
fn test_lsr_accumulator_clears_negative() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x80, LSR_ACCUMULATOR, BRK]));
    // A=0x80 (1000_0000), LSR A -> A=0x40 (0100_0000) -> Negative=0
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x40);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0); // no Negative
}

// LSR ZeroPage
#[test]
fn test_lsr_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LSR_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x0A);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0x05);
}

#[test]
fn test_lsr_zero_page_carry() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LSR_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0xFF);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0x7F);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b1000_0000, 0); // no Negative
}

// LSR ZeroPage,X
#[test]
fn test_lsr_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, LSR_ZERO_PAGE_X, 0x10, BRK]));
    cpu.bus.write(0x14, 0x06);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x14), 0x03);
}

// LSR Absolute
#[test]
fn test_lsr_absolute() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LSR_ABSOLUTE, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0200, 0x0A);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0200), 0x05);
}

// LSR Absolute,X
#[test]
fn test_lsr_absolute_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, LSR_ABSOLUTE_X, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0204, 0x0A);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0204), 0x05);
}

// ROL Accumulator
#[test]
fn test_rol_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, ROL_ACCUMULATOR, BRK]));
    // Carry=0, A=0x01, ROL A -> A=0x02 (bit0=old_carry=0)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x02);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
}

#[test]
fn test_rol_accumulator_carry_in() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x01, ROL_ACCUMULATOR, BRK]));
    // SEC, A=0x01, ROL A -> A=0x03 (bit0=old_carry=1)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x03);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
}

#[test]
fn test_rol_accumulator_carry_out() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x80, ROL_ACCUMULATOR, BRK]));
    // Carry=0, A=0x80, ROL A -> A=0x00, Carry=1, Zero=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_rol_accumulator_carry_through() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x80, ROL_ACCUMULATOR, BRK]));
    // SEC, A=0x80, ROL A -> A=0x01, Carry=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
}

#[test]
fn test_rol_accumulator_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x40, ROL_ACCUMULATOR, BRK]));
    // Carry=0, A=0x40, ROL A -> A=0x80, Negative=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// ROL ZeroPage
#[test]
fn test_rol_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ROL_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0x55);
    // Carry=0, ROL $10 -> 0x55(0101_0101) << 1 = 0xAA(1010_1010)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0xAA);
}

// ROL ZeroPage,X
#[test]
fn test_rol_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, ROL_ZERO_PAGE_X, 0x10, BRK]));
    cpu.bus.write(0x14, 0x03);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x14), 0x06);
}

// ROL Absolute
#[test]
fn test_rol_absolute() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ROL_ABSOLUTE, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0200, 0x05);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0200), 0x0A);
}

// ROL Absolute,X
#[test]
fn test_rol_absolute_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, ROL_ABSOLUTE_X, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0204, 0x05);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0204), 0x0A);
}

// ROR Accumulator
#[test]
fn test_ror_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x04, ROR_ACCUMULATOR, BRK]));
    // Carry=0, A=0x04, ROR A -> A=0x02 (bit7=old_carry=0)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x02);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
}

#[test]
fn test_ror_accumulator_carry_in() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x04, ROR_ACCUMULATOR, BRK]));
    // SEC, A=0x04, ROR A -> A=0x82 (bit7=old_carry=1)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x82);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0); // no Carry
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_ror_accumulator_carry_out() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, ROR_ACCUMULATOR, BRK]));
    // Carry=0, A=0x01, ROR A -> A=0x00, Carry=1, Zero=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_ror_accumulator_carry_through() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, LDA_IMMEDIATE, 0x01, ROR_ACCUMULATOR, BRK]));
    // SEC, A=0x01, ROR A -> A=0x80, Carry=1, Negative=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// ROR ZeroPage
#[test]
fn test_ror_zero_page() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ROR_ZERO_PAGE, 0x10, BRK]));
    cpu.bus.write(0x10, 0xAA);
    // Carry=0, ROR $10 -> 0xAA(1010_1010) >> 1 = 0x55(0101_0101)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x10), 0x55);
}

// ROR ZeroPage,X
#[test]
fn test_ror_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, ROR_ZERO_PAGE_X, 0x10, BRK]));
    cpu.bus.write(0x14, 0x06);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x14), 0x03);
}

// ROR Absolute
#[test]
fn test_ror_absolute() {
    let mut cpu = Cpu::new(NESBus::with_program(&[ROR_ABSOLUTE, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0200, 0x0A);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0200), 0x05);
}

// ROR Absolute,X
#[test]
fn test_ror_absolute_x() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x04, ROR_ABSOLUTE_X, 0x00, 0x02, BRK]));
    cpu.bus.write(0x0204, 0x0A);
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x0204), 0x05);
}
