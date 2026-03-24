use crate::cpu::Cpu;
use crate::cpu::constants::*;

#[test]
fn test_nop_does_nothing() {
    let mut cpu = Cpu::new();
    // NOP should not change any state, INX after to verify execution continues
    cpu.run(&[NOP_IMPLIED, NOP_IMPLIED, INX_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 1);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_y, 0);
}

// RTI
#[test]
fn test_rti_restores_status_and_pc() {
    let mut cpu = Cpu::new();
    // Pre-write interrupt stack frame at $01FB-$01FD
    // RTI pops: status(SP+1), PC_low(SP+2), PC_high(SP+3)
    // Use TXS to set SP=0xFA so RTI reads from $01FB, $01FC, $01FD
    cpu.memory.write(0x01FB, 0x01); // status: Carry=1
    cpu.memory.write(0x01FC, 0x07); // PC low -> $8007
    cpu.memory.write(0x01FD, 0x80); // PC high
    cpu.run(&[
        LDX_IMMEDIATE, 0xFA,   // $8000
        TXS_IMPLIED,            // $8002: SP=0xFA
        RTI_IMPLIED,            // $8003: pop status, pop PC -> $8007
        NOP_IMPLIED,            // $8004
        NOP_IMPLIED,            // $8005
        NOP_IMPLIED,            // $8006
        INY_IMPLIED,            // $8007: landed here
        BRK,                    // $8008
    ]);
    assert_eq!(cpu.register_y, 1); // reached $8007
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry restored
}

#[test]
fn test_rti_clears_break_sets_unused() {
    let mut cpu = Cpu::new();
    // status with Break + Unused + Carry
    cpu.memory.write(0x01FB, 0b0011_0001);
    cpu.memory.write(0x01FC, 0x07); // PC low -> $8007
    cpu.memory.write(0x01FD, 0x80); // PC high
    cpu.run(&[
        LDX_IMMEDIATE, 0xFA,   // $8000
        TXS_IMPLIED,            // $8002: SP=0xFA
        RTI_IMPLIED,            // $8003
        NOP_IMPLIED,            // $8004
        NOP_IMPLIED,            // $8005
        NOP_IMPLIED,            // $8006
        BRK,                    // $8007
    ]);
    assert_eq!(cpu.processor_status & 0b0001_0000, 0); // Break cleared
    assert_eq!(cpu.processor_status & 0b0010_0000, 0b0010_0000); // Unused set
    assert_eq!(cpu.processor_status & 0b0000_0001, 0b01); // Carry preserved
}
