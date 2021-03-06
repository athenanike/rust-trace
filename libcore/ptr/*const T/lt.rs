#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: ?Sized> PartialOrd for *const T {
    //     #[inline]
    //     fn partial_cmp(&self, other: &*const T) -> Option<Ordering> {
    //         Some(self.cmp(other))
    //     }
    //
    //     #[inline]
    //     fn lt(&self, other: &*const T) -> bool { *self < *other }
    //
    //     #[inline]
    //     fn le(&self, other: &*const T) -> bool { *self <= *other }
    //
    //     #[inline]
    //     fn gt(&self, other: &*const T) -> bool { *self > *other }
    //
    //     #[inline]
    //     fn ge(&self, other: &*const T) -> bool { *self >= *other }
    // }

    type T = i32;

    #[test]
    fn lt_test1() {
	let x: T = 68;
	let y: T = 500;
	let z: T = 999;
	let x_ptr: *const T = &x;
	let y_ptr: *const T = &y;
	let z_ptr: *const T = &z;

	assert_eq!(x_ptr.lt(&x_ptr), false);
	assert_eq!(x_ptr.lt(&y_ptr), false);
	assert_eq!(x_ptr.lt(&z_ptr), false);

	assert_eq!(y_ptr.lt(&x_ptr), true);
	assert_eq!(y_ptr.lt(&y_ptr), false);
	assert_eq!(y_ptr.lt(&z_ptr), false);

	assert_eq!(z_ptr.lt(&x_ptr), true);
	assert_eq!(z_ptr.lt(&y_ptr), true);
	assert_eq!(z_ptr.lt(&z_ptr), false);
    }

    #[test]
    fn lt_test2() {
	let x: T = 68;
	let y: T = 500;
	let z: T = 999;
	let x_ptr: *const T = &x;
	let y_ptr: *const T = &y;
	let z_ptr: *const T = &z;

	assert_eq!(x_ptr < x_ptr, false);
	assert_eq!(x_ptr < y_ptr, false);
	assert_eq!(x_ptr < z_ptr, false);

	assert_eq!(y_ptr < x_ptr, true);
	assert_eq!(y_ptr < y_ptr, false);
	assert_eq!(y_ptr < z_ptr, false);

	assert_eq!(z_ptr < x_ptr, true);
	assert_eq!(z_ptr < y_ptr, true);
	assert_eq!(z_ptr < z_ptr, false);
    }
}
