#![feature(test)]

extern crate test;
use world_dispatcher::*;

use test::Bencher;

#[derive(Default)]
struct A;
fn smol() -> SystemResult {Ok(())}
fn big(_a: &A, _b: &A, _c: &A, _d: &A, _e: &A, _f: &A, _g: &A, _h: &A, _i: &A, _j: &A, _k: &A, _l: &A) -> SystemResult {Ok(())}

#[bench]
fn convert_system_fn_small(b: &mut Bencher) {
    b.iter(|| {
        let _ = smol.system();
    });
}

#[bench]
fn convert_system_fn_big(b: &mut Bencher) {
    b.iter(|| {
        let _ = big.system();
    });
}

#[bench]
fn system_run_big(b: &mut Bencher) {
    let mut world = World::default();
    let mut sys = big.system();
    sys.initialize(&mut world);
    b.iter(|| {
        sys.run(&world).unwrap();
    });
}

#[bench]
fn system_run_small(b: &mut Bencher) {
    let mut world = World::default();
    let mut sys = smol.system();
    sys.initialize(&mut world);
    b.iter(|| {
        sys.run(&world).unwrap();
    });
}

