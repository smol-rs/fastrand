# fastrand

[![Build](https://github.com/stjepang/fastrand/workflows/Build%20and%20test/badge.svg)](
https://github.com/stjepang/fastrand/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/stjepang/fastrand)
[![Cargo](https://img.shields.io/crates/v/fastrand.svg)](
https://crates.io/crates/fastrand)
[![Documentation](https://docs.rs/fastrand/badge.svg)](
https://docs.rs/fastrand)

A simple random number generator.

Easy to use but not cryptographically secure.

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

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
