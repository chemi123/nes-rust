mod addressing_mode;
mod flags;
mod instructions;
mod bus_access;
mod opcodes;
#[cfg(test)]
mod tests;

use crate::memory::{CARTRIDGE_ROM_START, Memory, RESET_VECTOR, STACK_POINTER_INIT};
use addressing_mode::AddressingMode;
use opcodes::*;

pub(crate) struct Cpu {
    pub(crate) register_a: u8,
    pub(crate) register_x: u8,
    pub(crate) register_y: u8,
    pub(crate) processor_status: u8,
    pub(crate) stack_pointer: u8,
    pub(crate) program_counter: u16,
    pub(crate) memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
            stack_pointer: 0,
            program_counter: 0,
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self, program: &[u8]) {
        self.load_cartridge(program);
        self.run_with_callback(|_| {});
    }

    pub fn load(&mut self, addr: u16, program: &[u8]) {
        self.memory.load(addr, program);
        self.write_word(RESET_VECTOR, addr);
        self.reset();
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Cpu),
    {
        loop {
            let opcode = self.fetch_byte();
            callback(self);

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
                BRK => return,
                _ => todo!("opcode not implemented: {:02x}", opcode),
            }
        }
    }

    fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.processor_status = 0;
        self.stack_pointer = STACK_POINTER_INIT;

        self.program_counter = self.peek_word(RESET_VECTOR);
    }

    fn load_cartridge(&mut self, program: &[u8]) {
        self.load(CARTRIDGE_ROM_START, program);
    }
}
