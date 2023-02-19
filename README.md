# Pseudo-random number generation library
`randlib` is a Rust library which allows generating pseudo-random numbers using a LFSR ([Linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register)).

## Example
```rust
use randlib::{Random, RandomSeedSource};

fn main() {
    let src = RandomSeedSource::SystemTime;
    let mut rand = Random::new(src);
    
    println!("i32:  {}", rand.rand_i32());
    println!("u32:  {}", rand.rand_u32());
    println!("bool: {}", rand.rand_bool());
    println!("f32:  {}", rand.rand_f32());
}
```
<details>
  <summary>Cargo.toml</summary>

  ```toml
  [dependencies]
  randlib = { git = "..." }
  ```
</details>

## Features
- `libc`
    - Enables `RandomSeedSource::SystemTime` and `RandomSeedSource::Crand`
    - Allows creating a seed value from the current system time as well as from the `rand()` function from libc.
- `posix`
    - Enables `RandomSeedSource::UrandomDev` and `RandomSeedSource::RandomDev`
    - Allows creating a seed value by reading random bytes from `/dev/urandom` and `/dev/random`. These bytes are then used to create a `u128` integer.
    - ⚠️ Since `/dev/urandom` and `/dev/random` only exist on *nix based systems, such as Linux and macOS, you should disable this feature.

__By default both `libc` and `posix` are enabled.__
> The library should compile on Windows with default features, however using `RandomSeedSource::UrandomDev` and/or `RandomSeedSource::RandomDev` will crash your program!

## ⚠️ Warning
```
This library should not be used for cryptographic purposes!
```