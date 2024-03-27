use day_02::parser;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn part1_bench(c: &mut Criterion) {

    let input = black_box("./input_part1.txt");

    c.bench_function("bench_part_1", |b| {
        
        b.iter(|| {
            let mut submarine = parser::parse_submarine(input).unwrap_or_default();
            submarine.start_trip_part1();
            submarine.get_coordinate();
        });
    });
}

fn part2_bench(c: &mut Criterion) {
    let input = black_box("./input_part2.txt");

    c.bench_function("bench_part_1", |b| {
        
        b.iter(|| {
            let mut submarine = parser::parse_submarine(input).unwrap_or_default();
            submarine.start_trip_part2();
            submarine.get_coordinate();
        });
    });
}

criterion_group!(benches, part1_bench, part2_bench);

criterion_main!(benches);
