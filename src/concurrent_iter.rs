use crate::FixedVec;
use orx_concurrent_iter::{
    IntoConcurrentIter,
    implementations::{ConIterSlice, ConIterVec},
};

impl<T> IntoConcurrentIter for FixedVec<T>
where
    T: Send + Sync,
{
    type Item = T;

    type IntoIter = ConIterVec<T>;

    fn into_con_iter(self) -> Self::IntoIter {
        self.data.into_con_iter()
    }
}

impl<'a, T> IntoConcurrentIter for &'a FixedVec<T>
where
    T: Sync,
{
    type Item = &'a T;

    type IntoIter = ConIterSlice<'a, T>;

    fn into_con_iter(self) -> Self::IntoIter {
        self.data.into_con_iter()
    }
}
