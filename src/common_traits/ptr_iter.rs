pub struct FixedVecPtrIter<T> {
    ptr: *mut T,
    len: usize,
    current: usize,
}

impl<T> FixedVecPtrIter<T> {
    pub(crate) fn new(ptr: *mut T, len: usize) -> Self {
        Self {
            ptr,
            len,
            current: 0,
        }
    }
}

impl<T> Iterator for FixedVecPtrIter<T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current < self.len {
            true => {
                // SAFETY: current is within bounds of the vector
                let ptr = unsafe { self.ptr.add(self.current) };
                self.current += 1;
                Some(ptr)
            }
            false => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len - self.current;
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for FixedVecPtrIter<T> {
    fn len(&self) -> usize {
        self.len - self.current
    }
}
