use crate::FixedVec;
use orx_pinned_vec::PinnedVec;
use std::fmt::Debug;

impl<T> Debug for FixedVec<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as PinnedVec<T>>::debug(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn debug() {
        let mut vec = FixedVec::new(4);
        for i in 0..vec.capacity() {
            vec.push(i);
        }

        let debug_str = format!("{:?}", vec);
        assert_eq!("FixedVec [0, 1, 2, 3]\n", debug_str);
    }
}
