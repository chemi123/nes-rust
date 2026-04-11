// iNES Header Constants
const INES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A]; // "NES" + MS-DOS EOF
const INES_HEADER_SIZE: usize = 16;

// Offsets within the 16-byte header
const OFFSET_PRG_ROM_SIZE: usize = 4; // PRG ROM size in 16 KB units
const OFFSET_CHR_ROM_SIZE: usize = 5; // CHR ROM size in 8 KB units (0 means CHR RAM)
const OFFSET_FLAGS_6: usize = 6; // Mirroring, Battery, Trainer, Mapper Lo
const OFFSET_FLAGS_7: usize = 7; // Mapper Hi, NES 2.0, PlayChoice-10, VS Uni
const OFFSET_PRG_RAM_SIZE: usize = 8; // PRG RAM size in 8 KB units (rarely used in iNES 1.0)

// Bitmasks for Flags 6
const FLAG6_MIRRORING_BIT: u8 = 0b0000_0001; // Bit 0: 0=Horizontal, 1=Vertical
const FLAG6_HAS_BATTERY: u8 = 0b0000_0010; // Bit 1: Battery-backed PRG RAM
const FLAG6_HAS_TRAINER: u8 = 0b0000_0100; // Bit 2: 512-byte Trainer
const FLAG6_FOUR_SCREEN_BIT: u8 = 0b0000_1000; // Bit 3: Four-screen VRAM
const FLAG6_MAPPER_LOW_MASK: u8 = 0b1111_0000; // Bits 4-7: Mapper number (lower 4 bits)

// Bitmasks for Flags 7
const FLAG7_NES_2_0_MASK: u8 = 0b0000_1100; // Bits 2-3: If equal to 2, flags are NES 2.0
const FLAG7_MAPPER_HIGH: u8 = 0b1111_0000; // Bits 4-7: Upper 4 bits of mapper number

// Units
pub(crate) const PRG_ROM_PAGE_SIZE: usize = 16384; // 16 KiB
pub(crate) const CHR_ROM_PAGE_SIZE: usize = 8192; // 8 KiB
pub(crate) const TRAINER_SIZE: usize = 512;

use crate::Mirroring;

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    #[cfg(test)]
    pub fn with_program(program: &[u8]) -> Self {
        let mut prg_rom = vec![0; PRG_ROM_PAGE_SIZE * 2];
        prg_rom[..program.len()].copy_from_slice(program);
        // リセットベクタ(0xFFFC-0xFFFD)に0x8000を書き込む
        // PRG ROM内のオフセット: 0xFFFC - 0x8000 = 0x7FFC
        prg_rom[0x7FFC] = 0x00;
        prg_rom[0x7FFD] = 0x80;
        Rom {
            prg_rom,
            chr_rom: vec![],
            mapper: 0,
            screen_mirroring: Mirroring::Horizontal,
        }
    }

    pub fn new(raw: &[u8]) -> Result<Rom, String> {
        // "NES" + MS-DOS EOF
        // これによりiNESファイルであるかどうかのvalidationを行う
        if raw.len() < INES_HEADER_SIZE || !raw.starts_with(&INES_TAG) {
            return Err("File is not in iNES file format".to_string());
        }

        // iNES仕様: マッパー番号は2つのバイトに分割して格納されている。
        // Flags 6 の上位4bitをマッパーの下位4bitとして使用し、
        // Flags 7 の上位4bitをマッパーの上位4bitとして結合する。
        let mapper = (raw[OFFSET_FLAGS_7] & FLAG7_MAPPER_HIGH) | (raw[OFFSET_FLAGS_6] >> 4);

        // iNES仕様: Flags 7 の bit2-3 がフォーマットバージョンを表す。
        // 0: iNES 1.0, 2: NES 2.0
        let ines_ver = (raw[OFFSET_FLAGS_7] & FLAG7_NES_2_0_MASK) >> 2;
        if ines_ver == 2 {
            return Err("NES2.0 format is not supported".to_string());
        }

        let four_screen = raw[OFFSET_FLAGS_6] & FLAG6_FOUR_SCREEN_BIT != 0;
        let vertical_mirroring = raw[OFFSET_FLAGS_6] & FLAG6_MIRRORING_BIT != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[OFFSET_PRG_ROM_SIZE] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[OFFSET_CHR_ROM_SIZE] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = raw[OFFSET_FLAGS_6] & FLAG6_HAS_TRAINER != 0;
        let prg_rom_start = INES_HEADER_SIZE + if skip_trainer { TRAINER_SIZE } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        if raw.len() < chr_rom_start + chr_rom_size {
            return Err("ROM file is too short".to_string());
        }

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            screen_mirroring,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rom(prg_rom_size: u8, chr_rom_size: u8, mapper: u8, control1_flags: u8) -> Vec<u8> {
        let mut header = vec![
            0x4E,
            0x45,
            0x53,
            0x1A,
            prg_rom_size,
            chr_rom_size,
            (mapper & 0x0F) << 4 | control1_flags,
            mapper & 0xF0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        header.extend(vec![0u8; prg_rom_size as usize * PRG_ROM_PAGE_SIZE]);
        header.extend(vec![0u8; chr_rom_size as usize * CHR_ROM_PAGE_SIZE]);
        header
    }

    #[test]
    fn test_rom_new() {
        let rom_data = create_rom(2, 1, 0, 0);
        let rom = Rom::new(&rom_data).unwrap();

        assert_eq!(rom.prg_rom.len(), 2 * PRG_ROM_PAGE_SIZE);
        assert_eq!(rom.chr_rom.len(), CHR_ROM_PAGE_SIZE);
        assert_eq!(rom.mapper, 0);
        assert_eq!(rom.screen_mirroring, Mirroring::Horizontal);
    }

    #[test]
    fn test_rom_vertical_mirroring() {
        let rom_data = create_rom(1, 1, 0, 1);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.screen_mirroring, Mirroring::Vertical);
    }

    #[test]
    fn test_rom_four_screen() {
        let rom_data = create_rom(1, 1, 0, 0b1000);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.screen_mirroring, Mirroring::FourScreen);
    }

    #[test]
    fn test_rom_mapper() {
        let rom_data = create_rom(1, 1, 3, 0);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.mapper, 3);
    }

    #[test]
    fn test_rom_invalid_header() {
        let rom_data = vec![0u8; 16];
        assert!(Rom::new(&rom_data).is_err());
    }

    #[test]
    fn test_rom_nes2_not_supported() {
        let mut rom_data = create_rom(1, 1, 0, 0);
        rom_data[7] |= 0b0000_1000; // iNES 2.0 flag
        assert!(Rom::new(&rom_data).is_err());
    }

    #[test]
    fn test_rom_with_trainer() {
        let mut data = vec![
            0x4E,
            0x45,
            0x53,
            0x1A,
            1,
            1,
            0b0000_0100, // trainer flag
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        data.extend(vec![0xFF; 512]); // trainer
        data.extend(vec![0xAA; PRG_ROM_PAGE_SIZE]);
        data.extend(vec![0xBB; CHR_ROM_PAGE_SIZE]);

        let rom = Rom::new(&data).unwrap();
        assert_eq!(rom.prg_rom[0], 0xAA);
        assert_eq!(rom.chr_rom[0], 0xBB);
    }
}
