use alloc::boxed::Box;
use core::any::Any;
use core::cell::{RefMut};
use core::marker::PhantomData;
use core::ops::{Deref};
use core::ops::{DerefMut};

pub struct ResMut<'a, T: 'static> {
    pub(crate) value: RefMut<'a, Box<dyn Any>>,
    pub(crate) _marker: PhantomData<&'a mut T>,
}

impl<T> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}
