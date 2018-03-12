use std::mem;

/// Just carries a dtor
crate trait DropThunk {}

crate unsafe fn split_box<'bound, T: 'bound>(value: Box<T>) -> (*mut T, Box<DropThunk + 'bound>) {
    let ptr: *mut T = mem::transmute(value);
    let thunk = BoxDropThunk { ptr };
    (ptr, Box::new(thunk))
}

struct BoxDropThunk<T> {
    ptr: *mut T,
}

impl<T> Drop for BoxDropThunk<T> {
    fn drop(&mut self) {
        unsafe {
            let _value: Box<T> = mem::transmute(self.ptr);
        }
    }
}

impl<T> DropThunk for BoxDropThunk<T> {}
