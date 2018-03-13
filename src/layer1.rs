use std::mem;

use crate::drop_thunk;
use crate::{FreeSuspended, Suspend};

/// Defines the "closed" -- or "suspended" -- version of a reference
/// with one existential lifetime. In this version, the existential
/// lifetime we plan to re-open does not appear.
///
/// A `Close1` is always tied to an `Open1`, which defines the version
/// of the type where the existential lifetime *does* appear. You can
/// use the `Opened1<'a, Self>` alias to access that type.
///
/// Every `Close1` has a defined "base type" B from which it can be
/// constructed; to create one, you invoke `layer_on` with an instance
/// of this base type B. References with the existential lifetime are
/// (typically, anyway) referring into this base value (which is moved
/// into the heap, so that it has a stable address).
///
/// The actual construction of the `Close1` is done by the `build`
/// method, which is internal to `Close1`.
pub trait Close1: FreeSuspended + for<'a> Open1<'a> {
    type Input;

    fn build<'a>(input: &'a Self::Input) -> Opened1<'a, Self>;

    fn layer_on<'bound>(value: Self::Input) -> Suspend<'bound, Self>
    where
        Self::Input: 'bound,
    {
        unsafe {
            let (data_ptr, drop_thunk) = drop_thunk::split_box(Box::new(value));
            let open_data: Box<Opened1<'_, Self>> = Box::new(Self::build(&*data_ptr));
            let closed_data: Option<Box<Self>> = Some(mem::transmute(open_data));
            Suspend {
                closed_data,
                drop_thunk,
            }
        }
    }

    fn open<F>(self: &mut Suspend<'bound, Self>, func: F) -> F::Output
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

pub fn free_close1_data<L: Close1>(suspend: &mut Suspend<'bound, L>) {
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

/// Defines a closure that takes as input a "opened" layer-1 value.
/// The only reason this trait exists (versus a normal closure) is to
/// workaround normalization bugs in rustc.
pub trait Func1<Input: Close1> {
    type Output;

    fn invoke<'a>(self, data: &mut Opened1<'a, Input>) -> Self::Output;
}
