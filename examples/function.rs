use world_dispatcher::*;

#[derive(Default)]
pub struct A;
#[derive(Default)]
pub struct B;
#[derive(Default)]
pub struct C;
pub struct D;

fn system_function(_a: &A, _b: &B, _c: &mut C, d: &mut Option<D>) {
    assert!(d.is_some());
}

fn main() {
    let mut world = World::default();
    // Will automatically create A, B, C, Option<D>::None inside of world.
    let mut dispatch = DispatcherBuilder::new()
        .add(system_function)
        .build(&mut world);
    // Let's assign a value to D.
    *world.get_mut::<Option<D>>() = Some(D);

    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
}
