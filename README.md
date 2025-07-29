# orx-fixed-vec

[![orx-fixed-vec crate](https://img.shields.io/crates/v/orx-fixed-vec.svg)](https://crates.io/crates/orx-fixed-vec)
[![orx-fixed-vec crate](https://img.shields.io/crates/d/orx-fixed-vec.svg)](https://crates.io/crates/orx-fixed-vec)
[![orx-fixed-vec documentation](https://docs.rs/orx-fixed-vec/badge.svg)](https://docs.rs/orx-fixed-vec)

An efficient fixed capacity vector with pinned element guarantees.

A **FixedVec** implements [`PinnedVec`](https://crates.io/crates/orx-pinned-vec); you may read the detailed information about [pinned element guarantees](https://docs.rs/orx-pinned-vec/latest/orx_pinned_vec/#pinned-elements-guarantees) and why they are useful in the [motivation-and-examples](https://docs.rs/orx-pinned-vec/latest/orx_pinned_vec/#motivation--examples) section. In brief, a pinned vector does not allow implicit changes in memory locations of its elements; such as moving the entire vector to another memory location due to additional capacity requirement.

> This is **no-std** crate.

## Fixed Vector

A `FixedVec` is simply a wrapper around the standard vector with the following two key differences:
* It is always created with an initial fixed capacity which cannot implicitly change.
* If we add more elements than the fixed capacity, the vector panics.

This leads to the following properties:
* Its implementation is uninteresting as it does nothing but exposes standard vector methods.
* For the very same reason, it is as performant as the standard vector; details can be found at the [benches](https://github.com/orxfun/orx-fixed-vec/blob/main/benches) folder.
* It satisfies the pinned element guarantees, and hence, it implements `PinnedVec` unlike the standard vector.

Using a fixed capacity vector has limited use cases as this information is usually not available in situations where we use a vector. Therefore, we more frequently use the [`SplitVec`](https://crates.io/crates/orx-split-vec) as a dynamic capacity vector with pinned element guarantees. `FixedVec`, on the other hand, must be used only when we have the perfect knowledge on the upper bound of vector length.

In order to illustrate, consider an operation where we compute **n** outputs from **n** inputs; i.e., we map each element to a new element. Further, we want to collect or write the results in a new vector. In this case, we could safely use a `FixedVec` created with a capacity of **n** elements. This is exactly the parallel iterator [`Par`](https://crates.io/crates/orx-parallel) does under the hood when the length of the output is known with certainty. In other situations, `SplitVec` is used as the pinned vector.

## Parallelization

`FixedVec` implements [`ConcurrentCollection`](https://docs.rs/orx-concurrent-iter/latest/orx_concurrent_iter/trait.ConcurrentCollection.html).

Therefore, when [orx_parallel](https://crates.io/crates/orx-parallel) crate is included, `FixedVec` also automatically implements [`ParallelizableCollection`](https://docs.rs/orx-parallel/latest/orx_parallel/trait.ParallelizableCollection.html).

This means that computations over the fixed vector can be efficiently parallelized:

* `fixed_vec.par()` returns a parallel iterator over references to its elements, and
* `fixed_vec.into_par()` consumes the vector and returns a parallel iterator of the owned elements.

You may find demonstrations in [`demo_parallelization`](https://github.com/orxfun/orx-fixed-vec/blob/main/examples/demo_parallelization) and [`bench_parallelization`](https://github.com/orxfun/orx-fixed-vec/blob/main/examples/bench_parallelization) examples.

## Examples

FixedVec api resembles and aims to cover as much as possible the standard vector's api.

```rust
use orx_fixed_vec::prelude::*;

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

assert_eq!(&vec, &[0, 1, 2, 3]);

// allows zero-cost conversion into and from std Vec
let _std_vec: Vec<_> = vec.into();
```

Its main difference and objective is to provide pinned element guarantees as demonstrated in the example below.

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

// we can safely dereference it and read the correct value
// dereferencing is still unsafe for FixedVec,
// but the underlying guarantee will be used by wrappers such as ImpVec or SelfRefCol
assert_eq!(unsafe { *addr42 }, 42);

// the next push when `vec.is_full()` panics!
// vec.push(0);
```

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-fixed-vec/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
