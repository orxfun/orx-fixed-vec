use alloc::vec::Vec;

pub struct ConcurrentFixedVecIntoIter<T> {
    data: Vec<T>,
    ptr: *const T,
    current_capacity: usize,
}
