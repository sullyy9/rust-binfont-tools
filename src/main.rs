mod glyph;
mod font;

use glyph::Glyph;

fn main() {
    Glyph::<8, 8>::new(&[0b00000000, 
                         0b00000000,
                         0b00000000,
                         0b00000000,
                         0b00000000,
                         0b00000000,
                         0b00000000,
                         0b00000000]);
}
