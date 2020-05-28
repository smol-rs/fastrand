//! A simple random number generator.
//!
//! The implementation uses [PCG XSH RS 64/32][paper], a simple and fast generator but not
//! cryptographically secure.
//!
//! [paper]: https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf
//!
//! # Examples
//!
//! Flip a coin:
//!
//! ```
//! if fastrand::bool() {
//!     println!("heads");
//! } else {
//!     println!("tails");
//! }
//! ```
//!
//! Generate a random `i32`:
//!
//! ```
//! let num = fastrand::i32(..);
//! ```
//!
//! Choose a random element in an array:
//!
//! ```
//! let v = vec![1, 2, 3, 4, 5];
//! let i = fastrand::usize(..v.len());
//! let elem = v[i];
//! ```
//!
//! Shuffle an array:
//!
//! ```
//! let mut v = vec![1, 2, 3, 4, 5];
//! fastrand::shuffle(&mut v);
//! ```
//!
//! Generate a random [`Vec`] or [`String`]:
//!
//! ```
//! use std::iter::repeat_with;
//!
//! let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10).collect();
//! let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
//! ```
//!
//! To get reproducible results on every run, initialize the generator with a seed:
//!
//! ```
//! // Pick an arbitrary number as seed.
//! fastrand::seed(7);
//!
//! // Now this prints the same number on every run:
//! println!("{}", fastrand::u32(..));
//! ```

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{Bound, RangeBounds};
use std::thread;
use std::time::Instant;

thread_local! {
    static STATE: Cell<u64> = {
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        thread::current().id().hash(&mut hasher);
        let hash = hasher.finish();
        Cell::new((hash << 1) | 1)
    }
}

/// Initializes thread-local generator with the given seed.
pub fn seed(seed: u64) {
    STATE.with(|state| state.set((seed << 1) | 1));
    gen_u32();
}

/// Generates a random `u32`.
fn gen_u32() -> u32 {
    // Adapted from: https://en.wikipedia.org/wiki/Permuted_congruential_generator
    STATE
        .try_with(|state| {
            let s = state.get();
            state.set(s.wrapping_mul(6364136223846793005));
            let count = s >> 61;
            let x = s ^ (s >> 22);
            (x >> (22 + count)) as u32
        })
        .unwrap_or(1157102669)
}

/// Generates a random `u64`.
fn gen_u64() -> u64 {
    ((gen_u32() as u64) << 32) | (gen_u32() as u64)
}

/// Generates a random `u128`.
fn gen_u128() -> u128 {
    ((gen_u64() as u128) << 64) | (gen_u64() as u128)
}

/// Computes `(a * b) >> 32`.
fn mul_high_u32(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}

/// Computes `(a * b) >> 64`.
fn mul_high_u64(a: u64, b: u64) -> u64 {
    (((a as u128) * (b as u128)) >> 64) as u64
}

/// Computes `(a * b) >> 128`.
fn mul_high_u128(a: u128, b: u128) -> u128 {
    let a_lo = a as u64 as u128;
    let a_hi = (a >> 64) as u64 as u128;

    let b_lo = b as u64 as u128;
    let b_hi = (b >> 64) as u64 as u128;

    let carry = (a_lo * b_lo) >> 64;
    let carry = (a_hi * b_lo + a_lo * b_hi + carry) >> 64;
    a_hi * b_hi + carry
}

/// Generates a random `u32` in `0..n`.
fn gen_mod_u32(n: u32) -> u32 {
    mul_high_u32(gen_u64() as u32, n)
}

/// Generates a random `u64` in `0..n`.
fn gen_mod_u64(n: u64) -> u64 {
    mul_high_u64(gen_u64(), n)
}

/// Generates a random `u128` in `0..n`.
fn gen_mod_u128(n: u128) -> u128 {
    mul_high_u128(gen_u128(), n)
}

macro_rules! integer {
    ($t:tt, $gen:expr, $mod:ident, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        pub fn $t(range: impl RangeBounds<$t>) -> $t {
            let panic_empty_range = || {
                panic!(
                    "empty range: {:?}..{:?}",
                    range.start_bound(),
                    range.end_bound()
                )
            };

            let low = match range.start_bound() {
                Bound::Unbounded => $t::MIN,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x.checked_add(1).unwrap_or_else(panic_empty_range),
            };

            let high = match range.end_bound() {
                Bound::Unbounded => $t::MAX,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x.checked_sub(1).unwrap_or_else(panic_empty_range),
            };

            if low > high {
                panic_empty_range();
            }

            if low == $t::MIN && high == $t::MAX {
                $gen() as $t
            } else {
                let len = high.wrapping_sub(low).wrapping_add(1);
                low.wrapping_add($mod(len as _) as $t)
            }
        }
    };
}

integer!(
    u8,
    gen_u32,
    gen_mod_u32,
    "Generates a random `u8` in the given range."
);

integer!(
    i8,
    gen_u32,
    gen_mod_u32,
    "Generates a random `i8` in the given range."
);

integer!(
    u16,
    gen_u32,
    gen_mod_u32,
    "Generates a random `u16` in the given range."
);

integer!(
    i16,
    gen_u32,
    gen_mod_u32,
    "Generates a random `i16` in the given range."
);

integer!(
    u32,
    gen_u32,
    gen_mod_u32,
    "Generates a random `u32` in the given range."
);

integer!(
    i32,
    gen_u32,
    gen_mod_u32,
    "Generates a random `i32` in the given range."
);

integer!(
    u64,
    gen_u64,
    gen_mod_u64,
    "Generates a random `u64` in the given range."
);

integer!(
    i64,
    gen_u64,
    gen_mod_u64,
    "Generates a random `i64` in the given range."
);

integer!(
    u128,
    gen_u128,
    gen_mod_u128,
    "Generates a random `u128` in the given range."
);

integer!(
    i128,
    gen_u128,
    gen_mod_u128,
    "Generates a random `i128` in the given range."
);

#[cfg(target_pointer_width = "16")]
integer!(
    usize,
    gen_u32,
    gen_mod_u32,
    "Generates a random `usize` in the given range."
);

#[cfg(target_pointer_width = "16")]
integer!(
    isize,
    gen_u32,
    gen_mod_u32,
    "Generates a random `isize` in the given range."
);

#[cfg(target_pointer_width = "32")]
integer!(
    usize,
    gen_u32,
    gen_mod_u32,
    "Generates a random `usize` in the given range."
);

#[cfg(target_pointer_width = "32")]
integer!(
    isize,
    gen_u32,
    gen_mod_u32,
    "Generates a random `isize` in the given range."
);

#[cfg(target_pointer_width = "64")]
integer!(
    usize,
    gen_u64,
    gen_mod_u64,
    "Generates a random `usize` in the given range."
);

#[cfg(target_pointer_width = "64")]
integer!(
    isize,
    gen_u64,
    gen_mod_u64,
    "Generates a random `isize` in the given range."
);

/// Generates a random `bool`.
pub fn bool() -> bool {
    crate::u8(..) % 2 == 0
}

/// Generates a random `char` in ranges a-z, A-Z and 0-9.
pub fn alphanumeric() -> char {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let len = CHARS.len() as u8;
    let i = crate::u8(..len);
    CHARS[i as usize] as char
}

/// Shuffles a slice randomly.
pub fn shuffle<T>(slice: &mut [T]) {
    for i in 1..slice.len() {
        slice.swap(i, crate::usize(..=i));
    }
}
