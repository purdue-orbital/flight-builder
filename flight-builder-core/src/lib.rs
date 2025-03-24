//#![no_std]

pub mod query;
pub mod scheduler;
pub mod tasks;

extern crate alloc;
extern crate core;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::{Ref, RefCell, RefMut};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use std::time::{SystemTime, UNIX_EPOCH};

use scheduler::*;
use tasks::*;

pub mod prelude {
    pub use crate::query::*;
    pub use crate::scheduler::*;
    pub use crate::tasks::*;
}
