use crate::*;

/// A builder that accumulates systems to be inserted into a `Dispatcher`.
///
/// For more complex workflows, this builder can have new traits implemented for
/// it, using the public `systems` member.
#[derive(Default)]
pub struct DispatcherBuilder {
    #[doc(hidden)]
    /// The current systems in this builder. Hidden from docs to encourage use
    /// the API of the builder, but public to enable extension of the builder.
    pub systems: Vec<System>,
}

impl DispatcherBuilder {
    /// Creates a new `DispatcherBuilder`.
    pub fn new() -> Self {
        Self {
            systems: Vec::default(),
        }
    }

    /// Adds a function implementing `IntoSystem` to the system pool.
    pub fn add<R, F: IntoSystem<R>>(mut self, into_system: F) -> Self {
        self.systems.push(into_system.system());
        self
    }
    /// Adds a `System` to the system pool.
    pub fn add_system(mut self, system: System) -> Self {
        self.systems.push(system);
        self
    }
    /// Builds a `Dispatcher` from the accumulated set of `System`.
    /// This preserves the order from the inserted systems.
    pub fn build(self, world: &mut World) -> Dispatcher {
        for sys in self.systems.iter() {
            (sys.initialize)(world);
        }
        let mut stages: Vec<Vec<System>> = vec![];
        let mut stage: Vec<System> = vec![];
        let mut locks = vec![];
        for sys in self.systems {
            let mut fetch = (sys.lock)(world, &mut locks);
            if let Err(_) = fetch {
                stages.push(stage);
                stage = vec![];
                locks.clear();
                fetch = (sys.lock)(world, &mut locks);
            }
            if let Err(_) = fetch {
                panic!(
                    "System cannot be borrowed at all. This means it 
                    uses the same resource twice in its signature."
                );
            }
            stage.push(sys);
        }
        stages.push(stage);
        Dispatcher { stages }
    }
}

/// A dispatcher is used to execute a collection of `System` in order and
/// possibly in parallel using `World`'s resources.
/// A dispatcher automatically avoids mutable borrow collisions which would
/// normally lead to data corruption, dead locks and more.
pub struct Dispatcher {
    stages: Vec<Vec<System>>,
}
impl Dispatcher {
    /// Returns an iterator of all stages. This is not needed for regular use,
    /// but can be useful for debugging or for implementing custom executors.
    pub fn iter_stages(&self) -> impl Iterator<Item = &Vec<System>> {
        self.stages.iter()
    }

    /// Runs the systems one after the other, one at a time.
    pub fn run_seq(&mut self, world: &World) -> SystemResult {
        #[cfg(feature = "profiler")]
        profile_scope!("dispatcher_run_seq");

        for stage in &mut self.stages {
            let errors = stage
                .iter_mut()
                .map(|s| s.run(world))
                .flat_map(|r| r.err())
                .collect::<Vec<_>>();
            if errors.len() > 0 {
                return Err(EcsError::DispatcherExecutionFailed(errors));
            }
        }
        Ok(())
    }
    /// Runs the systems in parallel. Systems having conflicts in their
    /// dependencies (the resource reference they use are the same and at least
    /// one is mutable) are run sequentially relative to each other, while
    /// systems without conflict run in parallel.
    #[cfg(feature = "parallel")]
    pub fn run_par(&mut self, world: &World) -> SystemResult {
        #[cfg(feature = "profiler")]
        profile_scope!("dispatcher_run_par");

        for stage in &mut self.stages {
            let errors = stage
                .par_iter_mut()
                .map(|s| s.run(world))
                .flat_map(|r| r.err())
                .collect::<Vec<_>>();
            if errors.len() > 0 {
                return Err(EcsError::DispatcherExecutionFailed(errors));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn simple_dispatcher() {
        #[derive(Default)]
        pub struct A;
        let mut world = World::default();
        let sys = (|_comps: &A| Ok(())).system();
        let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
        dispatch.run_seq(&world).unwrap();
        dispatch.run_seq(&world).unwrap();
        dispatch.run_seq(&world).unwrap();
        assert!(world.get::<A>().is_ok());
        assert!(world.get_mut::<A>().is_ok());
    }

    #[test]
    #[wasm_bindgen_test]
    fn generic_simple_dispatcher() {
        #[derive(Default)]
        pub struct A;
        let mut world = World::default();
        fn sys<T>(_t: &T) -> SystemResult {
            Ok(())
        }
        let mut dispatch = DispatcherBuilder::new()
            .add(sys::<A>)
            .add_system(sys::<A>.system())
            .build(&mut world);
        dispatch.run_seq(&world).unwrap();
        dispatch.run_seq(&world).unwrap();
        dispatch.run_seq(&world).unwrap();
        assert!(world.get::<A>().is_ok());
        assert!(world.get_mut::<A>().is_ok());
    }

    #[cfg(feature = "parallel")]
    #[test]
    #[wasm_bindgen_test]
    fn par_distpach() {
        #[derive(Default)]
        pub struct A;
        let mut world = World::default();
        let sys = (|_comps: &A| Ok(())).system();
        let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
        dispatch.run_par(&world).unwrap();
        dispatch.run_par(&world).unwrap();
        dispatch.run_par(&world).unwrap();
        assert!(world.get::<A>().is_ok());
        assert!(world.get_mut::<A>().is_ok());
    }

    #[cfg(feature = "parallel")]
    #[test]
    #[wasm_bindgen_test]
    fn dispatch_par_stages() {
        #[derive(Default)]
        struct A;
        #[derive(Default)]
        struct B;
        let mut world = World::default();
        world.initialize::<A>();
        world.initialize::<B>();
        // Stage 1
        fn sys1(_a: &A, _b: &B) -> SystemResult {
            Ok(())
        }
        fn sys2(_a: &A, _b: &B) -> SystemResult {
            Ok(())
        }
        // Stage 2
        fn sys3(_a: &A, _b: &mut B) -> SystemResult {
            Ok(())
        }
        // Stage 3
        fn sys4(_a: &A, _b: &mut B) -> SystemResult {
            Ok(())
        }
        let mut dispatch = DispatcherBuilder::new()
            .add(sys1)
            .add(sys2)
            .add(sys3)
            .add(sys4)
            .build(&mut world);
        assert_eq!(dispatch.stages.len(), 3);
        assert_eq!(dispatch.stages[0].len(), 2);
        assert_eq!(dispatch.stages[1].len(), 1);
        assert_eq!(dispatch.stages[2].len(), 1);
        dispatch.run_par(&world).unwrap();

        let mut dispatch = DispatcherBuilder::new()
            .add(sys1)
            .add(sys2)
            .build(&mut world);
        assert_eq!(dispatch.stages.len(), 1);
        assert_eq!(dispatch.stages[0].len(), 2);
        dispatch.run_par(&world).unwrap();

        let mut dispatch = DispatcherBuilder::new().add(sys1).build(&mut world);
        assert_eq!(dispatch.stages.len(), 1);
        assert_eq!(dispatch.stages[0].len(), 1);
        dispatch.run_par(&world).unwrap();
    }
}
