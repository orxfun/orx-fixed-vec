use crate::{ConcurrentFixedVec, FixedVec};
use orx_pinned_vec::IntoConcurrentPinnedVec;

impl<T> IntoConcurrentPinnedVec<T> for FixedVec<T> {
    type ConPinnedVec = ConcurrentFixedVec<T>;

    fn into_concurrent(self) -> Self::ConPinnedVec {
        self.into()
    }
}
