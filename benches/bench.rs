#![feature(test)]

extern crate test;

use rand::prelude::*;
use rand_pcg::Pcg32;
use test::Bencher;

#[bench]
fn shuffle_rand_pcg32(b: &mut Bencher) {
    let mut rng = Pcg32::from_rng(thread_rng()).unwrap();
    let mut x = (0..100).collect::<Vec::<usize>>();
    b.iter(|| {
        x.shuffle(&mut rng);
        x[0]
    })
}

#[bench]
fn shuffle_fastrand(b: &mut Bencher) {
    let rng = fastrand::Rng::new();
    let mut x = (0..100).collect::<Vec::<usize>>();
    b.iter(|| {
        rng.shuffle(&mut x);
        x[0]
    })
}

#[bench]
fn u8_rand_pcg32(b: &mut Bencher) {
    let mut rng = Pcg32::from_rng(thread_rng()).unwrap();
    b.iter(|| {
        let mut sum = 0u8;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.gen::<u8>());
        }
        sum
    })
}

#[bench]
fn u8_fastrand(b: &mut Bencher) {
    let rng = fastrand::Rng::new();
    b.iter(|| {
        let mut sum = 0u8;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.u8(..));
        }
        sum
    })
}

#[bench]
fn u32_rand_pcg32(b: &mut Bencher) {
    let mut rng = Pcg32::from_rng(thread_rng()).unwrap();
    b.iter(|| {
        let mut sum = 0u32;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.gen::<u32>());
        }
        sum
    })
}

#[bench]
fn u32_fastrand(b: &mut Bencher) {
    let rng = fastrand::Rng::new();
    b.iter(|| {
        let mut sum = 0u32;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.u32(..));
        }
        sum
    })
}
