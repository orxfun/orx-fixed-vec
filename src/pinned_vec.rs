use crate::helpers::range::{range_end, range_start};
use crate::FixedVec;
use core::cmp::Ordering;
use core::iter::Rev;
use core::ops::RangeBounds;
use orx_iterable::Collection;
use orx_pinned_vec::utils::slice;
use orx_pinned_vec::{CapacityState, PinnedVec};
use orx_pseudo_default::PseudoDefault;

impl<T> PseudoDefault for FixedVec<T> {
    fn pseudo_default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> PinnedVec<T> for FixedVec<T> {
    type IterRev<'a>
        = Rev<core::slice::Iter<'a, T>>
    where
        T: 'a,
        Self: 'a;
    type IterMutRev<'a>
        = Rev<core::slice::IterMut<'a, T>>
    where
        T: 'a,
        Self: 'a;
    type SliceIter<'a>
        = Option<&'a [T]>
    where
        T: 'a,
        Self: 'a;
    type SliceMutIter<'a>
        = Option<&'a mut [T]>
    where
        T: 'a,
        Self: 'a;

    /// Returns the index of the `element` with the given reference.
    /// This method has *O(1)* time complexity.
    ///
    /// Note that `T: Eq` is not required; reference equality is used.
    ///
    /// # Safety
    ///
    /// Since `FixedVec` implements `PinnedVec`, the underlying memory
    /// of the vector stays pinned; i.e., is not carried to different memory
    /// locations.
    /// Therefore, it is possible and safe to compare an element's reference
    /// to find its position in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(4);
    /// for i in 0..4 {
    ///     vec.push(10 * i);
    /// }
    ///
    /// assert_eq!(Some(0), vec.index_of(&vec[0]));
    /// assert_eq!(Some(1), vec.index_of(&vec[1]));
    /// assert_eq!(Some(2), vec.index_of(&vec[2]));
    /// assert_eq!(Some(3), vec.index_of(&vec[3]));
    ///
    /// // num certainly does not belong to `vec`
    /// let num = 42;
    /// assert_eq!(None, vec.index_of(&num));
    ///
    /// // even if its value belongs
    /// let num = 20;
    /// assert_eq!(None, vec.index_of(&num));
    ///
    /// // as expected, querying elements of another vector will also fail
    /// let eq_vec = vec![0, 10, 20, 30];
    /// for i in 0..4 {
    ///     assert_eq!(None, vec.index_of(&eq_vec[i]));
    /// }
    /// ```
    #[inline(always)]
    fn index_of(&self, element: &T) -> Option<usize> {
        slice::index_of(&self.data, element)
    }

    fn index_of_ptr(&self, element_ptr: *const T) -> Option<usize> {
        slice::index_of_ptr(&self.data, element_ptr)
    }

    fn push_get_ptr(&mut self, value: T) -> *const T {
        let idx = self.data.len();
        self.data.push(value);
        unsafe { self.data.as_ptr().add(idx) }
    }

    unsafe fn iter_ptr<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i,
    {
        let ptr = self.data.as_ptr();
        (0..self.data.len()).map(move |i| unsafe { ptr.add(i) })
    }

    unsafe fn iter_ptr_rev<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i,
    {
        let ptr = self.data.as_ptr();
        (0..self.data.len())
            .rev()
            .map(move |i| unsafe { ptr.add(i) })
    }

    /// Returns whether or not the `element` with the given reference belongs to the vector.
    /// This method has *O(1)* time complexity.
    ///
    /// Note that `T: Eq` is not required; memory address is used.
    ///
    /// # Safety
    ///
    /// Since `FixedVec` implements `PinnedVec`, the underlying memory
    /// of the vector stays pinned; i.e., is not carried to different memory
    /// locations.
    /// Therefore, it is possible and safe to compare an element's reference
    /// to find its position in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(4);
    /// for i in 0..4 {
    ///     vec.push(10 * i);
    /// }
    ///
    /// assert!(vec.contains_reference(&vec[0]));
    /// assert!(vec.contains_reference(&vec[1]));
    /// assert!(vec.contains_reference(&vec[2]));
    /// assert!(vec.contains_reference(&vec[3]));
    ///
    /// // num certainly does not belong to `vec`
    /// let num = 42;
    /// assert!(!vec.contains_reference(&num));
    ///
    /// // even if its value belongs
    /// let num = 20;
    /// assert!(!vec.contains_reference(&num));
    ///
    /// // as expected, querying elements of another vector will also fail
    /// let eq_vec = vec![0, 10, 20, 30];
    /// for i in 0..4 {
    ///     assert!(!vec.contains_reference(&eq_vec[i]));
    /// }
    /// ```
    #[inline(always)]
    fn contains_reference(&self, element: &T) -> bool {
        slice::contains_reference(self.data.as_slice(), element)
    }

    /// Returns whether or not the `element` with the given reference belongs to the vector.
    /// This method has *O(1)* time complexity.
    ///
    /// Note that `T: Eq` is not required; memory pointer is used.
    ///
    /// # Safety
    ///
    /// Since `FixedVec` implements `PinnedVec`, the underlying memory
    /// of the vector stays pinned; i.e., is not carried to different memory
    /// locations.
    /// Therefore, it is possible and safe to compare an element's reference
    /// to find its position in the vector.
    #[inline(always)]
    fn contains_ptr(&self, element_ptr: *const T) -> bool {
        slice::contains_ptr(self.data.as_slice(), element_ptr)
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn capacity_state(&self) -> CapacityState {
        CapacityState::FixedCapacity(self.capacity())
    }

    /// Clones and appends all elements in a slice to the Vec.
    ///
    /// Iterates over the slice other, clones each element, and then appends it to this Vec. The other slice is traversed in-order.
    ///
    /// Note that this function is same as extend except that it is specialized to work with slices instead. If and when Rust gets specialization this function will likely be deprecated (but still available).
    ///
    /// # Panics
    ///
    /// Panics if there is not enough room in the vector for the elements in `other`;
    /// i.e., `self.room() < other.len()`.
    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.panic_if_not_enough_room_for(other.len());
        self.data.extend_from_slice(other);
    }

    #[inline(always)]
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.data.get_unchecked(index)
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index)
    }

    #[inline(always)]
    fn first(&self) -> Option<&T> {
        self.data.first()
    }

    #[inline(always)]
    fn last(&self) -> Option<&T> {
        self.data.last()
    }

    #[inline(always)]
    unsafe fn first_unchecked(&self) -> &T {
        self.data.get_unchecked(0)
    }

    #[inline(always)]
    unsafe fn last_unchecked(&self) -> &T {
        self.data.get_unchecked(self.data.len() - 1)
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if there is no available room in the vector;
    /// i.e., `self.is_full()` or equivalently `self.len() == self.capacity()`.
    #[inline(always)]
    fn push(&mut self, value: T) {
        self.push_or_panic(value)
    }

    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    ///
    /// Panics also if there is no available room in the vector;
    /// i.e., `self.is_full()` or equivalently `self.len() == self.capacity()`.
    #[inline(always)]
    fn insert(&mut self, index: usize, element: T) {
        self.panic_if_not_enough_room_for(1);
        self.data.insert(index, element)
    }

    #[inline(always)]
    fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    #[inline(always)]
    fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
    }

    #[inline(always)]
    fn truncate(&mut self, len: usize) {
        self.data.truncate(len)
    }

    #[inline(always)]
    fn iter_rev(&self) -> Self::IterRev<'_> {
        self.data.iter().rev()
    }

    #[inline(always)]
    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_> {
        self.data.iter_mut().rev()
    }

    /// Returns the view on the required `range` as an Option of slice:
    ///
    /// * returns None if the range is out of bounds;
    /// * returns Some of the slice when the range is within bounds of the fixed vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(10);
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// assert_eq!(vec.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// // within bounds
    /// assert_eq!(vec.slices(0..4).unwrap(), &[0, 1, 2, 3]);
    /// assert_eq!(vec.slices(5..7).unwrap(), &[5, 6]);
    /// assert_eq!(vec.slices(8..10).unwrap(), &[8, 9]);
    ///
    /// // OutOfBounds
    /// assert!(vec.slices(5..12).is_none());
    /// assert!(vec.slices(10..11).is_none());
    /// ```
    fn slices<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.len());

        match b.saturating_sub(a) {
            0 => Some(&[]),
            _ => match (a.cmp(&self.len()), b.cmp(&self.len())) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => Some(&self.data[a..b]),
            },
        }
    }

    /// Returns a mutable view on the required `range` as an Option of slice:
    ///
    /// * returns None if the range is out of bounds;
    /// * returns Some of the slice when the range is within bounds of the fixed vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(10);
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// assert_eq!(vec.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// // within bounds
    /// let mut slice = vec.slices_mut(0..4).unwrap();
    /// assert_eq!(slice, &[0, 1, 2, 3]);
    /// slice[1] *= 10;
    /// assert_eq!(vec.as_slice(), &[0, 10, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    ///
    /// let mut slice = vec.slices_mut(5..7).unwrap();
    /// assert_eq!(slice, &[5, 6]);
    /// slice[0] *= 10;
    /// assert_eq!(vec.as_slice(), &[0, 10, 2, 3, 4, 50, 6, 7, 8, 9]);
    ///
    /// // OutOfBounds
    /// assert!(vec.slices_mut(5..12).is_none());
    /// assert!(vec.slices_mut(10..11).is_none());
    /// ```
    #[inline(always)]
    fn slices_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Self::SliceMutIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.len());

        match b.saturating_sub(a) {
            0 => Some(&mut []),
            _ => match (a.cmp(&self.len()), b.cmp(&self.len())) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => Some(&mut self.data[a..b]),
            },
        }
    }

    #[inline(always)]
    fn get_ptr(&self, index: usize) -> Option<*const T> {
        (index < self.data.capacity()).then(|| unsafe { self.data.as_ptr().add(index) })
    }

    #[inline(always)]
    fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T> {
        (index < self.data.capacity()).then(|| unsafe { self.data.as_mut_ptr().add(index) })
    }

    #[inline(always)]
    unsafe fn set_len(&mut self, new_len: usize) {
        self.data.set_len(new_len)
    }

    fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.data.binary_search_by(f)
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        self.data.sort();
    }

    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.data.sort_by(compare)
    }

    fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.data.sort_by_key(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use alloc::string::String;
    use alloc::vec::Vec;
    use orx_pinned_vec::*;
    use orx_pseudo_default::PseudoDefault;

    #[test]
    fn pinned_vec_exact_capacity() {
        for cap in [0, 124, 5421] {
            test_pinned_vec(FixedVec::new(cap), cap);
        }
    }

    #[test]
    fn pinned_vec_loose_capacity() {
        for cap in [0, 124, 5421] {
            test_pinned_vec(FixedVec::new(cap * 2), cap);
        }
    }

    #[test]
    fn into_inner() {
        let fixed: FixedVec<_> = (0..16).collect();
        let vec = fixed.into_inner();
        assert_eq!(vec, (0..16).collect::<Vec<_>>())
    }

    #[test]
    fn index_of_and_contains() {
        fn test(mut vec: FixedVec<usize>) {
            let mut another_vec = Vec::new();
            for i in 0..42 {
                vec.push(i);
                another_vec.push(i);
            }
            for i in 0..vec.len() {
                assert_eq!(Some(i), vec.index_of(&vec[i]));
                assert!(vec.contains_reference(&vec[i]));

                assert_eq!(None, vec.index_of(&another_vec[i]));
                assert!(!vec.contains_reference(&another_vec[i]));

                let scalar = another_vec[i];
                assert_eq!(None, vec.index_of(&scalar));
                assert!(!vec.contains_reference(&scalar));
            }
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }

    #[test]
    fn len_and_is_empty() {
        fn test_len(mut vec: FixedVec<usize>) {
            for i in 0..42 {
                assert_eq!(i, vec.len());
                vec.push(i);
            }
            assert_eq!(42, vec.len());

            vec.clear();
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            assert_eq!(42, vec.len());

            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.pop();
            }
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.remove(vec.len() / 2);
            }
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.remove(0);
            }
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.remove(vec.len() - 1);
            }
            assert_eq!(0, vec.len());

            vec.clear();
            for i in 0..42 {
                assert_eq!(i, vec.len());
                vec.insert(i, i);
            }
            assert_eq!(42, vec.len());

            vec.clear();
            for i in 0..42 {
                assert_eq!(i, vec.len());
                vec.insert(0, i);
            }
            assert_eq!(42, vec.len());
        }

        test_len(FixedVec::new(42));
        test_len(FixedVec::new(1_000));
    }

    #[test]
    fn capacity() {
        let vec: FixedVec<char> = FixedVec::new(44);
        assert_eq!(44, vec.capacity());
        assert_eq!(CapacityState::FixedCapacity(44), vec.capacity_state());
    }

    #[test]
    fn clear() {
        fn clear_is_empty(mut vec: FixedVec<usize>) {
            vec.clear();
            assert!(vec.is_empty());
            assert_eq!(0, vec.len());

            vec.push(1);
            assert!(!vec.is_empty());
            for i in 0..42 {
                vec.push(i);
            }
            assert!(!vec.is_empty());

            vec.clear();
            assert!(vec.is_empty());
            assert_eq!(0, vec.len());
        }
        clear_is_empty(FixedVec::new(43));
        clear_is_empty(FixedVec::new(1000));
    }

    #[test]
    fn get() {
        fn test_get(mut vec: FixedVec<usize>) {
            assert!(vec.is_empty());

            for i in 0..53 {
                vec.push(i);

                assert_eq!(vec.get(i), Some(&i));
                assert_eq!(vec.get(i + 1), None);

                *vec.get_mut(i).expect("is-some") += 100;
                *unsafe { vec.get_unchecked_mut(i) } += 10;
            }

            for i in 0..53 {
                assert_eq!(vec.get(i), Some(&(110 + i)));
                assert_eq!(unsafe { vec.get_unchecked(i) }, &(110 + i));
            }
        }
        test_get(FixedVec::new(53));
        test_get(FixedVec::new(1000));
    }

    #[test]
    fn first_last() {
        let mut vec = FixedVec::new(6);

        assert!(vec.first().is_none());
        assert!(vec.last().is_none());

        vec.push(42);

        assert_eq!(vec.first(), Some(&42));
        assert_eq!(vec.last(), Some(&42));

        unsafe {
            assert_eq!(vec.first_unchecked(), &42);
            assert_eq!(vec.last_unchecked(), &42);
        }

        vec.push(7);

        assert_eq!(vec.first(), Some(&42));
        assert_eq!(vec.last(), Some(&7));

        unsafe {
            assert_eq!(vec.first_unchecked(), &42);
            assert_eq!(vec.last_unchecked(), &7);
        }

        vec.insert(1, 56421);

        assert_eq!(vec.first(), Some(&42));
        assert_eq!(vec.last(), Some(&7));

        unsafe {
            assert_eq!(vec.first_unchecked(), &42);
            assert_eq!(vec.last_unchecked(), &7);
        }

        vec.clear();

        assert!(vec.first().is_none());
        assert!(vec.last().is_none());
    }

    #[test]
    fn extend_from_slice() {
        fn test(mut vec: FixedVec<usize>) {
            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            vec.extend_from_slice(&(42..63).collect::<Vec<_>>());
            vec.extend_from_slice(&(63..100).collect::<Vec<_>>());

            assert_eq!(100, vec.len());
            for i in 0..100 {
                assert_eq!(Some(&i), vec.get(i));
            }
        }
        test(FixedVec::new(100));
        test(FixedVec::new(1000));
    }

    #[test]
    fn grow() {
        fn test(mut vec: FixedVec<usize>) {
            for i in 0..42 {
                vec.push(i);
            }
            for i in 0..42 {
                vec.insert(i, 100 + i);
            }

            for i in 0..42 {
                assert_eq!(Some(&i), vec.get(42 + i));
                assert_eq!(Some(&(100 + i)), vec.get(i));
            }
        }
        test(FixedVec::new(84));
        test(FixedVec::new(1000));
    }

    #[test]
    fn shrink() {
        fn test(mut vec: FixedVec<usize>) {
            for i in 0..42 {
                vec.push(i);
                assert_eq!(i, vec.remove(0));
                assert!(vec.is_empty());
            }

            for i in 0..42 {
                vec.push(i);
            }
            for i in 0..42 {
                assert_eq!(i, vec.remove(0));
            }
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(i);
            }
            for i in (0..42).rev() {
                assert_eq!(Some(i), vec.pop());
            }
            assert_eq!(None, vec.pop());
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(i);
            }
            for _ in 0..42 {
                vec.remove(vec.len() / 2);
            }
            assert!(vec.is_empty());
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }

    #[test]
    fn swap() {
        fn test(mut vec: FixedVec<usize>) {
            for i in 0..42 {
                vec.push(i);
            }

            for i in 0..21 {
                vec.swap(i, 21 + i);
            }

            for i in 0..21 {
                assert_eq!(21 + i, vec[i]);
            }
            for i in 21..42 {
                assert_eq!(i - 21, vec[i]);
            }
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }
    #[test]
    fn truncate() {
        fn test(mut vec: FixedVec<usize>) {
            for i in 0..42 {
                vec.push(i);
            }

            vec.truncate(100);
            assert_eq!(vec, (0..42).collect::<Vec<_>>());

            vec.truncate(21);
            assert_eq!(vec, (0..21).collect::<Vec<_>>());
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }

    #[test]
    fn eq() {
        fn test(mut vec: FixedVec<usize>) {
            let slice = &(0..42).collect::<Vec<usize>>();
            for i in 0..42 {
                vec.push(i);
            }
            assert!(vec.eq(slice));

            vec.remove(7);
            assert!(!vec.eq(slice));
        }

        test(FixedVec::new(142));
        test(FixedVec::new(1000));
    }

    #[test]
    fn clone() {
        fn test(mut vec: FixedVec<usize>) {
            assert!(vec.is_empty());

            for i in 0..53 {
                vec.push(i);
            }

            let clone = vec.clone();
            assert_eq!(vec, clone);
        }

        test(FixedVec::new(53));
        test(FixedVec::new(1000));
    }

    #[test]
    fn slices() {
        #![allow(for_loops_over_fallibles)]
        let mut vec = FixedVec::new(184);

        for i in 0..184 {
            assert!(vec.slices(i..i + 1).is_none());
            assert!(vec.slices(0..i + 1).is_none());
            vec.push(i);
        }

        let slice = vec.slices(0..vec.len());
        let mut combined = Vec::new();
        for s in slice {
            combined.extend_from_slice(s);
        }
        for i in 0..184 {
            assert_eq!(i, vec[i]);
            assert_eq!(i, combined[i]);
        }

        let begin = vec.len() / 4;
        let end = 3 * vec.len() / 4;
        let slice = vec.slices(begin..end);
        let mut combined = Vec::new();
        for s in slice {
            combined.extend_from_slice(s);
        }
        for i in begin..end {
            assert_eq!(i, vec[i]);
            assert_eq!(i, combined[i - begin]);
        }
    }

    #[test]
    fn slices_mut() {
        #![allow(for_loops_over_fallibles)]
        let mut vec = FixedVec::new(184);

        for i in 0..184 {
            assert!(vec.slices_mut(i..i + 1).is_none());
            assert!(vec.slices_mut(0..i + 1).is_none());
            vec.push(i);
        }

        let slice = vec.slices_mut(0..vec.len());
        let mut combined = Vec::new();
        for s in slice {
            combined.extend_from_slice(s);
        }
        for i in 0..184 {
            assert_eq!(i, vec[i]);
            assert_eq!(i, combined[i]);
        }

        let begin = vec.len() / 4;
        let end = 3 * vec.len() / 4;
        let slice = vec.slices_mut(begin..end);
        let mut combined = Vec::new();
        for s in slice {
            combined.extend_from_slice(s);
        }
        for i in begin..end {
            assert_eq!(i, vec[i]);
            assert_eq!(i, combined[i - begin]);
        }

        vec.clear();

        for _ in 0..184 {
            vec.push(0);
        }

        fn update(slice: Option<&mut [usize]>, begin: usize) {
            let mut val = begin;
            for s in slice {
                for x in s {
                    *x = val;
                    val += 1;
                }
            }
        }
        let mut fill = |begin: usize, end: usize| {
            let range = begin..end;
            update(vec.slices_mut(range), begin);
        };

        fill(0, 14);
        fill(14, 56);
        fill(56, 77);
        fill(77, 149);
        fill(149, 182);
        fill(182, 184);
        for i in 0..184 {
            assert_eq!(vec.get(i), Some(&i));
        }
    }

    #[test]
    fn iter_iter_mut() {
        let mut vec = FixedVec::new(4);
        vec.push('a');
        vec.push('b');

        let mut iter = vec.iter();
        assert_eq!(Some(&'a'), iter.next());
        assert_eq!(Some(&'b'), iter.next());
        assert_eq!(None, iter.next());

        for x in vec.iter_mut() {
            *x = 'x';
        }

        let mut iter = vec.iter();
        assert_eq!(Some(&'x'), iter.next());
        assert_eq!(Some(&'x'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn rev_iter_iter_mut() {
        let mut vec = FixedVec::new(4);
        vec.push('a');
        vec.push('b');

        let mut iter = vec.iter_rev();
        assert_eq!(Some(&'b'), iter.next());
        assert_eq!(Some(&'a'), iter.next());
        assert_eq!(None, iter.next());

        for x in vec.iter_mut_rev() {
            *x = 'x';
        }

        let mut iter = vec.iter_rev();
        assert_eq!(Some(&'x'), iter.next());
        assert_eq!(Some(&'x'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Num(usize);
    #[test]
    fn insert() {
        fn test(mut vec: FixedVec<Num>) {
            for i in 0..42 {
                vec.push(Num(i));
            }
            for i in 0..42 {
                vec.insert(i, Num(100 + i));
            }

            for i in 0..42 {
                assert_eq!(Some(&Num(i)), vec.get(42 + i));
                assert_eq!(Some(&Num(100 + i)), vec.get(i));
            }
        }
        test(FixedVec::new(84));
        test(FixedVec::new(1000));
    }

    #[test]
    fn pseudo_default() {
        let vec = FixedVec::<String>::pseudo_default();
        assert_eq!(0, vec.capacity());
        assert_eq!(0, vec.len());
    }
}
