#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate libc;

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate rand;

pub use in_circle::*;
pub use geometry::*;

pub mod in_circle;
pub mod geometry;
pub mod delaunay;
