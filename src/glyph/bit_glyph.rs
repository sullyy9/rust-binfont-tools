//! Basic glyph.
//!

use std::vec::Vec;

/// Glyph in a basic bitmap format where each bit represents a pixel.
pub struct BitGlyph<const ROWS: usize, const COLS: usize>(Box<[u8]>);

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
        let data_length = {
            let cols = C as f64;
            let bytes_per_row = (cols / 8.0).ceil() as usize;
            bytes_per_row * R
        };
        Self(Box::from(Vec::with_capacity(data_length)))
    }
}

impl<const R: usize, const C: usize> BitGlyph<R, C> {
    /// Create a new Glyph
    ///
    pub fn new(data: &[u8]) -> Option<BitGlyph<R, C>> {
        let data_length = {
            let (rows, cols) = (R, C as f64);
            let bytes_per_row = (cols as f64 / 8.0).ceil() as usize;
            bytes_per_row * rows
        };

        if data.len() < data_length {
            None
        } else {
            Some(Self(Box::from(data)))
        }
    }
}
