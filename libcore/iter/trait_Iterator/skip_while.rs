#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::SkipWhile;

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

		// fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> where
		//     Self: Sized, P: FnMut(&Self::Item) -> bool,
		// {
		//     SkipWhile{iter: self, flag: false, predicate: predicate}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    struct P;

    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for P {
	type Output = bool;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item < 0 => true,
		_ => false
	    }
	}
    }

    impl<'a> FnMut<Args<'a>> for P {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item < 0 => true,
		_ => false
	    }
	}
    }

    #[test]
    fn skip_while_test1() {
	let a: A<T> = A { begin: -10, end: 10 };
	let predicate: P = P;
	let mut skip_while: SkipWhile<A<T>, P> = a.skip_while::<P>(predicate);

	for x in 0..10 {
	    let y: Option<Item> = skip_while.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(skip_while.next(), None::<Item>);
    }
}
