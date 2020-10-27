pub use crate::*;

/// Contains data indexed by type.
/// World allows to dynamically enforce the rust rules of borrowing and ownership
/// at runtime:
/// - The same type cannot be borrowed immutably and mutably at the same time.
/// - The same type cannot be borrowed mutably more than once at the same time.
#[derive(Default)]
pub struct World {
    res: HashMap<TypeId, RefCell<Box<dyn Resource>>, BuildHasherDefault<TypeIdHasher>>,
}

// Safe as long as you don't call initialize in a thread other than main.
// This happens to always be the case as you can't borrow immutable and mutable
// at the same time.
unsafe impl Sync for World {}

impl World {
    /// Initializes a resource to its default value.
    /// This is the only way to "insert" a resource.
    ///
    /// It is suggested to use a macro to collect all
    /// the resources and initialize all of them.
    pub fn initialize<T: Default + 'static>(&mut self) {
        if !self.res.contains_key(&TypeId::of::<T>()) {
            self.res
                .insert(TypeId::of::<T>(), RefCell::new(Box::new(T::default())));
        }
    }
    /// Get an immutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed mutably
    pub fn get<T: 'static>(&self) -> Result<Ref<T>, EcsError> {
        self.res
            .get(&TypeId::of::<T>())
            .ok_or(EcsError::NotInitialized)
            .and_then(|i| i.try_borrow().map_err(|_| EcsError::AlreadyBorrowed))
            .and_then(|i| Ok(Ref::map(i, |j| j.downcast_ref::<T>().unwrap())))
    }
    /// Get a mutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed immutably
    /// - Already borrowed mutably
    pub fn get_mut<T: 'static>(&self) -> Result<RefMut<T>, EcsError> {
        self.res
            .get(&TypeId::of::<T>())
            .ok_or(EcsError::NotInitialized)
            .and_then(|i| i.try_borrow_mut().map_err(|_| EcsError::AlreadyBorrowed))
            .and_then(|i| Ok(RefMut::map(i, |j| j.downcast_mut::<T>().unwrap())))
    }
}
