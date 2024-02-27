use crate::FixedVec;
use std::fmt::Debug;

impl<T> Debug for FixedVec<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixedVec")
            .field("data", &self.data)
            .finish()
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
        assert_eq!("FixedVec { data: [0, 1, 2, 3] }", debug_str);
    }
}
