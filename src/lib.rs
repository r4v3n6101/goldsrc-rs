#![feature(new_uninit)]
#![cfg_attr(feature = "byteorder", feature(array_try_from_fn))]

#[macro_use]
extern crate static_assertions;

#[cfg(feature = "byteorder")]
pub mod byteorder;
#[cfg(feature = "nom")]
pub mod nom;
pub mod repr;
