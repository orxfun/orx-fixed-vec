use orx_fixed_vec::*;

#[test]
fn as_imp_vec_keeps_existing_references_valid() {
    let mut vec = FixedVec::new(8);
    vec.push(10);
    vec.push(20);

    let imp = vec.as_imp_vec();
    let first = &imp[0];

    imp.imp_push(30);
    imp.imp_extend_from_slice(&[40, 50]);

    assert_eq!(*first, 10);
    assert_eq!(imp.len(), 5);
    assert_eq!(imp.as_slice(), &[10, 20, 30, 40, 50]);
    assert_eq!(imp[2], 30);
}

#[test]
fn into_imp_vec_moves_vector_and_allows_immutable_push() {
    let mut vec = FixedVec::new(6);
    vec.push(1);
    vec.push(2);

    let imp = vec.into_imp_vec();
    let first = &imp[0];
    imp.imp_push(3);
    imp.imp_extend_from_slice(&[4]);

    assert_eq!(*first, 1);
    assert_eq!(imp.len(), 4);
    assert_eq!(imp.as_slice(), &[1, 2, 3, 4]);

    let recovered = imp.into_inner();
    assert_eq!(recovered.as_slice(), &[1, 2, 3, 4]);
}
