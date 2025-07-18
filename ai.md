# AI Documentation for world_dispatcher v2.1.0

## Description of the crate
Notes about resources:
- Resources MUST implement default
- Resources MAY use Mutex<Arc<T>> to be Send+Sync
- Resources MUST be 'static

### Module `root`
```
#[derive(Default)]
pub struct DispatcherBuilder;
impl DispatcherBuilder {
    /// Creates a new `DispatcherBuilder`.
    pub fn new() -> Self;
    /// Adds a function implementing `IntoSystem` to the system pool.
    pub fn add<R, F: IntoSystem<R>>(self, into_system: F) -> Self;
    /// Adds a `System` to the system pool.
    pub fn add_system(self, system: System) -> Self;
    /// Builds a `Dispatcher` from the accumulated set of `System`.
    /// This preserves the order from the inserted systems.
    pub fn build(self, world: &mut World) -> Dispatcher;
}

pub struct Dispatcher;
impl Dispatcher {
    /// Returns an iterator of all stages. This is not needed for regular use,
    /// but can be useful for debugging or for implementing custom executors.
    pub fn iter_stages(&self) -> impl Iterator<Item = &Vec<System>>;
    /// Runs the systems one after the other, one at a time.
    pub fn run_seq(&mut self, world: &World);
    /// Runs the systems in parallel. Systems having conflicts in their
    /// dependencies (the resource reference they use are the same and at least
    /// one is mutable) are run sequentially relative to each other, while
    /// systems without conflict run in parallel.
    #[cfg(feature = "parallel")]
    pub fn run_par(&mut self, world: &World);
}

/// The type of `Resource`s.
/// All types having a 'static lifetime automatically implement this.
pub trait Resource: Send + Sync + 'static + Downcast;

pub struct World;
impl World {
    /// Initializes a resource to its default value.
    /// This is the only way to "insert" a resource.
    ///
    /// It is suggested to use a macro to collect all
    /// the resources and initialize all of them.
    pub fn initialize<T: Default + Send + Sync + 'static>(&mut self);
    /// Get an immutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed mutably
    pub fn get<T: Send + Sync + 'static>(&self) -> AtomicRef<T>;
    /// Get a mutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed immutably
    /// - Already borrowed mutably
    pub fn get_mut<T: Send + Sync + 'static>(&self) -> AtomicRefMut<T>;
    /// Get a mutable reference to a resource by type, default-initializing it if not already
    /// initialized.
    pub fn get_mut_or_default<T: Default + Send + Sync + 'static>(&mut self) -> AtomicRefMut<T>;
}

pub struct System {
    /// Returns the underlying type name of the system. This is not guranteed to
    /// be stable or human-readable, but can be used for diagnostics.
    pub name: &'static str,
}
impl System {
    /// Initializes the resources required to run this system inside of the
    /// provided `World`, if those resources don't already exist.
    ///
    /// This is called automatically if you use a `Dispatcher`, so in most
    /// cases it is not required to call it manually.
    pub fn initialize(&self, world: &mut World);
    /// Runs the system's function using the provided `World`'s resources.
    pub fn run(&mut self, world: &World);
    /// Returns the underlying type name of the system. This is not guranteed to
    /// be stable or human-readable, but can be used for diagnostics.
    pub fn name(&self) -> &'static str;
}

/// Converts a function into a `System`. It is required to execute a function
/// automatically from `World`'s resources.
/// This trait is automatically implemented for functions taking 12 arguments (22 if using the
/// `big_systems` feature)
/// or less where:
/// - All arguments are immutable or mutable references.
/// - All immutable references are placed *before* all mutable references.
/// - All arguments implement `Default`.
/// - Does not use the same type twice.
pub trait IntoSystem<R> {
    fn system(self) -> System;
}
```
