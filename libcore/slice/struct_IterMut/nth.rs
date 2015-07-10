#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::SliceExt;
    use core::slice::IterMut;

    // fn size_from_ptr<T>(_: *const T) -> usize {
    //     mem::size_of::<T>()
    // }

    // macro_rules! slice_offset {
    //     ($ptr:expr, $by:expr) => {{
    //         let ptr = $ptr;
    //         if size_from_ptr(ptr) == 0 {
    //             ::intrinsics::arith_offset(ptr as *mut i8, $by) as *mut _
    //         } else {
    //             ptr.offset($by)
    //         }
    //     }};
    // }

    // macro_rules! slice_ref {
    //     ($ptr:expr) => {{
    //         let ptr = $ptr;
    //         if size_from_ptr(ptr) == 0 {
    //             // Use a non-null pointer value
    //             &mut *(1 as *mut _)
    //         } else {
    //             transmute(ptr)
    //         }
    //     }};
    // }

    // pub unsafe fn from_raw_parts<'a, T>(p: *const T, len: usize) -> &'a [T] {
    //     transmute(RawSlice { data: p, len: len })
    // }

    // macro_rules! make_slice {
    //     ($start: expr, $end: expr) => {{
    //         let start = $start;
    //         let diff = ($end as usize).wrapping_sub(start as usize);
    //         if size_from_ptr(start) == 0 {
    //             // use a non-null pointer value
    //             unsafe { from_raw_parts(1 as *const _, diff) }
    //         } else {
    //             let len = diff / size_from_ptr(start);
    //             unsafe { from_raw_parts(start, len) }
    //         }
    //     }}
    // }

    // impl<T> SliceExt for [T] {
    //     type Item = T;
    //
    //     #[inline]
    //     fn split_at(&self, mid: usize) -> (&[T], &[T]) {
    //         (&self[..mid], &self[mid..])
    //     }
    //
    //     #[inline]
    //     fn iter<'a>(&'a self) -> Iter<'a, T> {
    //         unsafe {
    //             let p = if mem::size_of::<T>() == 0 {
    //                 1 as *const _
    //             } else {
    //                 let p = self.as_ptr();
    //                 assume(!p.is_null());
    //                 p
    //             };
    //
    //             Iter {
    //                 ptr: p,
    //                 end: slice_offset!(p, self.len() as isize),
    //                 _marker: marker::PhantomData
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn split<'a, P>(&'a self, pred: P) -> Split<'a, T, P> where P: FnMut(&T) -> bool {
    //         Split {
    //             v: self,
    //             pred: pred,
    //             finished: false
    //         }
    //     }
    //
    //     #[inline]
    //     fn splitn<'a, P>(&'a self, n: usize, pred: P) -> SplitN<'a, T, P> where
    //         P: FnMut(&T) -> bool,
    //     {
    //         SplitN {
    //             inner: GenericSplitN {
    //                 iter: self.split(pred),
    //                 count: n,
    //                 invert: false
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn rsplitn<'a, P>(&'a self, n: usize, pred: P) -> RSplitN<'a, T, P> where
    //         P: FnMut(&T) -> bool,
    //     {
    //         RSplitN {
    //             inner: GenericSplitN {
    //                 iter: self.split(pred),
    //                 count: n,
    //                 invert: true
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn windows(&self, size: usize) -> Windows<T> {
    //         assert!(size != 0);
    //         Windows { v: self, size: size }
    //     }
    //
    //     #[inline]
    //     fn chunks(&self, size: usize) -> Chunks<T> {
    //         assert!(size != 0);
    //         Chunks { v: self, size: size }
    //     }
    //
    //     #[inline]
    //     fn get(&self, index: usize) -> Option<&T> {
    //         if index < self.len() { Some(&self[index]) } else { None }
    //     }
    //
    //     #[inline]
    //     fn first(&self) -> Option<&T> {
    //         if self.is_empty() { None } else { Some(&self[0]) }
    //     }
    //
    //     #[inline]
    //     fn tail(&self) -> &[T] { &self[1..] }
    //
    //     #[inline]
    //     fn init(&self) -> &[T] {
    //         &self[..self.len() - 1]
    //     }
    //
    //     #[inline]
    //     fn last(&self) -> Option<&T> {
    //         if self.is_empty() { None } else { Some(&self[self.len() - 1]) }
    //     }
    //
    //     #[inline]
    //     unsafe fn get_unchecked(&self, index: usize) -> &T {
    //         transmute(self.repr().data.offset(index as isize))
    //     }
    //
    //     #[inline]
    //     fn as_ptr(&self) -> *const T {
    //         self.repr().data
    //     }
    //
    //     #[unstable(feature = "core")]
    //     fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize> where
    //         F: FnMut(&T) -> Ordering
    //     {
    //         let mut base : usize = 0;
    //         let mut lim : usize = self.len();
    //
    //         while lim != 0 {
    //             let ix = base + (lim >> 1);
    //             match f(&self[ix]) {
    //                 Equal => return Ok(ix),
    //                 Less => {
    //                     base = ix + 1;
    //                     lim -= 1;
    //                 }
    //                 Greater => ()
    //             }
    //             lim >>= 1;
    //         }
    //         Err(base)
    //     }
    //
    //     #[inline]
    //     fn len(&self) -> usize { self.repr().len }
    //
    //     #[inline]
    //     fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    //         if index < self.len() { Some(&mut self[index]) } else { None }
    //     }
    //
    //     #[inline]
    //     fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    //         unsafe {
    //             let self2: &mut [T] = mem::transmute_copy(&self);
    //
    //             (ops::IndexMut::index_mut(self, ops::RangeTo { end: mid } ),
    //              ops::IndexMut::index_mut(self2, ops::RangeFrom { start: mid } ))
    //         }
    //     }
    //
    //     #[inline]
    //     fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
    //         unsafe {
    //             let p = if mem::size_of::<T>() == 0 {
    //                 1 as *mut _
    //             } else {
    //                 let p = self.as_mut_ptr();
    //                 assume(!p.is_null());
    //                 p
    //             };
    //
    //             IterMut {
    //                 ptr: p,
    //                 end: slice_offset!(p, self.len() as isize),
    //                 _marker: marker::PhantomData
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn last_mut(&mut self) -> Option<&mut T> {
    //         let len = self.len();
    //         if len == 0 { return None; }
    //         Some(&mut self[len - 1])
    //     }
    //
    //     #[inline]
    //     fn first_mut(&mut self) -> Option<&mut T> {
    //         if self.is_empty() { None } else { Some(&mut self[0]) }
    //     }
    //
    //     #[inline]
    //     fn tail_mut(&mut self) -> &mut [T] {
    //         &mut self[1 ..]
    //     }
    //
    //     #[inline]
    //     fn init_mut(&mut self) -> &mut [T] {
    //         let len = self.len();
    //         &mut self[.. (len - 1)]
    //     }
    //
    //     #[inline]
    //     fn split_mut<'a, P>(&'a mut self, pred: P) -> SplitMut<'a, T, P> where P: FnMut(&T) -> bool {
    //         SplitMut { v: self, pred: pred, finished: false }
    //     }
    //
    //     #[inline]
    //     fn splitn_mut<'a, P>(&'a mut self, n: usize, pred: P) -> SplitNMut<'a, T, P> where
    //         P: FnMut(&T) -> bool
    //     {
    //         SplitNMut {
    //             inner: GenericSplitN {
    //                 iter: self.split_mut(pred),
    //                 count: n,
    //                 invert: false
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn rsplitn_mut<'a, P>(&'a mut self, n: usize, pred: P) -> RSplitNMut<'a, T, P> where
    //         P: FnMut(&T) -> bool,
    //     {
    //         RSplitNMut {
    //             inner: GenericSplitN {
    //                 iter: self.split_mut(pred),
    //                 count: n,
    //                 invert: true
    //             }
    //         }
    //    }
    //
    //     #[inline]
    //     fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T> {
    //         assert!(chunk_size > 0);
    //         ChunksMut { v: self, chunk_size: chunk_size }
    //     }
    //
    //     #[inline]
    //     fn swap(&mut self, a: usize, b: usize) {
    //         unsafe {
    //             // Can't take two mutable loans from one vector, so instead just cast
    //             // them to their raw pointers to do the swap
    //             let pa: *mut T = &mut self[a];
    //             let pb: *mut T = &mut self[b];
    //             ptr::swap(pa, pb);
    //         }
    //     }
    //
    //     fn reverse(&mut self) {
    //         let mut i: usize = 0;
    //         let ln = self.len();
    //         while i < ln / 2 {
    //             // Unsafe swap to avoid the bounds check in safe swap.
    //             unsafe {
    //                 let pa: *mut T = self.get_unchecked_mut(i);
    //                 let pb: *mut T = self.get_unchecked_mut(ln - i - 1);
    //                 ptr::swap(pa, pb);
    //             }
    //             i += 1;
    //         }
    //     }
    //
    //     #[inline]
    //     unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
    //         transmute((self.repr().data as *mut T).offset(index as isize))
    //     }
    //
    //     #[inline]
    //     fn as_mut_ptr(&mut self) -> *mut T {
    //         self.repr().data as *mut T
    //     }
    //
    //     #[inline]
    //     fn position_elem(&self, x: &T) -> Option<usize> where T: PartialEq {
    //         self.iter().position(|y| *x == *y)
    //     }
    //
    //     #[inline]
    //     fn rposition_elem(&self, t: &T) -> Option<usize> where T: PartialEq {
    //         self.iter().rposition(|x| *x == *t)
    //     }
    //
    //     #[inline]
    //     fn contains(&self, x: &T) -> bool where T: PartialEq {
    //         self.iter().any(|elt| *x == *elt)
    //     }
    //
    //     #[inline]
    //     fn starts_with(&self, needle: &[T]) -> bool where T: PartialEq {
    //         let n = needle.len();
    //         self.len() >= n && needle == &self[..n]
    //     }
    //
    //     #[inline]
    //     fn ends_with(&self, needle: &[T]) -> bool where T: PartialEq {
    //         let (m, n) = (self.len(), needle.len());
    //         m >= n && needle == &self[m-n..]
    //     }
    //
    //     #[unstable(feature = "core")]
    //     fn binary_search(&self, x: &T) -> Result<usize, usize> where T: Ord {
    //         self.binary_search_by(|p| p.cmp(x))
    //     }
    //
    //     #[unstable(feature = "core")]
    //     fn next_permutation(&mut self) -> bool where T: Ord {
    //         // These cases only have 1 permutation each, so we can't do anything.
    //         if self.len() < 2 { return false; }
    //
    //         // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
    //         let mut i = self.len() - 1;
    //         while i > 0 && self[i-1] >= self[i] {
    //             i -= 1;
    //         }
    //
    //         // If that is the entire vector, this is the last-ordered permutation.
    //         if i == 0 {
    //             return false;
    //         }
    //
    //         // Step 2: Find the rightmost element larger than the pivot (i-1)
    //         let mut j = self.len() - 1;
    //         while j >= i && self[j] <= self[i-1]  {
    //             j -= 1;
    //         }
    //
    //         // Step 3: Swap that element with the pivot
    //         self.swap(j, i-1);
    //
    //         // Step 4: Reverse the (previously) weakly decreasing part
    //         self[i..].reverse();
    //
    //         true
    //     }
    //
    //     #[unstable(feature = "core")]
    //     fn prev_permutation(&mut self) -> bool where T: Ord {
    //         // These cases only have 1 permutation each, so we can't do anything.
    //         if self.len() < 2 { return false; }
    //
    //         // Step 1: Identify the longest, rightmost weakly increasing part of the vector
    //         let mut i = self.len() - 1;
    //         while i > 0 && self[i-1] <= self[i] {
    //             i -= 1;
    //         }
    //
    //         // If that is the entire vector, this is the first-ordered permutation.
    //         if i == 0 {
    //             return false;
    //         }
    //
    //         // Step 2: Reverse the weakly increasing part
    //         self[i..].reverse();
    //
    //         // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
    //         let mut j = self.len() - 1;
    //         while j >= i && self[j-1] < self[i-1]  {
    //             j -= 1;
    //         }
    //
    //         // Step 4: Swap that element with the pivot
    //         self.swap(i-1, j);
    // 
    //         true
    //     }
    //
    //     #[inline]
    //     fn clone_from_slice(&mut self, src: &[T]) -> usize where T: Clone {
    //         let min = cmp::min(self.len(), src.len());
    //         let dst = &mut self[.. min];
    //         let src = &src[.. min];
    //         for i in 0..min {
    //             dst[i].clone_from(&src[i]);
    //         }
    //         min
    //     }
    // }

    // pub struct IterMut<'a, T: 'a> {
    //     ptr: *mut T,
    //     end: *mut T,
    //     _marker: marker::PhantomData<&'a mut T>,
    // }

    // macro_rules! iterator {
    //     (struct $name:ident -> $ptr:ty, $elem:ty) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<'a, T> Iterator for $name<'a, T> {
    //             type Item = $elem;
    //
    //             #[inline]
    //             fn next(&mut self) -> Option<$elem> {
    //                 // could be implemented with slices, but this avoids bounds checks
    //                 unsafe {
    //                     if mem::size_of::<T>() != 0 {
    //                         assume(!self.ptr.is_null());
    //                         assume(!self.end.is_null());
    //                     }
    //                     if self.ptr == self.end {
    //                         None
    //                     } else {
    //                         let old = self.ptr;
    //                         self.ptr = slice_offset!(self.ptr, 1);
    //                         Some(slice_ref!(old))
    //                     }
    //                 }
    //             }
    //
    //             #[inline]
    //             fn size_hint(&self) -> (usize, Option<usize>) {
    //                 let diff = (self.end as usize).wrapping_sub(self.ptr as usize);
    //                 let size = mem::size_of::<T>();
    //                 let exact = diff / (if size == 0 {1} else {size});
    //                 (exact, Some(exact))
    //             }
    //
    //             #[inline]
    //             fn count(self) -> usize {
    //                 self.size_hint().0
    //             }
    //
    //             #[inline]
    //             fn nth(&mut self, n: usize) -> Option<$elem> {
    //                 // Call helper method. Can't put the definition here because mut versus const.
    //                 self.iter_nth(n)
    //             }
    //
    //             #[inline]
    //             fn last(mut self) -> Option<$elem> {
    //                 self.next_back()
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<'a, T> DoubleEndedIterator for $name<'a, T> {
    //             #[inline]
    //             fn next_back(&mut self) -> Option<$elem> {
    //                 // could be implemented with slices, but this avoids bounds checks
    //                 unsafe {
    //                     if mem::size_of::<T>() != 0 {
    //                         assume(!self.ptr.is_null());
    //                         assume(!self.end.is_null());
    //                     }
    //                     if self.end == self.ptr {
    //                         None
    //                     } else {
    //                         self.end = slice_offset!(self.end, -1);
    //                         Some(slice_ref!(self.end))
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // iterator!{struct IterMut -> *mut T, &'a mut T}

    type T = i32;

    #[test]
    fn nth_test1() {
	let slice: &mut [T] = &mut [1, 2, 3, 4, 5, 6];

	{
	    let mut iter_mut: IterMut<T> = slice.iter_mut();

	    for _ in 0..iter_mut.size_hint().0 {
		let x: Option<&mut T> = iter_mut.nth(0);
		match x {
		    Some(v) => *v += 10,
		    None => assert!(false)
		}
	    }

	    assert_eq!(iter_mut.next(), None::<&mut T>);
	}

	assert_eq!(slice, &mut [11, 12, 13, 14, 15, 16]);
    }
}
