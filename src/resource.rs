use crate::*;

/// The type of `Resource`s.
/// All types having a 'static lifetime automatically implement this.
#[doc(hidden)]
pub trait Resource: Send + Sync + 'static + Downcast {}
impl<T> Resource for T where T: Send + Sync + 'static {}
impl_downcast!(Resource);

/// Hacky trait to extend the lifetime of a Ref<'a, T>, which is used
/// internally in the `Dispatcher`'s logic.
/// Import this if you get errors where RefLifetime is not implemented for
/// your systems.
pub(crate) trait RefLifetime {}
impl<'a, T> RefLifetime for AtomicRef<'a, T> {}
impl<'a, T> RefLifetime for AtomicRefMut<'a, T> {}
impl<'a, T> RefLifetime for &'a T {}
impl<'a, T> RefLifetime for &'a mut T {}
