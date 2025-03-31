use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use example_rubiks_cube::{solve_cube, RubiksCube};



pub fn criterion_benchmark(c: &mut Criterion) {
    let cube_13_moves = RubiksCube::from_unvalidated_raw_colors(&[
        [4, 0, 3, 5, 0, 3, 5, 2, 5],
        [3, 1, 1, 4, 1, 0, 4, 2, 5],
        [4, 3, 4, 4, 2, 5, 2, 5, 2],
        [3, 0, 0, 3, 3, 4, 1, 1, 5],
        [2, 1, 0, 3, 4, 5, 1, 0, 1],
        [3, 2, 0, 1, 5, 4, 0, 2, 2]
    ]).unwrap();

    let cube_10_moves = RubiksCube::from_unvalidated_raw_colors(&[
        [2, 2, 2, 0, 0, 3, 0, 0, 4],
        [5, 3, 1, 1, 1, 1, 1, 2, 2],
        [2, 2, 1, 5, 2, 1, 0, 2, 5],
        [0, 4, 5, 0, 3, 0, 3, 3, 3],
        [3, 1, 1, 4, 4, 4, 0, 4, 4],
        [3, 5, 4, 3, 5, 5, 5, 5, 4]
    ]).unwrap();

    let cube_5_moves = RubiksCube::from_unvalidated_raw_colors(&[
        [0, 0, 3, 0, 0, 3, 3, 0, 3],
        [1, 1, 2, 2, 1, 1, 2, 1, 1],
        [5, 3, 5, 2, 2, 0, 2, 2, 0],
        [4, 4, 4, 2, 3, 5, 2, 3, 5],
        [0, 4, 4, 4, 4, 3, 4, 4, 3],
        [5, 5, 1, 5, 5, 5, 0, 1, 1]
    ]).unwrap();

    let mut group = c.benchmark_group("solve_cube");

    let cubes = vec![
        // (BenchmarkId::from_parameter("5 moves"), cube_5_moves),
        (BenchmarkId::from_parameter("10 moves"), cube_10_moves),
        // (BenchmarkId::from_parameter("13 moves"), cube_13_moves),
    ];

    for (id, cube) in cubes {
        group.bench_with_input(id, &cube, |b, c| {
            b.iter(|| solve_cube(c));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);