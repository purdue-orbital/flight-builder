use alloc::boxed::Box;
use crate::tasks::Task;

pub type StoredTask = Box<dyn Task>;
