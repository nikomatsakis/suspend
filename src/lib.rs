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
