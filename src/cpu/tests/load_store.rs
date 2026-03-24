use crate::cpu::Cpu;
use crate::cpu::constants::*;

// LDA
#[test]
fn test_lda_immediate() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn test_lda_zero_flag() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x00, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10);
    assert_eq!(cpu.register_a, 0x00);
}

#[test]
fn test_lda_negative_flag() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x80, BRK]);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000);
}

#[test]
fn test_lda_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x10, 0x55);
    cpu.run(&[LDA_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x0200, 0x77);
    cpu.run(&[LDA_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.register_a, 0x77);
}

// LDX
#[test]
fn test_ldx_immediate() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDX_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.register_x, 0x42);
}

#[test]
fn test_ldx_zero_flag() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDX_IMMEDIATE, 0x00, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10);
    assert_eq!(cpu.register_x, 0x00);
}

#[test]
fn test_ldx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x10, 0x33);
    cpu.run(&[LDX_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_x, 0x33);
}

#[test]
fn test_ldx_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x0300, 0x44);
    cpu.run(&[LDX_ABSOLUTE, 0x00, 0x03, BRK]);
    assert_eq!(cpu.register_x, 0x44);
}

// LDY
#[test]
fn test_ldy_immediate() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDY_IMMEDIATE, 0x42, BRK]);
    assert_eq!(cpu.register_y, 0x42);
}

#[test]
fn test_ldy_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x10, 0x33);
    cpu.run(&[LDY_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.register_y, 0x33);
}

#[test]
fn test_ldy_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory.write(0x0300, 0x44);
    cpu.run(&[LDY_ABSOLUTE, 0x00, 0x03, BRK]);
    assert_eq!(cpu.register_y, 0x44);
}

// STA
#[test]
fn test_sta_zero_page() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x55, STA_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.memory.read(0x10), 0x55);
}

#[test]
fn test_sta_absolute() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDA_IMMEDIATE, 0x55, STA_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.memory.read(0x0200), 0x55);
}

// STX
#[test]
fn test_stx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDX_IMMEDIATE, 0x33, STX_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.memory.read(0x10), 0x33);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDX_IMMEDIATE, 0x33, STX_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.memory.read(0x0200), 0x33);
}

// STY
#[test]
fn test_sty_zero_page() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDY_IMMEDIATE, 0x44, STY_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.memory.read(0x10), 0x44);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = Cpu::new();
    cpu.run(&[LDY_IMMEDIATE, 0x44, STY_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.memory.read(0x0200), 0x44);
}
