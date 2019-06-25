//! [Middle Square Weyl Sequence][1] pseudorandom number generator (`no_std`).
//!
//! [1]: https://en.wikipedia.org/wiki/Middle-square_method#Middle_Square_Weyl_Sequence_PRNG
//!
//! # Example
//!
//! ```
//! use msws::Rand;
//! let seed = 0xb5ad4eceda1ce2a9;
//! let mut r = Rand::new(seed).expect("invalid seed");
//! r.rand(); // => u32
//! ```
//!
//! # Crypto
//!
//! Pseudorandom number generators should not be used for crypto.

#![deny(missing_docs)]
#![no_std]

use core::result::Result;

/// x: Random output
/// w: Weyl sequence
/// s: Seed, must be odd
pub struct Rand {
    s: u64,
    x: u64,
    w: u64,
}

impl Rand {
    /// Generates a new Rand struct from an *odd* seed.
    ///
    /// Panics: If the supplied seed is not odd.
    pub fn new(s: u64) -> Result<Self, &'static str> {
        if s & 1 == 0 {
            return Err("seed must be odd");
        }

        Ok(Self { s, x: 0, w: 0 })
    }

    /// Returns a random integer.
    pub fn rand(&mut self) -> u32 {
        // Square the number
        self.x = self.x.wrapping_pow(2);

        // Update the Weyl sequence
        self.w = self.w.wrapping_add(self.s);

        // Apply to x
        self.x = self.x.wrapping_add(self.w);

        // Return the middle 32-bits
        ((self.x >> 32) | (self.x << 32)) as u32
    }
}
