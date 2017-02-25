use num::Num;


#[inline(always)]
pub fn sub<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], b: &'b [T; 2]) ->  &'a mut [T; 2] {
    out[0] = a[0] - b[0];
    out[1] = a[1] - b[1];
    out
}
#[test]
fn test_sub() {
    let mut v = [0, 0];
    sub(&mut v, &[1, 1], &[1, 1]);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
}

#[inline(always)]
pub fn ssub<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], s: T) ->  &'a mut [T; 2] {
    out[0] = a[0] - s;
    out[1] = a[1] - s;
    out
}
#[test]
fn test_ssub() {
    let mut v = [0, 0];
    ssub(&mut v, &[1, 1], 1);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
}
