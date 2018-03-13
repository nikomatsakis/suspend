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

/// Represents a "suspended" value. Suspended values may have
/// references into heap values that are owned by this `Suspend`.  The
/// lifetime(s) of those references are hidden and called the
/// "existential" lifetimes.
///
/// - The type `L` is the "closed" form, a marker type in which those
///   existential lifetimes do not appear.
/// - The bound `'bound` is a bound on the overall lifetime of the
///   data that the existential lifetimes may refer to (which does not
///   otherwise appear in `Suspend`).
pub struct Suspend<'bound, L: FreeSuspended> {
    /// Contains the closed over data. This `Box` *actually* stores
    /// the "opened" form of the data in `L`, but we give it the
    /// "closed" form of the type to hide the existential lifetime.
    ///
    /// Always `Some` except when dtor has run.
    closed_data: Option<Box<L>>,

    /// This drop-thunk, when dropped, will cause all the hidden data
    /// to be freed. The "hidden data" consists of boxes that were
    /// used to build the closed-data.
    drop_thunk: Box<DropThunk + 'bound>,
}

/// An implementation detail: this trait is used to open and free the
/// `closed_data` field.
pub trait FreeSuspended: Sized {
    fn free_closed_data(self: &mut Suspend<'bound, Self>);
}

impl<'bound, L> Drop for Suspend<'bound, L>
where
    L: FreeSuspended,
{
    fn drop(&mut self) {
        self.free_closed_data();
    }
}

impl<'bound, L> ::std::ops::Deref for Suspend<'bound, L>
where
    L: FreeSuspended,
{
    type Target = L;

    fn deref(&self) -> &Self::Target {
        self.closed_data.as_ref().unwrap()
    }
}
