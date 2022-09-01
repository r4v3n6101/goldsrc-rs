#![feature(new_uninit)]
#![cfg_attr(feature = "byteorder", feature(array_try_from_fn))]

#[macro_use]
extern crate static_assertions;

pub use smol_str::SmolStr;

pub use parser::*;
pub use repr::*;

mod parser;
mod repr;
