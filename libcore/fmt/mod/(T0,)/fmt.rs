#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! peel {
    //     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
    // }

    // macro_rules! tuple {
    //     () => ();
    //     ( $($name:ident,)+ ) => (
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<$($name:Debug),*> Debug for ($($name,)*) {
    //             #[allow(non_snake_case, unused_assignments)]
    //             fn fmt(&self, f: &mut Formatter) -> Result {
    //                 try!(write!(f, "("));
    //                 let ($(ref $name,)*) = *self;
    //                 let mut n = 0;
    //                 $(
    //                     if n > 0 {
    //                         try!(write!(f, ", "));
    //                     }
    //                     try!(write!(f, "{:?}", *$name));
    //                     n += 1;
    //                 )*
    //                 if n == 1 {
    //                     try!(write!(f, ","));
    //                 }
    //                 write!(f, ")")
    //             }
    //         }
    //         peel! { $($name,)* }
    //     )
    // }

    // tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

    type T0 = u8;

    #[test]
    fn fmt_test1() {
	let value: (T0,) = (
	    b'h' as T0,
	);
	let output: String = format!("{:?}", value);

	assert_eq!(output, "(104,)".to_string());
    }
}
