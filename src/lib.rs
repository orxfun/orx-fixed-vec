//! # orx-fixed-vec
//!
//! A fixed capacity vector with pinned elements.
//!
//! ## Motivation
//!
//! There might be various situations where pinned elements are helpful.
//!
//! * It is somehow required for async code. Not being an expert in the subject, leaving this [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) for the interested.
//! * It is a requirement to make self-referential types possible.
//!
//! This crate focuses more on the latter. Particularly, it aims to make it safely and conveniently possible to build **self-referential collections** such as linked list, tree or graph.
//!
//! See [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) for complete documentation.
//!
//! `FixedVec` is one of the pinned vec implementations which can be wrapped by an [`ImpVec`](https://crates.io/crates/orx-imp-vec) and allow building self referential collections.
//!
//! ## Comparison with `SplitVec`
//!
//! [`SplitVec`](https://crates.io/crates/orx-pinned-vec) is another `PinnedVec` implementation aiming the same goal but with different features. You may see the comparison in the table below.
//!
//! | **`FixedVec`**                                                               | **`SplitVec`**                                                                   |
//! |------------------------------------------------------------------------------|----------------------------------------------------------------------------------|
//! | Implements `PinnedVec`.                                                      | Implements `PinnedVec`.                                                          |
//! | Requires a capacity while creating.                                          | Capacity is optional.                                                            |
//! | Cannot grow beyond capacity; panics when `push` is called at capacity.       | Can grow dynamically. Further, it provides detailed control on how it must grow. |
//! | It is just a wrapper around `std::vec::Vec`; hence, has similar performance. | Performs additional tasks to provide flexibility; hence, slightly slower.        |
//!
//!
//! ## Pinned elements
//!
//! ```rust
//! use orx_fixed_vec::prelude::*;
//!
//! let mut vec = FixedVec::new(100);
//!
//! // push the first element
//! vec.push(42usize);
//! assert_eq!(vec, &[42]);
//!
//! // let's get a pointer to the first element
//! let addr42 = &vec[0] as *const usize;
//!
//! // let's push 99 new elements
//! for i in 1..100 {
//!     vec.push(i);
//! }
//!
//! for i in 0..100 {
//!     assert_eq!(if i == 0 { 42 } else { i }, vec[i]);
//! }
//!
//! // the memory location of the first element remains intact
//! assert_eq!(addr42, &vec[0] as *const usize);
//!
//! // we can safely (using unsafe!) dereference it and read the correct value
//! assert_eq!(unsafe { *addr42 }, 42);
//!
//! // the next push when `vec.is_full()` panics!
//! // vec.push(0);
//! ```
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

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

mod common_traits;
mod fixed_vec;
mod pinned_vec;
/// Common relevant traits, structs, enums.
pub mod prelude;

pub use fixed_vec::FixedVec;
