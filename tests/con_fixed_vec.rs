use orx_fixed_vec::*;

#[test]
fn reserve() {
    let vec = FixedVec::<String>::new(42);

    let mut con_vec = vec.into_concurrent();

    unsafe { con_vec.get_ptr_mut(0).write("first".to_string()) };

    assert_eq!(con_vec.capacity(), 42);
    assert_eq!(con_vec.max_capacity(), 42);

    unsafe { con_vec.reserve_maximum_concurrent_capacity(0, 74) };
    let new_capacity = con_vec.capacity();
    assert!(new_capacity >= 74);
    assert_eq!(con_vec.max_capacity(), new_capacity);

    let vec = unsafe { con_vec.into_inner(1) };

    assert_eq!(vec.len(), 1);
    assert_eq!(vec.capacity(), new_capacity);
    assert_eq!(&vec[0], &"first".to_string());
}

#[test]
fn into_concurrent_fill_with() {
    let vec = FixedVec::<String>::new(42);
    let con_vec = vec.into_concurrent_filled_with(|| "x".to_string());
    let vec = unsafe { con_vec.into_inner(42) };
    assert_eq!(vec, (0..42).map(|_| "x".to_string()).collect::<Vec<_>>());

    let mut vec = FixedVec::<String>::new(42);
    vec.push("y".to_string());
    let con_vec = vec.into_concurrent_filled_with(|| "x".to_string());
    let vec = unsafe { con_vec.into_inner(42) };
    assert_eq!(&vec[0], &"y".to_string());
    assert_eq!(
        vec[1..],
        (1..42).map(|_| "x".to_string()).collect::<Vec<_>>()
    );
}

#[test]
fn reserve_fill_with() {
    let vec = FixedVec::<String>::new(42);

    let mut con_vec = vec.into_concurrent_filled_with(|| "x".to_string());

    unsafe { con_vec.reserve_maximum_concurrent_capacity_fill_with(42, 74, || "y".to_string()) };
    let new_capacity = con_vec.capacity();
    assert!(new_capacity >= 74);
    assert_eq!(con_vec.max_capacity(), new_capacity);

    let vec = unsafe { con_vec.into_inner(new_capacity) };

    assert_eq!(
        vec[0..42],
        (0..42).map(|_| "x".to_string()).collect::<Vec<_>>()
    );
    assert_eq!(
        vec[42..],
        (42..new_capacity)
            .map(|_| "y".to_string())
            .collect::<Vec<_>>()
    );
}
