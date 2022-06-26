use bitvec::prelude::*;
use rust_binfont_utils::BitGlyph;

fn main() {
    let glyph: BitGlyph<8, 8> = {
        let glyph = bits![u8, Lsb0; 0, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 1, 1, 1, 1, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1];
        BitGlyph::new(glyph).expect("Failed to create glpyh\n")
    };

    print!("{}", glyph);
}
