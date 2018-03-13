#![feature(arbitrary_self_types)]
#![feature(crate_in_paths)]
#![feature(crate_visibility_modifier)]
#![feature(in_band_lifetimes)]
#![feature(existential_impl_trait)]
#![feature(underscore_lifetimes)]
#![feature(universal_impl_trait)]

mod drop_thunk;
mod test;

pub mod layer1;

use crate::drop_thunk::DropThunk;
use crate::layer1::{Close1, Opened1};

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
