#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::i16_mul_with_overflow;

    // pub fn i16_mul_with_overflow(x: i16, y: i16) -> (i16, bool);

    #[test]
    fn i16_mul_with_overflow_test1() {
	let x: i16 = 0x7fff; // 32767
	let y: i16 = 0x0001; // 1

	let (result, is_overflow): (i16, bool) = unsafe {
	    i16_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0x7fff); // 32767
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i16_mul_with_overflow_test2() {
	let x: i16 = 0x7fff; // 32767
	let y: i16 = 0x0002; // 2

	let (result, is_overflow): (i16, bool) = unsafe {
	    i16_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xfffe); // -2
	assert_eq!(is_overflow, true);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i16_mul_with_overflow_test3() {
	let x: i16 = 0x8000; // -32768
	let y: i16 = 0x0002; // 2

	let (result, is_overflow): (i16, bool) = unsafe {
	    i16_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0x0); // 0
	assert_eq!(is_overflow, true);
    }
}
