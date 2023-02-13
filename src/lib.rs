//! A simple and fast random number generator.
//!
//! The implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast
//! generator but **not** cryptographically secure.
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
//!
//! To be more efficient, create a new [`Rng`] instance instead of using the thread-local
//! generator:
//!
//! ```
//! use std::iter::repeat_with;
//!
//! let mut rng = fastrand::Rng::new();
//! let mut bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::ops::{Bound, RangeBounds};
use std::thread;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use instant::Instant;
#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
use std::time::Instant;

/// A random number generator.
#[derive(Debug, PartialEq, Eq)]
pub struct Rng(u64);

impl Default for Rng {
    #[inline]
    fn default() -> Rng {
        Rng::new()
    }
}

impl Clone for Rng {
    /// Clones the generator by creating a new generator with the same seed.
    fn clone(&self) -> Rng {
        Rng::with_seed(self.0)
    }
}

impl Rng {
    /// Generates a random `u32`.
    #[inline]
    fn gen_u32(&mut self) -> u32 {
        self.gen_u64() as u32
    }

    /// Generates a random `u64`.
    #[inline]
    fn gen_u64(&mut self) -> u64 {
        let s = self.0.wrapping_add(0xA0761D6478BD642F);
        self.0 = s;
        let t = u128::from(s) * u128::from(s ^ 0xE7037ED1A0B428DB);
        (t as u64) ^ (t >> 64) as u64
    }

    /// Generates a random `u128`.
    #[inline]
    fn gen_u128(&mut self) -> u128 {
        (u128::from(self.gen_u64()) << 64) | u128::from(self.gen_u64())
    }

