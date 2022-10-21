#![no_std]

#[cfg(feature = "std")]
extern crate std;

/// Traits for [`GenericVec`] paramenters.
///
/// You probably won't need them is you use only [`StaticVec`].
pub mod traits;

mod cmp;
mod generic;
mod iter;
mod static_;
mod utils;

#[cfg(test)]
mod tests;

pub use generic::GenericVec;
pub use iter::IntoIter;
pub use static_::StaticVec;
