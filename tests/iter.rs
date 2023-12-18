use orx_fixed_vec::prelude::*;

#[test]
fn iter() {
    let mut vec = FixedVec::new(4);
    for i in 0..vec.capacity() {
        vec.push(i);
    }

    let vec_from_iter: Vec<_> = vec.iter().map(|x| x + 1).collect();
    assert_eq!(vec_from_iter, &[1, 2, 3, 4]);
}
