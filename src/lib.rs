extern crate nalgebra as na;
extern crate rust_mpfr as mpfr;
extern crate libc;

#[cfg(test)]
extern crate quickcheck;

pub use in_circle::*;

pub mod in_circle;
