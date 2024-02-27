use crate::FixedVec;

impl<T> Clone for FixedVec<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut data = Vec::with_capacity(self.data.capacity());
        data.extend_from_slice(&self.data);
        Self { data }
    }
}
