use crate::{
    helpers::range::{range_end, range_start},
    FixedVec,
};
use orx_pinned_vec::{ConcurrentPinnedVec, PinnedVecGrowthError};
use std::{cmp::Ordering, fmt::Debug};

/// Concurrent wrapper ([`orx_pinned_vec::ConcurrentPinnedVec`]) for the `FixedVec`.
pub struct ConcurrentFixedVec<T> {
    data: Vec<T>,
    ptr: *mut T,
    fixed_capacity: usize,
}

impl<T> Debug for ConcurrentFixedVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConcurrentFixedVec")
            .field("fixed_capacity", &self.fixed_capacity)
            .finish()
    }
}

impl<T> From<FixedVec<T>> for ConcurrentFixedVec<T> {
    fn from(value: FixedVec<T>) -> Self {
        let mut data = value.data;
        let fixed_capacity = data.capacity();
        unsafe { data.set_len(fixed_capacity) };
        let ptr = data.as_mut_ptr();
        Self {
            data,
            ptr,
            fixed_capacity,
        }
    }
}

impl<T> ConcurrentPinnedVec<T> for ConcurrentFixedVec<T> {
    type P = FixedVec<T>;

    unsafe fn into_inner(mut self, len: usize) -> Self::P {
        unsafe { self.data.set_len(len) };
        self.data.into()
    }

    fn capacity(&self) -> usize {
        self.fixed_capacity
    }

    fn max_capacity(&self) -> usize {
        self.fixed_capacity
    }

    fn grow_to(&self, new_capacity: usize) -> Result<usize, PinnedVecGrowthError> {
        match new_capacity <= self.fixed_capacity {
            true => Ok(self.fixed_capacity),
            false => Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned),
        }
    }

    fn grow_to_and_fill_with<F>(
        &self,
        new_capacity: usize,
        _: F,
    ) -> Result<usize, PinnedVecGrowthError>
    where
        F: Fn() -> T,
    {
        match new_capacity <= self.fixed_capacity {
            true => Ok(self.fixed_capacity),
            false => Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned),
        }
    }

    fn slices<R: std::ops::RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as orx_pinned_vec::PinnedVec<T>>::SliceIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.fixed_capacity);

        match b.saturating_sub(a) {
            0 => Some(&[]),
            _ => match (a.cmp(&self.fixed_capacity), b.cmp(&self.fixed_capacity)) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => {
                    let p = unsafe { self.ptr.add(a) };
                    let slice = unsafe { std::slice::from_raw_parts(p, b - a) };
                    Some(slice)
                }
            },
        }
    }

    unsafe fn slices_mut<R: std::ops::RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as orx_pinned_vec::PinnedVec<T>>::SliceMutIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.fixed_capacity);

        match b.saturating_sub(a) {
            0 => Some(&mut []),
            _ => match (a.cmp(&self.fixed_capacity), b.cmp(&self.fixed_capacity)) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => {
                    let p = self.ptr.add(a);
                    let slice = unsafe { std::slice::from_raw_parts_mut(p, b - a) };
                    Some(slice)
                }
            },
        }
    }

    unsafe fn iter<'a>(&'a self, len: usize) -> impl Iterator<Item = &'a T> + 'a
    where
        T: 'a,
    {
        let p = self.data.as_ptr();
        let slice = std::slice::from_raw_parts(p, len);
        slice.iter()
    }

    unsafe fn iter_mut<'a>(&'a mut self, len: usize) -> impl Iterator<Item = &'a mut T> + 'a
    where
        T: 'a,
    {
        let p = self.data.as_mut_ptr();
        let slice = std::slice::from_raw_parts_mut(p, len);
        slice.iter_mut()
    }

    unsafe fn set_pinned_vec_len(&mut self, len: usize) {
        self.data.set_len(len);
    }

    unsafe fn get(&self, index: usize) -> Option<&T> {
        match index < self.fixed_capacity {
            true => Some(&*self.ptr.add(index)),
            false => None,
        }
    }

    unsafe fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.capacity() {
            true => Some(&mut self.data[index]),
            false => None,
        }
    }

    unsafe fn get_ptr_mut(&self, index: usize) -> *mut T {
        assert!(index < self.fixed_capacity);
        self.ptr.add(index)
    }

    unsafe fn reserve_maximum_concurrent_capacity(
        &mut self,
        _: usize,
        new_maximum_capacity: usize,
    ) -> usize {
        let additional = new_maximum_capacity.saturating_sub(self.fixed_capacity);
        self.data.reserve(additional);
        self.fixed_capacity = self.data.capacity();
        self.data.set_len(self.fixed_capacity);
        self.fixed_capacity
    }

    unsafe fn clear(&mut self, prior_len: usize) {
        self.set_pinned_vec_len(prior_len);
        self.data.clear()
    }
}
