use super::*;
use crate::cpu::bus_access::Bus;
use crate::ppu::controller_register::ControllerRegister;
use crate::ppu::{CYCLES_PER_SCANLINE, VBLANK_SCANLINE};

#[test]
fn test_poll_nmi_status_returns_true_and_clears() {
    let mut bus = NESBus::with_program(&[0x00]);
    bus.ppu.controller_register.update(ControllerRegister::GENERATE_NMI.bits());
    for _ in 0..VBLANK_SCANLINE {
        bus.ppu.tick(CYCLES_PER_SCANLINE as u16);
    }
    assert!(bus.ppu.nmi_interrupt);

    // 1回目: trueを返してクリア
    assert!(bus.poll_nmi_status());
    // 2回目: クリア済みなのでfalse
    assert!(!bus.poll_nmi_status());
}
