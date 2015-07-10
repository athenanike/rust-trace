#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::u8x16;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct u8x16(pub u8, pub u8, pub u8, pub u8,
    //                  pub u8, pub u8, pub u8, pub u8,
    //                  pub u8, pub u8, pub u8, pub u8,
    //                  pub u8, pub u8, pub u8, pub u8);

    #[test]
    fn mul_test1() {
	let x: u8x16 = u8x16(
	     0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
	    10, 11, 12, 13, 14, 15
	);
	let y: u8x16 = u8x16(
	    2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
	    2, 2, 2, 2, 2, 2
	);
	let z: u8x16 = x * y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "u8x16(\
	    0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30\
	    )".to_string());
    }
}
