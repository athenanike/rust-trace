#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::BorrowMut;
    use collections::borrow::Borrow;

    use core::default::Default;

    // pub trait BorrowMut<Borrowed: ?Sized> : Borrow<Borrowed> {
    //     /// Mutably borrows from an owned value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::BorrowMut;
    //     ///
    //     /// fn check<T: BorrowMut<[i32]>>(mut v: T) {
    //     ///     assert_eq!(&mut [1, 2, 3], v.borrow_mut());
    //     /// }
    //     ///
    //     /// let v = vec![1, 2, 3];
    //     ///
    //     /// check(v);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn borrow_mut(&mut self) -> &mut Borrowed;
    // }

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

    // impl<T: ?Sized> Borrow<T> for T {
    //     fn borrow(&self) -> &T { self }
    // }

    // impl<'a, T: ?Sized> Borrow<T> for &'a mut T {
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
	let mut x: T = T::default();

	{
	    let x_ref_mut: &mut T = x.borrow_mut();
	    x_ref_mut.value = 68;

	    {
		let borrow: &T = x_ref_mut.borrow();
		assert_eq!(borrow.value, 68);
	    }
	}

	assert_eq!(x.value,  68);
    }
}
