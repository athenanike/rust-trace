#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Take;

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

		fn size_hint(&self) -> (usize, Option<usize>) {
		    debug_assert!(self.begin <= self.end);
		    let exact: usize = (self.end - self.begin) as usize;
		    (exact, Some::<usize>(exact))
		}

		// fn take(self, n: usize) -> Take<Self> where Self: Sized, {
		//     Take{iter: self, n: n}
		// }
	    }
	}
    }

    impl RandomAccessIterator for A<T> {
	fn indexable(&self) -> usize {
	    let (exact, _) = self.size_hint();
	    exact
	}

	fn idx(&mut self, index: usize) -> Option<Self::Item> {
	    if index < self.indexable() {
		Some::<Self::Item>(self.begin + index as T)
	    } else {
		None::<Self::Item>
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I> RandomAccessIterator for Take<I> where I: RandomAccessIterator{
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         cmp::min(self.iter.indexable(), self.n)
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<<I as Iterator>::Item> {
    //         if index >= self.n {
    //             None
    //         } else {
    //             self.iter.idx(index)
    //         }
    //     }
    // }

    #[test]
    fn idx_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 5;
	let mut take: Take<A<T>> = a.take(n);

	for i in 0..(10 - n) {
	    let x: Option<T> = take.idx(i);
	    match x {
		Some(v) => assert_eq!(v, i as T),
		_ => assert!(false)
	    }
	}

	assert_eq!(take.idx(10 - n), None::<T>);
    }
}
