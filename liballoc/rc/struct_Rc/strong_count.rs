#![feature(core, alloc, rc_counts)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;

    use core::mem::drop;

    // pub struct Rc<T: ?Sized> {
    //     // FIXME #12808: strange names to try to avoid interfering with field
    //     // accesses of the contained type via Deref
    //     _ptr: NonZero<*mut RcBox<T>>,
    // }

    // impl<T: ?Sized> !marker::Send for Rc<T> {}
    // impl<T: ?Sized> !marker::Sync for Rc<T> {}

    // impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<Rc<U>> for Rc<T> {}

    // impl<T: ?Sized> Rc<T> {
    //     /// Downgrades the `Rc<T>` to a `Weak<T>` reference.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_weak)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// let weak_five = five.downgrade();
    //     /// ```
    //     #[unstable(feature = "rc_weak",
    //                reason = "Weak pointers may not belong in this module")]
    //     pub fn downgrade(&self) -> Weak<T> {
    //         self.inc_weak();
    //         Weak { _ptr: self._ptr }
    //     }
    //
    //     /// Get the number of weak references to this value.
    //     #[inline]
    //     #[unstable(feature = "rc_counts")]
    //     pub fn weak_count(this: &Rc<T>) -> usize { this.weak() - 1 }
    //
    //     /// Get the number of strong references to this value.
    //     #[inline]
    //     #[unstable(feature = "rc_counts")]
    //     pub fn strong_count(this: &Rc<T>) -> usize { this.strong() }
    //
    //     /// Returns true if there are no other `Rc` or `Weak<T>` values that share
    //     /// the same inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_unique)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// assert!(Rc::is_unique(&five));
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "rc_unique")]
    //     pub fn is_unique(rc: &Rc<T>) -> bool {
    //         Rc::weak_count(rc) == 0 && Rc::strong_count(rc) == 1
    //     }
    //
    //     /// Returns a mutable reference to the contained value if the `Rc<T>` is
    //     /// unique.
    //     ///
    //     /// Returns `None` if the `Rc<T>` is not unique.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_unique)]
    //     /// use std::rc::Rc;
    //     ///
    //     //     /// let mut x = Rc::new(3);
    //     /// *Rc::get_mut(&mut x).unwrap() = 4;
    //     /// assert_eq!(*x, 4);
    //     ///
    //     /// let _y = x.clone();
    //     /// assert!(Rc::get_mut(&mut x).is_none());
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "rc_unique")]
    //     pub fn get_mut(rc: &mut Rc<T>) -> Option<&mut T> {
    //         if Rc::is_unique(rc) {
    //             let inner = unsafe { &mut **rc._ptr };
    //             Some(&mut inner.value)
    //         } else {
    //             None
    //         }
    //     }
    // }

    // impl<T: ?Sized> Clone for Rc<T> {
    //
    //     /// Makes a clone of the `Rc<T>`.
    //     ///
    //     /// When you clone an `Rc<T>`, it will create another pointer to the data and
    //     /// increase the strong reference counter.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// five.clone();
    //     /// ```
    //     #[inline]
    //     fn clone(&self) -> Rc<T> {
    //         self.inc_strong();
    //         Rc { _ptr: self._ptr }
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

    type T = i32; // T: ?Sized

    #[test]
    fn strong_count_test1() {
	let value: T = 68;
	let rc: Rc<T> = Rc::<T>::new(value);

	let strong_count: usize = Rc::<T>::strong_count(&rc);
	assert_eq!(strong_count, 1);

	let clone1: Rc<T> = rc.clone();
	let strong_count: usize = Rc::<T>::strong_count(&rc);
	assert_eq!(strong_count, 2);

	let clone2: Rc<T> = rc.clone();
	let strong_count: usize = Rc::<T>::strong_count(&rc);
	assert_eq!(strong_count, 3);

	drop(clone1);
	let strong_count: usize = Rc::<T>::strong_count(&rc);
	assert_eq!(strong_count, 2);

	drop(clone2);
	let strong_count: usize = Rc::<T>::strong_count(&rc);
	assert_eq!(strong_count, 1);
    }
}