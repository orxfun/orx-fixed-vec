/// A fixed vector, `FixedVec`, is a vector with a strict predetermined capacity
/// (see [`SplitVec`](https://crates.io/crates/orx-split-vec) for dynamic capacity version).
///
/// It provides the following features:
///
/// * It provides operations with the same complexity and speed as the standard vector.
/// * It makes sure that the data stays **pinned** in place.
///     * `FixedVec<T>` implements [`PinnedVec<T>`](https://crates.io/crates/orx-pinned-vec) for any `T`;
///     * `FixedVec<T>` implements `PinnedVecSimple<T>` for `T: NotSelfRefVecItem`;
///     * Memory location of an item added to the fixed vector will never change
/// unless the vector is dropped or cleared.
///     * This allows the fixed vec to be converted into an [`ImpVec`](https://crates.io/crates/orx-imp-vec)
/// to enable immutable-push operations which allows for
/// convenient, efficient and safe implementations of self-referencing data structures.
pub struct FixedVec<T> {
    pub(crate) data: Vec<T>,
}

impl<T> FixedVec<T> {
    /// Creates a new vector with the given fixed capacity.
    ///
    /// Note that the vector can never grow beyond this capacity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(7);
    /// vec.push(42);
    ///
    /// assert_eq!(7, vec.capacity());
    /// ```
    pub fn new(fixed_capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(fixed_capacity),
        }
    }

    /// Returns the available room for new items; i.e.,
    /// `capacity() - len()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(7);
    /// vec.push(42);
    ///
    /// assert_eq!(7, vec.capacity());
    /// assert_eq!(1, vec.len());
    /// assert_eq!(6, vec.room());
    /// ```
    pub fn room(&self) -> usize {
        self.data.capacity() - self.data.len()
    }

    /// Return whether the fixed vector is full or not;
    /// equivalent to `capacity() == len()` or `room() == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_fixed_vec::prelude::*;
    ///
    /// let mut vec = FixedVec::new(2);
    /// assert!(!vec.is_full());
    ///
    /// vec.push(42);
    /// assert!(!vec.is_full());
    ///
    /// vec.push(7);
    /// assert!(vec.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.data.capacity() == self.data.len()
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to &s[..].
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    // helpers
    #[inline(always)]
    #[allow(clippy::panic)]
    pub(crate) fn panic_if_not_enough_room_for(&self, num_new_items: usize) {
        if self.data.len() + num_new_items > self.data.capacity() {
            panic!("{}", ERR_MSG_OUT_OF_ROOM);
        }
    }

    #[inline(always)]
    #[allow(clippy::panic)]
    pub(crate) fn push_or_panic(&mut self, value: T) {
        let len = self.data.len();
        if len == self.data.capacity() {
            panic!("{}", ERR_MSG_OUT_OF_ROOM);
        } else {
            *unsafe { self.data.get_unchecked_mut(len) } = value;
            unsafe { self.data.set_len(len + 1) };
        }
    }
}
impl<T> From<Vec<T>> for FixedVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self { data: value }
    }
}
impl<T> From<FixedVec<T>> for Vec<T> {
    fn from(value: FixedVec<T>) -> Self {
        value.data
    }
}
const ERR_MSG_OUT_OF_ROOM: &str =
    "FixedVec is full, a fixed capacity vector cannot exceed its initial capacity.";

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn new() {
        let vec: FixedVec<char> = FixedVec::new(17);
        assert_eq!(0, vec.len());
        assert!(vec.is_empty());
        assert_eq!(17, vec.capacity());
    }

    #[test]
    fn from() {
        let vec = vec![1, 3, 42];
        let fixed_vec: FixedVec<_> = vec.clone().into();
        assert_eq!(&vec, fixed_vec.as_ref());
        assert_eq!(vec.len(), fixed_vec.capacity());
        let into_vec: Vec<_> = fixed_vec.into();
        assert_eq!(&vec, &into_vec);

        let mut vec = Vec::with_capacity(7);
        vec.push(42);
        let fixed_vec: FixedVec<_> = vec.clone().into();
        assert_eq!(&vec, fixed_vec.as_ref());
        assert_eq!(1, fixed_vec.capacity());
        let into_vec: Vec<_> = fixed_vec.into();
        assert_eq!(&vec, &into_vec);
    }

    #[test]
    fn room() {
        let mut vec = FixedVec::new(10);

        for i in 0..vec.capacity() {
            assert_eq!(i, vec.len());
            assert_eq!(vec.capacity() - i, vec.room());
            vec.push(1.1);
        }

        assert_eq!(vec.len(), vec.capacity());
        assert_eq!(0, vec.room());
        assert!(vec.is_full());
    }

    #[test]
    fn as_slice() {
        let fixed_vec: FixedVec<_> = (0..20).collect();
        let vec: Vec<_> = (0..20).collect();

        let slice = fixed_vec.as_slice();
        assert_eq!(slice, &vec);
    }

    #[test]
    fn panic_if_not_enough_room_for_when_ok() {
        let mut vec = FixedVec::new(3);

        vec.panic_if_not_enough_room_for(3);

        vec.push("a");
        vec.panic_if_not_enough_room_for(2);

        vec.push("b");
        vec.panic_if_not_enough_room_for(1);

        vec.push("c");
        vec.panic_if_not_enough_room_for(0);
    }

    #[test]
    #[should_panic]
    fn panic_if_not_enough_room_for_when_not_ok() {
        let mut vec = FixedVec::new(3);
        vec.push("a");
        vec.panic_if_not_enough_room_for(3);
    }

    #[test]
    fn push_or_panic_when_ok() {
        let mut vec = FixedVec::new(3);

        vec.push_or_panic(0);
        vec.push_or_panic(1);
        vec.push_or_panic(2);

        assert_eq!(Some(&0), vec.get(0));
        assert_eq!(Some(&1), vec.get(1));
        assert_eq!(Some(&2), vec.get(2));
    }
    #[test]
    #[should_panic]
    fn push_or_panic_when_not_ok() {
        let mut vec = FixedVec::new(3);

        vec.push_or_panic(0);
        vec.push_or_panic(1);
        vec.push_or_panic(2);

        assert_eq!(Some(&0), vec.get(0));
        assert_eq!(Some(&1), vec.get(1));
        assert_eq!(Some(&2), vec.get(2));

        vec.push_or_panic(3);
    }
}
