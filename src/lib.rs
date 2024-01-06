//! # orx-fixed-vec
//!
//! An efficient constant access time vector with fixed capacity and pinned elements.
//!
//! ## A. Motivation
//!
//! There might be various situations where pinned elements are helpful.
//!
//! * It is somehow required for async code, following [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) could be useful for the interested.
//! * It is crucial in representing self-referential types with thin references.
//!
//! This crate focuses on the latter. Particularly, it aims to make it safe and convenient to build **performant self-referential collections** such as linked lists, trees or graphs. See [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) for complete documentation on the motivation.
//!
//! `FixedVec` is one of the pinned vec implementations which can be wrapped by an [`ImpVec`](https://crates.io/crates/orx-imp-vec) and allow building self referential collections.
//!
//! ## B. Comparison with `SplitVec`
//!
//! [`SplitVec`](https://crates.io/crates/orx-split-vec) is another `PinnedVec` implementation aiming the same goal but with different features. You may see the comparison in the table below.
//!
//! | **`FixedVec`**                                                               | **`SplitVec`**                                                                   |
//! |------------------------------------------------------------------------------|----------------------------------------------------------------------------------|
//! | Implements `PinnedVec` => can be wrapped by an `ImpVec`.                     | Implements `PinnedVec` => can be wrapped by an `ImpVec`.                         |
//! | Requires exact capacity to be known while creating.                          | Can be created with any level of prior information about required capacity.      |
//! | Cannot grow beyond capacity; panics when `push` is called at capacity.       | Can grow dynamically. Further, it provides control on how it must grow. |
//! | It is just a wrapper around `std::vec::Vec`; hence, has equivalent performance. | Performance-optimized built-in growth strategies also have `std::vec::Vec` equivalent performance. |
//!
//! After the performance optimizations on the `SplitVec`, it is now comparable to `std::vec::Vec` in terms of performance. This might make `SplitVec` a dominating choice over `FixedVec`.
//!
//! ## C. Examples
//!
//! ### C.1. Usage similar to `std::vec::Vec`
//!
//! Most common `std::vec::Vec` operations are available in `FixedVec` with the same signature.
//!
//! ```rust
//! use orx_fixed_vec::prelude::*;
//!
//! // capacity is not optional
//! let mut vec = FixedVec::new(4);
//!
//! assert_eq!(4, vec.capacity());
//!
//! vec.push(0);
//! assert!(!vec.is_full());
//! assert_eq!(3, vec.room());
//!
//! vec.extend_from_slice(&[1, 2, 3]);
//! assert_eq!(vec, &[0, 1, 2, 3]);
//! assert!(vec.is_full());
//!
//! // vec.push(42); // push would've panicked when vec.is_full()
//!
//! vec[0] = 10;
//! assert_eq!(10, vec[0]);
//!
//! vec.remove(0);
//! vec.insert(0, 0);
//!
//! assert_eq!(6, vec.iter().sum());
//!
//! assert_eq!(vec.clone(), vec);
//!
//! let stdvec: Vec<_> = vec.into();
//! assert_eq!(&stdvec, &[0, 1, 2, 3]);
//! ```
//!
//!
//! ### C.2. Pinned Elements
//!
//! Unless elements are removed from the vector, the memory location of an element priorly pushed to the `FixedVec` <ins>never</ins> changes. This guarantee is utilized by `ImpVec` in enabling immutable growth to build self referential collections.
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
//! // we can safely dereference it and read the correct value
//! // the method is still unsafe for FixedVec
//! // but the undelrying guarantee will be used by ImpVec
//! assert_eq!(unsafe { *addr42 }, 42);
//!
//! // the next push when `vec.is_full()` panics!
//! // vec.push(0);
//! ```
//!
//! ## D. Relation to the `ImpVec`
//!
//! Providing pinned memory location elements with `PinnedVec` is the first block for building self referential structures; the second building block is the [`ImpVec`](https://crates.io/crates/orx-imp-vec). An `ImpVec` wraps any `PinnedVec` implementation and provides specialized methods built on the pinned element guarantee in order to allow building self referential collections.
//!
//! ## E. Benchmarks
//!
//! Since `FixedVec` is just a wrapper around the `std::vec::Vec` with additional pinned element guarantee; it is expected to have equivalent performance. This is tested and confirmed by benchmarks that can be found at the at [benches](https://github.com/orxfun/orx-fixed-vec/blob/main/benches) folder.
//!
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
