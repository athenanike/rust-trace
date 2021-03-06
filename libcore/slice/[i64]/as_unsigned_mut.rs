#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::IntSliceExt;

    // pub trait IntSliceExt<U, S> {
    //     /// Converts the slice to an immutable slice of unsigned integers with the same width.
    //     fn as_unsigned<'a>(&'a self) -> &'a [U];
    //     /// Converts the slice to an immutable slice of signed integers with the same width.
    //     fn as_signed<'a>(&'a self) -> &'a [S];
    //
    //     /// Converts the slice to a mutable slice of unsigned integers with the same width.
    //     fn as_unsigned_mut<'a>(&'a mut self) -> &'a mut [U];
    //     /// Converts the slice to a mutable slice of signed integers with the same width.
    //     fn as_signed_mut<'a>(&'a mut self) -> &'a mut [S];
    // }

    // macro_rules! impl_int_slice {
    //     ($u:ty, $s:ty, $t:ty) => {
    //         #[unstable(feature = "core")]
    //         impl IntSliceExt<$u, $s> for [$t] {
    //             #[inline]
    //             fn as_unsigned(&self) -> &[$u] { unsafe { transmute(self) } }
    //             #[inline]
    //             fn as_signed(&self) -> &[$s] { unsafe { transmute(self) } }
    //             #[inline]
    //             fn as_unsigned_mut(&mut self) -> &mut [$u] { unsafe { transmute(self) } }
    //             #[inline]
    //             fn as_signed_mut(&mut self) -> &mut [$s] { unsafe { transmute(self) } }
    //         }
    //     }
    // }

    // macro_rules! impl_int_slices {
    //     ($u:ty, $s:ty) => {
    //         impl_int_slice! { $u, $s, $u }
    //         impl_int_slice! { $u, $s, $s }
    //     }
    // }

    // impl_int_slices! { u64,  i64 }

    type U = u64;
    type S = i64;
    type T = S;

    #[test]
    fn as_unsigned_mut_test1() {
	let slice: &mut [T] = &mut [0];

	{
	    let as_unsigned_mut: &mut [U] = slice.as_unsigned_mut();
	    as_unsigned_mut[0] = 0xffffffffffffffff;
	}

	assert_eq!(slice, &mut[-1]);
    }
}
