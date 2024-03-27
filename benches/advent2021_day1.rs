use day_01::part1::increase_count;
use day_01::part2::increase_count_window;
use day_01::file_reader::read_from_file;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn part1_bench(c: &mut Criterion) {

    let input = black_box(read_from_file("./input_part1.txt"));

    c.bench_function("bench_part_1", |b| {
        b.iter(|| increase_count(&input));
    });
}

fn part2_bench(c: &mut Criterion) {

    let input = black_box(read_from_file("./input_part2.txt"));

    c.bench_function("bench_part_2", |b| {
        b.iter(|| increase_count_window(&input));
    });
}

criterion_group!(benches, part1_bench, part2_bench);

criterion_main!(benches);
