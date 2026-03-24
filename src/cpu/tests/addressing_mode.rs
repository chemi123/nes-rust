use crate::cpu::Cpu;
use crate::cpu::constants::*;

#[test]
fn test_indirect_x_zero_page_wrap() {
    let mut cpu = Cpu::new();
    // base=0xF0, X=0x1F -> ptr=0x0F (wraps within zero page)
    // mem[0x0F]=0x80, mem[0x10]=0x01 -> target addr = 0x0180
    // mem[0x0180]=0x42
    cpu.memory.write(0x0F, 0x80);
    cpu.memory.write(0x10, 0x01);
    cpu.memory.write(0x0180, 0x42);
    cpu.run(&[LDX_IMMEDIATE, 0x1F, LDA_IMMEDIATE, 0x00, ADC_INDIRECT_X, 0xF0, BRK]);
    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn test_indirect_x_pointer_at_0xff() {
    let mut cpu = Cpu::new();
    // base=0xFF, X=0x00 -> ptr=0xFF
    // low=mem[0xFF]=0x80, high=mem[0x00]=0x02 (wraps to zero page!)
    // target addr = 0x0280
    // mem[0x0280]=0x37
    cpu.memory.write(0xFF, 0x80);
    cpu.memory.write(0x00, 0x02);
    cpu.memory.write(0x0280, 0x37);
    cpu.run(&[LDX_IMMEDIATE, 0x00, LDA_IMMEDIATE, 0x00, ADC_INDIRECT_X, 0xFF, BRK]);
    assert_eq!(cpu.register_a, 0x37);
}

#[test]
fn test_indirect_y_pointer_at_0xff() {
    let mut cpu = Cpu::new();
    // base=0xFF
    // low=mem[0xFF]=0x80, high=mem[0x00]=0x02 (wraps to zero page!)
    // deref_base = 0x0280, Y=0x05 -> target addr = 0x0285
    // mem[0x0285]=0x99
    cpu.memory.write(0xFF, 0x80);
    cpu.memory.write(0x00, 0x02);
    cpu.memory.write(0x0285, 0x99);
    cpu.run(&[LDY_IMMEDIATE, 0x05, LDA_IMMEDIATE, 0x00, ADC_INDIRECT_Y, 0xFF, BRK]);
    assert_eq!(cpu.register_a, 0x99);
}
