use rust_binfont_utils::BitGlyph;

fn main() {
    let glyph: BitGlyph<8, 8> = {
        let r1 = 0b00011000;
        let r2 = 0b01100110;
        let r3 = 0b01100110;
        let r4 = 0b01111110;
        let r5 = 0b01100110;
        let r6 = 0b01100110;
        let r7 = 0b00000000;
        let r8 = 0b11111111;
        BitGlyph::new(&[r1, r2, r3, r4, r5, r6, r7, r8]).expect("Failed to create glpyh\n")
    };

    print!("{}", glyph);
}
