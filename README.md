# fastrand

[![Build](https://github.com/stjepang/fastrand/workflows/Build%20and%20test/badge.svg)](
https://github.com/stjepang/fastrand/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/stjepang/fastrand)
[![Cargo](https://img.shields.io/crates/v/fastrand.svg)](
https://crates.io/crates/fastrand)
[![Documentation](https://docs.rs/fastrand/badge.svg)](
https://docs.rs/fastrand)

A simple and fast random number generator.

The implementation uses [PCG XSH RR 64/32][paper], a simple and fast generator but **not**
cryptographically secure.

[paper]: https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf

## Examples

Flip a coin:

```rust
if fastrand::bool() {
    println!("heads");
} else {
    println!("tails");
}
```

Generate a random `i32`:

```rust
let num = fastrand::i32(..);
```

Choose a random element in an array:

```rust
let v = vec![1, 2, 3, 4, 5];
let i = fastrand::usize(..v.len());
let elem = v[i];
```

Shuffle an array:

```rust
let mut v = vec![1, 2, 3, 4, 5];
fastrand::shuffle(&mut v);
```

Generate a random `Vec` or `String`:

```rust
use std::iter::repeat_with;

let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10).collect();
let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
```

To get reproducible results on every run, initialize the generator with a seed:

```rust
// Pick an arbitrary number as seed.
fastrand::seed(7);

// Now this prints the same number on every run:
println!("{}", fastrand::u32(..));
```

To be more efficient, create a new `Rng` instance instead of using the thread-local
generator:

```rust
use std::iter::repeat_with;

let rng = fastrand::Rng::new();
let mut bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
