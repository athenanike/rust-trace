#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    // pub struct Cell<T> {
    //     value: UnsafeCell<T>,
    // }

    // impl<T:Copy> Cell<T> {
    //     /// Creates a new `Cell` containing the given value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn new(value: T) -> Cell<T> {
    //         Cell {
    //             value: UnsafeCell::new(value),
    //         }
    //     }
    //
    //     /// Returns a copy of the contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     ///
    //     /// let five = c.get();
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn get(&self) -> T {
    //         unsafe{ *self.value.get() }
    //     }
    //
    //     /// Sets the contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     ///
    //     /// c.set(10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn set(&self, value: T) {
    //         unsafe {
    //             *self.value.get() = value;
    //         }
    //     }
    //
    //     /// Gets a reference to the underlying `UnsafeCell`.
    //     ///
    //     /// # Unsafety
    //     ///
    //     /// This function is `unsafe` because `UnsafeCell`'s field is public.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     ///
    //     /// let uc = unsafe { c.as_unsafe_cell() };
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "core")]
    //     pub unsafe fn as_unsafe_cell<'a>(&'a self) -> &'a UnsafeCell<T> {
    //         &self.value
    //     }
    // }

    // impl<T:PartialEq + Copy> PartialEq for Cell<T> {
    //     #[inline]
    //     fn eq(&self, other: &Cell<T>) -> bool {
    //         self.get() == other.get()
    //     }
    // }

    type T = i32;

    #[test]
    fn eq_test1() {
	let value: T = 68;
	let cell: Cell<T> = Cell::<T>::new(value);
	let clone: Cell<T> = cell.clone();

	assert_eq!(cell.eq(&clone), true);
    }

    #[test]
    fn eq_test2() {
	let value: T = 68;
	let cell: Cell<T> = Cell::<T>::new(value);
	let clone: Cell<T> = cell.clone();

	assert_eq!(cell == clone, true);
    }

    #[test]
    fn eq_test3() {
	let value: T = 68;
	let cell: Cell<T> = Cell::<T>::new(value);
	let default: Cell<T> = Cell::<T>::default();

	assert_eq!(cell.eq(&default), false);
    }

    #[test]
    fn eq_test4() {
	let value: T = 68;
	let cell: Cell<T> = Cell::<T>::new(value);
	let default: Cell<T> = Cell::<T>::default();

	assert_eq!(cell == default, false);
    }
}
