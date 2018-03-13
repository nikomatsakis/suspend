// layer operation:
// - given a `&'a T` to current value
//
// map operation:
// - given a `T`, replace with a `U`
//
// open operation:
// - give a `&'a mut T`, returns an R that doesn't capture `'a`

#![cfg(test)]

use crate::{FreeSuspended, Suspend0, Suspend1, Close1, Open1};

struct VecU32Ref { }

impl Close1 for VecU32Ref {
    type Input = (u32, u32, u32);

    fn build<'a>(tuple: &'a Self::Input) -> <Self as Open1<'a>>::Output {
        vec![&tuple.0, &tuple.1, &tuple.2]
    }
}

impl FreeSuspended for VecU32Ref {
    fn free_closed_data(self: &mut Suspend1<'bound, Self>) {
        ::free_close1_data(self)
    }
}

impl Open1<'a> for VecU32Ref {
    type Output = Vec<&'a u32>;

    fn open(data: &mut Self::Output) -> String {
        format!("{:?}", data)
    }
}

#[test]
fn test() {
    let x = Suspend0::new((1, 2, 3));
    let mut y = x.layer::<VecU32Ref>();
    let s = y.open();
    assert_eq!(s, "[1, 2, 3]");
}
