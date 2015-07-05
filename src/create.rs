use num::Num;


#[inline(always)]
pub fn create<T: Num>(x: T, y: T) -> [T; 2] {[x, y]}
#[test]
fn test_create() {
    let v = create(1, 2);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
}

#[inline(always)]
pub fn clone<T: Num>(v: [T; 2]) -> [T; 2] {create(v[0], v[1])}

#[inline(always)]
pub fn copy<T: Num>(out: &mut [T; 2], a: [T; 2]) -> &mut [T; 2] {
    out[0] = a[0];
    out[1] = a[1];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0];
    copy(&mut v, [1, 2]);
    assert!(v == [1, 2]);
}
