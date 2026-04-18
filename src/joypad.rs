use bitflags::bitflags;

bitflags! {
    // NES ジョイパッド ボタンビット
    //
    // 7  bit  0
    // ---- ----
    // RLDU SsBA
    // |||| ||||
    // |||| |||+- A          ← シフトレジスタ読み出しの先頭
    // |||| ||+-- B
    // |||| |+--- Select
    // |||| +---- Start
    // |||+------ Up
    // ||+------- Down
    // |+-------- Left
    // +--------- Right      ← 最後
    #[derive(Clone, Copy, Debug)]
    pub struct JoypadButton: u8 {
        const BUTTON_A = 0b0000_0001;
        const BUTTON_B = 0b0000_0010;
        const SELECT   = 0b0000_0100;
        const START    = 0b0000_1000;
        const UP       = 0b0001_0000;
        const DOWN     = 0b0010_0000;
        const LEFT     = 0b0100_0000;
        const RIGHT    = 0b1000_0000;
    }
}

pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: JoypadButton,
}

impl Joypad {
    const BUTTON_COUNT: u8 = JoypadButton::all().bits().count_ones() as u8;

    pub fn new() -> Self {
        Joypad {
            strobe: false,
            button_index: 0,
            button_status: JoypadButton::from_bits_truncate(0),
        }
    }

    // CPU による 0x4016 への書き込み。bit 0 が strobe。
    // strobe ON の間は button_index を 0 に張り付かせる
    // (= シフトレジスタが現在状態でロードされ続ける挙動を模倣)。
    pub fn write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.button_index = 0;
        }
    }

    // CPU による 0x4016 の読み取り。
    // 返り値は bit 0 に「現在の button_index が指すボタンの押下状態」を載せた u8。
    //
    // 具体例: A と Select が押されている状態 (button_status = 0b0000_0101) で
    // button_index = 2 (Select を読む番) の場合:
    //
    //   (1 << 2)                    = 0b0000_0100        ← bit 2 を指すマスク
    //   button_status & マスク      = 0b0000_0100        ← bit 2 が立っている
    //   >> 2                        = 0b0000_0001 = 1    ← bit 0 まで下ろす
    //
    // AND だけだと取り出した値が「そのビット位置のまま」(= 4 など) になってしまい
    // CPU が期待する 0/1 にならないので、 >> button_index で bit 0 まで落とす。
    //
    // strobe OFF のときだけ読み取りごとに button_index を進める (= シフト動作)。
    // strobe ON 中は index が 0 に張り付いているので、何度読んでも A を返す。
    //
    // 8 個読み切った後 (button_index >= BUTTON_COUNT) は、実機と同じく常に 1 を返す。
    pub fn read(&mut self) -> u8 {
        if self.button_index >= Self::BUTTON_COUNT {
            return 1;
        }

        let response = (self.button_status.bits() & (1 << self.button_index)) >> self.button_index;
        if !self.strobe {
            self.button_index += 1;
        }

        response
    }

    pub fn set_button_pressed_status(&mut self, button: JoypadButton, pressed: bool) {
        self.button_status.set(button, pressed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strobe_mode_keeps_returning_button_a() {
        let mut joypad = Joypad::new();
        joypad.write(1); // strobe ON
        joypad.set_button_pressed_status(JoypadButton::BUTTON_A, true);

        // strobe ON 中は何度読んでも A の状態を返す
        for _ in 0..10 {
            assert_eq!(joypad.read(), 1);
        }
    }

    #[test]
    fn test_sequential_read_after_strobe_off() {
        let mut joypad = Joypad::new();
        joypad.set_button_pressed_status(JoypadButton::BUTTON_A, true);
        joypad.set_button_pressed_status(JoypadButton::SELECT, true);
        joypad.set_button_pressed_status(JoypadButton::RIGHT, true);

        // ラッチ: 1 → 0 でボタン状態をスナップショット
        joypad.write(1);
        joypad.write(0);

        assert_eq!(joypad.read(), 1); // A
        assert_eq!(joypad.read(), 0); // B
        assert_eq!(joypad.read(), 1); // Select
        assert_eq!(joypad.read(), 0); // Start
        assert_eq!(joypad.read(), 0); // Up
        assert_eq!(joypad.read(), 0); // Down
        assert_eq!(joypad.read(), 0); // Left
        assert_eq!(joypad.read(), 1); // Right
    }

    #[test]
    fn test_read_after_all_buttons_returns_one() {
        let mut joypad = Joypad::new();
        joypad.write(1);
        joypad.write(0);

        for _ in 0..8 {
            joypad.read();
        }
        // 9 回目以降は常に 1
        for _ in 0..4 {
            assert_eq!(joypad.read(), 1);
        }
    }

    #[test]
    fn test_strobe_on_resets_button_index() {
        let mut joypad = Joypad::new();
        joypad.set_button_pressed_status(JoypadButton::BUTTON_A, true);

        joypad.write(1);
        joypad.write(0);
        assert_eq!(joypad.read(), 1); // A
        assert_eq!(joypad.read(), 0); // B

        // 再ラッチで button_index が 0 に戻る
        joypad.write(1);
        joypad.write(0);
        assert_eq!(joypad.read(), 1); // また A から
    }

    #[test]
    fn test_button_release_reflected_after_relatch() {
        let mut joypad = Joypad::new();
        joypad.set_button_pressed_status(JoypadButton::BUTTON_A, true);

        joypad.write(1);
        joypad.write(0);
        assert_eq!(joypad.read(), 1); // A 押下中

        // 離したあと再ラッチすると反映される
        joypad.set_button_pressed_status(JoypadButton::BUTTON_A, false);
        joypad.write(1);
        joypad.write(0);
        assert_eq!(joypad.read(), 0); // A 離された
    }
}