    /// Generates a random `u32` in `0..n`.
    #[inline]
    fn gen_mod_u32(&mut self, n: u32) -> u32 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u32();
        let mut hi = mul_high_u32(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u32();
                hi = mul_high_u32(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }

    /// Generates a random `u64` in `0..n`.
    #[inline]
    fn gen_mod_u64(&mut self, n: u64) -> u64 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u64();
        let mut hi = mul_high_u64(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u64();
                hi = mul_high_u64(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }

    /// Generates a random `u128` in `0..n`.
    #[inline]
    fn gen_mod_u128(&mut self, n: u128) -> u128 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u128();
        let mut hi = mul_high_u128(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u128();
                hi = mul_high_u128(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }
}

thread_local! {
    static RNG: Cell<Rng> = Cell::new(Rng({
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        thread::current().id().hash(&mut hasher);
        let hash = hasher.finish();
        (hash << 1) | 1
    }));
}

/// Computes `(a * b) >> 32`.
#[inline]
fn mul_high_u32(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}

/// Computes `(a * b) >> 64`.
#[inline]
fn mul_high_u64(a: u64, b: u64) -> u64 {
    (((a as u128) * (b as u128)) >> 64) as u64
}

/// Computes `(a * b) >> 128`.
#[inline]
fn mul_high_u128(a: u128, b: u128) -> u128 {
    // Adapted from: https://stackoverflow.com/a/28904636
    let a_lo = a as u64 as u128;
    let a_hi = (a >> 64) as u64 as u128;
    let b_lo = b as u64 as u128;
    let b_hi = (b >> 64) as u64 as u128;
    let carry = (a_lo * b_lo) >> 64;
    let carry = ((a_hi * b_lo) as u64 as u128 + (a_lo * b_hi) as u64 as u128 + carry) >> 64;
    a_hi * b_hi + ((a_hi * b_lo) >> 64) + ((a_lo * b_hi) >> 64) + carry
}

macro_rules! rng_integer {
    ($t:tt, $unsigned_t:tt, $gen:tt, $mod:tt, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        #[inline]
        pub fn $t(&mut self, range: impl RangeBounds<$t>) -> $t {
            let panic_empty_range = || {
                panic!(
                    "empty range: {:?}..{:?}",
                    range.start_bound(),
                    range.end_bound()
                )
            };

            let low = match range.start_bound() {
                Bound::Unbounded => std::$t::MIN,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x.checked_add(1).unwrap_or_else(panic_empty_range),
            };

            let high = match range.end_bound() {
                Bound::Unbounded => std::$t::MAX,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x.checked_sub(1).unwrap_or_else(panic_empty_range),
            };

            if low > high {
                panic_empty_range();
            }

            if low == std::$t::MIN && high == std::$t::MAX {
                self.$gen() as $t
            } else {
                let len = high.wrapping_sub(low).wrapping_add(1);
                low.wrapping_add(self.$mod(len as $unsigned_t as _) as $t)
            }
        }
    };
}

impl Rng {
    /// Creates a new random number generator.
    #[inline]
    pub fn new() -> Rng {
        try_with_rng(Rng::fork).unwrap_or_else(|_| Rng::with_seed(0x4d595df4d0f33173))
    }

    /// Creates a new random number generator with the initial seed.
    #[inline]
    #[must_use = "this creates a new instance of `Rng`; if you want to initialize the thread-local generator, use `fastrand::seed()` instead"]
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng(0);

        rng.seed(seed);
        rng
    }

    /// Clones the generator by deterministically deriving a new generator based on the initial
    /// seed.
    ///
    /// # Example
    ///
    /// ```
    /// // Seed two generators equally, and clone both of them.
    /// let mut base1 = fastrand::Rng::new();
    /// base1.seed(0x4d595df4d0f33173);
    /// base1.bool(); // Use the generator once.
    ///
    /// let mut base2 = fastrand::Rng::new();
    /// base2.seed(0x4d595df4d0f33173);
    /// base2.bool(); // Use the generator once.
    ///
    /// let mut rng1 = base1.clone();
    /// let mut rng2 = base2.clone();
    ///
    /// assert_eq!(rng1.u64(..), rng2.u64(..), "the cloned generators are identical");
    /// ```
    #[inline]
    #[must_use = "this creates a new instance of `Rng`"]
    pub fn fork(&mut self) -> Self {
        Rng::with_seed(self.gen_u64())
    }

    /// Generates a random `char` in ranges a-z and A-Z.
    #[inline]
    pub fn alphabetic(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let len = CHARS.len() as u8;
        let i = self.u8(..len);
        CHARS[i as usize] as char
    }

    /// Generates a random `char` in ranges a-z, A-Z and 0-9.
    #[inline]
    pub fn alphanumeric(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let len = CHARS.len() as u8;
        let i = self.u8(..len);
        CHARS[i as usize] as char
    }

    /// Generates a random `bool`.
    #[inline]
    pub fn bool(&mut self) -> bool {
        self.u8(..) % 2 == 0
    }

    /// Generates a random digit in the given `base`.
    ///
    /// Digits are represented by `char`s in ranges 0-9 and a-z.
    ///
    /// Panics if the base is zero or greater than 36.
    #[inline]
    pub fn digit(&mut self, base: u32) -> char {
        if base == 0 {
            panic!("base cannot be zero");
        }
        if base > 36 {
            panic!("base cannot be larger than 36");
        }
        let num = self.u8(..base as u8);
        if num < 10 {
            (b'0' + num) as char
        } else {
            (b'a' + num - 10) as char
        }
    }

    /// Generates a random `f32` in range `0..1`.
    pub fn f32(&mut self) -> f32 {
        let b = 32;
        let f = std::f32::MANTISSA_DIGITS - 1;
        f32::from_bits((1 << (b - 2)) - (1 << f) + (self.u32(..) >> (b - f))) - 1.0
    }

    /// Generates a random `f64` in range `0..1`.
    pub fn f64(&mut self) -> f64 {
        let b = 64;
        let f = std::f64::MANTISSA_DIGITS - 1;
        f64::from_bits((1 << (b - 2)) - (1 << f) + (self.u64(..) >> (b - f))) - 1.0
    }

    rng_integer!(
        i8,
        u8,
        gen_u32,
        gen_mod_u32,
        "Generates a random `i8` in the given range."
    );

    rng_integer!(
        i16,
        u16,
        gen_u32,
        gen_mod_u32,
        "Generates a random `i16` in the given range."
    );

    rng_integer!(
        i32,
        u32,
        gen_u32,
        gen_mod_u32,
        "Generates a random `i32` in the given range."
    );

    rng_integer!(
        i64,
        u64,
        gen_u64,
        gen_mod_u64,
        "Generates a random `i64` in the given range."
    );

    rng_integer!(
        i128,
        u128,
        gen_u128,
        gen_mod_u128,
        "Generates a random `i128` in the given range."
    );

    #[cfg(target_pointer_width = "16")]
    rng_integer!(
        isize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `isize` in the given range."
    );
    #[cfg(target_pointer_width = "32")]
    rng_integer!(
        isize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `isize` in the given range."
    );
    #[cfg(target_pointer_width = "64")]
    rng_integer!(
        isize,
        usize,
        gen_u64,
        gen_mod_u64,
        "Generates a random `isize` in the given range."
    );

    /// Generates a random `char` in range a-z.
    #[inline]
    pub fn lowercase(&mut self) -> char {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let len = CHARS.len() as u8;
        let i = self.u8(..len);
        CHARS[i as usize] as char
    }

    /// Initializes this generator with the given seed.
    #[inline]
    pub fn seed(&mut self, seed: u64) {
        self.0 = seed;
    }

    /// Gives back **current** seed that is being held by this generator.
    #[inline]
    pub fn get_seed(&self) -> u64 {
        self.0
    }

    /// Shuffles a slice randomly.
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in 1..slice.len() {
            slice.swap(i, self.usize(..=i));
        }
    }

    /// Fill a byte slice with random data.
    #[inline]
    pub fn fill(&mut self, slice: &mut [u8]) {
        // We fill the slice by chunks of 8 bytes, or one block of
        // WyRand output per new state.
        let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
        for chunk in chunks.by_ref() {
            let n = self.gen_u64().to_ne_bytes();
            // Safe because the chunks are always 8 bytes exactly.
            chunk.copy_from_slice(&n);
        }

        let remainder = chunks.into_remainder();

        // Any remainder will always be less than 8 bytes.
        if !remainder.is_empty() {
            // Generate one last block of 8 bytes of entropy
            let n = self.gen_u64().to_ne_bytes();

            // Use the remaining length to copy from block
            remainder.copy_from_slice(&n[..remainder.len()]);
        }
    }

    rng_integer!(
        u8,
        u8,
        gen_u32,
        gen_mod_u32,
        "Generates a random `u8` in the given range."
    );

    rng_integer!(
        u16,
        u16,
        gen_u32,
        gen_mod_u32,
        "Generates a random `u16` in the given range."
    );

    rng_integer!(
        u32,
        u32,
        gen_u32,
        gen_mod_u32,
        "Generates a random `u32` in the given range."
    );

    rng_integer!(
        u64,
        u64,
        gen_u64,
        gen_mod_u64,
        "Generates a random `u64` in the given range."
    );

    rng_integer!(
        u128,
        u128,
        gen_u128,
        gen_mod_u128,
        "Generates a random `u128` in the given range."
    );

    #[cfg(target_pointer_width = "16")]
    rng_integer!(
        usize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "32")]
    rng_integer!(
        usize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "64")]
    rng_integer!(
        usize,
        usize,
        gen_u64,
        gen_mod_u64,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "128")]
    rng_integer!(
        usize,
        usize,
        gen_u128,
        gen_mod_u128,
        "Generates a random `usize` in the given range."
    );

    /// Generates a random `char` in range A-Z.
    #[inline]
    pub fn uppercase(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let len = CHARS.len() as u8;
        let i = self.u8(..len);
        CHARS[i as usize] as char
    }

    /// Generates a random `char` in the given range.
    ///
    /// Panics if the range is empty.
    #[inline]
    pub fn char(&mut self, range: impl RangeBounds<char>) -> char {
        use std::convert::TryFrom;

        let panic_empty_range = || {
            panic!(
                "empty range: {:?}..{:?}",
                range.start_bound(),
                range.end_bound()
            )
        };

        let surrogate_start = 0xd800u32;
        let surrogate_len = 0x800u32;

        let low = match range.start_bound() {
            Bound::Unbounded => 0u8 as char,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start - 1 {
                    surrogate_start + surrogate_len
                } else {
                    x as u32 + 1
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };

        let high = match range.end_bound() {
            Bound::Unbounded => std::char::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start + surrogate_len {
                    surrogate_start - 1
                } else {
                    (x as u32).wrapping_sub(1)
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };

        if low > high {
            panic_empty_range();
        }

        let gap = if (low as u32) < surrogate_start && (high as u32) >= surrogate_start {
            surrogate_len
        } else {
            0
        };
        let range = high as u32 - low as u32 - gap;
        let mut val = self.u32(0..=range) + low as u32;
        if val >= surrogate_start {
            val += gap;
        }
        val.try_into().unwrap()
    }
}

/// Run an operation with the current thread-local generator.
#[inline]
fn with_rng<R>(f: impl FnOnce(&mut Rng) -> R) -> R {
    RNG.with(|rng| {
        let current = rng.replace(Rng(0));

        let mut restore = RestoreOnDrop { rng, current };

        f(&mut restore.current)
    })
}

/// Try to run an operation with the current thread-local generator.
#[inline]
fn try_with_rng<R>(f: impl FnOnce(&mut Rng) -> R) -> Result<R, std::thread::AccessError> {
    RNG.try_with(|rng| {
        let current = rng.replace(Rng(0));

        let mut restore = RestoreOnDrop { rng, current };

        f(&mut restore.current)
    })
}

/// Make sure the original RNG is restored even on panic.
struct RestoreOnDrop<'a> {
    rng: &'a Cell<Rng>,
    current: Rng,
}

