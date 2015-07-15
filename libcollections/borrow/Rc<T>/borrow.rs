#![feature(core, alloc, collections)]
extern crate core;
extern crate alloc;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::Borrow;

    use core::default::Default;

    use alloc::rc::Rc;

    // pub trait Borrow<Borrowed: ?Sized> {
    //     /// Immutably borrows from an owned value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Borrow;
    //     ///
    //     /// fn check<T: Borrow<str>>(s: T) {
    //     ///     assert_eq!("Hello", s.borrow());
    //     /// }
    //     ///
    //     /// let s = "Hello".to_string();
    //     ///
    //     /// check(s);
    //     ///
    //     /// let s = "Hello";
    //     ///
    //     /// check(s);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn borrow(&self) -> &Borrowed;
    // }

    // impl<T: ?Sized> Borrow<T> for rc::Rc<T> {
    //     fn borrow(&self) -> &T { &**self }
    // }

    struct A<T> {
	value: T
    }

    impl<T> A<T> {
	fn new(value: T) -> Self {
	    A { value: value }
	}
    }

    impl<T: Default> Default for A<T> {
	fn default() -> Self {
	    Self::new(Default::default())
	}
    }

    type T = A<i32>; // T: ?Sized
    type Borrowed = T; // Borrowed: ?Sized

    #[test]
    fn borrow_test1() {
	let x: T = T::default();
	let rc: Rc<T> = Rc::<T>::new(x);
	let borrow: &T = rc.borrow();

	assert_eq!(rc.value, 0);
	assert_eq!(borrow.value, 0);
    }
}
