#[macro_use]
extern crate criterion;

use criterion::{Criterion, black_box};
use q1_7::MyList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut list = MyList::new();
    c.bench_function("benchmark before: add", |b| b.iter(
        || list._add(0, 10)
    ));
    c.bench_function("benchmark before: add + remove", |b| b.iter(
        || {
            list._add(0, 10);
            list._remove(0);
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);