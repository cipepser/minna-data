#[macro_use]
extern crate criterion;

use criterion::Criterion;
use q2_1::array_stack_kai::ArrayStack;
use std::iter::repeat;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("addAll benchmarks(collection=1000)");
    let mut a: ArrayStack<char> = ArrayStack::with_capacity(1000);
    let collection = repeat('a')
        .take(1000)
        .collect::<Vec<char>>();
    let mut ak: ArrayStack<char> = ArrayStack::with_capacity(1000);

    group.bench_function("Benchmark array_stack", |b| b.iter(|| {
        for i in 0..collection.len() {
            a._add(i, collection[i].clone());
        }
    }));
    group.bench_function("Benchmark array_stack_kai", |b| b.iter(|| {
        ak.add_all(0, &collection);
    }));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);