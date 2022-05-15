#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
use fastrand::CellSeed;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn bool() {
    for x in &[false, true] {
        while fastrand::bool() != *x {}
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn u8() {
    for x in 0..10 {
        while fastrand::u8(..10) != x {}
    }

    for x in 200..=u8::MAX {
        while fastrand::u8(200..) != x {}
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn i8() {
    for x in -128..-120 {
        while fastrand::i8(..-120) != x {}
    }

    for x in 120..=127 {
        while fastrand::i8(120..) != x {}
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn u32() {
    for n in 1u32..10_000 {
        let n = n.wrapping_mul(n);
        let n = n.wrapping_mul(n);
        if n != 0 {
            for _ in 0..1000 {
                assert!(fastrand::u32(..n) < n);
            }
        }
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn u64() {
    for n in 1u64..10_000 {
        let n = n.wrapping_mul(n);
        let n = n.wrapping_mul(n);
        let n = n.wrapping_mul(n);
        if n != 0 {
            for _ in 0..1000 {
                assert!(fastrand::u64(..n) < n);
            }
        }
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn u128() {
    for n in 1u128..10_000 {
        let n = n.wrapping_mul(n);
        let n = n.wrapping_mul(n);
        let n = n.wrapping_mul(n);
        let n = n.wrapping_mul(n);
        if n != 0 {
            for _ in 0..1000 {
                assert!(fastrand::u128(..n) < n);
            }
        }
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn rng() {
    let r = fastrand::Rng::<CellSeed>::new();

    assert_ne!(r.u64(..), r.u64(..));

    r.seed(7);
    let a = r.u64(..);
    r.seed(7);
    let b = r.u64(..);
    assert_eq!(a, b);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn rng_init() {
    let a = fastrand::Rng::<CellSeed>::new();
    let b = fastrand::Rng::<CellSeed>::new();
    assert_ne!(a.u64(..), b.u64(..));

    a.seed(7);
    b.seed(7);
    assert_eq!(a.u64(..), b.u64(..));
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn with_seed() {
    let a = fastrand::Rng::<CellSeed>::with_seed(7);
    let b = fastrand::Rng::<CellSeed>::new();
    b.seed(7);
    assert_eq!(a.u64(..), b.u64(..));
}



mod atomic {
    use std::sync::atomic::{AtomicU64, Ordering};
    use fastrand::{Rng, Seed};

    struct AtomicSeed(AtomicU64);
    impl Seed for AtomicSeed {
        fn with_seed(seed: u64) -> Rng<Self> where Self: Sized {
            Rng::from(AtomicSeed(AtomicU64::new(seed)))
        }

        fn get(&self) -> u64 {
            self.0.load(Ordering::Relaxed)
        }

        fn set(&self, new_seed: u64) {
            self.0.store(new_seed, Ordering::Relaxed)
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn rng_atomic() {
        let r = fastrand::Rng::<AtomicSeed>::new();

        assert_ne!(r.u64(..), r.u64(..));

        r.seed(7);
        let a = r.u64(..);
        r.seed(7);
        let b = r.u64(..);
        assert_eq!(a, b);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn rng_init_atomic() {
        let a = fastrand::Rng::<AtomicSeed>::new();
        let b = fastrand::Rng::<AtomicSeed>::new();
        assert_ne!(a.u64(..), b.u64(..));

        a.seed(7);
        b.seed(7);
        assert_eq!(a.u64(..), b.u64(..));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn with_seed_atomic() {
        let a = fastrand::Rng::<AtomicSeed>::with_seed(7);
        let b = fastrand::Rng::<AtomicSeed>::new();
        b.seed(7);
        assert_eq!(a.u64(..), b.u64(..));
    }
}
