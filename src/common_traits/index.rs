use crate::FixedVec;
use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

impl<T, I> Index<I> for FixedVec<T>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, I> IndexMut<I> for FixedVec<T>
where
    I: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn index() {
        let mut vec = FixedVec::new(42);
        for i in 0..vec.capacity() / 2 {
            vec.push(i);
        }

        for i in 0..vec.len() {
            assert_eq!(i, vec[i]);
        }

        vec[7] = 77;
        assert_eq!(77, vec[7]);
    }
}
