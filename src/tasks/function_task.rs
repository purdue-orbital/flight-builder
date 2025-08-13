use core::marker::PhantomData;

pub struct FunctionTask<Input, F> {
    pub f: F,
    pub marker: PhantomData<fn() -> Input>,
}
