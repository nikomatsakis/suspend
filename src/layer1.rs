use std::mem;

use crate::{FreeSuspended, Suspend1};

pub trait Close1: FreeSuspended + for<'a> Open1<'a> {
    type Input;

    fn build<'a>(input: &'a Self::Input) -> <Self as Open1<'a>>::Output;

    fn open<F>(self: &mut Suspend1<'bound, Self>, func: F) -> F::Output
    where
        F: Func1<Self>,
    {
        unsafe {
            let closed_data_ref: &mut Self = self.closed_data.as_mut().unwrap();
            let open_data_ref: &mut Opened1<'_, Self> = mem::transmute(closed_data_ref);
            func.invoke(open_data_ref)
        }
    }
}

pub fn free_close1_data<L: Close1>(suspend: &mut Suspend1<'bound, L>) {
    unsafe {
        let closed_data: Box<L> = suspend.closed_data.take().unwrap();
        let open_data: Box<Opened1<'_, L>> = mem::transmute(closed_data);
        mem::drop(open_data);
    }
}

pub trait Open1<'a> {
    type Output;
}

pub type Opened1<'a, L> = <L as Open1<'a>>::Output;

pub trait Func1<Input: Close1> {
    type Output;

    fn invoke<'a>(self, data: &mut Opened1<'a, Input>) -> Self::Output;
}
