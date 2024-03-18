use crate::FixedVec;

impl<T> FromIterator<T> for FixedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec: Vec<_> = iter.into_iter().collect();
        vec.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn from_iter() {
        let fixed: FixedVec<_> = (0..20).collect();
        assert_eq!((0..20).collect::<Vec<_>>().as_slice(), fixed.as_slice());
        assert_eq!(fixed.len(), 20);
        assert!(fixed.capacity() >= 20);
    }
}
