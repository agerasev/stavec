#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod cmp;
mod container;
mod generic;
mod iter;
mod static_;

#[cfg(test)]
mod tests;

pub use container::{Container, SizedContainer};
pub use generic::GenericVec;
pub use iter::IntoIter;
pub use static_::StaticVec;
