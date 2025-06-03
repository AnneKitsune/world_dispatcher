use world_dispatcher::*;
fn main() {
    #[derive(Default)]
    pub struct A;

    let mut world = World::default();

    let sys = (|_comps: &A| Ok(())).system();

    let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();

    assert!(world.get::<A>().is_ok());
}
