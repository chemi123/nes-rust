use nes_rust::NesError;
use nes_rust::bus::NESBus;
use nes_rust::cartridge::Rom;
use nes_rust::cpu::Cpu;
use nes_rust::screen::Screen;

fn main() -> Result<(), NesError> {
    env_logger::init();

    let bytes = std::fs::read("roms/pacman.nes")?;
    let rom = Rom::new(&bytes).map_err(NesError::RomParse)?;

    let mut screen = Screen::new()?;
    let mut cpu = Cpu::new(NESBus::new(rom));

    cpu.run(|bus| {
        if let Err(error) = screen.update(bus.ppu()) {
            log::error!("screen update failed: {}", error);
            return false;
        }
        screen.poll_events(bus.joypad1_mut())
    })?;

    Ok(())
}
