#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::FilterMap;

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

		// fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> where
		//     Self: Sized, F: FnMut(Self::Item) -> Option<B>,
		// {
		//     FilterMap { iter: self, f: f }
		// }

		// fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator> IntoIterator for I {
    //     type Item = I::Item;
    //     type IntoIter = I;
    //
    //     fn into_iter(self) -> I {
    //         self
    //     }
    // }

    // impl<B, I: Iterator, F> Iterator for FilterMap<I, F>
    //     where F: FnMut(I::Item) -> Option<B>,
    // {
    //     type Item = B;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<B> {
    //         for x in self.iter.by_ref() {
    //             if let Some(y) = (self.f)(x) {
    //                 return Some(y);
    //             }
    //         }
    //         None
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (_, upper) = self.iter.size_hint();
    //         (0, upper) // can't know a lower bound, due to the predicate
    //     }
    // }

    struct F;

    type B = T;
    type Item = T;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = Option<B>;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    match item {
		_ if item >= 0 => Some(item),
		_ => None
	    }
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		_ if item >= 0 => Some(item),
		_ => None
	    }
	}
    }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: -10, end: 10 };
	let f: F = F;
	let mut filter_map: FilterMap<A<T>, F> = a.filter_map::<B, F>(f);

	for x in 0..10 {
	    let y: Option<B> = filter_map.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(filter_map.next(), None::<B>);
    }
}
