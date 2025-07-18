use crate::*;

/// Contains data indexed by type.
/// World allows to dynamically enforce the rust rules of borrowing and ownership
/// at runtime:
/// - The same type cannot be borrowed immutably and mutably at the same time.
/// - The same type cannot be borrowed mutably more than once at the same time.
#[derive(Default)]
pub struct World {
    pub(crate) res:
        HashMap<TypeId, AtomicRefCell<Box<dyn Resource>>, BuildHasherDefault<TypeIdHasher>>,
}

impl World {
    /// Initializes a resource to its default value.
    /// This is the only way to "insert" a resource.
    ///
    /// It is suggested to use a macro to collect all
    /// the resources and initialize all of them.
    pub fn initialize<T: Default + Send + Sync + 'static>(&mut self) {
        if !self.res.contains_key(&TypeId::of::<T>()) {
            self.res.insert(
                TypeId::of::<T>(),
                AtomicRefCell::new(Box::new(T::default())),
            );
        }
    }
    /// Get an immutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed mutably
    pub fn get<T: Send + Sync + 'static>(&self) -> AtomicRef<T> {
        let i = self
            .res
            .get(&TypeId::of::<T>())
            .expect(&format!(
                "Trying to world::get a resource that is not initialized. Type: {}",
                std::any::type_name::<T>()
            ))
            .try_borrow()
            .expect("Trying to world::get_by_typeid a resource that was not initialized.");
        AtomicRef::map(i, |j| j.downcast_ref::<T>().unwrap())
    }

    pub(crate) fn try_get<T: Send + Sync + 'static>(&self) -> Result<AtomicRef<T>, ()> {
        self.res
            .get(&TypeId::of::<T>())
            .ok_or(())
            .and_then(|i| i.try_borrow().map_err(|_| ()))
            .and_then(|i| Ok(AtomicRef::map(i, |j| j.downcast_ref::<T>().unwrap())))
    }

    /// Get a mutable reference to a resource by type.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed immutably
    /// - Already borrowed mutably
    pub fn get_mut<T: Send + Sync + 'static>(&self) -> AtomicRefMut<T> {
        let i = self
            .res
            .get(&TypeId::of::<T>())
            .expect(&format!(
                "Trying to world::get_mut a resource that is not initialized. Type: {}",
                std::any::type_name::<T>()
            ))
            .try_borrow_mut()
            .expect("Trying to world::get_by_typeid a resource that was not initialized.");
        AtomicRefMut::map(i, |j| j.downcast_mut::<T>().unwrap())
    }

    pub(crate) fn try_get_mut<T: Send + Sync + 'static>(&self) -> Result<AtomicRefMut<T>, ()> {
        self.res
            .get(&TypeId::of::<T>())
            .ok_or(())
            .and_then(|i| i.try_borrow_mut().map_err(|_| ()))
            .and_then(|i| Ok(AtomicRefMut::map(i, |j| j.downcast_mut::<T>().unwrap())))
    }

    /// Get a mutable reference to a resource by type, default-initializing it if not already
    /// initialized.
    pub fn get_mut_or_default<T: Default + Send + Sync + 'static>(&mut self) -> AtomicRefMut<T> {
        self.initialize::<T>();
        self.get_mut()
    }

    /// Get a mutable reference to a resource by its type id. Useful if using
    /// dynamic dispatching.
    /// Will return an error if the type is:
    /// - Non initialized
    /// - Already borrowed immutably
    /// - Already borrowed mutably
    #[doc(hidden)]
    pub fn get_by_typeid(&self, typeid: &TypeId) -> AtomicRefMut<Box<dyn Resource>> {
        self.res
            .get(typeid)
            .expect("Trying to world::get_by_typeid a resource that was not initialized.")
            .try_borrow_mut()
            .expect("Tried to borrow a resource that was already borrowed and is still in use!")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn init_borrow() {
        let mut world = World::default();
        world.initialize::<u32>();
        *world.get_mut::<u32>() = 5;
        *world.get_mut::<u32>() = 6;
        {
            let _long_borrow = world.get::<u32>();
            let _long_borrow2 = world.get::<u32>();
            let failing_borrow = world.try_get_mut::<u32>();
            if !failing_borrow.is_err() {
                panic!();
            }
        }
        {
            let _long_borrow = world.get_mut::<u32>();
            let failing_borrow = world.try_get::<u32>();
            if !failing_borrow.is_err() {
                panic!();
            }
        }
        assert_eq!(*world.get_mut::<u32>(), 6);
    }

    #[test]
    fn init_or_default() {
        let mut world = World::default();

        let mut data = world.get_mut_or_default::<u32>();
        *data += 1;
    }
}
