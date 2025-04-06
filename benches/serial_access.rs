use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
    measurement::WallTime,
};
use orx_fixed_vec::prelude::*;

fn get_value<const N: usize>(i: usize) -> [u64; N] {
    let modulo = i % 3;
    if modulo == 0 {
        [i as u64; N]
    } else if modulo == 1 {
        [(i + 1) as u64; N]
    } else {
        [(i + 2) as u64; N]
    }
}
fn add<const N: usize>(a: [u64; N], b: &[u64; N]) -> [u64; N] {
    let mut sum = [0u64; N];
    for i in 0..N {
        sum[i] = a[i] + b[i];
    }
    sum
}

fn std_vec_with_capacity<T, F: Fn(usize) -> T>(n: usize, value: F) -> Vec<T> {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        vec.push(value(i))
    }
    vec
}
fn fixed_vec<T, F: Fn(usize) -> T>(n: usize, value: F) -> FixedVec<T> {
    let mut vec = FixedVec::new(n);
    for i in 0..n {
        vec.push(value(i))
    }
    vec
}

fn calc<T: Default, F: Fn(T, &T) -> T>(add: F, vec: &[T]) -> T {
    let mut sum = T::default();
    for x in vec {
        sum = add(sum, x);
    }
    sum
}

fn test_for_type<T: Default + PartialEq + std::fmt::Debug>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_u64s: usize,
    treatments: &[usize],
    value: fn(usize) -> T,
    add: fn(T, &T) -> T,
) {
    for n in treatments {
        let treatment = format!("n={},elem-type=[u64;{}]", n, num_u64s);

        group.bench_with_input(BenchmarkId::new("std_vec", &treatment), n, |b, _| {
            let std_vec = std_vec_with_capacity(black_box(*n), value);
            b.iter(|| calc(black_box(add), black_box(&std_vec)))
        });

        group.bench_with_input(BenchmarkId::new("fixed_vec", &treatment), n, |b, _| {
            let fixed_vec = fixed_vec(black_box(*n), value);
            b.iter(|| calc(black_box(add), black_box(&fixed_vec)))
        });
    }
}

fn bench(c: &mut Criterion) {
    let treatments = vec![1_024, 16_384, 262_144, 4_194_304];

    let mut group = c.benchmark_group("serial_access");

    const N: usize = 16;
    test_for_type::<[u64; N]>(&mut group, N, &treatments, get_value, add);

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
