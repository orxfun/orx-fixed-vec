use alloc::vec::Vec;
use core::ops::Range;

pub struct ConcurrentFixedVecIntoIter<T> {
    data: Vec<T>,
    begin: *mut T,
    current: usize,
    end_exclusive: usize,
}

impl<T> ConcurrentFixedVecIntoIter<T> {
    pub(super) fn new(mut data: Vec<T>, range: Range<usize>) -> Self {
        // SAFETY: data contains items to be dropped within range
        // remaining positions are not initialized or already moved out
        unsafe { data.set_len(0) };
        let (current, end_exclusive) = (range.start, range.end);
        let begin = data.as_mut_ptr();
        Self {
            data,
            begin,
            current,
            end_exclusive,
        }
    }
}

impl<T> Drop for ConcurrentFixedVecIntoIter<T> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() {
            for i in self.current..self.end_exclusive {
                // SAFETY: begin + i is in bounds
                let ptr = unsafe { self.begin.add(i) };
                unsafe { ptr.drop_in_place() };
            }
        }
    }
}

impl<T> Iterator for ConcurrentFixedVecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current < self.end_exclusive {
            true => {
                // SAFETY: begin + current is in bounds
                let ptr = unsafe { self.begin.add(self.current) };
                self.current += 1;
                Some(unsafe { ptr.read() })
            }
            false => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end_exclusive - self.current;
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for ConcurrentFixedVecIntoIter<T> {
    fn len(&self) -> usize {
        self.end_exclusive - self.current
    }
}
