use macros::myVec;

#[test]
fn test_my_vec() {
    let vec = myVec![1, 2, 3];
    assert_eq!(vec, vec![1, 2, 3]);
}