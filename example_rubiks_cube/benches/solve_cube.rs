use std::array;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use example_rubiks_cube::{cube_with_random_moves, solve_cube};
use rand::{rngs, SeedableRng};

pub fn criterion_benchmark(c: &mut Criterion) {
    static NUM_CUBES: usize = 10;
    static MOVE_COUNT: u8 = 10;

    let mut rng = rngs::StdRng::seed_from_u64(1337);
    let cubes: [_; NUM_CUBES] = array::from_fn(|i| {
        (BenchmarkId::from_parameter(i), cube_with_random_moves(&mut rng, MOVE_COUNT))
    });

    let mut group = c.benchmark_group(format!("solve_{NUM_CUBES}_cubes_{MOVE_COUNT}_moves"));

    for (id, cube) in cubes {
        group.bench_with_input(id, &cube, |b, c| {
            b.iter(|| solve_cube(c));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);