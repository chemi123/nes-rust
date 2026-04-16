const STACK_BASE: u16 = 0x0100;

use super::Cpu;

pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    // 本来クロック同期はBus（メモリアクセス）の責務ではないが、
    // 実機では並行動作するCPUとPPUをシングルスレッドで逐次エミュレーションするため、
    // CPUからアクセス可能な唯一の経路であるBus経由でPPUを進める必要がある。
    // 戻り値: PPUがフレーム描画を完了した場合 true
    fn tick(&mut self, cycles: u8) -> bool;
    fn poll_nmi_status(&mut self) -> bool;
}

impl<B: Bus> Cpu<B> {
    pub(super) fn fetch_byte(&mut self) -> u8 {
        let data = self.bus.read(self.program_counter);
        self.program_counter += 1;
        data
    }

    pub(super) fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch_byte() as u16;
        let hi = self.fetch_byte() as u16;
        (hi << 8) | lo
    }

    pub(super) fn peek_byte(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    pub(super) fn peek_word(&mut self, position: u16) -> u16 {
        let low = self.peek_byte(position) as u16;
        let high = self.peek_byte(position + 1) as u16;
        (high << 8) | low
    }

    pub(super) fn write_byte(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    pub(super) fn write_word(&mut self, position: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.write_byte(position, low);
        self.write_byte(position + 1, high);
    }

    pub(super) fn push_byte(&mut self, data: u8) {
        let addr = STACK_BASE + self.stack_pointer as u16;
        self.bus.write(addr, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub(super) fn pop_byte(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = STACK_BASE + self.stack_pointer as u16;
        self.peek_byte(addr)
    }

    pub(super) fn push_word(&mut self, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.push_byte(high);
        self.push_byte(low);
    }

    pub(super) fn pop_word(&mut self) -> u16 {
        let low = self.pop_byte() as u16;
        let high = self.pop_byte() as u16;
        (high << 8) | low
    }
}
