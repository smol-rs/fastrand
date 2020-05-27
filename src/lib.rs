//! A simple random number generator.
//!
//! Easy to use but not cryptographically secure.
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

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{Bound, RangeBounds};
use std::thread;
use std::time::Instant;

/// Generates a random `u64`.
fn gen_u64() -> u64 {
    thread_local! {
        static RNG: Cell<(u64, u64)> = Cell::new((seed(), seed()));
    }

    #[cold]
    fn seed() -> u64 {
        let mut hasher = DefaultHasher::new();
        thread::current().id().hash(&mut hasher);
        Instant::now().hash(&mut hasher);
        hasher.finish()
    }

    // xorshift+: https://en.wikipedia.org/wiki/Xorshift
    RNG.try_with(|rng| {
        let (mut a, mut b) = rng.get();
        let (mut t, s) = (a, b);
        a = s;
        t ^= t << 23;
        t ^= t >> 17;
        t ^= s ^ (s >> 26);
        b = t;
        rng.set((a, b));
        t.wrapping_add(s)
    })
    .unwrap_or_else(|_| seed())
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

macro_rules! code {
    ($t:tt, $gen:expr, $mod:ident, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        pub fn $t(range: impl RangeBounds<$t>) -> $t {
            let low = match range.start_bound() {
                Bound::Unbounded => $t::MIN,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x
                    .checked_add(1)
                    .unwrap_or_else(|| panic!("invalid start bound: {:?}", range.start_bound())),
            };

            let high = match range.end_bound() {
                Bound::Unbounded => $t::MAX,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x
                    .checked_sub(1)
                    .unwrap_or_else(|| panic!("invalid end bound: {:?}", range.end_bound())),
            };

            if low > high {
                panic!(
                    "empty range: {:?}..{:?}",
                    range.start_bound(),
                    range.end_bound()
                );
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

code!(
    u8,
    gen_u64,
    gen_mod_u32,
    "Generates a random `u8` in the given range."
);
code!(
    i8,
    gen_u64,
    gen_mod_u32,
    "Generates a random `i8` in the given range."
);

code!(
    u16,
    gen_u64,
    gen_mod_u32,
    "Generates a random `u16` in the given range."
);
code!(
    i16,
    gen_u64,
    gen_mod_u32,
    "Generates a random `i16` in the given range."
);

code!(
    u32,
    gen_u64,
    gen_mod_u32,
    "Generates a random `u32` in the given range."
);
code!(
    i32,
    gen_u64,
    gen_mod_u32,
    "Generates a random `i32` in the given range."
);

code!(
    u64,
    gen_u64,
    gen_mod_u64,
    "Generates a random `u64` in the given range."
);
code!(
    i64,
    gen_u64,
    gen_mod_u64,
    "Generates a random `i64` in the given range."
);

code!(
    usize,
    gen_u64,
    gen_mod_u64,
    "Generates a random `usize` in the given range."
);
code!(
    isize,
    gen_u64,
    gen_mod_u64,
    "Generates a random `isize` in the given range."
);

code!(
    u128,
    gen_u128,
    gen_mod_u128,
    "Generates a random `u128` in the given range."
);
code!(
    i128,
    gen_u128,
    gen_mod_u128,
    "Generates a random `i128` in the given range."
);

/// Generates a random `bool`.
pub fn bool() -> bool {
    self::u8(..) % 2 == 0
}

/// Shuffles a slice randomly.
pub fn shuffle<T>(slice: &mut [T]) {
    for i in 1..slice.len() {
        slice.swap(i, self::usize(..=i));
    }
}
