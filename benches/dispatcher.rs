#![feature(test)]

extern crate test;
use world_dispatcher::*;

use test::Bencher;

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

#[bench]
fn build_dispatcher(b: &mut Bencher) {
    let mut world = init_world();
    b.iter(|| {
        let _ = DispatcherBuilder::new()
            .add(sys1)
            .add(sys2)
            .add(sys3)
            .add(sys4)
            .build(&mut world);
    });
}

#[bench]
fn run_dispatcher(b: &mut Bencher) {
    let mut world = init_world();
    let mut dispatch = DispatcherBuilder::new()
        .add(sys1)
        .add(sys2)
        .add(sys3)
        .add(sys4)
        .build(&mut world);
    b.iter(|| {
        dispatch.run_seq(&world);
    });
}