impl Drop for RestoreOnDrop<'_> {
    fn drop(&mut self) {
        self.rng.set(Rng(self.current.0));
    }
}

/// Initializes the thread-local generator with the given seed.
#[inline]
pub fn seed(seed: u64) {
    with_rng(|r| r.seed(seed));
}

/// Gives back **current** seed that is being held by the thread-local generator.
#[inline]
pub fn get_seed() -> u64 {
    with_rng(|r| r.get_seed())
}

/// Generates a random `bool`.
#[inline]
pub fn bool() -> bool {
    with_rng(|r| r.bool())
}

/// Generates a random `char` in ranges a-z and A-Z.
#[inline]
pub fn alphabetic() -> char {
    with_rng(|r| r.alphabetic())
}

/// Generates a random `char` in ranges a-z, A-Z and 0-9.
#[inline]
pub fn alphanumeric() -> char {
    with_rng(|r| r.alphanumeric())
}

/// Generates a random `char` in range a-z.
#[inline]
pub fn lowercase() -> char {
    with_rng(|r| r.lowercase())
}

/// Generates a random `char` in range A-Z.
#[inline]
pub fn uppercase() -> char {
    with_rng(|r| r.uppercase())
}

/// Generates a random digit in the given `base`.
///
/// Digits are represented by `char`s in ranges 0-9 and a-z.
///
/// Panics if the base is zero or greater than 36.
#[inline]
pub fn digit(base: u32) -> char {
    with_rng(|r| r.digit(base))
}

