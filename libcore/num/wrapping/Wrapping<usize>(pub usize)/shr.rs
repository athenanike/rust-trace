#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Wrapping;

    use core::ops::Shr;

    // macro_rules! sh_impl {
    //     ($t:ty, $f:ty) => (
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Shl<$f> for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn shl(self, other: $f) -> Wrapping<$t> {
    //                 Wrapping(self.0 << other)
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Shr<$f> for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn shr(self, other: $f) -> Wrapping<$t> {
    //                 Wrapping(self.0 >> other)
    //             }
    //         }
    //     )
    // }

    // macro_rules! sh_impl_all {
    //     ($($t:ty)*) => ($(
    //         // sh_impl! { $t, u8 }
    //         // sh_impl! { $t, u16 }
    //         // sh_impl! { $t, u32 }
    //         // sh_impl! { $t, u64 }
    //         sh_impl! { $t, usize }
    //
    //         // sh_impl! { $t, i8 }
    //         // sh_impl! { $t, i16 }
    //         // sh_impl! { $t, i32 }
    //         // sh_impl! { $t, i64 }
    //         // sh_impl! { $t, isize }
    //     )*)
    // }

    // sh_impl_all! { u8 u32 u32 u64 usize i8 i16 i32 i64 isize }

    macro_rules! shr_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let x: $T = $value;
	    let other: usize = $other;
	    let wrapping: Wrapping<$T> = Wrapping::<$T>(x);

	    let result: Wrapping<$T> = wrapping.shr(other);
	    assert_eq!(result.0, $result);

	    let result: Wrapping<$T> = wrapping >> other;
	    assert_eq!(result.0, $result);
	})
    }

    #[test]
    fn shr_test1() {
	if cfg!(target_pointer_width = "64") {
	    shr_test!( usize, 0x8000000000000000, 0, 0x8000000000000000 );
	    shr_test!( usize, 0x8000000000000000, 1, 0x4000000000000000 );
	    shr_test!( usize, 0x8000000000000000, 2, 0x2000000000000000 );
	    shr_test!( usize, 0x8000000000000000, 3, 0x1000000000000000 );
	    shr_test!( usize, 0x8000000000000000, 4, 0x0800000000000000 );
	    shr_test!( usize, 0x8000000000000000, 5, 0x0400000000000000 );
	    shr_test!( usize, 0x8000000000000000, 6, 0x0200000000000000 );
	    shr_test!( usize, 0x8000000000000000, 7, 0x0100000000000000 );
	    shr_test!( usize, 0x8000000000000000, 8, 0x0080000000000000 );
	    shr_test!( usize, 0x8000000000000000, 9, 0x0040000000000000 );
	    shr_test!( usize, 0x8000000000000000, 10, 0x0020000000000000 );
	    shr_test!( usize, 0x8000000000000000, 11, 0x0010000000000000 );
	    shr_test!( usize, 0x8000000000000000, 12, 0x0008000000000000 );
	    shr_test!( usize, 0x8000000000000000, 13, 0x0004000000000000 );
	    shr_test!( usize, 0x8000000000000000, 14, 0x0002000000000000 );
	    shr_test!( usize, 0x8000000000000000, 15, 0x0001000000000000 );
	    shr_test!( usize, 0x8000000000000000, 16, 0x0000800000000000 );
	    shr_test!( usize, 0x8000000000000000, 17, 0x0000400000000000 );
	    shr_test!( usize, 0x8000000000000000, 18, 0x0000200000000000 );
	    shr_test!( usize, 0x8000000000000000, 19, 0x0000100000000000 );
	    shr_test!( usize, 0x8000000000000000, 20, 0x0000080000000000 );
	    shr_test!( usize, 0x8000000000000000, 21, 0x0000040000000000 );
	    shr_test!( usize, 0x8000000000000000, 22, 0x0000020000000000 );
	    shr_test!( usize, 0x8000000000000000, 23, 0x0000010000000000 );
	    shr_test!( usize, 0x8000000000000000, 24, 0x0000008000000000 );
	    shr_test!( usize, 0x8000000000000000, 25, 0x0000004000000000 );
	    shr_test!( usize, 0x8000000000000000, 26, 0x0000002000000000 );
	    shr_test!( usize, 0x8000000000000000, 27, 0x0000001000000000 );
	    shr_test!( usize, 0x8000000000000000, 28, 0x0000000800000000 );
	    shr_test!( usize, 0x8000000000000000, 29, 0x0000000400000000 );
	    shr_test!( usize, 0x8000000000000000, 30, 0x0000000200000000 );
	    shr_test!( usize, 0x8000000000000000, 31, 0x0000000100000000 );
	    shr_test!( usize, 0x8000000000000000, 32, 0x0000000080000000 );
	    shr_test!( usize, 0x8000000000000000, 33, 0x0000000040000000 );
	    shr_test!( usize, 0x8000000000000000, 34, 0x0000000020000000 );
	    shr_test!( usize, 0x8000000000000000, 35, 0x0000000010000000 );
	    shr_test!( usize, 0x8000000000000000, 36, 0x0000000008000000 );
	    shr_test!( usize, 0x8000000000000000, 37, 0x0000000004000000 );
	    shr_test!( usize, 0x8000000000000000, 38, 0x0000000002000000 );
	    shr_test!( usize, 0x8000000000000000, 39, 0x0000000001000000 );
	    shr_test!( usize, 0x8000000000000000, 40, 0x0000000000800000 );
	    shr_test!( usize, 0x8000000000000000, 41, 0x0000000000400000 );
	    shr_test!( usize, 0x8000000000000000, 42, 0x0000000000200000 );
	    shr_test!( usize, 0x8000000000000000, 43, 0x0000000000100000 );
	    shr_test!( usize, 0x8000000000000000, 44, 0x0000000000080000 );
	    shr_test!( usize, 0x8000000000000000, 45, 0x0000000000040000 );
	    shr_test!( usize, 0x8000000000000000, 46, 0x0000000000020000 );
	    shr_test!( usize, 0x8000000000000000, 47, 0x0000000000010000 );
	    shr_test!( usize, 0x8000000000000000, 48, 0x0000000000008000 );
	    shr_test!( usize, 0x8000000000000000, 49, 0x0000000000004000 );
	    shr_test!( usize, 0x8000000000000000, 50, 0x0000000000002000 );
	    shr_test!( usize, 0x8000000000000000, 51, 0x0000000000001000 );
	    shr_test!( usize, 0x8000000000000000, 52, 0x0000000000000800 );
	    shr_test!( usize, 0x8000000000000000, 53, 0x0000000000000400 );
	    shr_test!( usize, 0x8000000000000000, 54, 0x0000000000000200 );
	    shr_test!( usize, 0x8000000000000000, 55, 0x0000000000000100 );
	    shr_test!( usize, 0x8000000000000000, 56, 0x0000000000000080 );
	    shr_test!( usize, 0x8000000000000000, 57, 0x0000000000000040 );
	    shr_test!( usize, 0x8000000000000000, 58, 0x0000000000000020 );
	    shr_test!( usize, 0x8000000000000000, 59, 0x0000000000000010 );
	    shr_test!( usize, 0x8000000000000000, 60, 0x0000000000000008 );
	    shr_test!( usize, 0x8000000000000000, 61, 0x0000000000000004 );
	    shr_test!( usize, 0x8000000000000000, 62, 0x0000000000000002 );
	    shr_test!( usize, 0x8000000000000000, 63, 0x0000000000000001 );
	} else {
	    shr_test!( usize, 0x80000000, 0, 0x80000000 );
	    shr_test!( usize, 0x80000000, 1, 0x40000000 );
	    shr_test!( usize, 0x80000000, 2, 0x20000000 );
	    shr_test!( usize, 0x80000000, 3, 0x10000000 );
	    shr_test!( usize, 0x80000000, 4, 0x08000000 );
	    shr_test!( usize, 0x80000000, 5, 0x04000000 );
	    shr_test!( usize, 0x80000000, 6, 0x02000000 );
	    shr_test!( usize, 0x80000000, 7, 0x01000000 );
	    shr_test!( usize, 0x80000000, 8, 0x00800000 );
	    shr_test!( usize, 0x80000000, 9, 0x00400000 );
	    shr_test!( usize, 0x80000000, 10, 0x00200000 );
	    shr_test!( usize, 0x80000000, 11, 0x00100000 );
	    shr_test!( usize, 0x80000000, 12, 0x00080000 );
	    shr_test!( usize, 0x80000000, 13, 0x00040000 );
	    shr_test!( usize, 0x80000000, 14, 0x00020000 );
	    shr_test!( usize, 0x80000000, 15, 0x00010000 );
	    shr_test!( usize, 0x80000000, 16, 0x00008000 );
	    shr_test!( usize, 0x80000000, 17, 0x00004000 );
	    shr_test!( usize, 0x80000000, 18, 0x00002000 );
	    shr_test!( usize, 0x80000000, 19, 0x00001000 );
	    shr_test!( usize, 0x80000000, 20, 0x00000800 );
	    shr_test!( usize, 0x80000000, 21, 0x00000400 );
	    shr_test!( usize, 0x80000000, 22, 0x00000200 );
	    shr_test!( usize, 0x80000000, 23, 0x00000100 );
	    shr_test!( usize, 0x80000000, 24, 0x00000080 );
	    shr_test!( usize, 0x80000000, 25, 0x00000040 );
	    shr_test!( usize, 0x80000000, 26, 0x00000020 );
	    shr_test!( usize, 0x80000000, 27, 0x00000010 );
	    shr_test!( usize, 0x80000000, 28, 0x00000008 );
	    shr_test!( usize, 0x80000000, 29, 0x00000004 );
	    shr_test!( usize, 0x80000000, 30, 0x00000002 );
	    shr_test!( usize, 0x80000000, 31, 0x00000001 );
	}
    }

    #[test]
    #[should_panic]
    fn shr_test2() {
	if cfg!(target_pointer_width = "64") {
	    shr_test!( usize, 0x8000000000000000, 64, 0x0000000000000000 ); // panicked at 'shift operation overflowed'
	} else {
	    shr_test!( usize, 0x80000000, 32, 0x00000000 ); // panicked at 'shift operation overflowed'
	}
    }
}
