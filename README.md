# orx-fixed-vec

[![orx-fixed-vec crate](https://img.shields.io/crates/v/orx-fixed-vec.svg)](https://crates.io/crates/orx-fixed-vec)
[![orx-fixed-vec documentation](https://docs.rs/orx-fixed-vec/badge.svg)](https://docs.rs/orx-fixed-vec)

An efficient constant access time vector with fixed capacity and pinned elements. 

## A. Motivation

There are various situations where pinned elements are necessary.

* It is critical in enabling **efficient, convenient and safe self-referential collections** with thin references, see [`SelfRefCol`](https://crates.io/crates/orx-selfref-col) for details.
* It is essential in allowing an **immutable push** vector; i.e., [`ImpVec`](https://crates.io/crates/orx-imp-vec). This is a very useful operation when the desired collection is a bag or a container of things, rather than having a collective meaning. In such cases, `ImpVec` avoids heap allocations and wide pointers such as `Box` or `Rc` or etc.
* It is important for **async** code; following [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) could be useful for the interested.

*As explained in [rust-docs](https://doc.rust-lang.org/std/pin/index.html), there exist `Pin` and `Unpin` types for similar purposes. However, the solution is complicated and low level using `PhantomPinned`, `NonNull`, `dangling`, `Box::pin`, pointer accesses, etc.*

## B. Comparison with `SplitVec`

[`SplitVec`](https://crates.io/crates/orx-split-vec) is another [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) implementation aiming the same goal but with different features. You may see the comparison in the table below.

| **`FixedVec`**                                                               | **`SplitVec`**                                                                   |
|------------------------------------------------------------------------------|----------------------------------------------------------------------------------|
| Implements `PinnedVec` => can be wrapped by an `ImpVec` or `SelfRefCol`.     | Implements `PinnedVec` => can be wrapped by an `ImpVec` or `SelfRefCol`.         |
| Requires exact capacity to be known while creating.                          | Can be created with any level of prior information about required capacity.      |
| Cannot grow beyond capacity; panics when `push` is called at capacity.       | Can grow dynamically. Further, it provides control on how it must grow. |
| It is just a wrapper around `std::vec::Vec`; hence, has equivalent performance. | Performance-optimized built-in growth strategies also have `std::vec::Vec` equivalent performance. |

After the performance optimizations on the `SplitVec`, it is now comparable to `std::vec::Vec` in terms of performance. This might make `SplitVec` a dominating choice over `FixedVec`.

## C. Examples

### C.1. Usage similar to `std::vec::Vec`

Most common `std::vec::Vec` operations are available in `FixedVec` with the same signature.

```rust
use orx_fixed_vec::*;

// capacity is not optional
let mut vec = FixedVec::new(4);

assert_eq!(4, vec.capacity());

vec.push(0);
assert!(!vec.is_full());
assert_eq!(3, vec.room());

vec.extend_from_slice(&[1, 2, 3]);
assert_eq!(vec, &[0, 1, 2, 3]);
assert!(vec.is_full());

// vec.push(42); // push would've panicked when vec.is_full()

vec[0] = 10;
assert_eq!(10, vec[0]);

vec.remove(0);
vec.insert(0, 0);

assert_eq!(6, vec.iter().sum());

assert_eq!(vec.clone(), vec);

let std_vec: Vec<_> = vec.into();
assert_eq!(&std_vec, &[0, 1, 2, 3]);
```

### C.2. Pinned Elements

Unless elements are removed from the vector, the memory location of an element already pushed to the `SplitVec` <ins>never</ins> changes unless explicitly changed.

```rust
use orx_fixed_vec::*;

let mut vec = FixedVec::new(100);

// push the first element
vec.push(42usize);
assert_eq!(vec, &[42]);

// let's get a pointer to the first element
let addr42 = &vec[0] as *const usize;

// let's push 99 new elements
for i in 1..100 {
    vec.push(i);
}

for i in 0..100 {
    assert_eq!(if i == 0 { 42 } else { i }, vec[i]);
}

// the memory location of the first element remains intact
assert_eq!(addr42, &vec[0] as *const usize);

// we can safely dereference it and read the correct value
// dereferencing is still unsafe for FixedVec,
// but the underlying guarantee will be used by wrappers such as ImpVec or SelfRefCol
assert_eq!(unsafe { *addr42 }, 42);

// the next push when `vec.is_full()` panics!
// vec.push(0);
```

## D. Benchmarks

Since `FixedVec` is just a wrapper around the `std::vec::Vec` with additional pinned element guarantee; it is expected to have equivalent performance. This is tested and confirmed by benchmarks that can be found at the at [benches](https://github.com/orxfun/orx-fixed-vec/blob/main/benches) folder.


## License

This library is licensed under MIT license. See LICENSE for details.
