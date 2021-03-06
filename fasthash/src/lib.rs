//! A suite of non-cryptographic hash functions for Rust.
//!
//! # Example
//!
//! ```rust
//! use std::hash::{Hash, Hasher};
//!
//! use fasthash::{metro, MetroHasher};
//!
//! fn hash<T: Hash>(t: &T) -> u64 {
//!     let mut s: MetroHasher = Default::default();
//!     t.hash(&mut s);
//!     s.finish()
//! }
//!
//! let h = metro::hash64(b"hello world\xff");
//!
//! assert_eq!(h, hash(&"hello world"));
//! ```
//!
//! By default, `HashMap` uses a hashing algorithm selected to
//! provide resistance against `HashDoS` attacks.
//! The hashing algorithm can be replaced on a per-`HashMap` basis
//! using the `HashMap::with_hasher` or
//! `HashMap::with_capacity_and_hasher` methods.
//!
//! It also cowork with `HashMap` or `HashSet`, act as a hash function
//!
//! ```rust
//! use std::collections::HashSet;
//!
//! use fasthash::spooky::Hash128;
//!
//! let mut set = HashSet::with_hasher(Hash128);
//! set.insert(2);
//! ```
//!
//! Or use `RandomState<CityHash64>` with a random seed.
#![warn(missing_docs)]

#[macro_use]
extern crate cfg_if;
extern crate lazy_static;
extern crate fasthash_sys as ffi;

cfg_if! {
    if #[cfg(feature = "digest")] {
        pub extern crate digest;

        pub use crate::hasher::Output;
    }
}

#[macro_use]
mod hasher;
pub mod farm;
pub mod highway;
pub mod lookup3;
pub mod metro;
pub mod mum;
pub mod murmur;
pub mod murmur2;
pub mod murmur3;
pub mod sea;
pub mod spooky;
pub mod xx;
pub mod xxh3;

pub use crate::hasher::{
    BufHasher, FastHash, FastHasher, Fingerprint, HasherExt, RandomState, Seed, StreamHasher,
};

pub use crate::farm::{Hasher128 as FarmHasherExt, Hasher64 as FarmHasher};
pub use crate::lookup3::Hasher32 as Lookup3Hasher;
pub use crate::mum::Hasher64 as MumHasher;
pub use crate::murmur::Hasher32 as MurmurHasher;
pub use crate::murmur3::Hasher32 as Murmur3Hasher;
#[doc(no_inline)]
pub use crate::sea::Hasher64 as SeaHasher;
pub use crate::spooky::{Hasher128 as SpookyHasherExt, Hasher64 as SpookyHasher};
pub use crate::xx::Hasher64 as XXHasher;
cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        pub use crate::murmur2::Hasher64_x64 as Murmur2Hasher;
        pub use crate::murmur3::Hasher128_x64 as Murmur3HasherExt;
    } else {
        pub use murmur2::Hasher64_x86 as Murmur2Hasher;
        pub use murmur3::Hasher128_x86 as Murmur3HasherExt;
    }
}
cfg_if! {
    if #[cfg(any(feature = "sse42", target_feature = "sse4.2"))] {
        pub use crate::metro::{crc::Hasher128_1 as MetroHasherExt, crc::Hasher64_1 as MetroHasher};
    } else {
        pub use metro::{Hasher128_1 as MetroHasherExt, Hasher64_1 as MetroHasher};
    }
}
