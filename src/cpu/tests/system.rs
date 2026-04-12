use crate::bus::NESBus;
use crate::cpu::Cpu;
use crate::cpu::bus_access::Bus;
use crate::cpu::opcodes::*;

#[test]
fn test_nop_does_nothing() {
    let mut cpu = Cpu::new(NESBus::with_program(&[NOP_IMPLIED, NOP_IMPLIED, INX_IMPLIED, BRK]));
    // NOP should not change any state, INX after to verify execution continues
    cpu.run();
    assert_eq!(cpu.register_x, 1);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_y, 0);
}

// interrupt_nmi
#[test]
fn test_interrupt_nmi_pushes_pc_and_status() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BRK]));
    cpu.reset();
    cpu.processor_status = 0b0100_0001; // OverFlow + Carry
    cpu.program_counter = 0x1234;

    let sp_before = cpu.stack_pointer;
    cpu.interrupt_nmi();

    // SPが3バイト分下がる (PC 2バイト + status 1バイト)
    assert_eq!(cpu.stack_pointer, sp_before.wrapping_sub(3));

    // スタックに退避されたPC
    let pushed_pc_hi = cpu.bus.read(0x0100 + sp_before as u16);
    let pushed_pc_lo = cpu.bus.read(0x0100 + sp_before.wrapping_sub(1) as u16);
    assert_eq!(pushed_pc_hi, 0x12);
    assert_eq!(pushed_pc_lo, 0x34);

    // スタックに退避されたstatus: Break=0, AlwaysSet=1
    let pushed_status = cpu.bus.read(0x0100 + sp_before.wrapping_sub(2) as u16);
    assert_eq!(pushed_status & 0b0001_0000, 0);          // Break cleared
    assert_eq!(pushed_status & 0b0010_0000, 0b0010_0000); // AlwaysSet
    assert_eq!(pushed_status & 0b0100_0001, 0b0100_0001); // OverFlow + Carry preserved
}

#[test]
fn test_interrupt_nmi_sets_interrupt_disable() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BRK]));
    cpu.reset();
    assert_eq!(cpu.processor_status & 0b0000_0100, 0);

    cpu.interrupt_nmi();

    assert_eq!(cpu.processor_status & 0b0000_0100, 0b0000_0100); // InterruptDisable set
}

#[test]
fn test_interrupt_nmi_jumps_to_nmi_vector() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BRK]));
    cpu.reset();

    // NMIベクタ (0xFFFA) の値を読んで、PCがその値になることを確認
    let nmi_addr = cpu.bus.read(0xFFFA) as u16 | (cpu.bus.read(0xFFFB) as u16) << 8;
    cpu.interrupt_nmi();

    assert_eq!(cpu.program_counter, nmi_addr);
}
