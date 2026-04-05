use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

#[test]
fn test_inx() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x05, INX_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0x06);
}

#[test]
fn test_inx_overflow() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0xff, INX_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10);
}

#[test]
fn test_iny() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x05, INY_IMPLIED, BRK]);
    assert_eq!(cpu.register_y, 0x06);
}

#[test]
fn test_iny_overflow() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0xff, INY_IMPLIED, BRK]);
    assert_eq!(cpu.register_y, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10);
}

#[test]
fn test_dex() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDX_IMMEDIATE, 0x05, DEX_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0x04);
}

#[test]
fn test_dex_underflow() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[DEX_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0xff);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000);
}

#[test]
fn test_dey() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[LDY_IMMEDIATE, 0x05, DEY_IMPLIED, BRK]);
    assert_eq!(cpu.register_y, 0x04);
}

#[test]
fn test_dey_underflow() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.run(&[DEY_IMPLIED, BRK]);
    assert_eq!(cpu.register_y, 0xff);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000);
}

// INC (memory)
#[test]
fn test_inc_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x05);
    cpu.run(&[INC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x06);
}

#[test]
fn test_inc_overflow() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0xFF);
    cpu.run(&[INC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_inc_negative_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x7F);
    cpu.run(&[INC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_inc_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0200, 0x05);
    cpu.run(&[INC_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0200), 0x06);
}

// DEC (memory)
#[test]
fn test_dec_zero_page() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x05);
    cpu.run(&[DEC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x04);
}

#[test]
fn test_dec_underflow() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x00);
    cpu.run(&[DEC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0xFF);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_dec_zero_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x10, 0x01);
    cpu.run(&[DEC_ZERO_PAGE, 0x10, BRK]);
    assert_eq!(cpu.bus.read(0x10), 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_dec_absolute() {
    let mut cpu = Cpu::new(NESBus::new());
    cpu.bus.write(0x0200, 0x05);
    cpu.run(&[DEC_ABSOLUTE, 0x00, 0x02, BRK]);
    assert_eq!(cpu.bus.read(0x0200), 0x04);
}
