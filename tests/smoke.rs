#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
use wasm_bindgen_test::wasm_bindgen_test as test;

#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn bool() {
    for x in &[false, true] {
        while fastrand::bool() != *x {}
    }
}

#[test]
fn u8() {
    for x in 0..10 {
        while fastrand::u8(..10) != x {}
    }

    for x in 200..=u8::MAX {
        while fastrand::u8(200..) != x {}
    }
}

#[test]
fn i8() {
    for x in -128..-120 {
        while fastrand::i8(..-120) != x {}
    }

    for x in 120..=127 {
        while fastrand::i8(120..) != x {}
    }
}

#[test]
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
fn fill() {
    let r = fastrand::Rng::new();
    let mut a = [0u8; 64];
    let mut b = [0u8; 64];

    r.fill(&mut a);
    r.fill(&mut b);

    assert_ne!(a, b);
}

#[test]
fn rng() {
    let r = fastrand::Rng::new();

    assert_ne!(r.u64(..), r.u64(..));

    r.seed(7);
    let a = r.u64(..);
    r.seed(7);
    let b = r.u64(..);
    assert_eq!(a, b);
}

#[test]
fn rng_init() {
    let a = fastrand::Rng::new();
    let b = fastrand::Rng::new();
    assert_ne!(a.u64(..), b.u64(..));

    a.seed(7);
    b.seed(7);
    assert_eq!(a.u64(..), b.u64(..));
}

#[test]
fn with_seed() {
    let a = fastrand::Rng::with_seed(7);
    let b = fastrand::Rng::new();
    b.seed(7);
    assert_eq!(a.u64(..), b.u64(..));
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_clone() {
    let seed = 0x4d595df4d0f33173;
    let rng1 = fastrand::Rng::with_seed(seed);
    let rng2 = rng1.clone();
    // clone should keep internal state same
    assert_eq!(rng1.get_seed(), seed);
    assert_eq!(rng2.get_seed(), seed);
    assert_eq!(rng1.u64(..), rng2.u64(..));
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_fork() {
    use std::collections::HashSet;

    let mut set = HashSet::<u64>::new();
    let seed = 0x4d595df4d0f33173;
    fn check_unique(hs: &mut HashSet<u64>, rng: &fastrand::Rng) {
        for _ in 0..3 {
            let r = rng.u64(..);
            assert!(!hs.contains(&r));
            hs.insert(r);
        }
    }
    let rg = fastrand::Rng::with_seed(seed);
    check_unique(&mut set, &rg);

    let g1 = rg.fork();
    let g2 = rg.fork();
    let g3 = rg.fork();

    check_unique(&mut set, &g1);
    check_unique(&mut set, &g2);
    check_unique(&mut set, &g3);
}
