mod addressing_mode;
pub mod bus_access;
mod flags;
mod instruction;
mod opcodes;
#[cfg(test)]
mod tests;

use crate::NesError;
use addressing_mode::AddressingMode;
use bus_access::Bus;
use flags::Flag;
use opcodes::*;

const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const STACK_POINTER_INIT: u8 = 0xFD;

// CPU サイクルに同期して進むクロック系の責務。
// 実機では CPU クロック境界でサンプルされる NMI ポーリングも同じ trait にまとめる。
pub trait Clock {
    // CPU サイクル分だけシステムを進める。
    // 戻り値: 直前の tick で NMI が立ち上がった場合 true (VBlank 開始)
    fn tick(&mut self, cycles: u8) -> bool;

    // NMI ペンディング状態を読み取り、同時にクリアする
    fn poll_nmi_status(&mut self) -> bool;
}

// CPU run ループの正常終了理由。fatal エラーは Err(NesError) で返る。
#[derive(Debug, PartialEq, Eq)]
pub enum ExitReason {
    // BRK 命令に到達
    Break,
    // on_frame コールバックが false を返した (ユーザ終了要求など)
    Halted,
}

pub struct Cpu<B: Bus> {
    pub(crate) register_a: u8,
    pub(crate) register_x: u8,
    pub(crate) register_y: u8,
    pub(crate) processor_status: u8,
    pub(crate) stack_pointer: u8,
    pub(crate) program_counter: u16,
    pub(crate) bus: B,
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: B) -> Self {
        let mut cpu = Cpu {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
            stack_pointer: 0,
            program_counter: 0,
            bus,
        };
        cpu.reset();
        cpu
    }

    fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.processor_status = 0;
        self.stack_pointer = STACK_POINTER_INIT;

        self.program_counter = self.peek_word(RESET_VECTOR);
    }
}

