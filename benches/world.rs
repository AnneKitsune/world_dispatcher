use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use world_dispatcher::*;

fn world_access(c: &mut Criterion) {
    #[derive(Default)]
    struct A;
    let mut world = World::default();
    world.initialize::<A>();

    c.bench_function("world_access", |b| {
        b.iter(|| {
            black_box(world.get_mut::<A>());
        })
    });
}

fn world_create_init(c: &mut Criterion) {
    #[derive(Default)]
    struct A;
    c.bench_function("world_create_init", |b| {
        b.iter(|| {
            let mut world = World::default();
            world.initialize::<A>();
            black_box(world);
        })
    });
}

criterion_group!(benches, world_access, world_create_init);
criterion_main!(benches);
