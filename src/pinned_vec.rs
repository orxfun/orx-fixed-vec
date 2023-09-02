use crate::FixedVec;
use orx_pinned_vec::PinnedVec;
use std::fmt::{Debug, Formatter, Result};

impl<T> PinnedVec<T> for FixedVec<T> {
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
    fn clear(&mut self) {
        self.data.clear();
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

    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    ///
    /// Panics also if there is no available room in the vector;
    /// i.e., `self.is_full()` or equivalently `self.len() == self.capacity()`.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `insert` is unsafe since insertion of a new element at an arbitrary position of the vector
    /// typically changes the positions of already existing elements.
    ///
    /// When the elements are holding references to other elements of the vector,
    /// this change in positions makes the references invalid.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
    #[inline(always)]
    unsafe fn unsafe_insert(&mut self, index: usize, element: T) {
        self.panic_if_not_enough_room_for(1);
        self.data.insert(index, element)
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }
    #[inline(always)]
    unsafe fn unsafe_pop(&mut self) -> Option<T> {
        self.data.pop()
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
    #[inline(always)]
    unsafe fn unsafe_remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }
    #[inline(always)]
    unsafe fn unsafe_swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
    }
    #[inline(always)]
    unsafe fn unsafe_truncate(&mut self, len: usize) {
        self.data.truncate(len)
    }

    // required for common trait implementations
    #[inline(always)]
    fn partial_eq<S>(&self, other: S) -> bool
    where
        S: AsRef<[T]>,
        T: PartialEq,
    {
        self.data == other.as_ref()
    }
    fn debug(&self, f: &mut Formatter<'_>) -> Result
    where
        T: Debug,
    {
        write!(f, "FixedVec ")?;
        self.data.fmt(f)?;
        writeln!(f)
    }

    unsafe fn unsafe_clone(&self) -> Self
    where
        T: Clone,
    {
        let mut data = Vec::with_capacity(self.data.capacity());
        data.extend_from_slice(&self.data);
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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

            unsafe { vec.unsafe_truncate(100) };
            assert_eq!(vec, (0..42).collect::<Vec<_>>());

            unsafe { vec.unsafe_truncate(21) };
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

    #[derive(Debug, PartialEq, Clone)]
    struct Num(usize);
    #[test]
    fn unsafe_insert() {
        fn test(mut vec: FixedVec<Num>) {
            for i in 0..42 {
                vec.push(Num(i));
            }
            for i in 0..42 {
                unsafe { vec.unsafe_insert(i, Num(100 + i)) };
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
    fn unsafe_shrink() {
        fn test(mut vec: FixedVec<Num>) {
            for i in 0..42 {
                vec.push(Num(i));
                assert_eq!(Num(i), unsafe { vec.unsafe_remove(0) });
                assert!(vec.is_empty());
            }

            for i in 0..42 {
                vec.push(Num(i));
            }
            for i in 0..42 {
                assert_eq!(Num(i), unsafe { vec.unsafe_remove(0) });
            }
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(Num(i));
            }
            for i in (0..42).rev() {
                assert_eq!(Some(Num(i)), unsafe { vec.unsafe_pop() });
            }
            assert_eq!(None, unsafe { vec.unsafe_pop() });
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(Num(i));
            }
            for _ in 0..42 {
                unsafe { vec.unsafe_remove(vec.len() / 2) };
            }
            assert!(vec.is_empty());
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }

    #[test]
    fn unsafe_swap() {
        fn test(mut vec: FixedVec<Num>) {
            for i in 0..42 {
                vec.push(Num(i));
            }

            for i in 0..21 {
                unsafe { vec.unsafe_swap(i, 21 + i) };
            }

            for i in 0..21 {
                assert_eq!(Num(21 + i), vec[i]);
            }
            for i in 21..42 {
                assert_eq!(Num(i - 21), vec[i]);
            }
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }
    #[test]
    fn unsafe_truncate() {
        fn test(mut vec: FixedVec<Num>) {
            for i in 0..42 {
                vec.push(Num(i));
            }

            unsafe { vec.unsafe_truncate(100) };
            assert_eq!(vec, (0..42).map(Num).collect::<Vec<_>>());

            unsafe { vec.unsafe_truncate(21) };
            assert_eq!(vec, (0..21).map(Num).collect::<Vec<_>>());
        }
        test(FixedVec::new(42));
        test(FixedVec::new(1000));
    }

    #[test]
    fn unsafe_clone() {
        fn test(mut vec: FixedVec<Num>) {
            assert!(vec.is_empty());

            for i in 0..53 {
                vec.push(Num(i));
            }

            let clone = unsafe { vec.unsafe_clone() };
            assert_eq!(vec, clone);
        }

        test(FixedVec::new(53));
        test(FixedVec::new(1000));
    }
}
