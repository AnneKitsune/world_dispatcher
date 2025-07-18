use world_dispatcher::*;
fn main() {
    #[derive(Default)]
    pub struct A;

    let mut world = World::default();

    let sys = (|_comps: &A| {}).system();

    let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
}
