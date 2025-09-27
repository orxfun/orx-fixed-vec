use crate::con_pinned_vec::into_iter::ConcurrentFixedVecIntoIter;

#[test]
fn into_iter_non_taken() {
    let iter = || {
        let data: Vec<_> = (0..20).map(|x| x.to_string()).collect();
        let range = 0..data.len();
        ConcurrentFixedVecIntoIter::new(data, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 20);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}
