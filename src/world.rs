use crate::*;

/// Contains data indexed by type.
/// World allows to dynamically enforce the rust rules of borrowing and ownership
/// at runtime:
/// - The same type cannot be borrowed immutably and mutably at the same time.
/// - The same type cannot be borrowed mutably more than once at the same time.
#[derive(Default)]
pub struct World {
    pub(crate) res: HashMap<TypeId, AtomicRefCell<Box<dyn Resource>>, BuildHasherDefault<TypeIdHasher>>,
}

// Safe as long as you don't call initialize in multiple threads at once.
// This happens to always be the case as you can't borrow immutable and mutable
// at the same time.
//unsafe impl Sync for World {}

impl World {
    /// Initializes a resource to its default value.
    /// This is the only way to "insert" a resource.
    ///
    /// It is suggested to use a macro to collect all
    /// the resources and initialize all of them.
    pub fn initialize<T: Default + 'static>(&mut self) {
        if !self.res.contains_key(&TypeId::of::<T>()) {
            self.res
                .insert(TypeId::of::<T>(), AtomicRefCell::new(Box::new(T::default())));
        }
    }
    /// Get an immutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed mutably
    pub fn get<T: 'static>(&self) -> Result<AtomicRef<T>, EcsError> {
        self.res
            .get(&TypeId::of::<T>())
            .ok_or(EcsError::NotInitialized)
            .and_then(|i| i.try_borrow().map_err(|_| EcsError::AlreadyBorrowed))
            .and_then(|i| Ok(AtomicRef::map(i, |j| j.downcast_ref::<T>().unwrap())))
    }
    /// Get a mutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed immutably
    /// - Already borrowed mutably
    pub fn get_mut<T: 'static>(&self) -> Result<AtomicRefMut<T>, EcsError> {
        self.res
            .get(&TypeId::of::<T>())
            .ok_or(EcsError::NotInitialized)
            .and_then(|i| i.try_borrow_mut().map_err(|_| EcsError::AlreadyBorrowed))
            .and_then(|i| Ok(AtomicRefMut::map(i, |j| j.downcast_mut::<T>().unwrap())))
    }
    /// Get a mutable reference to a resource by its type id. Useful if using
    /// dynamic dispatching.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed immutably
    /// - Already borrowed mutably
    #[doc(hidden)]
    pub fn get_by_typeid(&self, typeid: &TypeId) -> Result<AtomicRefMut<Box<dyn Resource>>, EcsError> {
        self.res
            .get(typeid)
            .ok_or(EcsError::NotInitialized)
            .and_then(|i| i.try_borrow_mut().map_err(|_| EcsError::AlreadyBorrowed))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn init_borrow() {
        let mut world = World::default();
        world.initialize::<u32>();
        *world.get_mut::<u32>().unwrap() = 5;
        *world.get_mut::<u32>().unwrap() = 6;
        {
            let _long_borrow = world.get::<u32>().unwrap();
            let _long_borrow2 = world.get::<u32>().unwrap();
            let failing_borrow = world.get_mut::<u32>();
            if let EcsError::AlreadyBorrowed = failing_borrow.err().unwrap() {
                // good
            } else {
                unreachable!();
            }
        }
        {
            let _long_borrow = world.get_mut::<u32>().unwrap();
            let failing_borrow = world.get::<u32>();
            if let EcsError::AlreadyBorrowed = failing_borrow.err().unwrap() {
                // good
            } else {
                unreachable!();
            }
        }
        assert_eq!(*world.get_mut::<u32>().unwrap(), 6);
    }
}

