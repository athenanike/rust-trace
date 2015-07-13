#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;

    // pub struct Rc<T: ?Sized> {
    //     // FIXME #12808: strange names to try to avoid interfering with field
    //     // accesses of the contained type via Deref
    //     _ptr: NonZero<*mut RcBox<T>>,
    // }

    // impl<T: ?Sized> !marker::Send for Rc<T> {}
    // impl<T: ?Sized> !marker::Sync for Rc<T> {}

    // impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<Rc<U>> for Rc<T> {}

    // impl<T> Rc<T> {
    //     /// Constructs a new `Rc<T>`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new(value: T) -> Rc<T> {
    //         unsafe {
    //             Rc {
    //                 // there is an implicit weak pointer owned by all the strong
    //                 // pointers, which ensures that the weak destructor never frees
    //                 // the allocation while the strong destructor is running, even
    //                 // if the weak pointer is stored inside the strong one.
    //                 _ptr: NonZero::new(Box::into_raw(box RcBox {
    //                     strong: Cell::new(1),
    //                     weak: Cell::new(1),
    //                     value: value
    //                 })),
    //             }
    //         }
    //     }
    //
    //     /// Unwraps the contained value if the `Rc<T>` is unique.
    //     ///
    //     /// If the `Rc<T>` is not unique, an `Err` is returned with the same
    //     /// `Rc<T>`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_unique)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// let x = Rc::new(3);
    //     /// assert_eq!(Rc::try_unwrap(x), Ok(3));
    //     ///
    //     /// let x = Rc::new(4);
    //     /// let _y = x.clone();
    //     /// assert_eq!(Rc::try_unwrap(x), Err(Rc::new(4)));
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "rc_unique")]
    //     pub fn try_unwrap(rc: Rc<T>) -> Result<T, Rc<T>> {
    //         if Rc::is_unique(&rc) {
    //             unsafe {
    //                 let val = ptr::read(&*rc); // copy the contained object
    //                 // destruct the box and skip our Drop
    //                 // we can ignore the refcounts because we know we're unique
    //                 deallocate(*rc._ptr as *mut u8, size_of::<RcBox<T>>(),
    //                             align_of::<RcBox<T>>());
    //                 forget(rc);
    //                 Ok(val)
    //             }
    //         } else {
    //             Err(rc)
    //         }
    //     }
    // }

    // impl<T: ?Sized> Drop for Rc<T> {
    //     /// Drops the `Rc<T>`.
    //     ///
    //     /// This will decrement the strong reference count. If the strong reference
    //     /// count becomes zero and the only other references are `Weak<T>` ones,
    //     /// `drop`s the inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// {
    //     ///     let five = Rc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     ///     drop(five); // explicit drop
    //     /// }
    //     /// {
    //     ///     let five = Rc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     /// } // implicit drop
    //     /// ```
    //     fn drop(&mut self) {
    //         unsafe {
    //             let ptr = *self._ptr;
    //             if !(*(&ptr as *const _ as *const *const ())).is_null() &&
    //                ptr as *const () as usize != mem::POST_DROP_USIZE {
    //                 self.dec_strong();
    //                 if self.strong() == 0 {
    //                     // destroy the contained object
    //                     drop_in_place(&mut (*ptr).value);
    //
    //                     // remove the implicit "strong weak" pointer now that we've
    //                     // destroyed the contents.
    //                     self.dec_weak();
    //
    //                     if self.weak() == 0 {
    //                         deallocate(ptr as *mut u8,
    //                                    size_of_val(&*ptr),
    //                                    align_of_val(&*ptr))
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // impl<T: ?Sized + PartialEq> PartialEq for Rc<T> {
    //     /// Equality for two `Rc<T>`s.
    //     ///
    //     /// Two `Rc<T>`s are equal if their inner value are equal.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// five == Rc::new(5);
    //     /// ```
    //     #[inline(always)]
    //     fn eq(&self, other: &Rc<T>) -> bool { **self == **other }
    //
    //     /// Inequality for two `Rc<T>`s.
    //     ///
    //     /// Two `Rc<T>`s are unequal if their inner value are unequal.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// five != Rc::new(5);
    //     /// ```
    //     #[inline(always)]
    //     fn ne(&self, other: &Rc<T>) -> bool { **self != **other }
    // }

    type T = i32; // T: ?Sized + PartialEq

    macro_rules! eq_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let rc1: Rc<$T> = Rc::<$T>::new($value);
	    let rc2: Rc<$T> = Rc::<$T>::new($other);

	    assert_eq!(rc1.eq(&rc2), $result);
	    assert_eq!(rc1 == rc2, $result);
	})
    }

    #[test]
    fn eq_test1() {
	eq_test!( T, 68, 68, true );
	eq_test!( T, 68, 500, false );
	eq_test!( T, 500, 68, false );
	eq_test!( T, 500, 500, true );
    }
}
