use num::Num;


#[inline(always)]
pub fn inverse<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2]) -> &'a mut [T; 2] {
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

#[inline(always)]
pub fn lerp<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2], b: &'a [T; 2], t: T) -> &'a mut [T; 2] {
    out[0] = a[0] + (b[0] - a[0]) * t;
    out[1] = a[1] + (b[1] - a[1]) * t;
    out
}
#[test]
fn test_lerp() {
    let mut v = [0.0, 0.0];
    lerp(&mut v, &[0.0, 0.0], &[1.0, 1.0], 0.5);
    assert!(v[0] == 0.5);
    assert!(v[1] == 0.5);
}

#[inline(always)]
pub fn perp<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2]) -> &'a mut [T; 2] {
    out[0] = a[1];
    out[1] = -a[0];
    out
}
#[test]
fn test_perp() {
    let mut v = [0, 0];
    perp(&mut v, &[1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == -1);
}

#[inline(always)]
pub fn perp_r<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2]) -> &'a mut [T; 2] {
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

#[inline(always)]
pub fn min<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2], b: &'a [T; 2]) -> &'a mut [T; 2] {
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

#[inline(always)]
pub fn max<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2], b: &'a [T; 2]) -> &'a mut [T; 2] {
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

#[inline(always)]
pub fn clamp<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2], min: &'a [T; 2], max: &'a [T; 2]) -> &'a mut [T; 2] {
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
