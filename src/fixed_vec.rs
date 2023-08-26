pub struct FixedVec<T> {
    pub(crate) data: Vec<T>,
}

impl<T> FixedVec<T> {
    pub fn new(fixed_capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(fixed_capacity),
        }
    }
    pub fn room(&self) -> usize {
        self.data.capacity() - self.data.len()
    }
    pub fn is_full(&self) -> bool {
        self.data.capacity() == self.data.len()
    }

    #[inline(always)]
    pub(crate) fn panic_if_not_enough_room_for(&self, num_new_items: usize) {
        if self.data.len() + num_new_items > self.data.capacity() {
            panic!("{}", ERR_MSG_OUT_OF_ROOM);
        }
    }
    #[inline(always)]
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
