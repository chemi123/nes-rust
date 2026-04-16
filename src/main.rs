use nes_rust::bus::NESBus;
use nes_rust::cartridge::Rom;
use nes_rust::cpu::Cpu;
use nes_rust::screen::Screen;

fn main() {
    let bytes = std::fs::read("roms/pacman.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let mut screen = Screen::new();

    let bus = NESBus::new(rom, move |ppu| {
        screen.update(ppu);
        screen.poll_events();
    });

    let mut cpu = Cpu::new(bus);
    cpu.run();
}
