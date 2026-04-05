use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

// LDA
#[test]
fn test_lda_immediate() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDA_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn test_lda_zero_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDA_IMMEDIATE, 0x00, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10);
    assert_eq!(cpu.register_a, 0x00);
}

#[test]
fn test_lda_negative_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDA_IMMEDIATE, 0x80, BRK]);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000);
}

#[test]
fn test_lda_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x55);
    cpu.run(&[LDA_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0200, 0x77);
    cpu.run(&[LDA_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_a, 0x77);
}

#[test]
fn test_lda_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x15, 0x66);
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDA_ZERO_PAGE_X, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x66);
}

#[test]
fn test_lda_absolute_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0205, 0x77);
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDA_ABSOLUTE_X, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_a, 0x77);
}

#[test]
fn test_lda_absolute_y() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0203, 0x88);
    cpu.run(&[LDY_IMMEDIATE, 0x03, LDA_ABSOLUTE_Y, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_a, 0x88);
}

#[test]
fn test_lda_indirect_x() {
    let mut cpu = Cpu::new(NESBus::new());
    // pointer at (0x05 + 0x10) = 0x15 -> 0x0300
    cpu.bus.write(0x15, 0x00);
    cpu.bus.write(0x16, 0x03);
    cpu.bus.write(0x0300, 0x99);
    cpu.run(&[LDX_IMMEDIATE, 0x10, LDA_INDIRECT_X, 0x05, BRK]);
    assert_eq!(cpu.register_a, 0x99);
}

#[test]
fn test_lda_indirect_y() {
    let mut cpu = Cpu::new(NESBus::new());
    // pointer at 0x20 -> 0x0300, + Y(0x05) = 0x0305
    cpu.bus.write(0x20, 0x00);
    cpu.bus.write(0x21, 0x03);
    cpu.bus.write(0x0305, 0xAA);
    cpu.run(&[LDY_IMMEDIATE, 0x05, LDA_INDIRECT_Y, 0x20, BRK]);
    assert_eq!(cpu.register_a, 0xAA);
}

// LDX
#[test]
fn test_ldx_immediate() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.register_x, 0x42);
}

#[test]
fn test_ldx_zero_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x00, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10);
    assert_eq!(cpu.register_x, 0x00);
}

#[test]
fn test_ldx_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x33);
    cpu.run(&[LDX_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_x, 0x33);
}

#[test]
fn test_ldx_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0300, 0x44);
    cpu.run(&[LDX_ABSOLUTE, 0x00, 0x03, BRK]);
    assert_eq!(cpu.register_x, 0x44);
}

#[test]
fn test_ldx_zero_page_y() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x15, 0x55);
    cpu.run(&[LDY_IMMEDIATE, 0x05, LDX_ZERO_PAGE_Y, 0x10, BRK]);
    assert_eq!(cpu.register_x, 0x55);
}

#[test]
fn test_ldx_absolute_y() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0205, 0x66);
    cpu.run(&[LDY_IMMEDIATE, 0x05, LDX_ABSOLUTE_Y, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_x, 0x66);
}

// LDY
#[test]
fn test_ldy_immediate() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.register_y, 0x42);
}

#[test]
fn test_ldy_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x33);
    cpu.run(&[LDY_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_y, 0x33);
}

#[test]
fn test_ldy_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0300, 0x44);
    cpu.run(&[LDY_ABSOLUTE, 0x00, 0x03, BRK]);
    assert_eq!(cpu.register_y, 0x44);
}

#[test]
fn test_ldy_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x15, 0x55);
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDY_ZERO_PAGE_X, 0x10, BRK]);
    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn test_ldy_absolute_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0205, 0x66);
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDY_ABSOLUTE_X, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_y, 0x66);
}

// STA
#[test]
fn test_sta_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDA_IMMEDIATE, 0x55, STA_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x55);
}

#[test]
fn test_sta_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDA_IMMEDIATE, 0x55, STA_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0200), 0x55);
}

#[test]
fn test_sta_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDA_IMMEDIATE, 0x55, STA_ZERO_PAGE_X, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x15), 0x55);
}

#[test]
fn test_sta_absolute_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDA_IMMEDIATE, 0x55, STA_ABSOLUTE_X, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0205), 0x55);
}

#[test]
fn test_sta_absolute_y() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x03, LDA_IMMEDIATE, 0x55, STA_ABSOLUTE_Y, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0203), 0x55);
}

#[test]
fn test_sta_indirect_x() {
    let mut cpu = Cpu::new(NESBus::new());
    // pointer at (0x05 + 0x10) = 0x15 -> 0x0300
    cpu.bus.write(0x15, 0x00);
    cpu.bus.write(0x16, 0x03);
    cpu.run(&[LDX_IMMEDIATE, 0x10, LDA_IMMEDIATE, 0x55, STA_INDIRECT_X, 0x05, BRK]);
    assert_eq!(cpu.bus.read(0x0300), 0x55);
}

#[test]
fn test_sta_indirect_y() {
    let mut cpu = Cpu::new(NESBus::new());
    // pointer at 0x20 -> 0x0300, + Y(0x05) = 0x0305
    cpu.bus.write(0x20, 0x00);
    cpu.bus.write(0x21, 0x03);
    cpu.run(&[LDY_IMMEDIATE, 0x05, LDA_IMMEDIATE, 0x55, STA_INDIRECT_Y, 0x20, BRK]);
    assert_eq!(cpu.bus.read(0x0305), 0x55);
}

// STX
#[test]
fn test_stx_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x33, STX_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x33);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x33, STX_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0200), 0x33);
}

#[test]
fn test_stx_zero_page_y() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x05, LDX_IMMEDIATE, 0x33, STX_ZERO_PAGE_Y, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x15), 0x33);
}

// STY
#[test]
fn test_sty_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x44, STY_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x44);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x44, STY_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0200), 0x44);
}

#[test]
fn test_sty_zero_page_x() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x05, LDY_IMMEDIATE, 0x44, STY_ZERO_PAGE_X, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x15), 0x44);
}
