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
