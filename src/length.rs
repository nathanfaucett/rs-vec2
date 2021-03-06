use num::Num;
use div::sdiv;


#[inline]
pub fn length_values_sq<'a, 'b, T: Copy + Num>(x: T, y: T) -> T {
    x * x + y * y
}
#[test]
fn test_length_values_sq() {
    assert!(length_values_sq(1, 1) == 2);
}

#[inline]
pub fn length_values<'a, 'b, T: Copy + Num>(x: T, y: T) -> T {
    let lsq = length_values_sq(x, y);
    if lsq == T::zero() {lsq} else {lsq.sqrt()}
}
#[test]
fn test_length_values() {
    assert!(length_values(3, 4) == 5);
}

#[inline]
pub fn inv_length_values<'a, 'b, T: Copy + Num>(x: T, y: T) -> T {
    let lsq = length_values_sq(x, y);
    if lsq == T::zero() {lsq} else {T::one() / lsq.sqrt()}
}
#[test]
fn test_inv_length_values() {
    assert!(inv_length_values(3.0, 4.0) == 1.0 / 5.0);
}

#[inline]
pub fn dot<'a, 'b, T: Copy + Num>(a: &'b [T; 2], b: &'b [T; 2]) -> T {
    a[0] * b[0] + a[1] * b[1]
}
#[test]
fn test_dot() {
    assert!(dot(&[1, 1], &[1, 1]) == 2);
}

#[inline]
pub fn cross<'a, 'b, T: Copy + Num>(a: &'b [T; 2], b: &'b [T; 2]) -> T {
    a[0] * b[1] - a[1] * b[0]
}
#[test]
fn test_cross() {
    assert!(cross(&[1, 1], &[1, 1]) == 0);
}

#[inline]
pub fn length<'a, 'b, T: Copy + Num>(out: &'b [T; 2]) -> T {
    length_values(out[0], out[1])
}
#[test]
fn test_length() {
    assert!(length(&[3, 4]) == 5);
}

#[inline]
pub fn normalize<'a, 'b, T: Copy + Num>(out: &'a mut [T; 2], a: &'b [T; 2]) -> &'a mut [T; 2] {
    sdiv(out, a, length(a))
}
#[test]
fn test_normalize() {
    let mut v = [0, 0];
    normalize(&mut v, &[0, 1]);
    assert!(v[0] == 0);
    assert!(v[1] == 1);
}
