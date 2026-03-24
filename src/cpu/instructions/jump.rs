use crate::cpu::Cpu;

impl Cpu {
    pub(in crate::cpu) fn jmp_absolute(&mut self) {
        self.program_counter = self.fetch_word();
    }

    pub(in crate::cpu) fn jmp_indirect(&mut self) {
        let addr = self.fetch_word();
        // 6502 page boundary bug: if addr is $xxFF,
        // high byte is read from $xx00 instead of $(xx+1)00
        let low = self.peek_byte(addr) as u16;
        let high = if addr & 0x00FF == 0x00FF {
            self.peek_byte(addr & 0xFF00) as u16
        } else {
            self.peek_byte(addr + 1) as u16
        };
        self.program_counter = (high << 8) | low;
    }

    pub(in crate::cpu) fn jsr(&mut self) {
        let target = self.fetch_word();
        self.push_word(self.program_counter - 1);
        self.program_counter = target;
    }

    pub(in crate::cpu) fn rts(&mut self) {
        self.program_counter = self.pop_word() + 1;
    }
}
