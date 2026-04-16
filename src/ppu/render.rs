use super::Ppu;
use super::frame::Frame;
use super::palette::SYSTEM_PALETTE;

const ATTRIBUTE_TABLE_OFFSET: usize = 0x3C0;
const ATTRIBUTE_TABLE_COLS: usize = 8;
const TILES_PER_BLOCK: usize = 4;
const TILES_PER_AREA: usize = 2;
const PALETTE_SIZE: usize = 4;
const PALETTE_COLORS_START: usize = 1;

const SPRITE_PALETTE_START: usize = 0x11;
const OAM_ENTRY_SIZE: usize = 4;
const TILE_SIZE_BYTES: u16 = 16;
const TILE_WIDTH: usize = 8;
const NAMETABLE_TILE_COLS: usize = 32;

pub(crate) fn render(ppu: &Ppu, frame: &mut Frame) {
    render_background(ppu, frame);
    render_sprites(ppu, frame);
}

fn render_background(ppu: &Ppu, frame: &mut Frame) {
    let bank = ppu.controller_register.background_pattern_address();

    for i in 0..ATTRIBUTE_TABLE_OFFSET {
        let tile_number = ppu.vram[i] as u16;
        let tile_column = i % NAMETABLE_TILE_COLS;
        let tile_row = i / NAMETABLE_TILE_COLS;
        let tile_start = (bank + tile_number * TILE_SIZE_BYTES) as usize;
        let tile = &ppu.chr_rom[tile_start..=(tile_start + 15)];
        let palette = background_palette(ppu, tile_column, tile_row);

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + TILE_WIDTH];

            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper >>= 1;
                lower >>= 1;
                let rgb = match value {
                    0 => SYSTEM_PALETTE[ppu.palette_table[0] as usize],
                    1 => SYSTEM_PALETTE[palette[1] as usize],
                    2 => SYSTEM_PALETTE[palette[2] as usize],
                    3 => SYSTEM_PALETTE[palette[3] as usize],
                    _ => unreachable!(),
                };
                frame.set_pixel(tile_column * TILE_WIDTH + x, tile_row * TILE_WIDTH + y, rgb);
            }
        }
    }
}

fn render_sprites(ppu: &Ppu, frame: &mut Frame) {
    let bank = ppu.controller_register.sprite_pattern_address();

    for i in (0..ppu.oam_data.len()).step_by(OAM_ENTRY_SIZE).rev() {
        let tile_index = ppu.oam_data[i + 1] as u16;
        let tile_x = ppu.oam_data[i + 3] as usize;
        let tile_y = ppu.oam_data[i] as usize;

        let flip_vertical = ppu.oam_data[i + 2] >> 7 & 1 == 1;
        let flip_horizontal = ppu.oam_data[i + 2] >> 6 & 1 == 1;
        let palette_index = ppu.oam_data[i + 2] & 0b11;
        let palette = sprite_palette(ppu, palette_index);

        let tile_start = (bank + tile_index * TILE_SIZE_BYTES) as usize;
        let tile = &ppu.chr_rom[tile_start..=(tile_start + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + TILE_WIDTH];

            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper >>= 1;
                lower >>= 1;
                let rgb = match value {
                    0 => continue,
                    1 => SYSTEM_PALETTE[palette[1] as usize],
                    2 => SYSTEM_PALETTE[palette[2] as usize],
                    3 => SYSTEM_PALETTE[palette[3] as usize],
                    _ => unreachable!(),
                };
                match (flip_horizontal, flip_vertical) {
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                }
            }
        }
    }
}

fn background_palette(ppu: &Ppu, tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attribute_table_index =
        tile_row / TILES_PER_BLOCK * ATTRIBUTE_TABLE_COLS + tile_column / TILES_PER_BLOCK;
    let attribute_byte = ppu.vram[ATTRIBUTE_TABLE_OFFSET + attribute_table_index];

    let palette_index = match (
        tile_column % TILES_PER_BLOCK / TILES_PER_AREA,
        tile_row % TILES_PER_BLOCK / TILES_PER_AREA,
    ) {
        (0, 0) => attribute_byte & 0b11,
        (1, 0) => (attribute_byte >> 2) & 0b11,
        (0, 1) => (attribute_byte >> 4) & 0b11,
        (1, 1) => (attribute_byte >> 6) & 0b11,
        (_, _) => unreachable!(),
    };

    let palette_start = PALETTE_COLORS_START + (palette_index as usize) * PALETTE_SIZE;
    [
        ppu.palette_table[0],
        ppu.palette_table[palette_start],
        ppu.palette_table[palette_start + 1],
        ppu.palette_table[palette_start + 2],
    ]
}

fn sprite_palette(ppu: &Ppu, palette_index: u8) -> [u8; 4] {
    let start = SPRITE_PALETTE_START + (palette_index as usize) * PALETTE_SIZE;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}
