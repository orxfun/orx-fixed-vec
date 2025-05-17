#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod common_traits;
mod concurrent_iter;
mod concurrent_pinned_vec;
mod fixed_vec;
mod helpers;
mod into_concurrent_pinned_vec;
mod pinned_vec;

/// Common relevant traits, structs, enums.
pub mod prelude;

pub use concurrent_pinned_vec::ConcurrentFixedVec;
pub use fixed_vec::FixedVec;
pub use orx_iterable::{Collection, CollectionMut, Iterable};
pub use orx_pinned_vec::{
    ConcurrentPinnedVec, IntoConcurrentPinnedVec, PinnedVec, PinnedVecGrowthError,
};
