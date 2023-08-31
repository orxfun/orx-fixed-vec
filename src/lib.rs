//! `FixedVec` implements [`PinnedVec`](https://crates.io/crates/orx-pinned-vec); therefore,
//!
//! * it preserves the memory locations of already pushed elements.
//!
//! This feature eliminates a specific set of errors leading to undefined behavior,
//! and hence, allows to work with a more flexible borrow checker.
//!
//! Furthermore, it can be used as the underlying pinned vector of an
//! [`ImpVec`](https://crates.io/crates/orx-imp-vec), which adds the additional feature
//! to push to the vector with an immutable reference.
//!
//! Unlike, another pinned vector implementation [`SplitVec`](https://crates.io/crates/orx-split-vec),
//! `FixedVec` allows operations with same complexity and speed of `std::vec::Vec`.
//! Its drawback, on the other hand, is that:
//! * it has a hard limit on its capacity,
//! * it will panic if the caller attempts to extend the vector beyond this limit.
//!
//! ```rust
//! use orx_fixed_vec::prelude::*;
//!
//! let fixed_capacity = 42;
//!
//! let mut vec = FixedVec::new(fixed_capacity);
//! assert_eq!(fixed_capacity, vec.capacity());
//!
//! let mut initial_addresses = vec![];
//! for i in 0..fixed_capacity {
//!     vec.push(i);
//!     initial_addresses.push(vec.get(i).unwrap() as *const usize);
//! }
//!
//! assert_eq!(fixed_capacity, vec.len());
//! assert_eq!(0, vec.room());
//! assert!(vec.is_full());
//!
//! // addresses of already pushed elements stay intact
//! let final_addresses: Vec<_> = (0..fixed_capacity)
//!     .map(|i| vec.get(i).unwrap() as *const usize)
//!     .collect();
//! assert_eq!(initial_addresses, final_addresses);
//!
//! // the next push when `vec.is_full()` panics!
//! // vec.push(42);
//! ```

mod common_traits;
mod fixed_vec;
mod pinned_vec;
/// Common relevant traits, structs, enums.
pub mod prelude;

pub use fixed_vec::FixedVec;
