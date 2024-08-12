use crate::{ConcurrentFixedVec, FixedVec};
use orx_pinned_vec::IntoConcurrentPinnedVec;

impl<T> IntoConcurrentPinnedVec<T> for FixedVec<T> {
    type ConPinnedVec = ConcurrentFixedVec<T>;

    fn into_concurrent(self) -> Self::ConPinnedVec {
        self.into()
    }

    fn into_concurrent_filled_with<F>(mut self, fill_with: F) -> Self::ConPinnedVec
    where
        F: Fn() -> T,
    {
        let (len, capacity) = (self.data.len(), self.data.capacity());
        for _ in len..capacity {
            self.data.push(fill_with());
        }
        self.into()
    }
}
