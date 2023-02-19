//! A simple random number generator library.
//!
//! The random number generation is based on LFSR ([Linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register)).
//!
//! ```rust
//! use randlib::{Random, RandomSeedSource};
//!
//! fn main() {
//!     let src = RandomSeedSource::SystemTime;
//!     let mut rand = Random::new(src);
//!
//!     println!("i32:  {}", rand.rand_i32());
//!     println!("u32:  {}", rand.rand_u32());
//!     println!("bool: {}", rand.rand_bool());
//!     println!("f32:  {}", rand.rand_f32());
//! }
//! ```

#[cfg(feature = "libc")]
use libc::{rand, time};
#[cfg(feature = "posix")]
use std::{fs, io::Read};

/// Represents a seed value
type Seed = u128;
/// Size of the [`Seed`](Seed) type in bytes
const SEED_SIZE: usize = core::mem::size_of::<Seed>();
/// Size of the [`Seed`](Seed) type in bits
const SEED_SIZE_BITS: usize = SEED_SIZE * 8;

/// A random number generator.
pub struct Random {
    seed: Seed,
}

/// A source for a random seed.
pub enum RandomSeedSource {
    /// Manually assigned value
    Manual(Seed),

    /// Get a seed based on the current system time.  
    /// __Requires feature: *`libc`*__
    #[cfg(feature = "libc")]
    SystemTime,

    /// Get a random number using the [`rand()`](https://man7.org/linux/man-pages/man3/rand.3.html) function from libc.
    /// ### Warning
    /// Libc uses a constant value as a seed.  
    /// __Requires feature: *`libc`*__
    #[cfg(feature = "libc")]
    Crand,

    /// Create a seed value by reading bytes from `/dev/urandom`.
    /// ### Warning
    /// This only works on *nix-based systems such as Linux and macOS.  
    /// __Requires feature: *`posix`*__
    #[cfg(feature = "posix")]
    UrandomDev,

    /// Create a seed value by reading bytes from `/dev/random`.
    /// ### Warning
    /// This only works on *nix-based systems such as Linux and macOS.  
    /// __Requires feature: *`posix`*__
    #[cfg(feature = "posix")]
    RandomDev,
}

macro_rules! implement_unsigned {
    ($T: ty, $func_name: ident) => {
        pub fn $func_name(&mut self) -> $T {
            (self.random() % (<$T>::MAX as Seed)) as $T
        }
    };
}

macro_rules! implement_signed {
    ($T: ty, $func_name: ident) => {
        pub fn $func_name(&mut self) -> $T {
            let mut n = (self.random() % (<$T>::MAX as Seed)) as $T;
            if self.rand_bool() {
                n *= -1;
            }
            n
        }
    };
}

macro_rules! implement_floating {
    ($FT: ty, $UT: ty, $func_name: ident) => {
        /// Generate a random float from range `<0.0, 1.0>`.
        pub fn $func_name(&mut self) -> $FT {
            self.rand_u64() as $FT / <$UT>::MAX as $FT
        }
    };
}

/// ## Notes
/// 1. Unsigned number generation is faster.
/// 2. Methods for generating signed integers will rotate the seed twice.
///    The first rotation is to generate an unsigned number, and the second one
///    is used to generate a random boolean to determine whether the number
///    should be negative.
impl Random {
    /// Create a new Random generator.
    ///
    /// ### Note
    /// You can create multiple `Random`s in a single program.
    /// Just make sure they use different seeds so they won't generate the same numbers.
    pub fn new(seed_src: RandomSeedSource) -> Self {
        Self {
            seed: seed_src.get_seed(),
        }
    }

    /// Rotates the current seed and returns it.
    pub fn random(&mut self) -> Seed {
        self.rotate();
        self.seed
    }

    /// Alias to [`random()`](Self::random).
    pub fn rand_u128(&mut self) -> u128 {
        self.random()
    }

    /// Get a random boolean.
    pub fn rand_bool(&mut self) -> bool {
        (self.random() >> (SEED_SIZE_BITS - 1) & 1) == 1
    }

    implement_floating!(f64, u64, rand_f64);
    implement_floating!(f32, u32, rand_f32);

    implement_unsigned!(u8, rand_u8);
    implement_unsigned!(u16, rand_u16);
    implement_unsigned!(u32, rand_u32);
    implement_unsigned!(u64, rand_u64);

    implement_signed!(i8, rand_i8);
    implement_signed!(i16, rand_i16);
    implement_signed!(i32, rand_i32);
    implement_signed!(i64, rand_i64);
    implement_signed!(i128, rand_i128);

    fn rotate(&mut self) {
        let newbit = self.seed ^ (self.seed >> 1) ^ (self.seed >> 2) ^ (self.seed >> 7);
        self.seed = (self.seed >> 1) | (newbit << 127);
    }
}

impl RandomSeedSource {
    fn get_seed(&self) -> Seed {
        match self {
            RandomSeedSource::Manual(n) => *n,
            #[cfg(feature = "libc")]
            RandomSeedSource::SystemTime => self.get_system_time(),
            #[cfg(feature = "libc")]
            RandomSeedSource::Crand => self.crand(),
            #[cfg(feature = "posix")]
            RandomSeedSource::UrandomDev => self.read_from_randdev("urandom"),
            #[cfg(feature = "posix")]
            RandomSeedSource::RandomDev => self.read_from_randdev("random"),
        }
    }

    #[cfg(feature = "libc")]
    fn get_system_time(&self) -> Seed {
        unsafe { time(core::ptr::null_mut()) }.abs() as Seed
    }

    #[cfg(feature = "libc")]
    fn crand(&self) -> Seed {
        unsafe { rand() }.abs() as Seed
    }

    #[cfg(feature = "posix")]
    fn read_from_randdev(&self, dev: &str) -> Seed {
        let mut bytesbuf: [u8; SEED_SIZE] = [0; SEED_SIZE];
        let mut file = fs::File::open(format!("/dev/{dev}")).unwrap();
        file.read_exact(&mut bytesbuf).unwrap();

        if cfg!(target_endian = "big") {
            Seed::from_be_bytes(bytesbuf)
        } else {
            Seed::from_le_bytes(bytesbuf)
        }
    }
}
