use crate::FixedVec;
use std::ops::Deref;

impl<T> AsRef<[T]> for FixedVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}
impl<T> AsMut<[T]> for FixedVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Deref for FixedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn as_ref() {
        let mut vec = FixedVec::new(4);
        for i in 0..vec.capacity() {
            vec.push(i);
        }

        assert_eq!(vec.as_ref(), &[0, 1, 2, 3]);
    }

    #[test]
    fn as_mut() {
        let mut vec = FixedVec::new(4);
        for i in 0..vec.capacity() {
            vec.push(i);
        }

        for x in vec.as_mut() {
            *x *= 10;
        }

        assert_eq!(vec.as_ref(), &[0, 10, 20, 30]);
    }
}