impl<B: Bus + Clock> Cpu<B> {
    // 毎フレーム (NMI エッジ) ごとに on_frame を呼ぶ。
    // on_frame が false を返すと ExitReason::Halted で終了する。
    pub fn run<F>(&mut self, mut on_frame: F) -> Result<ExitReason, NesError>
    where
        F: FnMut(&mut B) -> bool,
    {
        loop {
            if self.bus.poll_nmi_status() {
                self.interrupt_nmi();
            }

            // unknown opcode の PC を正しく報告するため fetch 前の値を保持
            let opcode_pc = self.program_counter;
            let opcode = self.fetch_byte();

            match opcode {
                // LDA
                LDA_IMMEDIATE => self.lda(AddressingMode::Immediate),
                LDA_ZERO_PAGE => self.lda(AddressingMode::ZeroPage),
                LDA_ZERO_PAGE_X => self.lda(AddressingMode::ZeroPage_X),
                LDA_ABSOLUTE => self.lda(AddressingMode::Absolute),
                LDA_ABSOLUTE_X => self.lda(AddressingMode::Absolute_X),
                LDA_ABSOLUTE_Y => self.lda(AddressingMode::Absolute_Y),
                LDA_INDIRECT_X => self.lda(AddressingMode::Indirect_X),
                LDA_INDIRECT_Y => self.lda(AddressingMode::Indirect_Y),

                // LDX
                LDX_IMMEDIATE => self.ldx(AddressingMode::Immediate),
                LDX_ZERO_PAGE => self.ldx(AddressingMode::ZeroPage),
                LDX_ZERO_PAGE_Y => self.ldx(AddressingMode::ZeroPage_Y),
                LDX_ABSOLUTE => self.ldx(AddressingMode::Absolute),
                LDX_ABSOLUTE_Y => self.ldx(AddressingMode::Absolute_Y),

                // LDY
                LDY_IMMEDIATE => self.ldy(AddressingMode::Immediate),
                LDY_ZERO_PAGE => self.ldy(AddressingMode::ZeroPage),
                LDY_ZERO_PAGE_X => self.ldy(AddressingMode::ZeroPage_X),
                LDY_ABSOLUTE => self.ldy(AddressingMode::Absolute),
                LDY_ABSOLUTE_X => self.ldy(AddressingMode::Absolute_X),

                // STA
                STA_ZERO_PAGE => self.sta(AddressingMode::ZeroPage),
                STA_ZERO_PAGE_X => self.sta(AddressingMode::ZeroPage_X),
                STA_ABSOLUTE => self.sta(AddressingMode::Absolute),
                STA_ABSOLUTE_X => self.sta(AddressingMode::Absolute_X),
                STA_ABSOLUTE_Y => self.sta(AddressingMode::Absolute_Y),
                STA_INDIRECT_X => self.sta(AddressingMode::Indirect_X),
                STA_INDIRECT_Y => self.sta(AddressingMode::Indirect_Y),

                // STX
                STX_ZERO_PAGE => self.stx(AddressingMode::ZeroPage),
                STX_ZERO_PAGE_Y => self.stx(AddressingMode::ZeroPage_Y),
                STX_ABSOLUTE => self.stx(AddressingMode::Absolute),

                // STY
                STY_ZERO_PAGE => self.sty(AddressingMode::ZeroPage),
                STY_ZERO_PAGE_X => self.sty(AddressingMode::ZeroPage_X),
                STY_ABSOLUTE => self.sty(AddressingMode::Absolute),

                // Transfer
                TAX_IMPLIED => self.tax(),
                TAY_IMPLIED => self.tay(),
                TXA_IMPLIED => self.txa(),
                TYA_IMPLIED => self.tya(),
                TSX_IMPLIED => self.tsx(),
                TXS_IMPLIED => self.txs(),

                // Jump
                JMP_ABSOLUTE => self.jmp_absolute(),
                JMP_INDIRECT => self.jmp_indirect(),
                JSR_ABSOLUTE => self.jsr(),
                RTS_IMPLIED => self.rts(),

                // NOP
                NOP_IMPLIED => self.nop(),

                // Increment/Decrement
                INC_ZERO_PAGE => self.inc(AddressingMode::ZeroPage),
                INC_ZERO_PAGE_X => self.inc(AddressingMode::ZeroPage_X),
                INC_ABSOLUTE => self.inc(AddressingMode::Absolute),
                INC_ABSOLUTE_X => self.inc(AddressingMode::Absolute_X),
                DEC_ZERO_PAGE => self.dec(AddressingMode::ZeroPage),
                DEC_ZERO_PAGE_X => self.dec(AddressingMode::ZeroPage_X),
                DEC_ABSOLUTE => self.dec(AddressingMode::Absolute),
                DEC_ABSOLUTE_X => self.dec(AddressingMode::Absolute_X),
                INX_IMPLIED => self.inx(),
                INY_IMPLIED => self.iny(),
                DEX_IMPLIED => self.dex(),
                DEY_IMPLIED => self.dey(),

                // Arithmetic
                ADC_IMMEDIATE => self.adc(AddressingMode::Immediate),
                ADC_ZERO_PAGE => self.adc(AddressingMode::ZeroPage),
                ADC_ZERO_PAGE_X => self.adc(AddressingMode::ZeroPage_X),
                ADC_ABSOLUTE => self.adc(AddressingMode::Absolute),
                ADC_ABSOLUTE_X => self.adc(AddressingMode::Absolute_X),
                ADC_ABSOLUTE_Y => self.adc(AddressingMode::Absolute_Y),
                ADC_INDIRECT_X => self.adc(AddressingMode::Indirect_X),
                ADC_INDIRECT_Y => self.adc(AddressingMode::Indirect_Y),

                SBC_IMMEDIATE => self.sbc(AddressingMode::Immediate),
                SBC_ZERO_PAGE => self.sbc(AddressingMode::ZeroPage),
                SBC_ZERO_PAGE_X => self.sbc(AddressingMode::ZeroPage_X),
                SBC_ABSOLUTE => self.sbc(AddressingMode::Absolute),
                SBC_ABSOLUTE_X => self.sbc(AddressingMode::Absolute_X),
                SBC_ABSOLUTE_Y => self.sbc(AddressingMode::Absolute_Y),
                SBC_INDIRECT_X => self.sbc(AddressingMode::Indirect_X),
                SBC_INDIRECT_Y => self.sbc(AddressingMode::Indirect_Y),

                // Logical
                AND_IMMEDIATE => self.and(AddressingMode::Immediate),
                AND_ZERO_PAGE => self.and(AddressingMode::ZeroPage),
                AND_ZERO_PAGE_X => self.and(AddressingMode::ZeroPage_X),
                AND_ABSOLUTE => self.and(AddressingMode::Absolute),
                AND_ABSOLUTE_X => self.and(AddressingMode::Absolute_X),
                AND_ABSOLUTE_Y => self.and(AddressingMode::Absolute_Y),
                AND_INDIRECT_X => self.and(AddressingMode::Indirect_X),
                AND_INDIRECT_Y => self.and(AddressingMode::Indirect_Y),

                ORA_IMMEDIATE => self.ora(AddressingMode::Immediate),
                ORA_ZERO_PAGE => self.ora(AddressingMode::ZeroPage),
                ORA_ZERO_PAGE_X => self.ora(AddressingMode::ZeroPage_X),
                ORA_ABSOLUTE => self.ora(AddressingMode::Absolute),
                ORA_ABSOLUTE_X => self.ora(AddressingMode::Absolute_X),
                ORA_ABSOLUTE_Y => self.ora(AddressingMode::Absolute_Y),
                ORA_INDIRECT_X => self.ora(AddressingMode::Indirect_X),
                ORA_INDIRECT_Y => self.ora(AddressingMode::Indirect_Y),

                EOR_IMMEDIATE => self.eor(AddressingMode::Immediate),
                EOR_ZERO_PAGE => self.eor(AddressingMode::ZeroPage),
                EOR_ZERO_PAGE_X => self.eor(AddressingMode::ZeroPage_X),
                EOR_ABSOLUTE => self.eor(AddressingMode::Absolute),
                EOR_ABSOLUTE_X => self.eor(AddressingMode::Absolute_X),
                EOR_ABSOLUTE_Y => self.eor(AddressingMode::Absolute_Y),
                EOR_INDIRECT_X => self.eor(AddressingMode::Indirect_X),
                EOR_INDIRECT_Y => self.eor(AddressingMode::Indirect_Y),

                // Compare
                CMP_IMMEDIATE => self.cmp(AddressingMode::Immediate),
                CMP_ZERO_PAGE => self.cmp(AddressingMode::ZeroPage),
                CMP_ZERO_PAGE_X => self.cmp(AddressingMode::ZeroPage_X),
                CMP_ABSOLUTE => self.cmp(AddressingMode::Absolute),
                CMP_ABSOLUTE_X => self.cmp(AddressingMode::Absolute_X),
                CMP_ABSOLUTE_Y => self.cmp(AddressingMode::Absolute_Y),
                CMP_INDIRECT_X => self.cmp(AddressingMode::Indirect_X),
                CMP_INDIRECT_Y => self.cmp(AddressingMode::Indirect_Y),

                CPX_IMMEDIATE => self.cpx(AddressingMode::Immediate),
                CPX_ZERO_PAGE => self.cpx(AddressingMode::ZeroPage),
                CPX_ABSOLUTE => self.cpx(AddressingMode::Absolute),

                CPY_IMMEDIATE => self.cpy(AddressingMode::Immediate),
                CPY_ZERO_PAGE => self.cpy(AddressingMode::ZeroPage),
                CPY_ABSOLUTE => self.cpy(AddressingMode::Absolute),

                // Shift
                ASL_ACCUMULATOR => self.asl(AddressingMode::Accumulator),
                ASL_ZERO_PAGE => self.asl(AddressingMode::ZeroPage),
                ASL_ZERO_PAGE_X => self.asl(AddressingMode::ZeroPage_X),
                ASL_ABSOLUTE => self.asl(AddressingMode::Absolute),
                ASL_ABSOLUTE_X => self.asl(AddressingMode::Absolute_X),

                LSR_ACCUMULATOR => self.lsr(AddressingMode::Accumulator),
                LSR_ZERO_PAGE => self.lsr(AddressingMode::ZeroPage),
                LSR_ZERO_PAGE_X => self.lsr(AddressingMode::ZeroPage_X),
                LSR_ABSOLUTE => self.lsr(AddressingMode::Absolute),
                LSR_ABSOLUTE_X => self.lsr(AddressingMode::Absolute_X),

                ROL_ACCUMULATOR => self.rol(AddressingMode::Accumulator),
                ROL_ZERO_PAGE => self.rol(AddressingMode::ZeroPage),
                ROL_ZERO_PAGE_X => self.rol(AddressingMode::ZeroPage_X),
                ROL_ABSOLUTE => self.rol(AddressingMode::Absolute),
                ROL_ABSOLUTE_X => self.rol(AddressingMode::Absolute_X),

                ROR_ACCUMULATOR => self.ror(AddressingMode::Accumulator),
                ROR_ZERO_PAGE => self.ror(AddressingMode::ZeroPage),
                ROR_ZERO_PAGE_X => self.ror(AddressingMode::ZeroPage_X),
                ROR_ABSOLUTE => self.ror(AddressingMode::Absolute),
                ROR_ABSOLUTE_X => self.ror(AddressingMode::Absolute_X),

                // Branch
                BCC_RELATIVE => self.bcc(),
                BCS_RELATIVE => self.bcs(),
                BEQ_RELATIVE => self.beq(),
                BNE_RELATIVE => self.bne(),
                BMI_RELATIVE => self.bmi(),
                BPL_RELATIVE => self.bpl(),
                BVC_RELATIVE => self.bvc(),
                BVS_RELATIVE => self.bvs(),

                // Stack
                PHA_IMPLIED => self.pha(),
                PHP_IMPLIED => self.php(),
                PLA_IMPLIED => self.pla(),
                PLP_IMPLIED => self.plp(),

                // Flag
                CLC_IMPLIED => self.clc(),
                SEC_IMPLIED => self.sec(),
                CLD_IMPLIED => self.cld(),
                SED_IMPLIED => self.sed(),
                CLI_IMPLIED => self.cli(),
                SEI_IMPLIED => self.sei(),
                CLV_IMPLIED => self.clv(),

                // BIT
                BIT_ZERO_PAGE => self.bit(AddressingMode::ZeroPage),
                BIT_ABSOLUTE => self.bit(AddressingMode::Absolute),

                // RTI
                RTI_IMPLIED => self.rti(),

                // System
                BRK => return Ok(ExitReason::Break),
                // 実機の JAM opcode 相当として扱い、CPU ループを抜ける
                _ => {
                    return Err(NesError::UnknownOpcode {
                        opcode,
                        pc: opcode_pc,
                    });
                }
            }

            // tick は NMI エッジ (VBlank 開始) を true で通知する。
            // このタイミングで host (画面描画・入力ポーリング) に制御を渡す。
            if self.bus.tick(opcodes::cycles(opcode)) && !on_frame(&mut self.bus) {
                return Ok(ExitReason::Halted);
            }
        }
    }

    // BRK で止まる用途向けのテスト/スクリプト用ヘルパ。
    pub fn run_until_break(&mut self) -> Result<(), NesError> {
        self.run(|_| true).map(|_| ())
    }

    pub(super) fn interrupt_nmi(&mut self) {
        // 現在のPCをstackに退避
        self.push_word(self.program_counter);

        // ステータスをスタックに退避（Break=0, AlwaysSet=1 でハードウェア割り込みを示す）
        let flags = (self.processor_status & !(Flag::Break as u8)) | Flag::AlwaysSet as u8;
        self.push_byte(flags);

        // 割り込み中の際割り込みを防止
        self.set_flag(Flag::InterruptDisable, true);

        self.bus.tick(2);

        // NMIベクタからハンドラアドレスを読んでジャンプ
        self.program_counter = self.peek_word(NMI_VECTOR);
    }
}
