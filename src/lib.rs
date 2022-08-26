#![feature(new_uninit)]
#![cfg_attr(feature = "byteorder", feature(array_try_from_fn))]
#![cfg_attr(feature = "bytes", feature(array_try_from_fn))]

#[macro_use]
extern crate static_assertions;

pub use smol_str::SmolStr;

#[cfg(feature = "byteorder")]
pub mod byteorder;
#[cfg(feature = "bytes")]
pub mod bytes;
#[cfg(feature = "nom")]
pub mod nom;

pub mod repr;
