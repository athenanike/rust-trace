#![feature(alloc, arc_counts, arc_unique, arc_weak)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::arc::Arc;
    use alloc::arc::Weak;

    use std::thread;

    // pub struct Arc<T: ?Sized> {
    //     // FIXME #12808: strange name to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _ptr: NonZero<*mut ArcInner<T>>,
    // }

    // unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> { }
    // unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> { }

    // impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Arc<U>> for Arc<T> {}

    // impl<T> Arc<T> {
    //     /// Constructs a new `Arc<T>`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new(data: T) -> Arc<T> {
    //         // Start the weak pointer count as 1 which is the weak pointer that's
    //         // held by all the strong pointers (kinda), see std/rc.rs for more info
    //         let x: Box<_> = box ArcInner {
    //             strong: atomic::AtomicUsize::new(1),
    //             weak: atomic::AtomicUsize::new(1),
    //             data: data,
    //         };
    //         Arc { _ptr: unsafe { NonZero::new(mem::transmute(x)) } }
    //     }
    // }

    // impl<T: ?Sized> Arc<T> {
    //     /// Downgrades the `Arc<T>` to a `Weak<T>` reference.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(arc_weak)]
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// let weak_five = five.downgrade();
    //     /// ```
    //     #[unstable(feature = "arc_weak",
    //                reason = "Weak pointers may not belong in this module.")]
    //     pub fn downgrade(&self) -> Weak<T> {
    //         loop {
    //             // This Relaxed is OK because we're checking the value in the CAS
    //             // below.
    //             let cur = self.inner().weak.load(Relaxed);
    //
    //             // check if the weak counter is currently "locked"; if so, spin.
    //             if cur == usize::MAX { continue }
    //
    //             // NOTE: this code currently ignores the possibility of overflow
    //             // into usize::MAX; in general both Rc and Arc need to be adjusted
    //             // to deal with overflow.
    //
    //             // Unlike with Clone(), we need this to be an Acquire read to
    //             // synchronize with the write coming from `is_unique`, so that the
    //             // events prior to that write happen before this read.
    //             if self.inner().weak.compare_and_swap(cur, cur + 1, Acquire) == cur {
    //                 return Weak { _ptr: self._ptr }
    //             }
    //         }
    //     }
    //
    //     /// Get the number of weak references to this value.
    //     #[inline]
    //     #[unstable(feature = "arc_counts")]
    //     pub fn weak_count(this: &Arc<T>) -> usize {
    //         this.inner().weak.load(SeqCst) - 1
    //     }
    //
    //     /// Get the number of strong references to this value.
    //     #[inline]
    //     #[unstable(feature = "arc_counts")]
    //     pub fn strong_count(this: &Arc<T>) -> usize {
    //         this.inner().strong.load(SeqCst)
    //     }
    //
    //     #[inline]
    //     fn inner(&self) -> &ArcInner<T> {
    //         // This unsafety is ok because while this arc is alive we're guaranteed
    //         // that the inner pointer is valid. Furthermore, we know that the
    //         // `ArcInner` structure itself is `Sync` because the inner data is
    //         // `Sync` as well, so we're ok loaning out an immutable pointer to these
    //         // contents.
    //         unsafe { &**self._ptr }
    //     }
    //
    //     // Non-inlined part of `drop`.
    //     #[inline(never)]
    //     unsafe fn drop_slow(&mut self) {
    //         let ptr = *self._ptr;
    //
    //         // Destroy the data at this time, even though we may not free the box
    //         // allocation itself (there may still be weak pointers lying around).
    //         drop_in_place(&mut (*ptr).data);
    //
    //         if self.inner().weak.fetch_sub(1, Release) == 1 {
    //             atomic::fence(Acquire);
    //             deallocate(ptr as *mut u8, size_of_val(&*ptr), align_of_val(&*ptr))
    //         }
    //     }
    // }

    // impl<T: Clone> Arc<T> {
    //     /// Make a mutable reference from the given `Arc<T>`.
    //     ///
    //     /// This is also referred to as a copy-on-write operation because the inner
    //     /// data is cloned if the (strong) reference count is greater than one. If
    //     /// we hold the only strong reference, any existing weak references will no
    //     /// longer be upgradeable.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(arc_unique)]
    //     /// use std::sync::Arc;
    //     ///
    //     /// let mut five = Arc::new(5);
    //     ///
    //     /// let mut_five = Arc::make_unique(&mut five);
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "arc_unique")]
    //     pub fn make_unique(this: &mut Arc<T>) -> &mut T {
    //         // Note that we hold both a strong reference and a weak reference.
    //         // Thus, releasing our strong reference only will not, by itself, cause
    //         // the memory to be deallocated.
    //         //
    //         // Use Acquire to ensure that we see any writes to `weak` that happen
    //         // before release writes (i.e., decrements) to `strong`. Since we hold a
    //         // weak count, there's no chance the ArcInner itself could be
    //         // deallocated.
    //         if this.inner().strong.compare_and_swap(1, 0, Acquire) != 1 {
    //             // Another srong pointer exists; clone
    //             *this = Arc::new((**this).clone());
    //         } else if this.inner().weak.load(Relaxed) != 1 {
    //             // Relaxed suffices in the above because this is fundamentally an
    //             // optimization: we are always racing with weak pointers being
    //             // dropped. Worst case, we end up allocated a new Arc unnecessarily.
    //
    //             // We removed the last strong ref, but there are additional weak
    //             // refs remaining. We'll move the contents to a new Arc, and
    //             // invalidate the other weak refs.
    //
    //             // Note that it is not possible for the read of `weak` to yield
    //             // usize::MAX (i.e., locked), since the weak count can only be
    //             // locked by a thread with a strong reference.
    //
    //             // Materialize our own implicit weak pointer, so that it can clean
    //             // up the ArcInner as needed.
    //             let weak = Weak { _ptr: this._ptr };
    //
    //             // mark the data itself as already deallocated
    //             unsafe {
    //                 // there is no data race in the implicit write caused by `read`
    //                 // here (due to zeroing) because data is no longer accessed by
    //                 // other threads (due to there being no more strong refs at this
    //                 // point).
    //                 let mut swap = Arc::new(ptr::read(&(**weak._ptr).data));
    //                 mem::swap(this, &mut swap);
    //                 mem::forget(swap);
    //             }
    //         } else {
    //             // We were the sole reference of either kind; bump back up the
    //             // strong ref count.
    //             this.inner().strong.store(1, Release);
    //         }
    //
    //         // As with `get_mut()`, the unsafety is ok because our reference was
    //         // either unique to begin with, or became one upon cloning the contents.
    //         unsafe {
    //             let inner = &mut **this._ptr;
    //             &mut inner.data
    //         }
    //     }
    // }

    // impl<T: ?Sized> Drop for Arc<T> {
    //     /// Drops the `Arc<T>`.
    //     ///
    //     /// This will decrement the strong reference count. If the strong reference
    //     /// count becomes zero and the only other references are `Weak<T>` ones,
    //     /// `drop`s the inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// {
    //     ///     let five = Arc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     ///     drop(five); // explicit drop
    //     /// }
    //     /// {
    //     ///     let five = Arc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     /// } // implicit drop
    //     /// ```
    //     #[inline]
    //     fn drop(&mut self) {
    //         // This structure has #[unsafe_no_drop_flag], so this drop glue may run
    //         // more than once (but it is guaranteed to be zeroed after the first if
    //         // it's run more than once)
    //         let ptr = *self._ptr;
    //         // if ptr.is_null() { return }
    //         if ptr as *mut u8 as usize == 0 || ptr as *mut u8 as usize == mem::POST_DROP_USIZE {
    //             return
    //         }
    //
    //         // Because `fetch_sub` is already atomic, we do not need to synchronize
    //         // with other threads unless we are going to delete the object. This
    //         // same logic applies to the below `fetch_sub` to the `weak` count.
    //         if self.inner().strong.fetch_sub(1, Release) != 1 { return }
    //
    //         // This fence is needed to prevent reordering of use of the data and
    //         // deletion of the data.  Because it is marked `Release`, the decreasing
    //         // of the reference count synchronizes with this `Acquire` fence. This
    //         // means that use of the data happens before decreasing the reference
    //         // count, which happens before this fence, which happens before the
    //         // deletion of the data.
    //         //
    //         // As explained in the [Boost documentation][1],
    //         //
    //         // > It is important to enforce any possible access to the object in one
    //         // > thread (through an existing reference) to *happen before* deleting
    //         // > the object in a different thread. This is achieved by a "release"
    //         // > operation after dropping a reference (any access to the object
    //         // > through this reference must obviously happened before), and an
    //         // > "acquire" operation before deleting the object.
    //         //
    //         // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
    //         atomic::fence(Acquire);
    //
    //         unsafe {
    //             self.drop_slow()
    //         }
    //     }
    // }

    // pub struct Weak<T: ?Sized> {
    //     // FIXME #12808: strange name to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _ptr: NonZero<*mut ArcInner<T>>,
    // }

    // unsafe impl<T: ?Sized + Sync + Send> Send for Weak<T> { }
    // unsafe impl<T: ?Sized + Sync + Send> Sync for Weak<T> { }

    // impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Weak<U>> for Weak<T> {}

    // impl<T: ?Sized> Weak<T> {
    //     /// Upgrades a weak reference to a strong reference.
    //     ///
    //     /// Upgrades the `Weak<T>` reference to an `Arc<T>`, if possible.
    //     ///
    //     /// Returns `None` if there were no strong references and the data was
    //     /// destroyed.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(arc_weak)]
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// let weak_five = five.downgrade();
    //     ///
    //     /// let strong_five: Option<Arc<_>> = weak_five.upgrade();
    //     /// ```
    //     pub fn upgrade(&self) -> Option<Arc<T>> {
    //         // We use a CAS loop to increment the strong count instead of a
    //         // fetch_add because once the count hits 0 it must never be above 0.
    //         let inner = self.inner();
    //         loop {
    //             // Relaxed load because any write of 0 that we can observe
    //             // leaves the field in a permanently zero state (so a
    //             // "stale" read of 0 is fine), and any other value is
    //             // confirmed via the CAS below.
    //             let n = inner.strong.load(Relaxed);
    //             if n == 0 { return None }
    //
    //             // Relaxed is valid for the same reason it is on Arc's Clone impl
    //             let old = inner.strong.compare_and_swap(n, n + 1, Relaxed);
    //             if old == n { return Some(Arc { _ptr: self._ptr }) }
    //         }
    //     }
    //
    //     #[inline]
    //     fn inner(&self) -> &ArcInner<T> {
    //         // See comments above for why this is "safe"
    //         unsafe { &**self._ptr }
    //     }
    // }

    // impl<T: ?Sized> Drop for Weak<T> {
    //     /// Drops the `Weak<T>`.
    //     ///
    //     /// This will decrement the weak reference count.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(arc_weak)]
    //     /// use std::sync::Arc;
    //     ///
    //     /// {
    //     ///     let five = Arc::new(5);
    //     ///     let weak_five = five.downgrade();
    //     ///
    //     ///     // stuff
    //     ///
    //     ///     drop(weak_five); // explicit drop
    //     /// }
    //     /// {
    //     ///     let five = Arc::new(5);
    //     ///     let weak_five = five.downgrade();
    //     ///
    //     ///     // stuff
    //     ///
    //     /// } // implicit drop
    //     /// ```
    //     fn drop(&mut self) {
    //         let ptr = *self._ptr;
    //
    //         // see comments above for why this check is here
    //         if ptr as *mut u8 as usize == 0 || ptr as *mut u8 as usize == mem::POST_DROP_USIZE {
    //             return
    //         }
    //
    //         // If we find out that we were the last weak pointer, then its time to
    //         // deallocate the data entirely. See the discussion in Arc::drop() about
    //         // the memory orderings
    //         //
    //         // It's not necessary to check for the locked state here, because the
    //         // weak count can only be locked if there was precisely one weak ref,
    //         // meaning that drop could only subsequently run ON that remaining weak
    //         // ref, which can only happen after the lock is released.
    //         if self.inner().weak.fetch_sub(1, Release) == 1 {
    //             atomic::fence(Acquire);
    //             unsafe { deallocate(ptr as *mut u8,
    //                                 size_of_val(&*ptr),
    //                                 align_of_val(&*ptr)) }
    //         }
    //     }
    // }

    type T = i32; // T: Clone

    #[test]
    fn make_unique_test1() {
	let value: T = 68;
	let arc: Arc<T> = Arc::<T>::new(value);
	assert_eq!(*arc, 68);

	let strong_count: usize = Arc::<T>::strong_count(&arc);
	assert_eq!(strong_count, 1);

	let mut clone: Arc<T> = arc.clone();
	let strong_count: usize = Arc::<T>::strong_count(&arc);
	assert_eq!(strong_count, 2);

	thread::spawn(move || {
	    let unique: &mut T = Arc::<T>::make_unique(&mut clone);
	    assert_eq!(*unique, 68);
	    *unique = 500;
	});
	thread::sleep_ms(10);

	let strong_count: usize = Arc::<T>::strong_count(&arc);
	assert_eq!(strong_count, 1);

	assert_eq!(*arc, 68);
    }

    #[test]
    fn make_unique_test2() {
	let value: T = 68;
	let mut arc: Arc<T> = Arc::<T>::new(value);
	assert_eq!(*arc, 68);

	let weak_count: usize = Arc::<T>::weak_count(&arc);
	assert_eq!(weak_count, 0);

	let weak: Weak<T> = arc.downgrade();
	let weak_count: usize = Arc::<T>::weak_count(&arc);
	assert_eq!(weak_count, 1);

	thread::spawn(move || {
	    let unique: &mut T = Arc::<T>::make_unique(&mut arc);
	    assert_eq!(*unique, 68);
	    *unique = 500;
	});
	thread::sleep_ms(10);

	let result: Option<Arc<T>> = weak.upgrade();
	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }

    #[test]
    fn make_unique_test3() {
	let value: T = 68;
	let mut arc: Arc<T> = Arc::<T>::new(value);
	assert_eq!(*arc, 68);

	thread::spawn(move || {
	    let unique: &mut T = Arc::<T>::make_unique(&mut arc);
	    assert_eq!(*unique, 68);
	    *unique = 500;
	});
	thread::sleep_ms(10);
    }
}
