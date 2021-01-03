use crate::*;

/// A hasher for `TypeId<T>`.
/// According to benchmarks, using it seems to increase performance
/// in most cases. Sometimes not. Computers are weird.
#[derive(Default)]
pub(crate) struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        use core::convert::TryInto;
        self.0 = u64::from_ne_bytes(bytes.try_into().unwrap());
    }
}
