use crate::FixedVec;

impl<T, U> PartialEq<U> for FixedVec<T>
where
    U: AsRef<[T]>,
    T: PartialEq,
{
    fn eq(&self, other: &U) -> bool {
        self.data.as_slice() == other.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn eq() {
        let mut vec = FixedVec::new(42);
        for i in 0..vec.capacity() {
            vec.push(i);
        }

        let slice = &(0..vec.capacity()).collect::<Vec<_>>();
        assert_eq!(vec, slice);
    }
}
