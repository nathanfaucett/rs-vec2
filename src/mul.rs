use num::Num;


#[inline(always)]
pub fn mul<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2], b: &'a [T; 2]) ->  &'a mut [T; 2] {
    out[0] = a[0] * b[0];
    out[1] = a[1] * b[1];
    out
}
#[test]
fn test_mul() {
    let mut v = [0, 0];
    mul(&mut v, &[1, 1], &[1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
}

#[inline(always)]
pub fn smul<'a, T: Num>(out: &'a mut [T; 2], a: &'a [T; 2], s: T) ->  &'a mut [T; 2] {
    out[0] = a[0] * s;
    out[1] = a[1] * s;
    out
}
#[test]
fn test_smul() {
    let mut v = [0, 0];
    smul(&mut v, &[1, 1], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
}
