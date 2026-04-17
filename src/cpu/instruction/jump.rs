use crate::cpu::Cpu;
use crate::cpu::bus_access::Bus;

const PAGE_BASE_MASK: u16 = 0xFF00;
const PAGE_OFFSET_MASK: u16 = 0x00FF;

impl<B: Bus> Cpu<B> {
    pub(in crate::cpu) fn jmp_absolute(&mut self) {
        self.program_counter = self.fetch_word();
    }

    pub(in crate::cpu) fn jmp_indirect(&mut self) {
        let address = self.fetch_word();
        // 6502 page boundary bug: high byte は同一ページ内から読まれるため、
        // インクリメントは low byte 内のみで wrap する ($xxFF + 1 → $xx00)
        let low = self.peek_byte(address) as u16;
        let high_address =
            (address & PAGE_BASE_MASK) | (address.wrapping_add(1) & PAGE_OFFSET_MASK);
        let high = self.peek_byte(high_address) as u16;
        self.program_counter = (high << 8) | low;
    }

    pub(in crate::cpu) fn jsr(&mut self) {
        let target = self.fetch_word();
        self.push_word(self.program_counter.wrapping_sub(1));
        self.program_counter = target;
    }

    pub(in crate::cpu) fn rts(&mut self) {
        self.program_counter = self.pop_word().wrapping_add(1);
    }
}
