//! Basic glyph.
//!
use std::{
    fmt::Display,
    ops::{BitAnd, Not, Shl},
    vec::Vec,
};

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

    /// Return the number of bytes needed to represent all the bits in a row.
    ///
    /// # Safety
    /// The result of ceil cannot be NaN, infinite or negative.
    /// As the input is a usize / 8, we know this is the case.
    ///
    fn bytes_per_row() -> usize {
        unsafe { (C as f64 / 8.0).ceil().to_int_unchecked() }
    }
}

impl<const R: usize, const C: usize> Display for BitGlyph<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.chunks(Self::bytes_per_row()).for_each(|row| {
            row.iter().for_each(|byte| {
                let test: std::string::String = (0..8)
                    .map(|shift| if byte & (128 >> shift) != 0 { '█' } else { '░' })
                    .collect();
                write!(f, "{}", test).unwrap();
            });
            writeln!(f).unwrap();
        });
        std::fmt::Result::Ok(())
    }
}
