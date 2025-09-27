use crate::common_traits::ptr_iter::FixedVecPtrIter;

#[test]
fn ptr_iter_default() {
    let iter = FixedVecPtrIter::<String>::default();
    for _ in iter {}
}
