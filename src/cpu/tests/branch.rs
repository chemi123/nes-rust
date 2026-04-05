use crate::cpu::Cpu;
use crate::bus::NESBus;
use crate::cpu::opcodes::*;

// BCC - Branch if Carry Clear
#[test]
fn test_bcc_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BCC_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // Carry=0, BCC $02 -> skip 2 bytes (INX, INX), land on INY
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bcc_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, BCC_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // Carry=1, BCC should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BCS - Branch if Carry Set
#[test]
fn test_bcs_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[SEC_IMPLIED, BCS_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bcs_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BCS_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // Carry=0, BCS should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BEQ - Branch if Zero Set
#[test]
fn test_beq_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, BEQ_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // LDA #$00 -> Zero=1, BEQ $02 -> skip INX, INX
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_beq_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, BEQ_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // LDA #$01 -> Zero=0, BEQ should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BNE - Branch if Zero Clear
#[test]
fn test_bne_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, BNE_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // LDA #$01 -> Zero=0, BNE $02 -> skip INX, INX
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bne_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x00, BNE_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // LDA #$00 -> Zero=1, BNE should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BMI - Branch if Negative Set
#[test]
fn test_bmi_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x80, BMI_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // LDA #$80 -> Negative=1, BMI $02
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bmi_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, BMI_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // LDA #$01 -> Negative=0, BMI should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BPL - Branch if Negative Clear
#[test]
fn test_bpl_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x01, BPL_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // LDA #$01 -> Negative=0, BPL $02
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bpl_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x80, BPL_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // LDA #$80 -> Negative=1, BPL should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BVC - Branch if Overflow Clear
#[test]
fn test_bvc_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BVC_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // No overflow, BVC $02
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bvc_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x50, ADC_IMMEDIATE, 0x50, BVC_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // 0x50 + 0x50 = overflow, BVC should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// BVS - Branch if Overflow Set
#[test]
fn test_bvs_branch_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDA_IMMEDIATE, 0x50, ADC_IMMEDIATE, 0x50, BVS_RELATIVE, 0x02, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]));
    // 0x50 + 0x50 = overflow, BVS $02
    cpu.run();
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

#[test]
fn test_bvs_branch_not_taken() {
    let mut cpu = Cpu::new(NESBus::with_program(&[BVS_RELATIVE, 0x02, INX_IMPLIED, BRK]));
    // No overflow, BVS should not branch
    cpu.run();
    assert_eq!(cpu.register_x, 1);
}

// Backward branch (loop)
#[test]
fn test_bne_backward_branch() {
    let mut cpu = Cpu::new(NESBus::with_program(&[LDX_IMMEDIATE, 0x03, DEX_IMPLIED, BNE_RELATIVE, 0xFD, BRK]));
    // LDX #$03, DEX, BNE $FC(-4) -> loop until X=0
    // DEX=1byte, BNE=1byte, offset=1byte, total loop body=3bytes
    // BNE offset is from after BNE operand, so $FD (-3) to go back to DEX
    cpu.run();
    assert_eq!(cpu.register_x, 0);
}
