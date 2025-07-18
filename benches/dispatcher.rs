use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use world_dispatcher::*;

#[derive(Default)]
struct A;
#[derive(Default)]
struct B;

// Stage 1
fn sys1(_a: &A, _b: &B) {}
fn sys2(_a: &A, _b: &B) {}
// Stage 2
fn sys3(_a: &A, _b: &mut B) {}
// Stage 3
fn sys4(_a: &A, _b: &mut B) {}

fn init_world() -> World {
    let mut world = World::default();
    world.initialize::<A>();
    world.initialize::<B>();
    world
}

fn build_dispatcher(c: &mut Criterion) {
    let mut world = init_world();
    c.bench_function("dispatcher_build", |b| {
        b.iter(|| {
            black_box(
                DispatcherBuilder::new()
                    .add(sys1)
                    .add(sys2)
                    .add(sys3)
                    .add(sys4)
                    .build(&mut world),
            );
        })
    });
}

fn run_dispatcher(c: &mut Criterion) {
    let mut world = init_world();
    let mut dispatch = DispatcherBuilder::new()
        .add(sys1)
        .add(sys2)
        .add(sys3)
        .add(sys4)
        .build(&mut world);

    c.bench_function("dispatcher_run", |b| {
        b.iter(|| {
            black_box(dispatch.run_seq(&world));
        })
    });
}

criterion_group!(benches, build_dispatcher, run_dispatcher);
criterion_main!(benches);
