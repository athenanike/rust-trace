#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    //     impl<_R> PartialEq for extern "C" fn() -> _R {
    //         #[inline]
    //         fn eq(&self, other: &extern "C" fn() -> _R) -> bool {
    //             let self_: *const () = unsafe { mem::transmute(*self) };
    //             let other_: *const () = unsafe { mem::transmute(*other) };
    //             self_ == other_
    //         }
    //     }

    //     impl<_R> PartialEq for extern "C" fn() -> _R {
    //         #[inline]
    //         fn eq(&self, other: &extern "C" fn() -> _R) -> bool {
    //             let self_: *const () = unsafe { mem::transmute(*self) };
    //             let other_: *const () = unsafe { mem::transmute(*other) };
    //             self_ == other_
    //         }
    //     }
    //     macro_rules! fnptreq {
    //         ($($p:ident),*) => {
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<_R,$($p),*> PartialEq for extern "C" fn($($p),*) -> _R {
    //                 #[inline]
    //                 fn eq(&self, other: &extern "C" fn($($p),*) -> _R) -> bool {
    //                     let self_: *const () = unsafe { mem::transmute(*self) };
    // 
    //                     let other_: *const () = unsafe { mem::transmute(*other) };
    //                     self_ == other_
    //                 }
    //             }
    //         }
    //     }
    //     fnptreq! { A }
    //     fnptreq! { A,B }
    //     fnptreq! { A,B,C }
    //     fnptreq! { A,B,C,D }
    //     fnptreq! { A,B,C,D,E }

    macro_rules! ne_check {
	($($p:ident),*) => ({
	    extern "C" fn bar<$($p),*>($(_: $p),*) -> () {}
	    extern "C" fn foo<$($p),*>($(_: $p),*) -> () {}

	    let f1: extern "C" fn($($p),*) -> () = bar::<$($p),*>;
	    let f2: extern "C" fn($($p),*) -> () = foo::<$($p),*>;

	    assert_eq!(f1.ne(&f1), false);
	    assert_eq!(f1.ne(&f2), true);
	})
    }

    type T = i32;
    type A = T;
    type B = T;
    type C = T;
    type D = T;
    type E = T;

    #[test]
    fn ne_test1() {
	ne_check! { };
	ne_check! { A };
	ne_check! { A,B };
	ne_check! { A,B,C };
	ne_check! { A,B,C,D };
	ne_check! { A,B,C,D,E };
    }
}
