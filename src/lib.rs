#![feature(unboxed_closures)]

//! Resources MUST implement default
//! Resources MAY use Mutex<Arc<T>> to be Send+Sync
//! Resources MUST be 'static

use derive_new::new;
use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::error::Error;
use std::hash::{BuildHasherDefault, Hasher};

#[cfg(feature = "parallel")]
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

mod dispatcher;
mod error;
mod resource;
mod system;
mod typeid;
mod world;

pub use self::dispatcher::*;
pub use self::error::*;
pub use self::resource::*;
pub use self::system::*;
pub use self::typeid::*;
pub use self::world::*;
