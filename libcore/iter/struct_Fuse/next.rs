#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Fuse;

    struct A<T> {
	begin: T,
	end: T
    }

    macro_rules! Iterator_impl {
	($T:ty) => {
	    impl Iterator for A<$T> {
		type Item = $T;

		fn next(&mut self) -> Option<Self::Item> {
		    if self.begin < self.end {
			let result = self.begin;
			self.begin = self.begin.wrapping_add(1);
			Some::<Self::Item>(result)
		    } else {
			None::<Self::Item>
		    }
		}

		// fn fuse(self) -> Fuse<Self> where Self: Sized {
		//     Fuse{iter: self, done: false}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I> Iterator for Fuse<I> where I: Iterator {
    //     type Item = <I as Iterator>::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<<I as Iterator>::Item> {
    //         if self.done {
    //             None
    //         } else {
    //             let next = self.iter.next();
    //             self.done = next.is_none();
    //             next
    //         }
    //     }
    //
    //     #[inline]
    //     fn nth(&mut self, n: usize) -> Option<I::Item> {
    //         if self.done {
    //             None
    //         } else {
    //             let nth = self.iter.nth(n);
    //             self.done = nth.is_none();
    //             nth
    //         }
    //     }
    //
    //     #[inline]
    //     fn last(self) -> Option<I::Item> {
    //         if self.done {
    //             None
    //         } else {
    //             self.iter.last()
    //         }
    //     }
    //
    //     #[inline]
    //     fn count(self) -> usize {
    //         if self.done {
    //             0
    //         } else {
    //             self.iter.count()
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         if self.done {
    //             (0, Some(0))
    //         } else {
    //             self.iter.size_hint()
    //         }
    //     }
    // }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut fuse: Fuse<A<T>> = a.fuse();

	for x in 0..10 {
	    let y: Option<T> = fuse.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(fuse.next(), None::<T>);
    }
}
