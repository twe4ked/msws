//! [Middle Square Weyl Sequence][1] pseudorandom number generator (`no_std`).
//!
//! [1]: https://en.wikipedia.org/wiki/Middle-square_method#Middle_Square_Weyl_Sequence_PRNG
//!
//! # Example
//!
//! ```
//! use msws::Rand;
//!
//! // This will always return the same seed.
//! msws::seed(0); // => 0xb5ad4eceda1ce2a9
//!
//! let seed = 0xb5ad4eceda1ce2a9;
//! let mut r = Rand::new(seed).expect("invalid seed");
//! r.rand(); // => 0xb5ad4ece
//! ```
//!
//! # Crypto
//!
//! Pseudorandom number generators should not be used for crypto.

#![deny(missing_docs)]
#![no_std]

use core::result::Result;

/// This struct holds the state necessary to generate random numbers.
/// You should continue to call `rand()` on the same instance of the struct.
pub struct Rand {
    // Seed, must be odd
    s: u64,
    // Random output
    x: u64,
    // Weyl sequence
    w: u64,
}

impl Rand {
    /// Generates a new Rand struct from an *odd* seed.
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

        // Store the middle 32-bits
        self.x = (self.x >> 32) | (self.x << 32);

        self.x as u32
    }
}

// Each value provides 100 million unique outputs.
const S: [u64; 30] = [
    0x8b5ad4ce914ecdf7,
    0xdbc8915f4b1cd961,
    0x3a16e0c51fa593d9,
    0x1794da529ec6d70b,
    0x8fc49b2a752f643b,
    0xde07a518fba03571,
    0xb1d2e4762d58906b,
    0x478f6219da719b05,
    0x41857dc34a2fdc05,
    0xb9425ed8e351a06f,
    0x9235eb64c35eab7d,
    0x91f0e7b8e0536af7,
    0x4f0581abb194f75b,
    0xdab4e53c95408d1f,
    0xf23ba0c5410ceb3b,
    0x912a0b4ce102a36d,
    0x92a73b40b46a2e71,
    0x46ca273b5fde168d,
    0xf9b8ad61743910b5,
    0x490ceb3d865e4bc9,
    0xa12e0dcfbf6471cf,
    0xa54c91db6dc0fe37,
    0x08c3564a5c031727,
    0xe3296d17c14795bd,
    0x5387014db793f24f,
    0x6d47af052931fe47,
    0xd138c9ef735c0e8f,
    0xa790fbc8ebf02d3b,
    0x4a1b027867c953fb,
    0x49a180de9567182d,
];

/// Returns a seed for a given integer.
///
/// # Example
///
/// ```
/// use msws::seed;
/// seed(0); // => 0x8b5ad4ceb9c1fe73
/// ```
pub fn seed(n: u64) -> u64 {
    let mut rand = {
        let mut r: u64 = n / 100_000_000;
        let t: u64 = n % 100_000_000;
        let s = S[r as usize % 30];
        r /= 30;
        let w = t.wrapping_mul(s) + r.wrapping_mul(s).wrapping_mul(100_000_000);
        let x = w;
        Rand { s, x, w }
    };

    ((different_digits(&mut rand) as u64) << 32) | (different_digits(&mut rand) as u64) | 1
}

fn different_digits(rand: &mut Rand) -> u32 {
    let mut m: u32 = 0;
    let mut a: u32 = 0;
    let mut c: u32 = 0;

    while m < 32 {
        let j = rand.rand();
        let mut i = 0;
        while i < 32 {
            let k: u32 = (j >> i) & 0xf;
            if (c & (1 << k)) == 0 {
                c |= 1 << k;
                a |= k << m;
                m += 4;
                if m >= 32 {
                    break;
                }
            }

            i += 4;
        }
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand() {
        let mut r = Rand::new(0xb5ad4eceda1ce2a9).unwrap();

        assert_eq!(r.rand(), 0xb5ad4ece);
        assert_eq!(r.rand(), 0xdf4ee85c);
        assert_eq!(r.rand(), 0x1889155f);
        assert_eq!(r.rand(), 0xc6dcbccf);
        assert_eq!(r.rand(), 0x1106e0c5);
        assert_eq!(r.rand(), 0x473066ae);
        assert_eq!(r.rand(), 0x374ac427);
        assert_eq!(r.rand(), 0x21e9e9bf);
        assert_eq!(r.rand(), 0x7294da52);
        assert_eq!(r.rand(), 0x212dbe1a);
    }

    #[test]
    fn test_seed() {
        assert_eq!(seed(0), 0x8b5ad4ceb9c1fe73);
        assert_eq!(seed(1), 0x64d098b5c4f26d37);
        assert_eq!(seed(2), 0x45973acb0ad43b97);
        assert_eq!(seed(3), 0x6e9c5db170d261c9);
        assert_eq!(seed(4), 0x4fa75198bc653efb);
        assert_eq!(seed(5), 0x3d4c562ea9451fed);
        assert_eq!(seed(6), 0xd8b57104d2850b1b);
        assert_eq!(seed(7), 0x2bdfe60a326fdab1);
        assert_eq!(seed(8), 0x86ced140fe30d875);
        assert_eq!(seed(9), 0x7c981fa6257d863d);
    }
}
