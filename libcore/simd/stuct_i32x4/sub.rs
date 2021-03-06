#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::i32x4;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct i32x4(pub i32, pub i32, pub i32, pub i32);

    #[test]
    fn sub_test1() {
	let x: i32x4 = i32x4(
	     0,  1,  2,  3
	);
	let y: i32x4 = i32x4(
	    2, 2, 2, 2
	);
	let z: i32x4 = x - y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "i32x4(-2, -1, 0, 1)".to_string());
    }
}
