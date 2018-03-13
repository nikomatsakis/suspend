#![feature(arbitrary_self_types)]
#![feature(crate_in_paths)]
#![feature(crate_visibility_modifier)]
#![feature(in_band_lifetimes)]
#![feature(existential_impl_trait)]
#![feature(underscore_lifetimes)]
#![feature(universal_impl_trait)]

mod drop_thunk;
mod test;

use crate::drop_thunk::DropThunk;

use std::mem;

pub struct Suspend0<T> {
    data: Box<T>,
}

impl<T> Suspend0<T> {
    pub fn new(value: T) -> Suspend0<T> {
        Suspend0 {
            data: Box::new(value),
        }
    }

    pub fn open<R>(&mut self, op: impl for<'a> FnOnce(&'a mut T) -> R) -> R {
        op(&mut self.data)
    }

    pub fn layer<'bound, L>(self) -> Suspend1<'bound, L>
    where
        L: Close1<Input = T>,
        T: 'bound,
    {
        unsafe {
            let (data_ptr, drop_thunk) = drop_thunk::split_box(self.data);
            let open_data: Box<Opened1<'_, L>> = Box::new(L::build(&*data_ptr));
            let closed_data: Option<Box<L>> = Some(mem::transmute(open_data));
            Suspend1 {
                closed_data,
                drop_thunk,
            }
        }
    }
}

pub struct Suspend1<'bound, L: FreeSuspended> {
    closed_data: Option<Box<L>>,
    drop_thunk: Box<DropThunk + 'bound>,
}

pub trait FreeSuspended: Sized {
    fn free_closed_data(self: &mut Suspend1<'bound, Self>);
}

pub trait Close1: FreeSuspended + for<'a> Open1<'a> {
    type Input;

    fn build<'a>(input: &'a Self::Input) -> <Self as Open1<'a>>::Output;

    fn open(self: &mut Suspend1<'bound, Self>) -> String {
        unsafe {
            let closed_data_ref: &mut Self = self.closed_data.as_mut().unwrap();
            let open_data_ref: &mut Opened1<'_, Self> = mem::transmute(closed_data_ref);
            <Self as Open1<'_>>::open(open_data_ref)
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

    fn open(data: &mut Self::Output) -> String;
}

type Opened1<'a, L> = <L as Open1<'a>>::Output;

impl<'bound, L> Drop for Suspend1<'bound, L>
where
    L: FreeSuspended,
{
    fn drop(&mut self) {
        self.free_closed_data();
    }
}

impl<'bound, L> ::std::ops::Deref for Suspend1<'bound, L>
where
    L: FreeSuspended,
{
    type Target = L;

    fn deref(&self) -> &Self::Target {
        self.closed_data.as_ref().unwrap()
    }
}
