#![feature(arbitrary_self_types)]

mod drop_thunk;
mod test;

pub mod layer1;

use drop_thunk::DropThunk;

pub struct Suspend1<'bound, L: FreeSuspended> {
    closed_data: Option<Box<L>>,
    drop_thunk: Box<DropThunk + 'bound>,
}

pub trait FreeSuspended: Sized {
    fn free_closed_data<'bound>(self: &mut Suspend1<'bound, Self>);
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
