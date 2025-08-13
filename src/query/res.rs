use alloc::boxed::Box;
use core::any::Any;
use core::cell::{Ref};
use core::marker::PhantomData;
use core::ops::{Deref};

pub struct Res<'a, T: 'static> {
    pub(crate) value: Ref<'a, Box<dyn Any>>,
    pub(crate) _marker: PhantomData<&'a T>,
}

impl<T> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}
