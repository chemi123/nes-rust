use super::Cpu;
use super::bus_access::Bus;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub(super) enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    Accumulator,
}

/*

少し混乱するので下記に認識を整理する

(1): Immediate
メモリ = [...,0xa9, 0x01...], PC = 0x8000, 0x8000から左記のメモリのように0xa9, 0x01...が来るとする。
このケースでは逆アセンブルすると下記のような命令になる
LDA #$01 -> PCのポインタ(index = 0x8001)をそのまま返す。なので0x8001をreturn

(2): ZeroPage
メモリ = [0x01, 0xa0, ...,0xa5, 0x01...], PC = 0x8000, 0x8000から左記のメモリのように0xa9, 0x01...が来るとする。
このケースでは逆アセンブルすると下記のような命令になる
LDA $01 -> PCのポインタ(index = 0x8001)が0x01を指している。なのでx0001をreturn

(3): Absolute
メモリ = [0x01, 0xa0, ...0xab, ...,0xad, 0x01, 0x02, ...], PC = 0x8000, 0x8000から左記のメモリのように0xad, 0x01, 0x02...が来るとする。また0xabのアドレスは0x0101だとする。
このケースでは逆アセンブルすると下記のような命令になる
LDA $0201 -> PCのポインタ(index = 0x8001)が0x01を指しており、その次のindexが0x02を指している。なので0x0201をreturn
*/

impl<B: Bus> Cpu<B> {
    pub(super) fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                let addr = self.program_counter;
                self.program_counter += 1;
                addr
            }
            AddressingMode::ZeroPage => self.fetch_byte() as u16,
            AddressingMode::ZeroPage_X => {
                let base = self.fetch_byte();
                base.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let base = self.fetch_byte();
                base.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute => self.fetch_word(),
            AddressingMode::Absolute_X => {
                let base = self.fetch_word();
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.fetch_word();
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.fetch_byte();
                let ptr = base.wrapping_add(self.register_x);
                let low = self.peek_byte(ptr as u16) as u16;
                let high = self.peek_byte(ptr.wrapping_add(1) as u16) as u16;
                (high << 8) | low
            }
            AddressingMode::Indirect_Y => {
                let base = self.fetch_byte();
                let low = self.peek_byte(base as u16) as u16;
                let high = self.peek_byte(base.wrapping_add(1) as u16) as u16;
                let deref_base = (high << 8) | low;
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Accumulator => {
                panic!("Accumulator mode never call get_operand_address.")
            }
        }
    }
}
