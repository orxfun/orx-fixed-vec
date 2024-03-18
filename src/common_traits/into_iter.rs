use crate::FixedVec;
use std::vec::IntoIter;

impl<T> IntoIterator for FixedVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn into_iter() {
        let mut vec = FixedVec::new(4);
        for i in 0..vec.capacity() {
            vec.push(i);
        }

        let vec_from_iter: Vec<_> = vec.into_iter().map(|x| x + 1).collect();
        assert_eq!(vec_from_iter, &[1, 2, 3, 4]);
    }
}
