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

fn std_vec_new<T, F: Fn(usize) -> T>(n: usize, value: F) {
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(value(i))
    }
}
fn std_vec_with_capacity<T, F: Fn(usize) -> T>(n: usize, value: F) {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        vec.push(value(i))
    }
}
fn fixed_vec<T, F: Fn(usize) -> T>(n: usize, value: F) {
    let mut vec = FixedVec::new(n);
    for i in 0..n {
        vec.push(value(i))
    }
}

fn test_for_type<T>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_u64s: usize,
    treatments: &[usize],
    value: fn(usize) -> T,
) {
    for n in treatments {
        let treatment = format!("n={},elem-type=[u64;{}]", n, num_u64s);

        group.bench_with_input(
            BenchmarkId::new("std_vec_with_capacity", &treatment),
            n,
            |b, _| b.iter(|| std_vec_with_capacity(black_box(*n), value)),
        );

        group.bench_with_input(BenchmarkId::new("std_vec_new", &treatment), n, |b, _| {
            b.iter(|| std_vec_new(black_box(*n), value))
        });

        group.bench_with_input(BenchmarkId::new("fixed_vec", &treatment), n, |b, _| {
            b.iter(|| fixed_vec(black_box(*n), value))
        });
    }
}

fn bench_grow(c: &mut Criterion) {
    let treatments = vec![1_024, 16_384, 262_144, 4_194_304];

    let mut group = c.benchmark_group("grow");

    const N: usize = 16;

    test_for_type(&mut group, N, &treatments, get_value::<N>);

    group.finish();
}

criterion_group!(benches, bench_grow);
criterion_main!(benches);
