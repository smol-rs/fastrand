#![feature(test)]

extern crate test;

use rand::prelude::*;
use rand_pcg::Pcg32 as SmallRng;
use test::Bencher;

#[bench]
fn rand_smallrng(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let x: &mut [usize] = &mut [1; 100];
    b.iter(|| {
        x.shuffle(&mut rng);
        x[0]
    })
}

#[bench]
fn fastrand_rng(b: &mut Bencher) {
    let rng = fastrand::Rng::new();
    let x: &mut [usize] = &mut [1; 100];
    b.iter(|| {
        rng.shuffle(x);
        x[0]
    })
}
