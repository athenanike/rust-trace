#![feature(core, core_float, float_from_str_radix)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Float;
    use core::num::ParseFloatError;

    // macro_rules! from_str_radix_float_impl {
    //     ($T:ty) => {
    //         fn from_str_radix(src: &str, radix: u32)
    //                           -> Result<$T, ParseFloatError> {
    //             use num::FloatErrorKind::*;
    //             use num::ParseFloatError as PFE;
    //
    //             // Special values
    //             match src {
    //                 "inf"   => return Ok(Float::infinity()),
    //                 "-inf"  => return Ok(Float::neg_infinity()),
    //                 "NaN"   => return Ok(Float::nan()),
    //                 _       => {},
    //             }
    //
    //             let (is_positive, src) =  match src.slice_shift_char() {
    //                 None             => return Err(PFE { __kind: Empty }),
    //                 Some(('-', ""))  => return Err(PFE { __kind: Empty }),
    //                 Some(('-', src)) => (false, src),
    //                 Some((_, _))     => (true,  src),
    //             };
    //
    //             // The significand to accumulate
    //             let mut sig = if is_positive { 0.0 } else { -0.0 };
    //             // Necessary to detect overflow
    //             let mut prev_sig = sig;
    //             let mut cs = src.chars().enumerate();
    //             // Exponent prefix and exponent index offset
    //             let mut exp_info = None::<(char, usize)>;
    //
    //             // Parse the integer part of the significand
    //             for (i, c) in cs.by_ref() {
    //                 match c.to_digit(radix) {
    //                     Some(digit) => {
    //                         // shift significand one digit left
    //                         sig = sig * (radix as $T);
    //
    //                         // add/subtract current digit depending on sign
    //                         if is_positive {
    //                             sig = sig + ((digit as isize) as $T);
    //                         } else {
    //                             sig = sig - ((digit as isize) as $T);
    //                         }
    //
    //                         // Detect overflow by comparing to last value, except
    //                         // if we've not seen any non-zero digits.
    //                         if prev_sig != 0.0 {
    //                             if is_positive && sig <= prev_sig
    //                                 { return Ok(Float::infinity()); }
    //                             if !is_positive && sig >= prev_sig
    //                                 { return Ok(Float::neg_infinity()); }
    //
    //                             // Detect overflow by reversing the shift-and-add process
    //                             if is_positive && (prev_sig != (sig - digit as $T) / radix as $T)
    //                                 { return Ok(Float::infinity()); }
    //                             if !is_positive && (prev_sig != (sig + digit as $T) / radix as $T)
    //                                 { return Ok(Float::neg_infinity()); }
    //                         }
    //                         prev_sig = sig;
    //                     },
    //                     None => match c {
    //                         'e' | 'E' | 'p' | 'P' => {
    //                             exp_info = Some((c, i + 1));
    //                             break;  // start of exponent
    //                         },
    //                         '.' => {
    //                             break;  // start of fractional part
    //                         },
    //                         _ => {
    //                             return Err(PFE { __kind: Invalid });
    //                         },
    //                     },
    //                 }
    //             }
    //
    //             // If we are not yet at the exponent parse the fractional
    //             // part of the significand
    //             if exp_info.is_none() {
    //                 let mut power = 1.0;
    //                 for (i, c) in cs.by_ref() {
    //                     match c.to_digit(radix) {
    //                         Some(digit) => {
    //                             // Decrease power one order of magnitude
    //                             power = power / (radix as $T);
    //                             // add/subtract current digit depending on sign
    //                             sig = if is_positive {
    //                                 sig + (digit as $T) * power
    //                             } else {
    //                                 sig - (digit as $T) * power
    //                             };
    //                             // Detect overflow by comparing to last value
    //                             if is_positive && sig < prev_sig
    //                                 { return Ok(Float::infinity()); }
    //                             if !is_positive && sig > prev_sig
    //                                 { return Ok(Float::neg_infinity()); }
    //                             prev_sig = sig;
    //                         },
    //                         None => match c {
    //                             'e' | 'E' | 'p' | 'P' => {
    //                                 exp_info = Some((c, i + 1));
    //                                 break; // start of exponent
    //                             },
    //                             _ => {
    //                                 return Err(PFE { __kind: Invalid });
    //                             },
    //                         },
    //                     }
    //                 }
    //             }
    //
    //             // Parse and calculate the exponent
    //             let exp = match exp_info {
    //                 Some((c, offset)) => {
    //                     let base = match c {
    //                         'E' | 'e' if radix == 10 => 10.0,
    //                         'P' | 'p' if radix == 16 => 2.0,
    //                         _ => return Err(PFE { __kind: Invalid }),
    //                     };
    //
    //                     // Parse the exponent as decimal integer
    //                     let src = &src[offset..];
    //                     let (is_positive, exp) = match src.slice_shift_char() {
    //                         Some(('-', src)) => (false, src.parse::<usize>()),
    //                         Some(('+', src)) => (true,  src.parse::<usize>()),
    //                         Some((_, _))     => (true,  src.parse::<usize>()),
    //                         None             => return Err(PFE { __kind: Invalid }),
    //                     };
    //
    //                     match (is_positive, exp) {
    //                         (true,  Ok(exp)) => base.powi(exp as i32),
    //                         (false, Ok(exp)) => 1.0 / base.powi(exp as i32),
    //                         (_, Err(_))      => return Err(PFE { __kind: Invalid }),
    //                     }
    //                 },
    //                 None => 1.0, // no exponent
    //             };
    //
    //             Ok(sig * exp)
    //         }
    //     }
    // }

    // impl Float for f64 {
    //     #[inline]
    //     fn nan() -> f64 { NAN }
    //
    //     #[inline]
    //     fn infinity() -> f64 { INFINITY }
    //
    //     #[inline]
    //     fn neg_infinity() -> f64 { NEG_INFINITY }
    //
    //     #[inline]
    //     fn zero() -> f64 { 0.0 }
    //
    //     #[inline]
    //     fn neg_zero() -> f64 { -0.0 }
    //
    //     #[inline]
    //     fn one() -> f64 { 1.0 }
    //
    //     from_str_radix_float_impl! { f64 }
    //
    //     /// Returns `true` if the number is NaN.
    //     #[inline]
    //     fn is_nan(self) -> bool { self != self }
    //
    //     /// Returns `true` if the number is infinite.
    //     #[inline]
    //     fn is_infinite(self) -> bool {
    //         self == Float::infinity() || self == Float::neg_infinity()
    //     }
    //
    //     /// Returns `true` if the number is neither infinite or NaN.
    //     #[inline]
    //     fn is_finite(self) -> bool {
    //         !(self.is_nan() || self.is_infinite())
    //     }
    //
    //     /// Returns `true` if the number is neither zero, infinite, subnormal or NaN.
    //     #[inline]
    //     fn is_normal(self) -> bool {
    //         self.classify() == Fp::Normal
    //     }
    //
    //     /// Returns the floating point category of the number. If only one property
    //     /// is going to be tested, it is generally faster to use the specific
    //     /// predicate instead.
    //     fn classify(self) -> Fp {
    //         const EXP_MASK: u64 = 0x7ff0000000000000;
    //         const MAN_MASK: u64 = 0x000fffffffffffff;
    // 
    //         let bits: u64 = unsafe { mem::transmute(self) };
    //         match (bits & MAN_MASK, bits & EXP_MASK) {
    //             (0, 0)        => Fp::Zero,
    //             (_, 0)        => Fp::Subnormal,
    //             (0, EXP_MASK) => Fp::Infinite,
    //             (_, EXP_MASK) => Fp::Nan,
    //             _             => Fp::Normal,
    //         }
    //     }
    //
    //     /// Returns the mantissa, exponent and sign as integers.
    //     fn integer_decode(self) -> (u64, i16, i8) {
    //         let bits: u64 = unsafe { mem::transmute(self) };
    //         let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    //         let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    //         let mantissa = if exponent == 0 {
    //             (bits & 0xfffffffffffff) << 1
    //         } else {
    //             (bits & 0xfffffffffffff) | 0x10000000000000
    //         };
    //         // Exponent bias + mantissa shift
    //         exponent -= 1023 + 52;
    //         (mantissa, exponent, sign)
    //     }
    //
    //     /// Rounds towards minus infinity.
    //     #[inline]
    //     fn floor(self) -> f64 {
    //         unsafe { intrinsics::floorf64(self) }
    //     }
    //
    //     /// Rounds towards plus infinity.
    //     #[inline]
    //     fn ceil(self) -> f64 {
    //         unsafe { intrinsics::ceilf64(self) }
    //     }
    //
    //     /// Rounds to nearest integer. Rounds half-way cases away from zero.
    //     #[inline]
    //     fn round(self) -> f64 {
    //         unsafe { intrinsics::roundf64(self) }
    //     }
    //
    //     /// Returns the integer part of the number (rounds towards zero).
    //     #[inline]
    //     fn trunc(self) -> f64 {
    //         unsafe { intrinsics::truncf64(self) }
    //     }
    //
    //     /// The fractional part of the number, satisfying:
    //     ///
    //     /// ```
    //     /// let x = 1.65f64;
    //     /// assert!(x == x.trunc() + x.fract())
    //     /// ```
    //     #[inline]
    //     fn fract(self) -> f64 { self - self.trunc() }
    //
    //     /// Computes the absolute value of `self`. Returns `Float::nan()` if the
    //     /// number is `Float::nan()`.
    //     #[inline]
    //     fn abs(self) -> f64 {
    //         unsafe { intrinsics::fabsf64(self) }
    //     }
    //
    //     /// Returns a number that represents the sign of `self`.
    //     ///
    //     /// - `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    //     /// - `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    //     /// - `Float::nan()` if the number is `Float::nan()`
    //     #[inline]
    //     fn signum(self) -> f64 {
    //         if self.is_nan() {
    //             Float::nan()
    //         } else {
    //             unsafe { intrinsics::copysignf64(1.0, self) }
    //         }
    //     }
    //
    //     /// Returns `true` if `self` is positive, including `+0.0` and
    //     /// `Float::infinity()`.
    //     #[inline]
    //     fn is_positive(self) -> bool {
    //         self > 0.0 || (1.0 / self) == Float::infinity()
    //     }
    //
    //     /// Returns `true` if `self` is negative, including `-0.0` and
    //     /// `Float::neg_infinity()`.
    //     #[inline]
    //     fn is_negative(self) -> bool {
    //         self < 0.0 || (1.0 / self) == Float::neg_infinity()
    //     }
    //
    //     /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    //     /// error. This produces a more accurate result with better performance than
    //     /// a separate multiplication operation followed by an add.
    //     #[inline]
    //     fn mul_add(self, a: f64, b: f64) -> f64 {
    //         unsafe { intrinsics::fmaf64(self, a, b) }
    //     }
    //
    //     /// Returns the reciprocal (multiplicative inverse) of the number.
    //     #[inline]
    //     fn recip(self) -> f64 { 1.0 / self }
    //
    //     #[inline]
    //     fn powf(self, n: f64) -> f64 {
    //         unsafe { intrinsics::powf64(self, n) }
    //     }
    //
    //     #[inline]
    //     fn powi(self, n: i32) -> f64 {
    //         unsafe { intrinsics::powif64(self, n) }
    //     }
    //
    //     #[inline]
    //     fn sqrt(self) -> f64 {
    //         if self < 0.0 {
    //             NAN
    //         } else {
    //             unsafe { intrinsics::sqrtf64(self) }
    //         }
    //     }
    //
    //     #[inline]
    //     fn rsqrt(self) -> f64 { self.sqrt().recip() }
    //
    //     /// Returns the exponential of the number.
    //     #[inline]
    //     fn exp(self) -> f64 {
    //         unsafe { intrinsics::expf64(self) }
    //     }
    //
    //     /// Returns 2 raised to the power of the number.
    //     #[inline]
    //     fn exp2(self) -> f64 {
    //         unsafe { intrinsics::exp2f64(self) }
    //     }
    //
    //     /// Returns the natural logarithm of the number.
    //     #[inline]
    //     fn ln(self) -> f64 {
    //         unsafe { intrinsics::logf64(self) }
    //     }
    //
    //     /// Returns the logarithm of the number with respect to an arbitrary base.
    //     #[inline]
    //     fn log(self, base: f64) -> f64 { self.ln() / base.ln() }
    //
    //     /// Returns the base 2 logarithm of the number.
    //     #[inline]
    //     fn log2(self) -> f64 {
    //         unsafe { intrinsics::log2f64(self) }
    //     }
    //
    //     /// Returns the base 10 logarithm of the number.
    //     #[inline]
    //     fn log10(self) -> f64 {
    //         unsafe { intrinsics::log10f64(self) }
    //     }
    //
    //     /// Converts to degrees, assuming the number is in radians.
    //     #[inline]
    //     fn to_degrees(self) -> f64 { self * (180.0f64 / consts::PI) }
    //
    //     /// Converts to radians, assuming the number is in degrees.
    //     #[inline]
    //     fn to_radians(self) -> f64 {
    //         let value: f64 = consts::PI;
    //         self * (value / 180.0)
    //     }
    // }

    #[test]
    fn from_str_radix_test1() {
	let src: &'static str = "inf";
	for radix in 2..33u32 {
	    let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);
	    assert_eq!(result, Ok(f64::infinity()));
	}
    }

    #[test]
    fn from_str_radix_test2() {
	let src: &'static str = "-inf";
	for radix in 2..33u32 {
	    let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);
	    assert_eq!(result, Ok(f64::neg_infinity()));
	}
    }

    #[test]
    fn from_str_radix_test3() {
	let src: &'static str = "NaN";
	for radix in 2..33u32 {
	    let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	    match result {
		Ok(v) => assert_eq!(v.is_nan(), true),
		Err(_) => assert!(false)
	    }
	}
    }

    #[test]
    fn from_str_radix_test4() {
	let src: &'static str = "";
	for radix in 2..33u32 {
	    let from_str_radix: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);
	    let parsefloaterror: ParseFloatError = from_str_radix.unwrap_err();
	    let message: String = format!("{}", parsefloaterror);

	    assert_eq!(message, "cannot parse float from empty string");
	}
    }

    #[test]
    fn from_str_radix_test5() {
	let src: &'static str = "-";
	for radix in 2..33u32 {
	    let from_str_radix: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);
	    let parsefloaterror: ParseFloatError = from_str_radix.unwrap_err();
	    let message: String = format!("{}", parsefloaterror);

	    assert_eq!(message, "cannot parse float from empty string");
	}
    }

    #[test]
    fn from_str_radix_test6() {
	let src: &'static str = "Hello, World!";
	for radix in 2..33u32 {
	    let from_str_radix: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);
	    let parsefloaterror: ParseFloatError = from_str_radix.unwrap_err();
	    let message: String = format!("{}", parsefloaterror);

	    assert_eq!(message, "invalid float literal");
	}
    }

    #[test]
    fn from_str_radix_test7() {
	let radix: u32 = 10;

	let src: &'static str = "1.23e4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.23e4),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "1.23E4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.23e4),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_radix_test8() {
	let radix: u32 = 10;

	let src: &'static str = "1.23e+4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.23e+4),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "1.23E+4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.23e+4),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_radix_test9() {
	let radix: u32 = 10;

	let src: &'static str = "1.23e-4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.23e-4),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "1.23E-4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);
	match result {
	    Ok(v) => assert_eq!(v, 1.23e-4),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_radix_test10() {
	let radix: u32 = 16;

	let src: &'static str = "1.23p4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.13671875_f64 * 2_f64 * 2_f64 * 2_f64 * 2_f64),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "1.23P4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.13671875_f64 * 2_f64 * 2_f64 * 2_f64 * 2_f64),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_radix_test11() {
	let radix: u32 = 16;

	let src: &'static str = "1.23p+4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.13671875_f64 * 2_f64 * 2_f64 * 2_f64 * 2_f64),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "1.23P+4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.13671875_f64 * 2_f64 * 2_f64 * 2_f64 * 2_f64),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_radix_test12() {
	let radix: u32 = 16;

	let src: &'static str = "1.23p-4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.13671875_f64 / 2_f64 / 2_f64 / 2_f64 / 2_f64),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "1.23P-4";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, 1.13671875_f64 / 2_f64 / 2_f64 / 2_f64 / 2_f64),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_radix_test13() {
	let radix: u32 = 10;

	let src: &'static str = "1.8e+308";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, f64::infinity()),
	    Err(_) => assert!(false)
	}

	let src: &'static str = "-1.8E+308";
	let result: Result<f64, ParseFloatError> = f64::from_str_radix(src, radix);

	match result {
	    Ok(v) => assert_eq!(v, f64::neg_infinity()),
	    Err(_) => assert!(false)
	}
    }
}
