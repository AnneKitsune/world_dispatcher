use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use world_dispatcher::*;

#[derive(Default)]
struct A;
fn smol() {}
fn big(
    _a: &A,
    _b: &A,
    _c: &A,
    _d: &A,
    _e: &A,
    _f: &A,
    _g: &A,
    _h: &A,
    _i: &A,
    _j: &A,
    _k: &A,
    _l: &A,
) {
}

fn convert_system_fn_small(c: &mut Criterion) {
    c.bench_function("convert_system_fn_small", |b| {
        b.iter(|| {
            black_box(smol.system());
        })
    });
}

fn convert_system_fn_big(c: &mut Criterion) {
    c.bench_function("convert_system_fn_big", |b| {
        b.iter(|| {
            black_box(big.system());
        })
    });
}

fn system_run_big(c: &mut Criterion) {
    let mut world = World::default();
    let mut sys = big.system();
    sys.initialize(&mut world);

    c.bench_function("system_run_big", |b| {
        b.iter(|| {
            black_box(sys.run(&world));
        })
    });
}

fn system_run_small(c: &mut Criterion) {
    let mut world = World::default();
    let mut sys = smol.system();
    sys.initialize(&mut world);

    c.bench_function("system_run_small", |b| {
        b.iter(|| {
            black_box(sys.run(&world));
        })
    });
}

criterion_group!(
    benches,
    convert_system_fn_small,
    convert_system_fn_big,
    system_run_big,
    system_run_small
);
criterion_main!(benches);
