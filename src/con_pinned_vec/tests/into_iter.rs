use crate::con_pinned_vec::into_iter::ConcurrentFixedVecIntoIter;

#[test]
fn into_iter_empty() {
    let iter = || {
        let data: Vec<_> = (0..0).map(|x| x.to_string()).collect();
        let range = 0..data.len();
        ConcurrentFixedVecIntoIter::new(data, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 0);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}

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

#[test]
fn into_iter_taken_from_beg() {
    let iter = || {
        let mut data: Vec<_> = (0..20).map(|x| x.to_string()).collect();
        let range = 5..data.len();

        let p = data.as_mut_ptr();
        for i in 0..range.start {
            let p = unsafe { p.add(i) };
            let _value = unsafe { p.read() };
        }

        ConcurrentFixedVecIntoIter::new(data, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 15);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}

#[test]
fn into_iter_taken_from_end() {
    let iter = || {
        let mut data: Vec<_> = (0..20).map(|x| x.to_string()).collect();
        let range = 0..15;

        let p = data.as_mut_ptr();
        for i in range.end..data.len() {
            let p = unsafe { p.add(i) };
            let _value = unsafe { p.read() };
        }

        ConcurrentFixedVecIntoIter::new(data, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 15);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}

#[test]
fn into_iter_taken_from_both_ends() {
    let iter = || {
        let mut data: Vec<_> = (0..20).map(|x| x.to_string()).collect();
        let range = 4..15;

        let p = data.as_mut_ptr();
        for i in (0..range.start).chain(range.end..data.len()) {
            let p = unsafe { p.add(i) };
            let _value = unsafe { p.read() };
        }

        ConcurrentFixedVecIntoIter::new(data, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 11);

    let mut consume_half = iter();
    for _ in 0..7 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}
