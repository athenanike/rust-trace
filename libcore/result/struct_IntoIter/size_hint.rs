#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::IntoIter;

    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Result<T, E> {
    //     /// Contains the success value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Ok(T),
    //
    //     /// Contains the error value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Err(E)
    // }

    // impl<T, E> IntoIterator for Result<T, E> {
    //     type Item = T;
    //     type IntoIter = IntoIter<T>;
    //
    //     /// Returns a consuming iterator over the possibly contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(5);
    //     /// let v: Vec<u32> = x.into_iter().collect();
    //     /// assert_eq!(v, [5]);
    //     ///
    //     /// let x: Result<u32, &str> = Err("nothing!");
    //     /// let v: Vec<u32> = x.into_iter().collect();
    //     /// assert_eq!(v, []);
    //     /// ```
    //     #[inline]
    //     fn into_iter(self) -> IntoIter<T> {
    //         IntoIter { inner: self.ok() }
    //     }
    // }

    // pub struct IntoIter<T> { inner: Option<T> }

    // impl<T> Iterator for IntoIter<T> {
    //     type Item = T;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<T> { self.inner.take() }
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let n = if self.inner.is_some() {1} else {0};
    //         (n, Some(n))
    //     }
    // }

    type T = u32;
    type E = &'static str;

    #[test]
    fn size_hint_test1() {
	let x: Result<T, E> = Ok::<T, E>(7);
	let into_iter: IntoIter<T> = x.into_iter();
	let (lower, upper): (usize, Option<usize>) = into_iter.size_hint();

	assert_eq!(lower, 1);
	assert_eq!(upper, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test2() {
	let x: Result<T, E> = Err::<T, E>("nothing!");
	let into_iter: IntoIter<T> = x.into_iter();
	let (lower, upper): (usize, Option<usize>) = into_iter.size_hint();

	assert_eq!(lower, 0);
	assert_eq!(upper, Some::<usize>(0));
    }
}
