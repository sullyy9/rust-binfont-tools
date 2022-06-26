//! Basic glyph.
//!
use std::fmt::Display;

use bitvec::prelude::*;

/// Glyph in a basic bitmap format where each bit represents a pixel.
pub struct BitGlyph<const ROWS: usize, const COLS: usize>(BitBox<u8, Lsb0>);

impl<const R: usize, const C: usize> Default for BitGlyph<R, C> {
    /// Create a new Glyph with each pixel set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_binfont_utils::BitGlyph;
    /// let glyph: BitGlyph<16, 24> = BitGlyph::default();
    /// ```
    ///
    fn default() -> Self {
        Self(BitBox::from(BitVec::with_capacity(R * C)))
    }
}

impl<const R: usize, const C: usize> BitGlyph<R, C> {
    /// Create a new Glyph
    ///
    pub fn new(data: &BitSlice<u8, Lsb0>) -> Option<BitGlyph<R, C>> {
        if data.len() < (R * C) {
            None
        } else {
            Some(Self(BitBox::from(data)))
        }
    }
}

impl<const R: usize, const C: usize> Display for BitGlyph<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.chunks(C).for_each(|row| {
            let row_string: String = row.iter().map(|bit| if *bit { "██" } else { "░░" }).collect();
            writeln!(f, "{}", row_string).unwrap();
        });
        std::fmt::Result::Ok(())
    }
}
