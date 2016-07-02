use num::Num;


#[inline(always)]
pub fn new<T: Num>(x: T, y: T) -> [T; 2] {[x, y]}
#[inline(always)]
pub fn create<T: Num>(x: T, y: T) -> [T; 2] {new(x, y)}
#[test]
fn test_new() {
    let v = new(1, 2);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
}

#[inline(always)]
pub fn clone<'b, T: Num>(v: &'b [T; 2]) -> [T; 2] {new(v[0], v[1])}

#[inline(always)]
pub fn copy<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = a[0];
    out[1] = a[1];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0];
    copy(&mut v, &[1, 2]);
    assert!(v == [1, 2]);
}
