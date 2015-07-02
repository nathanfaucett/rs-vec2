use num::Num;


#[inline(always)]
pub fn inverse<T: Num>(out: &mut [T; 2], a: [T; 2]) -> &mut [T; 2] {
    out[0] = a[0] * -T::one();
    out[1] = a[1] * -T::one();
    out
}
#[test]
fn test_inverse() {
    let mut v = [0, 0];
    inverse(&mut v, [1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == -1);
}

#[inline(always)]
pub fn lerp<T: Num>(out: &mut [T; 2], a: [T; 2], b: [T; 2], t: T) -> &mut [T; 2] {
    out[0] = a[0] + (b[0] - a[0]) * t;
    out[1] = a[1] + (b[1] - a[1]) * t;
    out
}
#[test]
fn test_lerp() {
    let mut v = [0.0, 0.0];
    lerp(&mut v, [0.0, 0.0], [1.0, 1.0], 0.5);
    assert!(v[0] == 0.5);
    assert!(v[1] == 0.5);
}

#[inline(always)]
pub fn perp<T: Num>(out: &mut [T; 2], a: [T; 2]) -> &mut [T; 2] {
    out[0] = a[1];
    out[1] = -a[0];
    out
}
#[test]
fn test_perp() {
    let mut v = [0, 0];
    perp(&mut v, [1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == -1);
}

#[inline(always)]
pub fn perp_r<T: Num>(out: &mut [T; 2], a: [T; 2]) -> &mut [T; 2] {
    out[0] = -a[1];
    out[1] = a[0];
    out
}
#[test]
fn test_perp_r() {
    let mut v = [0, 0];
    perp_r(&mut v, [1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == 1);
}

#[inline(always)]
pub fn min<T: Num>(out: &mut [T; 2], a: [T; 2], b: [T; 2]) -> &mut [T; 2] {
    out[0] = if b[0] < a[0] {b[0]} else {a[0]};
    out[1] = if b[1] < a[1] {b[1]} else {a[1]};
    out
}
#[test]
fn test_min() {
    let mut v = [0, 0];
    min(&mut v, [1, 0], [0, 1]);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
}

#[inline(always)]
pub fn max<T: Num>(out: &mut [T; 2], a: [T; 2], b: [T; 2]) -> &mut [T; 2] {
    out[0] = if b[0] > a[0] {b[0]} else {a[0]};
    out[1] = if b[1] > a[1] {b[1]} else {a[1]};
    out
}
#[test]
fn test_max() {
    let mut v = [0, 0];
    max(&mut v, [1, 0], [0, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
}
