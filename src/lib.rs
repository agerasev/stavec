#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod cmp;
mod generic;
mod iter;
mod static_;
mod traits;
mod utils;

#[cfg(test)]
mod tests;

pub use generic::GenericVec;
pub use iter::IntoIter;
pub use static_::StaticVec;