/// Shuffles a slice randomly.
#[inline]
pub fn shuffle<T>(slice: &mut [T]) {
    with_rng(|r| r.shuffle(slice))
}

macro_rules! integer {
    ($t:tt, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        #[inline]
        pub fn $t(range: impl RangeBounds<$t>) -> $t {
            with_rng(|r| r.$t(range))
        }
    };
}

integer!(u8, "Generates a random `u8` in the given range.");
integer!(i8, "Generates a random `i8` in the given range.");
integer!(u16, "Generates a random `u16` in the given range.");
integer!(i16, "Generates a random `i16` in the given range.");
integer!(u32, "Generates a random `u32` in the given range.");
integer!(i32, "Generates a random `i32` in the given range.");
integer!(u64, "Generates a random `u64` in the given range.");
integer!(i64, "Generates a random `i64` in the given range.");
integer!(u128, "Generates a random `u128` in the given range.");
integer!(i128, "Generates a random `i128` in the given range.");
integer!(usize, "Generates a random `usize` in the given range.");
integer!(isize, "Generates a random `isize` in the given range.");
integer!(char, "Generates a random `char` in the given range.");

/// Generates a random `f32` in range `0..1`.
pub fn f32() -> f32 {
    with_rng(|r| r.f32())
}

/// Generates a random `f64` in range `0..1`.
pub fn f64() -> f64 {
    with_rng(|r| r.f64())
}
