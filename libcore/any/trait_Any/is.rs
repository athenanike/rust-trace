#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::any::Any;

    // pub trait Any: Reflect + 'static {
    //     /// Gets the `TypeId` of `self`.
    //     #[unstable(feature = "core",
    //                reason = "this method will likely be replaced by an associated static")]
    //     fn get_type_id(&self) -> TypeId;
    // }
    //
    // impl<T: Reflect + 'static> Any for T {
    //     fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
    // }

    // impl Any {
    //     /// Returns true if the boxed type is the same as `T`
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn is<T: Any>(&self) -> bool {
    //         // Get TypeId of the type this function is instantiated with
    //         let t = TypeId::of::<T>();
    //
    //         // Get TypeId of the type in the trait object
    //         let boxed = self.get_type_id();
    //
    //         // Compare both TypeIds on equality
    //         t == boxed
    //     }
    //
    //     /// Returns some reference to the boxed value if it is of type `T`, or
    //     /// `None` if it isn't.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
    //         if self.is::<T>() {
    //             unsafe {
    //                 // Get the raw representation of the trait object
    //                 let to: TraitObject = transmute(self);
    //
    //                 // Extract the data pointer
    //                 Some(transmute(to.data))
    //             }
    //         } else {
    //             None
    //         }
    //     }
    //
    //     /// Returns some mutable reference to the boxed value if it is of type `T`, or
    //     /// `None` if it isn't.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
    //         if self.is::<T>() {
    //             unsafe {
    //                 // Get the raw representation of the trait object
    //                 let to: TraitObject = transmute(self);
    // 
    //                 // Extract the data pointer
    //                 Some(transmute(to.data))
    //             }
    //         } else {
    //             None
    //         }
    //     }
    // }

    type T = i32;

    #[test]
    fn is_test1() {
	let x: T = 68;
	let any: &Any = &x;
	let is: bool = any.is::<T>();

	assert_eq!(is, true)
    }

    #[test]
    fn is_test2() {
	struct A;

	let x: T = 68;
	let any: &Any = &x;
	let is: bool = any.is::<A>();

	assert_eq!(is, false)
    }
}
