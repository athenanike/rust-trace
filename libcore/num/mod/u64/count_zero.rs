#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! uint_impl {
    //     ($ActualT:ty, $BITS:expr,
    //      $ctpop:path,
    //      $ctlz:path,
    //      $cttz:path,
    //      $bswap:path,
    //      $add_with_overflow:path,
    //      $sub_with_overflow:path,
    //      $mul_with_overflow:path) => {
    //         /// Returns the smallest value that can be represented by this integer type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         pub fn min_value() -> Self { 0 }
    //
    //         /// Returns the largest value that can be represented by this integer type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         pub fn max_value() -> Self { !0 }
    //
    //         /// Converts a string slice in a given base to an integer.
    //         ///
    //         /// Leading and trailing whitespace represent an error.
    //         ///
    //         /// # Arguments
    //         ///
    //         /// * src - A string slice
    //         /// * radix - The base to use. Must lie in the range [2 .. 36]
    //         ///
    //         /// # Return value
    //         ///
    //         /// `Err(ParseIntError)` if the string did not represent a valid number.
    //         /// Otherwise, `Ok(n)` where `n` is the integer represented by `src`.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[allow(deprecated)]
    //         pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
    //             from_str_radix(src, radix)
    //         }
    //
    //         /// Returns the number of ones in the binary representation of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b01001100u8;
    //         ///
    //         /// assert_eq!(n.count_ones(), 3);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn count_ones(self) -> u32 {
    //             unsafe { $ctpop(self as $ActualT) as u32 }
    //         }
    //
    //         /// Returns the number of zeros in the binary representation of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b01001100u8;
    //         ///
    //         /// assert_eq!(n.count_zeros(), 5);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn count_zeros(self) -> u32 {
    //             (!self).count_ones()
    //         }
    //
    //         /// Returns the number of leading zeros in the binary representation
    //         /// of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b0101000u16;
    //         ///
    //         /// assert_eq!(n.leading_zeros(), 10);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn leading_zeros(self) -> u32 {
    //             unsafe { $ctlz(self as $ActualT) as u32 }
    //         }
    //
    //         /// Returns the number of trailing zeros in the binary representation
    //         /// of `self`.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0b0101000u16;
    //         ///
    //         /// assert_eq!(n.trailing_zeros(), 3);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn trailing_zeros(self) -> u32 {
    //             // As of LLVM 3.6 the codegen for the zero-safe cttz8 intrinsic
    //             // emits two conditional moves on x86_64. By promoting the value to
    //             // u16 and setting bit 8, we get better code without any conditional
    //             // operations.
    //             // FIXME: There's a LLVM patch (http://reviews.llvm.org/D9284)
    //             // pending, remove this workaround once LLVM generates better code
    //             // for cttz8.
    //             unsafe {
    //                 if $BITS == 8 {
    //                     intrinsics::cttz16(self as u16 | 0x100) as u32
    //                 } else {
    //                     $cttz(self as $ActualT) as u32
    //                 }
    //             }
    //         }
    //
    //         /// Shifts the bits to the left by a specified amount, `n`,
    //         /// wrapping the truncated bits to the end of the resulting integer.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         /// let m = 0x3456789ABCDEF012u64;
    //         ///
    //         /// assert_eq!(n.rotate_left(12), m);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn rotate_left(self, n: u32) -> Self {
    //             // Protect against undefined behaviour for over-long bit shifts
    //             let n = n % $BITS;
    //             (self << n) | (self >> (($BITS - n) % $BITS))
    //         }
    //
    //         /// Shifts the bits to the right by a specified amount, `n`,
    //         /// wrapping the truncated bits to the beginning of the resulting
    //         /// integer.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         /// let m = 0xDEF0123456789ABCu64;
    //         ///
    //         /// assert_eq!(n.rotate_right(12), m);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn rotate_right(self, n: u32) -> Self {
    //             // Protect against undefined behaviour for over-long bit shifts
    //             let n = n % $BITS;
    //             (self >> n) | (self << (($BITS - n) % $BITS))
    //         }
    //
    //         /// Reverses the byte order of the integer.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         /// let m = 0xEFCDAB8967452301u64;
    //         ///
    //         /// assert_eq!(n.swap_bytes(), m);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn swap_bytes(self) -> Self {
    //             unsafe { $bswap(self as $ActualT) as Self }
    //         }
    //
    //         /// Converts an integer from big endian to the target's endianness.
    //         ///
    //         /// On big endian this is a no-op. On little endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "big") {
    //         ///     assert_eq!(u64::from_be(n), n)
    //         /// } else {
    //         ///     assert_eq!(u64::from_be(n), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn from_be(x: Self) -> Self {
    //             if cfg!(target_endian = "big") { x } else { x.swap_bytes() }
    //         }
    //
    //         /// Converts an integer from little endian to the target's endianness.
    //         ///
    //         /// On little endian this is a no-op. On big endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "little") {
    //         ///     assert_eq!(u64::from_le(n), n)
    //         /// } else {
    //         ///     assert_eq!(u64::from_le(n), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn from_le(x: Self) -> Self {
    //             if cfg!(target_endian = "little") { x } else { x.swap_bytes() }
    //         }
    //
    //         /// Converts `self` to big endian from the target's endianness.
    //         ///
    //         /// On big endian this is a no-op. On little endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "big") {
    //         ///     assert_eq!(n.to_be(), n)
    //         /// } else {
    //         ///     assert_eq!(n.to_be(), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn to_be(self) -> Self { // or not to be?
    //             if cfg!(target_endian = "big") { self } else { self.swap_bytes() }
    //         }
    //
    //         /// Converts `self` to little endian from the target's endianness.
    //         ///
    //         /// On little endian this is a no-op. On big endian the bytes are
    //         /// swapped.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// let n = 0x0123456789ABCDEFu64;
    //         ///
    //         /// if cfg!(target_endian = "little") {
    //         ///     assert_eq!(n.to_le(), n)
    //         /// } else {
    //         ///     assert_eq!(n.to_le(), n.swap_bytes())
    //         /// }
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn to_le(self) -> Self {
    //             if cfg!(target_endian = "little") { self } else { self.swap_bytes() }
    //         }
    //
    //         /// Checked integer addition. Computes `self + other`, returning `None`
    //         /// if overflow occurred.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!(5u16.checked_add(65530), Some(65535));
    //         /// assert_eq!(6u16.checked_add(65530), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_add(self, other: Self) -> Option<Self> {
    //             checked_op!($ActualT, $add_with_overflow, self, other)
    //         }
    //
    //         /// Checked integer subtraction. Computes `self - other`, returning
    //         /// `None` if underflow occurred.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!((-127i8).checked_sub(1), Some(-128));
    //         /// assert_eq!((-128i8).checked_sub(1), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_sub(self, other: Self) -> Option<Self> {
    //             checked_op!($ActualT, $sub_with_overflow, self, other)
    //         }
    //
    //         /// Checked integer multiplication. Computes `self * other`, returning
    //         /// `None` if underflow or overflow occurred.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!(5u8.checked_mul(51), Some(255));
    //         /// assert_eq!(5u8.checked_mul(52), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_mul(self, other: Self) -> Option<Self> {
    //             checked_op!($ActualT, $mul_with_overflow, self, other)
    //         }
    //
    //         /// Checked integer division. Computes `self / other`, returning `None`
    //         /// if `other == 0` or the operation results in underflow or overflow.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!((-127i8).checked_div(-1), Some(127));
    //         /// assert_eq!((-128i8).checked_div(-1), None);
    //         /// assert_eq!((1i8).checked_div(0), None);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn checked_div(self, v: Self) -> Option<Self> {
    //             match v {
    //                 0 => None,
    //                 v => Some(self / v),
    //             }
    //         }
    //
    //         /// Saturating integer addition. Computes `self + other`, saturating at
    //         /// the numeric bounds instead of overflowing.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn saturating_add(self, other: Self) -> Self {
    //             match self.checked_add(other) {
    //                 Some(x)                       => x,
    //                 None if other >= Self::zero() => Self::max_value(),
    //                 None => Self::min_value(),
    //             }
    //         }
    //
    //         /// Saturating integer subtraction. Computes `self - other`, saturating
    //         /// at the numeric bounds instead of overflowing.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn saturating_sub(self, other: Self) -> Self {
    //             match self.checked_sub(other) {
    //                 Some(x)                       => x,
    //                 None if other >= Self::zero() => Self::min_value(),
    //                 None => Self::max_value(),
    //             }
    //         }
    //
    //         /// Wrapping (modular) addition. Computes `self + other`,
    //         /// wrapping around at the boundary of the type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn wrapping_add(self, rhs: Self) -> Self {
    //             unsafe {
    //                 intrinsics::overflowing_add(self, rhs)
    //             }
    //         }
    //
    //         /// Wrapping (modular) subtraction. Computes `self - other`,
    //         /// wrapping around at the boundary of the type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn wrapping_sub(self, rhs: Self) -> Self {
    //             unsafe {
    //                 intrinsics::overflowing_sub(self, rhs)
    //             }
    //         }
    //
    //         /// Wrapping (modular) multiplication. Computes `self *
    //         /// other`, wrapping around at the boundary of the type.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn wrapping_mul(self, rhs: Self) -> Self {
    //             unsafe {
    //                 intrinsics::overflowing_mul(self, rhs)
    //             }
    //         }
    //
    //         /// Wrapping (modular) division. Computes `floor(self / other)`,
    //         /// wrapping around at the boundary of the type.
    //         ///
    //         /// The only case where such wrapping can occur is when one
    //         /// divides `MIN / -1` on a signed type (where `MIN` is the
    //         /// negative minimal value for the type); this is equivalent
    //         /// to `-MIN`, a positive value that is too large to represent
    //         /// in the type. In such a case, this function returns `MIN`
    //         /// itself..
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_div(self, rhs: Self) -> Self {
    //             self.overflowing_div(rhs).0
    //         }
    //
    //         /// Wrapping (modular) remainder. Computes `self % other`,
    //         /// wrapping around at the boundary of the type.
    //         ///
    //         /// Such wrap-around never actually occurs mathematically;
    //         /// implementation artifacts make `x % y` illegal for `MIN /
    //         /// -1` on a signed type illegal (where `MIN` is the negative
    //         /// minimal value). In such a case, this function returns `0`.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_rem(self, rhs: Self) -> Self {
    //             self.overflowing_rem(rhs).0
    //         }
    //
    //         /// Wrapping (modular) negation. Computes `-self`,
    //         /// wrapping around at the boundary of the type.
    //         ///
    //         /// The only case where such wrapping can occur is when one
    //         /// negates `MIN` on a signed type (where `MIN` is the
    //         /// negative minimal value for the type); this is a positive
    //         /// value that is too large to represent in the type. In such
    //         /// a case, this function returns `MIN` itself.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_neg(self) -> Self {
    //             self.overflowing_neg().0
    //         }
    //
    //         /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    //         /// where `mask` removes any high-order bits of `rhs` that
    //         /// would cause the shift to exceed the bitwidth of the type.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_shl(self, rhs: u32) -> Self {
    //             self.overflowing_shl(rhs).0
    //         }
    //
    //         /// Panic-free bitwise shift-left; yields `self >> mask(rhs)`,
    //         /// where `mask` removes any high-order bits of `rhs` that
    //         /// would cause the shift to exceed the bitwidth of the type.
    //         #[unstable(feature = "core", since = "1.0.0")]
    //         #[inline(always)]
    //         pub fn wrapping_shr(self, rhs: u32) -> Self {
    //             self.overflowing_shr(rhs).0
    //         }
    //
    //         /// Raises self to the power of `exp`, using exponentiation by squaring.
    //         ///
    //         /// # Examples
    //         ///
    //         /// ```rust
    //         /// assert_eq!(2i32.pow(4), 16);
    //         /// ```
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn pow(self, mut exp: u32) -> Self {
    //             let mut base = self;
    //             let mut acc = Self::one();
    //
    //             let mut prev_base = self;
    //             let mut base_oflo = false;
    //             while exp > 0 {
    //                 if (exp & 1) == 1 {
    //                     if base_oflo {
    //                         // ensure overflow occurs in the same manner it
    //                         // would have otherwise (i.e. signal any exception
    //                         // it would have otherwise).
    //                         acc = acc * (prev_base * prev_base);
    //                     } else {
    //                         acc = acc * base;
    //                     }
    //                 }
    //                 prev_base = base;
    //                 let (new_base, new_base_oflo) = base.overflowing_mul(base);
    //                 base = new_base;
    //                 base_oflo = new_base_oflo;
    //                 exp /= 2;
    //             }
    //             acc
    //         }
    //
    //         /// Returns `true` iff `self == 2^k` for some `k`.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn is_power_of_two(self) -> bool {
    //             (self.wrapping_sub(Self::one())) & self == Self::zero() &&
    //                 !(self == Self::zero())
    //         }
    //
    //         /// Returns the smallest power of two greater than or equal to `self`.
    //         /// Unspecified behavior on overflow.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[inline]
    //         pub fn next_power_of_two(self) -> Self {
    //             let bits = size_of::<Self>() * 8;
    //             let one: Self = Self::one();
    //             one << ((bits - self.wrapping_sub(one).leading_zeros() as usize) % bits)
    //         }
    //
    //         /// Returns the smallest power of two greater than or equal to `n`. If
    //         /// the next power of two is greater than the type's maximum value,
    //         /// `None` is returned, otherwise the power of two is wrapped in `Some`.
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         pub fn checked_next_power_of_two(self) -> Option<Self> {
    //             let npot = self.next_power_of_two();
    //             if npot >= self {
    //                 Some(npot)
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // }

    // impl u64 {
    //     uint_impl! { u64, 64,
    //         intrinsics::ctpop64,
    //         intrinsics::ctlz64,
    //         intrinsics::cttz64,
    //         intrinsics::bswap64,
    //         intrinsics::u64_add_with_overflow,
    //         intrinsics::u64_sub_with_overflow,
    //         intrinsics::u64_mul_with_overflow }
    // }

    macro_rules! count_zeros_test {
	($value:expr, $zeros:expr) => ({
	    let value: u64 = $value;
	    let result: u32 = value.count_zeros();

	    assert_eq!(result, $zeros);
	})
    }

    #[test]
    fn count_zeros_test1() {
	count_zeros_test!( 0x0000000000000000, 64 );
	count_zeros_test!( 0x0000000000000001, 63 );
	count_zeros_test!( 0x0000000000000003, 62 );
	count_zeros_test!( 0x0000000000000007, 61 );
	count_zeros_test!( 0x000000000000000f, 60 );
	count_zeros_test!( 0x000000000000001f, 59 );
	count_zeros_test!( 0x000000000000003f, 58 );
	count_zeros_test!( 0x000000000000007f, 57 );
	count_zeros_test!( 0x00000000000000ff, 56 );
	count_zeros_test!( 0x00000000000001ff, 55 );
	count_zeros_test!( 0x00000000000003ff, 54 );
	count_zeros_test!( 0x00000000000007ff, 53 );
	count_zeros_test!( 0x0000000000000fff, 52 );
	count_zeros_test!( 0x0000000000001fff, 51 );
	count_zeros_test!( 0x0000000000003fff, 50 );
	count_zeros_test!( 0x0000000000007fff, 49 );
	count_zeros_test!( 0x000000000000ffff, 48 );
	count_zeros_test!( 0x000000000001ffff, 47 );
	count_zeros_test!( 0x000000000003ffff, 46 );
	count_zeros_test!( 0x000000000007ffff, 45 );
	count_zeros_test!( 0x00000000000fffff, 44 );
	count_zeros_test!( 0x00000000001fffff, 43 );
	count_zeros_test!( 0x00000000003fffff, 42 );
	count_zeros_test!( 0x00000000007fffff, 41 );
	count_zeros_test!( 0x0000000000ffffff, 40 );
	count_zeros_test!( 0x0000000001ffffff, 39 );
	count_zeros_test!( 0x0000000003ffffff, 38 );
	count_zeros_test!( 0x0000000007ffffff, 37 );
	count_zeros_test!( 0x000000000fffffff, 36 );
	count_zeros_test!( 0x000000001fffffff, 35 );
	count_zeros_test!( 0x000000003fffffff, 34 );
	count_zeros_test!( 0x000000007fffffff, 33 );
	count_zeros_test!( 0x00000000ffffffff, 32 );
	count_zeros_test!( 0x00000001ffffffff, 31 );
	count_zeros_test!( 0x00000003ffffffff, 30 );
	count_zeros_test!( 0x00000007ffffffff, 29 );
	count_zeros_test!( 0x0000000fffffffff, 28 );
	count_zeros_test!( 0x0000001fffffffff, 27 );
	count_zeros_test!( 0x0000003fffffffff, 26 );
	count_zeros_test!( 0x0000007fffffffff, 25 );
	count_zeros_test!( 0x000000ffffffffff, 24 );
	count_zeros_test!( 0x000001ffffffffff, 23 );
	count_zeros_test!( 0x000003ffffffffff, 22 );
	count_zeros_test!( 0x000007ffffffffff, 21 );
	count_zeros_test!( 0x00000fffffffffff, 20 );
	count_zeros_test!( 0x00001fffffffffff, 19 );
	count_zeros_test!( 0x00003fffffffffff, 18 );
	count_zeros_test!( 0x00007fffffffffff, 17 );
	count_zeros_test!( 0x0000ffffffffffff, 16 );
	count_zeros_test!( 0x0001ffffffffffff, 15 );
	count_zeros_test!( 0x0003ffffffffffff, 14 );
	count_zeros_test!( 0x0007ffffffffffff, 13 );
	count_zeros_test!( 0x000fffffffffffff, 12 );
	count_zeros_test!( 0x001fffffffffffff, 11 );
	count_zeros_test!( 0x003fffffffffffff, 10 );
	count_zeros_test!( 0x007fffffffffffff, 9 );
	count_zeros_test!( 0x00ffffffffffffff, 8 );
	count_zeros_test!( 0x01ffffffffffffff, 7 );
	count_zeros_test!( 0x03ffffffffffffff, 6 );
	count_zeros_test!( 0x07ffffffffffffff, 5 );
	count_zeros_test!( 0x0fffffffffffffff, 4 );
	count_zeros_test!( 0x1fffffffffffffff, 3 );
	count_zeros_test!( 0x3fffffffffffffff, 2 );
	count_zeros_test!( 0x7fffffffffffffff, 1 );
	count_zeros_test!( 0xffffffffffffffff, 0 );
    }
}
