#![feature(core, wrapping)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::wrapping::OverflowingOps;

    // mod shift_max {
    //     #![allow(non_upper_case_globals)]
    //
    //     pub const  i8: u32 = (1 << 3) - 1;
    //     pub const i16: u32 = (1 << 4) - 1;
    //     pub const i32: u32 = (1 << 5) - 1;
    //     pub const i64: u32 = (1 << 6) - 1;
    //
    //     pub const  u8: u32 = i8;
    //     pub const u16: u32 = i16;
    //     pub const u32: u32 = i32;
    //     pub const u64: u32 = i64;
    // }

    // macro_rules! signed_overflowing_impl {
    //     ($($t:ident)*) => ($(
    //         impl OverflowingOps for $t {
    //             #[inline(always)]
    //             fn overflowing_add(self, rhs: $t) -> ($t, bool) {
    //                 unsafe {
    //                     concat_idents!($t, _add_with_overflow)(self, rhs)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn overflowing_sub(self, rhs: $t) -> ($t, bool) {
    //                 unsafe {
    //                     concat_idents!($t, _sub_with_overflow)(self, rhs)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn overflowing_mul(self, rhs: $t) -> ($t, bool) {
    //                 unsafe {
    //                     concat_idents!($t, _mul_with_overflow)(self, rhs)
    //                 }
    //             }
    //
    //             #[inline(always)]
    //             fn overflowing_div(self, rhs: $t) -> ($t, bool) {
    //                 if self == $t::MIN && rhs == -1 {
    //                     (self, true)
    //                 } else {
    //                     (self/rhs, false)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn overflowing_rem(self, rhs: $t) -> ($t, bool) {
    //                 if self == $t::MIN && rhs == -1 {
    //                     (0, true)
    //                 } else {
    //                     (self % rhs, false)
    //                 }
    //             }
    //
    //             #[inline(always)]
    //             fn overflowing_shl(self, rhs: u32) -> ($t, bool) {
    //                 (self << (rhs & self::shift_max::$t),
    //                  (rhs > self::shift_max::$t))
    //             }
    //             #[inline(always)]
    //             fn overflowing_shr(self, rhs: u32) -> ($t, bool) {
    //                 (self >> (rhs & self::shift_max::$t),
    //                  (rhs > self::shift_max::$t))
    //             }
    //
    //             #[inline(always)]
    //             fn overflowing_neg(self) -> ($t, bool) {
    //                 if self == $t::MIN {
    //                     ($t::MIN, true)
    //                 } else {
    //                     (-self, false)
    //                 }
    //             }
    //         }
    //     )*)
    // }

    // signed_overflowing_impl! { i8 i16 i32 i64 }

    macro_rules! overflowing_add_test {
	($T:ty, $value:expr, $rhs:expr, $result:expr) => ({
	    let value: $T = $value;
	    let rhs: $T = $rhs;
	    let result: ($T, bool) = value.overflowing_add(rhs);

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn overflowing_add_test1() {
	overflowing_add_test!( i32, 0x7fff0000, 0x0000ffff, (0x7fffffff, false) );
	overflowing_add_test!( i32, 0x7fffffff, 0x00000001, (0x80000000, true) );
    }
}
