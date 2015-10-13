extern crate libc;

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate rand;

pub use in_circle::*;
pub use geometry::*;

pub mod in_circle;
pub mod geometry;
