use num::Num;


#[inline]
pub fn div<'a, 'b, T: Copy + Num>(out: &'a mut [T; 2], a: &'b [T; 2], b: &'b [T; 2]) ->  &'a mut [T; 2] {
    out[0] = if b[0] != T::zero() {a[0] / b[0]} else {T::zero()};
    out[1] = if b[1] != T::zero() {a[1] / b[1]} else {T::zero()};
    out
}
#[test]
fn test_div() {
    let mut v = [0, 0];
    div(&mut v, &[1, 1], &[1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
}

#[inline]
pub fn sdiv<'a, 'b, T: Copy + Num>(out: &'a mut [T; 2], a: &'b [T; 2], s: T) ->  &'a mut [T; 2] {
    let not_zero = s != T::zero();
    out[0] = if not_zero {a[0] / s} else  {T::zero()};
    out[1] = if not_zero {a[1] / s} else  {T::zero()};
    out
}
#[test]
fn test_sdiv() {
    let mut v = [0, 0];
    sdiv(&mut v, &[1, 1], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
}
