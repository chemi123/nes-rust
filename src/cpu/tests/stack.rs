use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

// PHA
#[test]
fn test_pha_pushes_accumulator() {
    let mut cpu = Cpu::new(NESBus::new());
    // LDA #$42, PHA -> stack should contain 0x42
    cpu.run(&[LDA_IMMEDIATE, 0x42, PHA_IMPLIED, BRK]);
    assert_eq!(cpu.bus.read(0x01FD), 0x42);
    assert_eq!(cpu.stack_pointer, 0xFC);
}

// PLA
#[test]
fn test_pla_pulls_to_accumulator() {
    let mut cpu = Cpu::new(NESBus::new());
    // LDA #$42, PHA, LDA #$00, PLA -> A=0x42
    cpu.run(&[LDA_IMMEDIATE, 0x42, PHA_IMPLIED, LDA_IMMEDIATE, 0x00, PLA_IMPLIED, BRK]);
    assert_eq!(cpu.register_a, 0x42);
    assert_eq!(cpu.stack_pointer, 0xFD);
}

#[test]
fn test_pla_sets_zero_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    // LDA #$00, PHA, LDA #$01, PLA -> A=0x00, Zero=1
    cpu.run(&[LDA_IMMEDIATE, 0x00, PHA_IMPLIED, LDA_IMMEDIATE, 0x01, PLA_IMPLIED, BRK]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_pla_sets_negative_flag() {
    let mut cpu = Cpu::new(NESBus::new());
    // LDA #$80, PHA, LDA #$00, PLA -> A=0x80, Negative=1
    cpu.run(&[LDA_IMMEDIATE, 0x80, PHA_IMPLIED, LDA_IMMEDIATE, 0x00, PLA_IMPLIED, BRK]);
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// PHP
#[test]
fn test_php_pushes_status_with_break_and_unused() {
    let mut cpu = Cpu::new(NESBus::new());
    // SEC -> Carry=1, PHP
    cpu.run(&[SEC_IMPLIED, PHP_IMPLIED, BRK]);
    let pushed = cpu.bus.read(0x01FD);
    assert_eq!(pushed & 0b0000_0001, 0b01); // Carry
    assert_eq!(pushed & 0b0011_0000, 0b0011_0000); // Break + Unused set
}

// PLP
#[test]
fn test_plp_pulls_status() {
    let mut cpu = Cpu::new(NESBus::new());
    // SEC, PHP, CLC, PLP -> Carry should be restored
    cpu.run(&[SEC_IMPLIED, PHP_IMPLIED, CLC_IMPLIED, PLP_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry restored
}

#[test]
fn test_plp_clears_break_and_sets_unused() {
    let mut cpu = Cpu::new(NESBus::new());
    // PHP pushes with Break+Unused set, PLP should clear Break and keep Unused
    cpu.run(&[PHP_IMPLIED, PLP_IMPLIED, BRK]);
    assert_eq!(cpu.processor_status & 0b0001_0000, 0); // Break cleared
    assert_eq!(cpu.processor_status & 0b0010_0000, 0b0010_0000); // Unused set
}

// PHA + PLA multiple
#[test]
fn test_stack_multiple_push_pull() {
    let mut cpu = Cpu::new(NESBus::new());
    // Push 0x11, 0x22, then pull -> should get 0x22 first (LIFO)
    cpu.run(&[
        LDA_IMMEDIATE, 0x11,
        PHA_IMPLIED,
        LDA_IMMEDIATE, 0x22,
        PHA_IMPLIED,
        LDA_IMMEDIATE, 0x00,
        PLA_IMPLIED,
        BRK,
    ]);
    assert_eq!(cpu.register_a, 0x22);
    assert_eq!(cpu.stack_pointer, 0xFC);
}
