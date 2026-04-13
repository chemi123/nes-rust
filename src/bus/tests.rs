use super::*;
use crate::cpu::bus_access::Bus;
use crate::ppu::controller_register::ControllerRegister;

#[test]
fn test_poll_nmi_status_returns_true_and_clears() {
    let mut bus = NESBus::with_program(&[0x00]);
    // Bus経由でNMI有効化 (0x2000 = PPU_CTRL)
    bus.write(0x2000, ControllerRegister::GENERATE_NMI.bits());

    // VBlankに到達させる (scanline 241)
    // 1 CPUサイクル = 3 PPUサイクル、1スキャンライン = 341 PPUサイクル
    // 241スキャンライン × 341 PPUサイクル = 82,181 PPUサイクル
    // 82,181 / 3 = 27,393.67 → 27,394 CPUサイクルでscanline 241に到達
    //
    // ただしフレーム完了(scanline 262)まで進めるとNMIがクリアされるので、
    // VBlank開始直後で止める必要がある。
    // 1回のtickで1スキャンライン分(341 PPUサイクル = 113.67 CPUサイクル)ずつ進める。
    // 114 CPUサイクル = 342 PPUサイクル > 341 なのでスキャンラインが1つ進む。
    for _ in 0..241 {
        bus.tick(114);
    }

    // 1回目: trueを返してクリア
    assert!(bus.poll_nmi_status());
    // 2回目: クリア済みなのでfalse
    assert!(!bus.poll_nmi_status());
}
