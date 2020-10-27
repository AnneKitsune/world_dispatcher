#![feature(test)]

extern crate test;
use world_dispatcher::*;

use test::Bencher;

#[bench]
fn world_access(b: &mut Bencher) {
    #[derive(Default)]
    struct A(f32);
    let mut world = World::default();
    world.initialize::<A>();
    b.iter(|| {
        world.get_mut::<A>().unwrap();
    });
}

#[bench]
fn world_create_init(b: &mut Bencher) {
    #[derive(Default)]
    struct A(f32);
    b.iter(|| {
        let mut world = World::default();
        world.initialize::<A>();
    });
}
