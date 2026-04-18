use crate::bus::NESBus;
use crate::cpu::Cpu;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;


// PHA
#[test]
fn test_pha_pushes_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x42, PHA_IMPLIED, BRK]));
    // LDA #$42, PHA -> stack should contain 0x42
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.bus.read(0x01FD), 0x42);
    assert_eq!(cpu.stack_pointer, 0xFC);
}

// PLA
#[test]
fn test_pla_pulls_to_accumulator() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x42, PHA_IMPLIED, LDA_IMMEDIATE, 0x00, PLA_IMPLIED, BRK]));
    // LDA #$42, PHA, LDA #$00, PLA -> A=0x42
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x42);
    assert_eq!(cpu.stack_pointer, 0xFD);
}

#[test]
fn test_pla_sets_zero_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, PHA_IMPLIED, LDA_IMMEDIATE, 0x01, PLA_IMPLIED, BRK]));
    // LDA #$00, PHA, LDA #$01, PLA -> A=0x00, Zero=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_pla_sets_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x80, PHA_IMPLIED, LDA_IMMEDIATE, 0x00, PLA_IMPLIED, BRK]));
    // LDA #$80, PHA, LDA #$00, PLA -> A=0x80, Negative=1
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

// PHP
#[test]
fn test_php_pushes_status_with_break_and_unused() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, PHP_IMPLIED, BRK]));
    // SEC -> Carry=1, PHP
    cpu.run_until_break().unwrap();
    let pushed = cpu.bus.read(0x01FD);
    assert_eq!(pushed & 0b0000_0001, 0b01); // Carry
    assert_eq!(pushed & 0b0011_0000, 0b0011_0000); // Break + AlwaysSet set
}

// PLP
#[test]
fn test_plp_pulls_status() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, PHP_IMPLIED, CLC_IMPLIED, PLP_IMPLIED, BRK]));
    // SEC, PHP, CLC, PLP -> Carry should be restored
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry restored
}

#[test]
fn test_plp_clears_break_and_sets_unused() {
    let mut cpu = Cpu::new(NESBus::with_program(&[PHP_IMPLIED, PLP_IMPLIED, BRK]));
    // PHP pushes with Break+AlwaysSet set, PLP should clear Break and keep AlwaysSet
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.processor_status & 0b0001_0000, 0); // Break cleared
    assert_eq!(cpu.processor_status & 0b0010_0000, 0b0010_0000); // AlwaysSet set
}

// PHA + PLA multiple
#[test]
fn test_stack_multiple_push_pull() {
    let mut cpu = Cpu::new(NESBus::with_program(&[
        LDA_IMMEDIATE, 0x11,
        PHA_IMPLIED,
        LDA_IMMEDIATE, 0x22,
        PHA_IMPLIED,
        LDA_IMMEDIATE, 0x00,
        PLA_IMPLIED,
        BRK,
    ]));
    // Push 0x11, 0x22, then pull -> should get 0x22 first (LIFO)
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_a, 0x22);
    assert_eq!(cpu.stack_pointer, 0xFC);
}

// RTI
#[test]
fn test_rti_restores_status_and_pc() {
    let mut cpu = Cpu::new(NESBus::with_program(&[
        LDX_IMMEDIATE, 0xFA,   // $8000
        TXS_IMPLIED,            // $8002: SP=0xFA
        RTI_IMPLIED,            // $8003: pop status, pop PC -> $8007
        NOP_IMPLIED,            // $8004
        NOP_IMPLIED,            // $8005
        NOP_IMPLIED,            // $8006
        INY_IMPLIED,            // $8007: landed here
        BRK,                    // $8008
    ]));
    cpu.bus.write(0x01FB, 0x01); // status: Carry=1
    cpu.bus.write(0x01FC, 0x07); // PC low -> $8007
    cpu.bus.write(0x01FD, 0x80); // PC high
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.register_y, 1); // reached $8007
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry restored
}

#[test]
fn test_rti_clears_break_sets_always_set() {
    let mut cpu = Cpu::new(NESBus::with_program(&[
        LDX_IMMEDIATE, 0xFA,   // $8000
        TXS_IMPLIED,            // $8002: SP=0xFA
        RTI_IMPLIED,            // $8003
        NOP_IMPLIED,            // $8004
        NOP_IMPLIED,            // $8005
        NOP_IMPLIED,            // $8006
        BRK,                    // $8007
    ]));
    // status with Break + AlwaysSet + Carry
    cpu.bus.write(0x01FB, 0b0011_0001);
    cpu.bus.write(0x01FC, 0x07); // PC low -> $8007
    cpu.bus.write(0x01FD, 0x80); // PC high
    cpu.run_until_break().unwrap();
    assert_eq!(cpu.processor_status & 0b0001_0000, 0); // Break cleared
    assert_eq!(cpu.processor_status & 0b0010_0000, 0b0010_0000); // AlwaysSet set
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry preserved
}
