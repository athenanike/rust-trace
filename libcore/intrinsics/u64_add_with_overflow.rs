#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::u64_add_with_overflow;

    // pub fn u64_add_with_overflow(x: u64, y: u64) -> (u64, bool);

    #[test]
    fn u64_add_with_overflow_test1() {
	let x: u64 = 0xffffffff00000000; // 18446744069414584320
	let y: u64 = 0x00000000ffffffff; // 4294967295

	let (result, is_overflow): (u64, bool) = unsafe {
	    u64_add_with_overflow(x, y)
	};

	assert_eq!(result, 0xffffffffffffffff); // 18446744073709551615
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u64_add_with_overflow_test2() {
	let x: u64 = 0xffffffffffffffff; // 18446744073709551615
	let y: u64 = 0x0000000000000001; // 1

	let (result, is_overflow): (u64, bool) = unsafe {
	    u64_add_with_overflow(x, y)
	};

	assert_eq!(result, 0x0); // 0
	assert_eq!(is_overflow, true);
    }
}
