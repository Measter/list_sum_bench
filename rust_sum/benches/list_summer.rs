use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use criterion::{criterion_group, criterion_main, Criterion};

fn sum_for(numbers: &[i32]) -> i32 {
    let mut sum = 0;

    for i in 0..numbers.len() {
        sum += numbers[i];
    }

    sum
}

fn sum_foreach(numbers: &[i32]) -> i32 {
    let mut sum = 0;

    for &num in numbers {
        sum += num;
    }

    sum
}

fn sum_iter(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

pub fn list_summer(c: &mut Criterion) {
    let file = File::open("../generator/numbers.txt").unwrap();
    let file = BufReader::new(file);
    let numbers: Vec<i32> = file
        .lines()
        .map(|l| l.unwrap().parse())
        .collect::<Result<_, _>>()
        .unwrap();

    c.bench_function("sum_for", |b| b.iter(|| sum_for(&numbers)));
    c.bench_function("sum_foreach", |b| b.iter(|| sum_foreach(&numbers)));
    c.bench_function("sum_iter", |b| b.iter(|| sum_iter(&numbers)));
}

criterion_group!(benches, list_summer);
criterion_main!(benches);
