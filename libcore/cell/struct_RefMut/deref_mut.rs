#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::cell::RefMut;

    use core::ops::DerefMut;
    use core::ops::Deref;

    // pub struct RefCell<T: ?Sized> {
    //     borrow: Cell<BorrowFlag>,
    //     value: UnsafeCell<T>,
    // }

    // impl<T> RefCell<T> {
    //     /// Creates a new `RefCell` containing `value`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::RefCell;
    //     ///
    //     /// let c = RefCell::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn new(value: T) -> RefCell<T> {
    //         RefCell {
    //             value: UnsafeCell::new(value),
    //             borrow: Cell::new(UNUSED),
    //         }
    //     }
    //
    //     /// Consumes the `RefCell`, returning the wrapped value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::RefCell;
    //     ///
    //     /// let c = RefCell::new(5);
    //     ///
    //     /// let five = c.into_inner();
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn into_inner(self) -> T {
    //         // Since this function takes `self` (the `RefCell`) by value, the
    //         // compiler statically verifies that it is not currently borrowed.
    //         // Therefore the following assertion is just a `debug_assert!`.
    //         debug_assert!(self.borrow.get() == UNUSED);
    //         unsafe { self.value.into_inner() }
    //     }
    // }

    // pub struct RefMut<'b, T: ?Sized + 'b> {
    //     // FIXME #12808: strange name to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _value: &'b mut T,
    //     _borrow: BorrowRefMut<'b>,
    // }

    // impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
    //     type Target = T;
    //
    //     #[inline]
    //     fn deref<'a>(&'a self) -> &'a T {
    //         self._value
    //     }
    // }

    // impl<'b, T: ?Sized> DerefMut for RefMut<'b, T> {
    //     #[inline]
    //     fn deref_mut<'a>(&'a mut self) -> &'a mut T {
    //         self._value
    //     }
    // }

    type T = i32;

    #[test]
    fn deref_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let mut value_refmut: RefMut<T> = refcell.borrow_mut();

	{
	    let deref_mut: &mut T = value_refmut.deref_mut();
	    *deref_mut = 500;
	}

	let value_ref: &T = value_refmut.deref();
	assert_eq!(*value_ref, 500);
    }
}
