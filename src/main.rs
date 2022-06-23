use rust_binfont_utils::Glyph;

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
