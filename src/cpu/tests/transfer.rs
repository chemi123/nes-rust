use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::opcodes::*;

#[test]
fn test_tax() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xAA, TAX_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xaa);
}

#[test]
fn test_tay() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0xBB, TAY_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.register_y, 0xbb);
}

#[test]
fn test_txa() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0xCC, TXA_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xcc);
}

#[test]
fn test_tya() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDY_IMMEDIATE, 0xDD, TYA_IMPLIED, BRK]));
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xdd);
}

#[test]
fn test_tsx() {
    let mut cpu = Cpu::new(NESBus::with_program(&[TSX_IMPLIED, BRK]));
    // SP starts at 0xFD after reset, TSX -> X=0xFD
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xFD);
}

#[test]
fn test_tsx_sets_zero_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x00, TXS_IMPLIED, TSX_IMPLIED, BRK]));
    // Set SP to 0 via TXS, then TSX -> X=0, Zero=1
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero
}

#[test]
fn test_tsx_sets_negative_flag() {
    let mut cpu = Cpu::new(NESBus::with_program(&[TSX_IMPLIED, BRK]));
    // SP=0xFD after reset, TSX -> X=0xFD, Negative=1
    cpu.run().unwrap();
    assert_eq!(cpu.processor_status & 0b1000_0000, 0b1000_0000); // Negative
}

#[test]
fn test_txs() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x42, TXS_IMPLIED, BRK]));
    // LDX #$42, TXS -> SP=0x42
    cpu.run().unwrap();
    assert_eq!(cpu.stack_pointer, 0x42);
}

#[test]
fn test_txs_no_flags() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x00, TXS_IMPLIED, BRK]));
    // TXS should NOT affect flags. LDX #$00 sets Zero, then TXS, Zero should remain from LDX
    cpu.run().unwrap();
    assert_eq!(cpu.stack_pointer, 0x00);
    assert_eq!(cpu.processor_status & 0b0000_0010, 0b10); // Zero (from LDX, not TXS)
}
