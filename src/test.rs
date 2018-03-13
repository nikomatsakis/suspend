// layer operation:
// - given a `&'a T` to current value
//
// map operation:
// - given a `T`, replace with a `U`
//
// open operation:
// - give a `&'a mut T`, returns an R that doesn't capture `'a`

#![cfg(test)]

use crate::{FreeSuspended, Suspend1};
use crate::layer1::{Close1, Open1, Opened1, Func1};

struct VecU32Ref { }

impl Close1 for VecU32Ref {
    type Input = (u32, u32, u32);

    fn build<'a>(tuple: &'a Self::Input) -> <Self as Open1<'a>>::Output {
        vec![&tuple.0, &tuple.1, &tuple.2]
    }
}

impl FreeSuspended for VecU32Ref {
    fn free_closed_data(self: &mut Suspend1<'bound, Self>) {
        ::crate::layer1::free_close1_data(self)
    }
}

impl Open1<'a> for VecU32Ref {
    type Output = Vec<&'a u32>;
}

struct FormatVecU32Ref;

impl Func1<VecU32Ref> for FormatVecU32Ref {
    type Output = String;

    fn invoke<'a>(self, data: &mut Opened1<'a, VecU32Ref>) -> Self::Output {
        format!("{:?}", data)
    }
}

#[test]
fn test() {
    let mut y = VecU32Ref::layer_on((1, 2, 3));
    let s = y.open(FormatVecU32Ref);
    assert_eq!(s, "[1, 2, 3]");
}
