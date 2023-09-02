# orx-fixed-vec

A fixed vector, `FixedVec`, is a vector with a strict predetermined capacity
(see [`SplitVec`](https://crates.io/crates/orx-split-vec) for dynamic capacity version).

It provides the following features:

* It provides operations with the same complexity and speed as the standard vector.
* It makes sure that the data stays **pinned** in place.
    * `FixedVec<T>` implements [`PinnedVec<T>`](https://crates.io/crates/orx-pinned-vec) for any `T`;
    * `FixedVec<T>` implements `PinnedVecSimple<T>` for `T: NotSelfRefVecItem`;
    * Memory location of an item added to the fixed vector will never change
    unless the vector is dropped or cleared.
    * This allows the fixed vec to be converted into an [`ImpVec`](https://crates.io/crates/orx-imp-vec)
    to enable immutable-push operations which allows for 
    convenient, efficient and safe implementations of self-referencing data structures.

## Pinned elements

```rust
use orx_fixed_vec::prelude::*;

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

// we can safely (using unsafe!) dereference it and read the correct value
assert_eq!(unsafe { *addr42 }, 42);

// the next push when `vec.is_full()` panics!
// vec.push(0);
```

## Vector with self referencing elements

`FixedVec` is not meant to be a replacement for `std::vec::Vec`.

However, it is useful and convenient in defining data structures, child structures of which
hold references to each other.
This is a very common and useful property for trees, graphs, etc.
SplitVec allows to store children of such structures in a vector with the following features:

* holding children close to each other allows for better cache locality,
* reduces heap allocations and utilizes **thin** references rather than wide pointers,
* while still guaranteeing that the references will remain valid.

`FixedVec` helps this goal as follows:

* `FixedVec` implements `PinnedVec`; and hence, it can be wrapped by an `ImpVec`,
* `ImpVec` allows safely building the vector where items are referencing each other,
* `ImpVec` can then be converted back to the underlying `FixedVec`
having the abovementioned features and safety guarantees.
