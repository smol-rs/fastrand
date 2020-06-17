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
