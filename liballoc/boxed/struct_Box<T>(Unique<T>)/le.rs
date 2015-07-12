#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    // pub struct Box<T>(Unique<T>);

    // impl<T: ?Sized + PartialOrd> PartialOrd for Box<T> {
    //     #[inline]
    //     fn partial_cmp(&self, other: &Box<T>) -> Option<Ordering> {
    //         PartialOrd::partial_cmp(&**self, &**other)
    //     }
    //     #[inline]
    //     fn lt(&self, other: &Box<T>) -> bool { PartialOrd::lt(&**self, &**other) }
    //     #[inline]
    //     fn le(&self, other: &Box<T>) -> bool { PartialOrd::le(&**self, &**other) }
    //     #[inline]
    //     fn ge(&self, other: &Box<T>) -> bool { PartialOrd::ge(&**self, &**other) }
    //     #[inline]
    //     fn gt(&self, other: &Box<T>) -> bool { PartialOrd::gt(&**self, &**other) }
    // }

    type T = i32; // T: ?Sized + PartialOrd

    macro_rules! le_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let b: Box<$T> = Box::<$T>::new($value);
	    let other: Box<$T> = Box::<$T>::new($other);

	    assert_eq!(b.le(&other), $result);
	    assert_eq!(b <= other, $result);
	})
    }

    #[test]
    fn le_test1() {
	le_test!( T, 68, 68, true );
	le_test!( T, 68, 500, true );
	le_test!( T, 500, 68, false );
	le_test!( T, 500, 500, true );
    }
}
