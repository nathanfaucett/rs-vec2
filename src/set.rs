use num::Num;


#[inline(always)]
pub fn set<T: Num>(out: &mut [T; 2], x: T, y: T) -> &mut [T; 2] {
    out[0] = x;
    out[1] = y;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0];
    set(&mut v, 1, 2);
    assert!(v == [1, 2]);
}
