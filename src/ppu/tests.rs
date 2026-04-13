use crate::Mirroring;
use crate::ppu::Ppu;
use crate::ppu::register::controller::ControllerRegister;
use crate::ppu::{CYCLES_PER_SCANLINE, REG_CONTROLLER, SCANLINES_PER_FRAME, VBLANK_SCANLINE};

fn new_ppu() -> Ppu {
    Ppu::new(vec![0; 2048], Mirroring::Horizontal)
}

// tick: スキャンライン進行
#[test]
fn test_tick_increments_scanline() {
    let mut ppu = new_ppu();
    ppu.tick(CYCLES_PER_SCANLINE as u16);
    assert_eq!(ppu.scanline(), 1);
}

#[test]
fn test_tick_does_not_increment_scanline_before_341_cycles() {
    let mut ppu = new_ppu();
    ppu.tick(340);
    assert_eq!(ppu.scanline(), 0);
}

#[test]
fn test_tick_carries_over_remaining_cycles() {
    let mut ppu = new_ppu();
    ppu.tick(CYCLES_PER_SCANLINE as u16 + 10);
    assert_eq!(ppu.scanline(), 1);
    assert_eq!(ppu.cycles(), 10);
}

// tick: VBLANK でのNMI発生
#[test]
fn test_tick_triggers_nmi_at_vblank_with_nmi_enabled() {
    let mut ppu = new_ppu();
    ppu.write_to_register(REG_CONTROLLER, ControllerRegister::GENERATE_NMI.bits());

    // スキャンライン241まで進める
    for _ in 0..VBLANK_SCANLINE {
        ppu.tick(CYCLES_PER_SCANLINE as u16);
    }
    assert!(ppu.nmi_interrupt());
}

#[test]
fn test_tick_does_not_trigger_nmi_at_vblank_with_nmi_disabled() {
    let mut ppu = new_ppu();
    // GENERATE_NMI は無効のまま

    for _ in 0..VBLANK_SCANLINE {
        ppu.tick(CYCLES_PER_SCANLINE as u16);
    }
    assert!(!ppu.nmi_interrupt());
}

// tick: フレーム完了
#[test]
fn test_tick_returns_true_on_frame_complete() {
    let mut ppu = new_ppu();

    for _ in 0..SCANLINES_PER_FRAME - 1 {
        assert!(!ppu.tick(CYCLES_PER_SCANLINE as u16));
    }
    assert!(ppu.tick(CYCLES_PER_SCANLINE as u16)); // 262本目でフレーム完了
}

#[test]
fn test_tick_resets_scanline_on_frame_complete() {
    let mut ppu = new_ppu();
    ppu.write_to_register(REG_CONTROLLER, ControllerRegister::GENERATE_NMI.bits());

    for _ in 0..SCANLINES_PER_FRAME {
        ppu.tick(CYCLES_PER_SCANLINE as u16);
    }
    assert_eq!(ppu.scanline(), 0);
    assert!(!ppu.nmi_interrupt()); // フレーム完了でNMIクリア
}

// エッジ検出: GENERATE_NMI 0→1 かつ VBLANK中
#[test]
fn test_edge_detection_nmi_enabled_during_vblank() {
    let mut ppu = new_ppu();

    // VBLANK状態にする (NMI無効のまま)
    for _ in 0..VBLANK_SCANLINE {
        ppu.tick(CYCLES_PER_SCANLINE as u16);
    }
    ppu.set_nmi_interrupt(false); // tickでのNMIをクリア

    // GENERATE_NMI を 0→1 に変更 → VBLANK中なのでNMI発生
    ppu.write_to_register(REG_CONTROLLER, ControllerRegister::GENERATE_NMI.bits());
    assert!(ppu.nmi_interrupt());
}

// エッジ検出: GENERATE_NMI 1→1 ではNMI発生しない
#[test]
fn test_edge_detection_nmi_already_enabled_no_trigger() {
    let mut ppu = new_ppu();
    ppu.write_to_register(REG_CONTROLLER, ControllerRegister::GENERATE_NMI.bits());

    // VBLANK状態にする
    for _ in 0..VBLANK_SCANLINE {
        ppu.tick(CYCLES_PER_SCANLINE as u16);
    }
    ppu.set_nmi_interrupt(false);

    // GENERATE_NMI は既に1 → 再度書き込んでもNMI発生しない
    ppu.write_to_register(REG_CONTROLLER, ControllerRegister::GENERATE_NMI.bits());
    assert!(!ppu.nmi_interrupt());
}

// エッジ検出: GENERATE_NMI 0→1 だがVBLANK外ではNMI発生しない
#[test]
fn test_edge_detection_nmi_enabled_outside_vblank() {
    let mut ppu = new_ppu();

    // スキャンライン10（VBLANK外）
    for _ in 0..10 {
        ppu.tick(CYCLES_PER_SCANLINE as u16);
    }

    ppu.write_to_register(REG_CONTROLLER, ControllerRegister::GENERATE_NMI.bits());
    assert!(!ppu.nmi_interrupt());
}
