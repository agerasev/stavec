#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod error;
/// Traits for [`GenericVec`] parameters.
///
/// You probably won't need them is you use only [`StaticVec`].
pub mod traits;

mod cmp;
mod default;
mod generic;
mod iter;
mod sized;
mod static_;
mod string;
mod utils;

#[cfg(test)]
mod tests;

pub use generic::GenericVec;
pub use iter::IntoIter;
pub use static_::StaticVec;
pub use string::GenericString;
