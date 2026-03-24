// Load instructions
pub(super) const LDA_IMMEDIATE: u8 = 0xa9;
pub(super) const LDA_ZERO_PAGE: u8 = 0xa5;
pub(super) const LDA_ABSOLUTE: u8 = 0xad;

pub(super) const LDX_IMMEDIATE: u8 = 0xa2;
pub(super) const LDX_ZERO_PAGE: u8 = 0xa6;
pub(super) const LDX_ABSOLUTE: u8 = 0xae;

pub(super) const LDY_IMMEDIATE: u8 = 0xa0;
pub(super) const LDY_ZERO_PAGE: u8 = 0xa4;
pub(super) const LDY_ABSOLUTE: u8 = 0xac;

// Store instructions
pub(super) const STA_ZERO_PAGE: u8 = 0x85;
pub(super) const STA_ABSOLUTE: u8 = 0x8d;

pub(super) const STX_ZERO_PAGE: u8 = 0x86;
pub(super) const STX_ABSOLUTE: u8 = 0x8e;

pub(super) const STY_ZERO_PAGE: u8 = 0x84;
pub(super) const STY_ABSOLUTE: u8 = 0x8c;

// Transfer instructions
pub(super) const TAX_IMPLIED: u8 = 0xaa;
pub(super) const TAY_IMPLIED: u8 = 0xa8;
pub(super) const TXA_IMPLIED: u8 = 0x8a;
pub(super) const TYA_IMPLIED: u8 = 0x98;

// Increment/Decrement instructions
pub(super) const INC_ZERO_PAGE: u8 = 0xe6;
pub(super) const INC_ZERO_PAGE_X: u8 = 0xf6;
pub(super) const INC_ABSOLUTE: u8 = 0xee;
pub(super) const INC_ABSOLUTE_X: u8 = 0xfe;

pub(super) const DEC_ZERO_PAGE: u8 = 0xc6;
pub(super) const DEC_ZERO_PAGE_X: u8 = 0xd6;
pub(super) const DEC_ABSOLUTE: u8 = 0xce;
pub(super) const DEC_ABSOLUTE_X: u8 = 0xde;

pub(super) const INX_IMPLIED: u8 = 0xe8;
pub(super) const INY_IMPLIED: u8 = 0xc8;
pub(super) const DEX_IMPLIED: u8 = 0xca;
pub(super) const DEY_IMPLIED: u8 = 0x88;

// Arithmetic instructions
pub(super) const ADC_IMMEDIATE: u8 = 0x69;
pub(super) const ADC_ZERO_PAGE: u8 = 0x65;
pub(super) const ADC_ZERO_PAGE_X: u8 = 0x75;
pub(super) const ADC_ABSOLUTE: u8 = 0x6d;
pub(super) const ADC_ABSOLUTE_X: u8 = 0x7d;
pub(super) const ADC_ABSOLUTE_Y: u8 = 0x79;
pub(super) const ADC_INDIRECT_X: u8 = 0x61;
pub(super) const ADC_INDIRECT_Y: u8 = 0x71;

pub(super) const SBC_IMMEDIATE: u8 = 0xe9;
pub(super) const SBC_ZERO_PAGE: u8 = 0xe5;
pub(super) const SBC_ZERO_PAGE_X: u8 = 0xf5;
pub(super) const SBC_ABSOLUTE: u8 = 0xed;
pub(super) const SBC_ABSOLUTE_X: u8 = 0xfd;
pub(super) const SBC_ABSOLUTE_Y: u8 = 0xf9;
pub(super) const SBC_INDIRECT_X: u8 = 0xe1;
pub(super) const SBC_INDIRECT_Y: u8 = 0xf1;

// Logical instructions
pub(super) const AND_IMMEDIATE: u8 = 0x29;
pub(super) const AND_ZERO_PAGE: u8 = 0x25;
pub(super) const AND_ZERO_PAGE_X: u8 = 0x35;
pub(super) const AND_ABSOLUTE: u8 = 0x2d;
pub(super) const AND_ABSOLUTE_X: u8 = 0x3d;
pub(super) const AND_ABSOLUTE_Y: u8 = 0x39;
pub(super) const AND_INDIRECT_X: u8 = 0x21;
pub(super) const AND_INDIRECT_Y: u8 = 0x31;

pub(super) const ORA_IMMEDIATE: u8 = 0x09;
pub(super) const ORA_ZERO_PAGE: u8 = 0x05;
pub(super) const ORA_ZERO_PAGE_X: u8 = 0x15;
pub(super) const ORA_ABSOLUTE: u8 = 0x0d;
pub(super) const ORA_ABSOLUTE_X: u8 = 0x1d;
pub(super) const ORA_ABSOLUTE_Y: u8 = 0x19;
pub(super) const ORA_INDIRECT_X: u8 = 0x01;
pub(super) const ORA_INDIRECT_Y: u8 = 0x11;

pub(super) const EOR_IMMEDIATE: u8 = 0x49;
pub(super) const EOR_ZERO_PAGE: u8 = 0x45;
pub(super) const EOR_ZERO_PAGE_X: u8 = 0x55;
pub(super) const EOR_ABSOLUTE: u8 = 0x4d;
pub(super) const EOR_ABSOLUTE_X: u8 = 0x5d;
pub(super) const EOR_ABSOLUTE_Y: u8 = 0x59;
pub(super) const EOR_INDIRECT_X: u8 = 0x41;
pub(super) const EOR_INDIRECT_Y: u8 = 0x51;

