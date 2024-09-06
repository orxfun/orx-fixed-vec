use crate::{
    helpers::range::{range_end, range_start},
    FixedVec,
};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::Debug;
use orx_pinned_vec::{ConcurrentPinnedVec, PinnedVecGrowthError};

/// Concurrent wrapper ([`orx_pinned_vec::ConcurrentPinnedVec`]) for the `FixedVec`.
pub struct ConcurrentFixedVec<T> {
    data: Vec<T>,
    ptr: *const T,
    current_capacity: usize,
}

impl<T> Debug for ConcurrentFixedVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConcurrentFixedVec")
            .field("fixed_capacity", &self.current_capacity)
            .finish()
    }
}

impl<T> From<FixedVec<T>> for ConcurrentFixedVec<T> {
    fn from(value: FixedVec<T>) -> Self {
        let mut data = value.data;
        let current_capacity = data.capacity();
        unsafe { data.set_len(current_capacity) };
        let ptr = data.as_mut_ptr();
        Self {
            data,
            ptr,
            current_capacity,
        }
    }
}

impl<T> ConcurrentPinnedVec<T> for ConcurrentFixedVec<T> {
    type P = FixedVec<T>;

    unsafe fn into_inner(mut self, len: usize) -> Self::P {
        unsafe { self.data.set_len(len) };
        self.data.into()
    }

    unsafe fn clone_with_len(&self, len: usize) -> Self
    where
        T: Clone,
    {
        assert!(len <= self.capacity());
        let mut clone = Vec::with_capacity(self.capacity());
        for i in 0..len {
            clone.push(self.data[i].clone());
        }
        FixedVec::from(clone).into()
    }

    fn capacity(&self) -> usize {
        self.current_capacity
    }

    fn max_capacity(&self) -> usize {
        self.current_capacity
    }

    fn grow_to(&self, new_capacity: usize) -> Result<usize, PinnedVecGrowthError> {
        match new_capacity <= self.capacity() {
            true => Ok(self.capacity()),
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
        match new_capacity <= self.capacity() {
            true => Ok(self.capacity()),
            false => Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned),
        }
    }

    fn fill_with<F>(&self, range: core::ops::Range<usize>, fill_with: F)
    where
        F: Fn() -> T,
    {
        for i in range {
            unsafe { self.get_ptr_mut(i).write(fill_with()) };
        }
    }

    fn slices<R: core::ops::RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as orx_pinned_vec::PinnedVec<T>>::SliceIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.capacity());

        match b.saturating_sub(a) {
            0 => Some(&[]),
            _ => match (a.cmp(&self.capacity()), b.cmp(&self.capacity())) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => {
                    let p = unsafe { self.ptr.add(a) };
                    let slice = unsafe { core::slice::from_raw_parts(p, b - a) };
                    Some(slice)
                }
            },
        }
    }

    unsafe fn slices_mut<R: core::ops::RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as orx_pinned_vec::PinnedVec<T>>::SliceMutIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.capacity());

        match b.saturating_sub(a) {
            0 => Some(&mut []),
            _ => match (a.cmp(&self.capacity()), b.cmp(&self.capacity())) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => {
                    let p = self.ptr.add(a);
                    let slice = unsafe { core::slice::from_raw_parts_mut(p as *mut T, b - a) };
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
        let slice = core::slice::from_raw_parts(p, len);
        slice.iter()
    }

    unsafe fn iter_mut<'a>(&'a mut self, len: usize) -> impl Iterator<Item = &'a mut T> + 'a
    where
        T: 'a,
    {
        let p = self.data.as_mut_ptr();
        let slice = core::slice::from_raw_parts_mut(p, len);
        slice.iter_mut()
    }

    unsafe fn set_pinned_vec_len(&mut self, len: usize) {
        self.data.set_len(len);
    }

    unsafe fn get(&self, index: usize) -> Option<&T> {
        match index < self.capacity() {
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
        assert!(index < self.capacity());
        self.ptr.add(index) as *mut T
    }

    unsafe fn reserve_maximum_concurrent_capacity(
        &mut self,
        _: usize,
        new_maximum_capacity: usize,
    ) -> usize {
        let additional = new_maximum_capacity.saturating_sub(self.capacity());
        self.data.reserve(additional);

        let new_capacity = self.data.capacity();
        self.current_capacity = new_capacity;

        new_capacity
    }

    unsafe fn reserve_maximum_concurrent_capacity_fill_with<F>(
        &mut self,
        current_len: usize,
        new_maximum_capacity: usize,
        fill_with: F,
    ) -> usize
    where
        F: Fn() -> T,
    {
        let additional = new_maximum_capacity.saturating_sub(self.capacity());
        self.data.reserve(additional);

        self.current_capacity = self.data.capacity();

        self.data.set_len(current_len);

        for _ in current_len..self.current_capacity {
            self.data.push(fill_with());
        }

        self.current_capacity
    }

    unsafe fn clear(&mut self, prior_len: usize) {
        self.set_pinned_vec_len(prior_len);
        self.data.clear()
    }
}
