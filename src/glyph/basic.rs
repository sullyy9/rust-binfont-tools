//! Basic glyph.
//!

use std::vec::Vec;

/// Glyph in a basic bitmap format where each bit represents a pixel.
pub struct Glyph<const ROWS: usize, const COLS: usize>(Box<[u8]>);

impl<const R: usize, const C: usize> Default for Glyph<R, C> {
    fn default() -> Self {
        let data_length = {
            let cols = C as f64;
            let bytes_per_row = (cols / 8.0).ceil() as usize;
            bytes_per_row * R
        };
        Self(Box::from(Vec::with_capacity(data_length)))
    }
}

impl<const R: usize, const C: usize> Glyph<R, C> {
    pub fn new(data: &[u8]) -> Option<Glyph<R, C>> {
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