// Compare instructions
pub(super) const CMP_IMMEDIATE: u8 = 0xc9;
pub(super) const CMP_ZERO_PAGE: u8 = 0xc5;
pub(super) const CMP_ZERO_PAGE_X: u8 = 0xd5;
pub(super) const CMP_ABSOLUTE: u8 = 0xcd;
pub(super) const CMP_ABSOLUTE_X: u8 = 0xdd;
pub(super) const CMP_ABSOLUTE_Y: u8 = 0xd9;
pub(super) const CMP_INDIRECT_X: u8 = 0xc1;
pub(super) const CMP_INDIRECT_Y: u8 = 0xd1;

pub(super) const CPX_IMMEDIATE: u8 = 0xe0;
pub(super) const CPX_ZERO_PAGE: u8 = 0xe4;
pub(super) const CPX_ABSOLUTE: u8 = 0xec;

pub(super) const CPY_IMMEDIATE: u8 = 0xc0;
pub(super) const CPY_ZERO_PAGE: u8 = 0xc4;
pub(super) const CPY_ABSOLUTE: u8 = 0xcc;

// Flag instructions
pub(super) const CLC_IMPLIED: u8 = 0x18;
pub(super) const SEC_IMPLIED: u8 = 0x38;
pub(super) const CLD_IMPLIED: u8 = 0xd8;
pub(super) const SED_IMPLIED: u8 = 0xf8;
pub(super) const CLI_IMPLIED: u8 = 0x58;
pub(super) const SEI_IMPLIED: u8 = 0x78;
pub(super) const CLV_IMPLIED: u8 = 0xb8;

// Shift instructions
pub(super) const ASL_ACCUMULATOR: u8 = 0x0a;
pub(super) const ASL_ZERO_PAGE: u8 = 0x06;
pub(super) const ASL_ZERO_PAGE_X: u8 = 0x16;
pub(super) const ASL_ABSOLUTE: u8 = 0x0e;
pub(super) const ASL_ABSOLUTE_X: u8 = 0x1e;

pub(super) const LSR_ACCUMULATOR: u8 = 0x4a;
pub(super) const LSR_ZERO_PAGE: u8 = 0x46;
pub(super) const LSR_ZERO_PAGE_X: u8 = 0x56;
pub(super) const LSR_ABSOLUTE: u8 = 0x4e;
pub(super) const LSR_ABSOLUTE_X: u8 = 0x5e;

pub(super) const ROL_ACCUMULATOR: u8 = 0x2a;
pub(super) const ROL_ZERO_PAGE: u8 = 0x26;
pub(super) const ROL_ZERO_PAGE_X: u8 = 0x36;
pub(super) const ROL_ABSOLUTE: u8 = 0x2e;
pub(super) const ROL_ABSOLUTE_X: u8 = 0x3e;

pub(super) const ROR_ACCUMULATOR: u8 = 0x6a;
pub(super) const ROR_ZERO_PAGE: u8 = 0x66;
pub(super) const ROR_ZERO_PAGE_X: u8 = 0x76;
pub(super) const ROR_ABSOLUTE: u8 = 0x6e;
pub(super) const ROR_ABSOLUTE_X: u8 = 0x7e;

// Branch instructions
pub(super) const BCC_RELATIVE: u8 = 0x90;
pub(super) const BCS_RELATIVE: u8 = 0xb0;
pub(super) const BEQ_RELATIVE: u8 = 0xf0;
pub(super) const BNE_RELATIVE: u8 = 0xd0;
pub(super) const BMI_RELATIVE: u8 = 0x30;
pub(super) const BPL_RELATIVE: u8 = 0x10;
pub(super) const BVC_RELATIVE: u8 = 0x50;
pub(super) const BVS_RELATIVE: u8 = 0x70;

// Transfer (stack)
pub(super) const TSX_IMPLIED: u8 = 0xba;
pub(super) const TXS_IMPLIED: u8 = 0x9a;

// Jump instructions
pub(super) const JMP_ABSOLUTE: u8 = 0x4c;
pub(super) const JMP_INDIRECT: u8 = 0x6c;
pub(super) const JSR_ABSOLUTE: u8 = 0x20;
pub(super) const RTS_IMPLIED: u8 = 0x60;

// NOP
pub(super) const NOP_IMPLIED: u8 = 0xea;

// Stack instructions
pub(super) const PHA_IMPLIED: u8 = 0x48;
pub(super) const PHP_IMPLIED: u8 = 0x08;
pub(super) const PLA_IMPLIED: u8 = 0x68;
pub(super) const PLP_IMPLIED: u8 = 0x28;

// BIT
pub(super) const BIT_ZERO_PAGE: u8 = 0x24;
pub(super) const BIT_ABSOLUTE: u8 = 0x2c;

// RTI
pub(super) const RTI_IMPLIED: u8 = 0x40;

// System instructions
pub(super) const BRK: u8 = 0x00;
