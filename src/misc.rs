use num::Num;
use signed::Signed;
use approx_eq::ApproxEq;


#[inline]
pub fn inverse<'a, 'b, T: Signed>(out: &'a mut [T; 2], a: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = -a[0];
    out[1] = -a[1];
    out
}
#[test]
fn test_inverse() {
    let mut v = [0, 0];
    inverse(&mut v, &[1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == -1);
}

#[inline]
pub fn lerp<'a, 'b, T: Num, N: Num>(out: &'a mut [T; 2], a: &'b [T; 2], b: &'b [T; 2], t: N) -> &'a mut [T; 2] {
    let t_f64 = t.to_f64();
    out[0] = T::from_f64((a[0] + (b[0] - a[0])).to_f64() * t_f64);
    out[1] = T::from_f64((a[1] + (b[1] - a[1])).to_f64() * t_f64);
    out
}
#[test]
fn test_lerp() {
    let mut v = [0, 0];
    lerp(&mut v, &[0, 0], &[2, 2], 0.5);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 1);
}

#[inline]
pub fn perp_l<'a, 'b, T: Signed>(out: &'a mut [T; 2], a: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = a[1];
    out[1] = -a[0];
    out
}
#[test]
fn test_perp_l() {
    let mut v = [0, 0];
    perp_l(&mut v, &[1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == -1);
}

#[inline]
pub fn perp_r<'a, 'b, T: Signed>(out: &'a mut [T; 2], a: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = -a[1];
    out[1] = a[0];
    out
}
#[test]
fn test_perp_r() {
    let mut v = [0, 0];
    perp_r(&mut v, &[1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == 1);
}

#[inline]
pub fn min<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], b: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = if b[0] < a[0] {b[0]} else {a[0]};
    out[1] = if b[1] < a[1] {b[1]} else {a[1]};
    out
}
#[test]
fn test_min() {
    let mut v = [0, 0];
    min(&mut v, &[1, 0], &[0, 1]);
    assert!(v == [0, 0]);
}

#[inline]
pub fn max<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], b: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = if b[0] > a[0] {b[0]} else {a[0]};
    out[1] = if b[1] > a[1] {b[1]} else {a[1]};
    out
}
#[test]
fn test_max() {
    let mut v = [0, 0];
    max(&mut v, &[1, 0], &[0, 1]);
    assert!(v == [1, 1]);
}

#[inline]
pub fn clamp<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], min: &'b [T; 2], max: &'b [T; 2]) -> &'a mut [T; 2] {
    out[0] = if a[0] < min[0] {min[0]} else if a[0] > max[0] {max[0]} else {a[0]};
    out[1] = if a[1] < min[1] {min[1]} else if a[1] > max[1] {max[1]} else {a[1]};
    out
}
#[test]
fn test_clamp() {
    let mut v = [0, 0];
    clamp(&mut v, &[2, 2], &[0, 0], &[1, 1]);
    assert!(v == [1, 1]);
}


#[inline]
pub fn eq<'a, T: Num + ApproxEq>(a: &'a [T; 2], b: &'a [T; 2]) -> bool {
    !ne(a, b)
}
#[test]
fn test_eq() {
    assert_eq!(eq(&[1f32, 1f32], &[1f32, 1f32]), true);
    assert_eq!(eq(&[0f32, 0f32], &[1f32, 1f32]), false);
}

#[inline]
pub fn ne<'a, T: Num + ApproxEq>(a: &'a [T; 2], b: &'a [T; 2]) -> bool {
    !a[0].approx_eq(b[0]) ||
    !a[1].approx_eq(b[1])
}
#[test]
fn test_ne() {
    assert_eq!(ne(&[1f32, 1f32], &[1f32, 1f32]), false);
    assert_eq!(ne(&[0f32, 0f32], &[1f32, 1f32]), true);
}
