use crate::cpu::Cpu;
use crate::cpu::constants::*;

// JMP Absolute
#[test]
fn test_jmp_absolute() {
    let mut cpu = Cpu::new();
    // JMP $8005, skip INX INX, land on INY
    // program starts at $8000
    // $8000: JMP $8005 (3 bytes)
    // $8003: INX (1 byte)
    // $8004: INX (1 byte)
    // $8005: INY (1 byte)
    // $8006: BRK
    cpu.run(&[JMP_ABSOLUTE, 0x05, 0x80, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

// JMP Indirect
#[test]
fn test_jmp_indirect() {
    let mut cpu = Cpu::new();
    // Store jump target $8007 at $0200
    cpu.memory.write(0x0200, 0x07);
    cpu.memory.write(0x0201, 0x80);
    // $8000: JMP ($0200) (3 bytes) -> reads $8007 from $0200
    // $8003: INX
    // $8004: INX
    // $8005: INX
    // $8006: INX
    // $8007: INY
    // $8008: BRK
    cpu.run(&[JMP_INDIRECT, 0x00, 0x02, INX_IMPLIED, INX_IMPLIED, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

// JMP Indirect - page boundary bug
#[test]
fn test_jmp_indirect_page_boundary_bug() {
    let mut cpu = Cpu::new();
    // Pointer at $02FF: low byte from $02FF, high byte from $0200 (not $0300)
    cpu.memory.write(0x02FF, 0x07);
    cpu.memory.write(0x0200, 0x80); // bug: reads from $0200 instead of $0300
    cpu.memory.write(0x0300, 0x90); // this should NOT be used
    cpu.run(&[JMP_INDIRECT, 0xFF, 0x02, INX_IMPLIED, INX_IMPLIED, INX_IMPLIED, INX_IMPLIED, INY_IMPLIED, BRK]);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 1);
}

// JSR + RTS
#[test]
fn test_jsr_rts() {
    let mut cpu = Cpu::new();
    // $8000: JSR $8005 (3 bytes)
    // $8003: INX (return here after RTS)
    // $8004: BRK
    // $8005: INY (subroutine)
    // $8006: RTS
    cpu.run(&[JSR_ABSOLUTE, 0x05, 0x80, INX_IMPLIED, BRK, INY_IMPLIED, RTS_IMPLIED]);
    assert_eq!(cpu.register_y, 1); // subroutine executed
    assert_eq!(cpu.register_x, 1); // returned and continued
}

// JSR pushes correct return address
#[test]
fn test_jsr_pushes_return_address() {
    let mut cpu = Cpu::new();
    // $8000: JSR $8005 (3 bytes) -> push $8002 (PC-1 = last byte of JSR operand)
    // $8003: BRK
    // $8005: BRK
    cpu.run(&[JSR_ABSOLUTE, 0x05, 0x80, BRK, BRK, BRK]);
    // SP should have decreased by 2 (pushed 16-bit address)
    assert_eq!(cpu.stack_pointer, 0xFB);
}

// Nested JSR
#[test]
fn test_nested_jsr_rts() {
    let mut cpu = Cpu::new();
    // $8000: JSR $8006 (3 bytes)    -> call sub1
    // $8003: INX                     -> X=1 after return
    // $8004: INX                     -> X=2
    // $8005: BRK
    // $8006: JSR $800A (3 bytes)    -> sub1: call sub2
    // $8009: RTS                     -> sub1: return
    // $800A: INY                     -> sub2: Y=1
    // $800B: RTS                     -> sub2: return
    cpu.run(&[
        JSR_ABSOLUTE, 0x06, 0x80,
        INX_IMPLIED,
        INX_IMPLIED,
        BRK,
        JSR_ABSOLUTE, 0x0A, 0x80,
        RTS_IMPLIED,
        INY_IMPLIED,
        RTS_IMPLIED,
    ]);
    assert_eq!(cpu.register_y, 1); // sub2 executed
    assert_eq!(cpu.register_x, 2); // returned through both levels
}
