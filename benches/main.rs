use boxy::{BoxedCallback, BoxedEmpty, Callback, Empty};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn resolve(c: &mut Criterion) {
    let mut g = c.benchmark_group("resolve");

    g.bench_function("double", |b| {
        b.iter(|| {
            let mut ptr = Box::new(Box::new(2_u32) as Box<dyn Empty>);
            let ptr = (&mut *ptr) as *mut Box<dyn Empty>;

            let res = boxy::resolve_double(black_box(ptr.cast()));
            assert_eq!(4, res);
        })
    });

    g.bench_function("typed", |b| {
        b.iter(|| {
            let mut ptr = Box::new(2_u32);
            let ptr = (&mut *ptr) as *mut u32;

            let res = boxy::resolve_typed::<u32>(black_box(ptr.cast()));
            assert_eq!(4, res);
        })
    });

    g.bench_function("thin", |b| {
        b.iter(|| {
            let ptr = BoxedEmpty::new(2_u32);

            let res = boxy::resolve_thin(black_box(ptr.as_raw().cast()));
            assert_eq!(4, res);
        })
    });
}

fn callback(c: &mut Criterion) {
    let mut g = c.benchmark_group("callback");

    g.bench_function("double", |b| {
        b.iter(|| {
            let mut ptr = Box::new(Box::new(|value: u32| value + value) as Box<dyn Callback>);
            let ptr = (&mut *ptr) as *mut Box<dyn Callback>;

            let res = boxy::callback_double(black_box(ptr.cast()), black_box(2));
            assert_eq!(4, res);
        })
    });

    g.bench_function("thin", |b| {
        b.iter(|| {
            let ptr = BoxedCallback::new(|value: u32| value + value);

            let res = boxy::callback_thin(black_box(ptr.as_raw().cast()), black_box(2));
            assert_eq!(4, res);
        })
    });
}

criterion_group!(benches, resolve, callback);
criterion_main!(benches);
